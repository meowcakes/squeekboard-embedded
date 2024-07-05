/* Copyright (C) 2022 Purism SPC
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

/*! Panel state management.
 *
 * This is effectively a mirror of the previous C code,
 * with an explicit state machine managing the panel size.
 *
 * It still relies on a callback from Wayland to accept the panel size,
 * which makes this code somewhat prone to mistakes.
 *
 * An alternative to the callback would be
 * to send a message all the way to `state::State`
 * every time the allocated size changes.
 * That would allow for a more holistic view
 * of interactions of different pieces of state.
 * 
 * However, `state::State` already has the potential to become a ball of mud,
 * tightly coupling different functionality and making it difficult to see independent units.
 * 
 * For this reason, I'm taking a light touch approach with the panel manager,
 * and moving it just a bit closer to `state::State`.
 * Hopefully ths still allows us to expose assumptions that were not stated yet
 * (e.g. can the output disappear between size request andallocation?).
 *
 * Tight coupling, e.g. a future one between presented hints and layout size,
 * will have to be taken into account later.
 */

use crate::logging;
use crate::outputs::OutputId;
use crate::util::c::Wrapped;


pub mod c {
    use super::*;
    use glib;
    use std::os::raw::c_void;

    use crate::outputs::c::WlOutput;

    /// struct panel_manager*
    #[repr(transparent)]
    #[derive(Clone, Copy)]
    pub struct PanelManager(*const c_void);
    
    extern "C" {
        #[allow(improper_ctypes)]
        pub fn panel_manager_request_widget(
            service: PanelManager,
            output: WlOutput,
            height: u32,
            // for callbacks
            panel: Wrapped<Manager>,
        );
        pub fn panel_manager_resize(service: PanelManager, height: u32);
        pub fn panel_manager_hide(service: PanelManager);
    }

    #[no_mangle]
    pub extern "C"
    fn squeek_panel_manager_configured(panel: Wrapped<Manager>, width: u32, height: u32) {
        // This is why this needs to be moved into state::State:
        // it's getting too coupled to glib.
        glib::idle_add_local(move || {
            let panel = panel.clone_ref();
            panel.borrow_mut().set_configured(Size{width, height});
            glib::ControlFlow::Break
        });
    }
}


/// Size in pixels that is aware of scaling
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct PixelSize {
    pub pixels: u32,
    pub scale_factor: u32,
}

fn div_ceil(a: u32, b: u32) -> u32 {
    // Given that it's for pixels on a screen, an overflow is unlikely.
    (a + b - 1) / b
}

impl PixelSize {
    pub fn as_scaled_floor(&self) -> u32 {
        self.pixels / self.scale_factor
    }

    pub fn as_scaled_ceiling(&self) -> u32 {
        div_ceil(self.pixels, self.scale_factor)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Size {
    width: u32,
    height: u32,
}

/// This state requests the Wayland layer shell protocol synchronization:
/// the application asks for some size,
/// and then receives a size that the compositor thought appropriate.
/// Stores raw values passed to Wayland, i.e. scaled dimensions.
#[derive(Clone, Debug, PartialEq)]
enum State {
    Hidden,
    SizeRequested {
        output: OutputId,
        height: u32,
        //width: u32,
    },
    SizeAllocated {
        output: OutputId,
        wanted_height: u32,
        allocated: Size,
    },
}

/// A command to send out to the next layer of processing.
/// Here, it's the C side of the panel.
#[derive(Debug, PartialEq)]
enum Update {
    Hide,
    Resize { height: u32 },
    RequestWidget {
        output: OutputId,
        height: u32,
    },
}

#[derive(Clone, PartialEq, Debug)]
pub enum Command {
    Show {
        output: OutputId,
        height: PixelSize,
    },
    Hide,
}

/// Tries to contain all the panel sizing duties.
pub struct Manager {
    panel: c::PanelManager,
    state: State,
    // This should be part of State, if it ever actually gets unhardcoded.
    // It's here because State doesn't need to become more complex
    // until this becomes properly used.
    debug: bool,
}

impl Manager {
    pub fn new(panel: c::PanelManager) -> Self {
        Self {
            panel,
            state: State::Hidden,
            debug: false,
        }
    }
    // TODO: mabe send the allocated size back to state::State,
    // to perform layout adjustments
    fn set_configured(&mut self, size: Size) {
        if self.debug {
            eprintln!("Panel received configure {:?}", &size);
        }

        self.state = self.state.clone().configure(size);

        if self.debug {
            eprintln!("Panel now {:?}", &self.state);
        }
    }

    pub fn update(mgr: Wrapped<Manager>, cmd: Command) {
        let copied = mgr.clone();

        let mgr = mgr.clone_ref();
        let mut mgr = mgr.borrow_mut();
        
        if mgr.debug {
            eprintln!("Panel received {:?}", &cmd);
        }
        
        let (state, updates) = mgr.state.clone().update(cmd);
        (*mgr).state = state;
        
        for update in &updates {
            unsafe {
                match update {
                    Update::Hide => c::panel_manager_hide(mgr.panel),
                    Update::Resize { height }
                        => c::panel_manager_resize(mgr.panel, *height),
                    Update::RequestWidget{output, height}
                        => c::panel_manager_request_widget(mgr.panel, output.0, *height, copied.clone()),
                }
            }
        }
        
        if mgr.debug {
            for update in &updates {
                eprintln!("Panel updates: {:?}", &update);
            }
            eprintln!("Panel is now {:?}", &(*mgr).state);
        }
    }
}

impl State {
    fn configure(self, size: Size) -> Self {
         match self {
            State::Hidden => {
                // This may happen if a hide is scheduled immediately after a show.
                log_print!(
                    logging::Level::Surprise,
                    "Panel has been configured, but no request is pending. Ignoring",
                );
                State::Hidden
            },
            State::SizeAllocated{output, wanted_height, ..} => {
                log_print!(
                    logging::Level::Surprise,
                    "Panel received new configuration without asking",
                );
                State::SizeAllocated{output, wanted_height, allocated: size}
            },
            State::SizeRequested{output, height} => State::SizeAllocated {
                output,
                wanted_height: height,
                allocated: size,
            },
        }
    }

    fn update(self, cmd: Command) -> (Self, Vec<Update>) {
        match (cmd, self) {
            (Command::Hide, State::Hidden) => (State::Hidden, Vec::new()),
            (Command::Hide, State::SizeAllocated{..}) => (
                State::Hidden, vec![Update::Hide],
            ),
            (Command::Hide, State::SizeRequested{..}) => (
                State::Hidden, vec![Update::Hide],
            ),
            (Command::Show{output, height}, State::Hidden) => {
                let height = height.as_scaled_ceiling();
                (
                    State::SizeRequested{output, height},
                    vec![Update::RequestWidget{ output, height }],
                )
            },
            (
                Command::Show{output, height},
                State::SizeRequested{output: req_output, height: req_height},
            ) => {
                let height = height.as_scaled_ceiling();
                if output == req_output && height == req_height {(
                    State::SizeRequested{output: req_output, height: req_height},
                    Vec::new(),
                )} else if output == req_output {(
                    // I'm not sure about that.
                    // This could cause a busy loop,
                    // when two requests are being processed at the same time:
                    // one message in the compositor to allocate size A,
                    // causing the state to update to height A'
                    // the other from the state wanting height B',
                    // causing the compositor to change size to B.
                    // So better cut this short here, despite artifacts.
                    
                    // Doing nothing means that Squeekboard will occasionally use the stale size (see test),
                    // so instead always listen to the higher layer and request a new size.
                    // If this causes problems, maybe count requests/configures, or track what was allocated in response to what request.
                    State::SizeRequested{output, height},
                    vec![Update::Resize { height }],
                )} else {(
                    // This looks weird, but should be safe.
                    // The stack seems to handle
                    // configure events on a dead surface.
                    State::SizeRequested{output, height},
                    vec![
                        Update::Hide,
                        Update::RequestWidget { output, height },
                    ],
                )}
            },
            (
                Command::Show{output, height},
                State::SizeAllocated{output: alloc_output, allocated, wanted_height},
            ) => {
                let height = height.as_scaled_ceiling();
                if output == alloc_output && height == wanted_height {(
                    State::SizeAllocated{output: alloc_output, wanted_height, allocated},
                    Vec::new(),
                )} else if output == alloc_output && height == allocated.height {(
                    State::SizeAllocated{output: alloc_output, wanted_height: height, allocated},
                    Vec::new(),
                )} else if output == alloc_output {(
                    // Should *all* other heights cause a resize?
                    // What about those between wanted and allocated?
                    State::SizeRequested{output, height},
                    vec![Update::Resize{height}],
                )} else {(
                    State::SizeRequested{output, height},
                    vec![
                        Update::Hide,
                        Update::RequestWidget{output, height},
                    ]
                   )}
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::outputs::c::WlOutput;
    
    #[test]
    fn resize_before_configured() {
        // allow to make typing fields easier
        #[allow(non_upper_case_globals)]
        const output: OutputId = OutputId(WlOutput::dummy());
        
        let state = State::Hidden;
        
        // Initial show
        let (state, cmds) = state.update(Command::Show {
            output,
            height: PixelSize { pixels: 100, scale_factor: 1 },
        });
        assert_eq!(
            cmds,
            vec![Update::RequestWidget { output, height: 100 }],
        );
        // layer shell requests a resize

        // but another show comes before first can be confirmed
        let (state, cmds) = dbg!(state).update(Command::Show {
            output,
            height: PixelSize { pixels: 50, scale_factor: 1 },
        });
        assert_eq!(
            cmds,
            vec![Update::Resize { height: 50 }],
            "{:?}",
            state,
        );
        // This is too many layers of indirection, but as long as layer shell is tied to gtk widgets, there's not much to be done.
        // The main issue is that as the outputs change, we acknowledge the wrong (maintained) size:
        /*
        [346947.774] wl_output@31.geometry(0, 0, 65, 130, 0, "<Unknown>", "<Unknown>", 3)
[346948.117] wl_output@17.geometry(0, 0, 65, 130, 0, "<Unknown>", "<Unknown>", 3)
[346948.198] zwlr_layer_surface_v1@41.configure(1709, 720, 210)
[346948.268]  -> zwlr_layer_surface_v1@41.ack_configure(1709)
        */
        // TODO: check if layer_shell allows not acknowledging a configure event, and which part of squeekboard is responsible for that
        // (there are no messages in between, so it's not PanelMgr; panel.c almost-unconditionally calls to Rust too; could it be layer-shell.c?).
        
        // event we want
        let good_state = state.clone().configure(Size { width: 50, height: 50 });
        assert_eq!(
            good_state,
            State::SizeAllocated {
                output,
                wanted_height: 50,
                allocated: Size { width: 50, height: 50 },
            },
        );
        
        // or stale event we do not want
        let state = state.configure(Size { width: 50, height: 100 });
        // followed by the good one
        let state = state.configure(Size { width: 50, height: 50 });
        assert_eq!(
            state,
            State::SizeAllocated {
                output,
                wanted_height: 50,
                allocated: Size { width: 50, height: 50 },
            },
        );
    }
}

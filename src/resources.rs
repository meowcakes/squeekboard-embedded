/*! Statically linked resources.
 * This could be done using GResource, but that would need additional work.
 */

// TODO: keep a list of what is a language layout,
// and what a convenience layout. "_wide" is not a layout,
// neither is "number"
/// List of builtin layouts
static KEYBOARDS: &[(&'static str, &'static str)] = &[
    // layouts: us must be left as first, as it is the,
    // fallback layout.
    ("us", include_str!("../data/keyboards/us.yaml")),
    ("us_wide", include_str!("../data/keyboards/us_wide.yaml")),

    // Language layouts: keep alphabetical.
    ("am", include_str!("../data/keyboards/am.yaml")),
    ("am_wide", include_str!("../data/keyboards/am_wide.yaml")),
    ("am+phonetic", include_str!("../data/keyboards/am+phonetic.yaml")),
    ("am+phonetic_wide", include_str!("../data/keyboards/am+phonetic_wide.yaml")),

    ("ara", include_str!("../data/keyboards/ara.yaml")),
    ("ara_wide", include_str!("../data/keyboards/ara_wide.yaml")),

    ("be", include_str!("../data/keyboards/be.yaml")),
    ("be_wide", include_str!("../data/keyboards/be_wide.yaml")),

    ("bg", include_str!("../data/keyboards/bg.yaml")),
    ("bg_wide", include_str!("../data/keyboards/bg_wide.yaml")),
    ("bg+phonetic", include_str!("../data/keyboards/bg+phonetic.yaml")),
    ("bg+phonetic_wide", include_str!("../data/keyboards/bg+phonetic_wide.yaml")),

    ("br", include_str!("../data/keyboards/br.yaml")),
    ("br_wide", include_str!("../data/keyboards/br_wide.yaml")),
    
    ("ca", include_str!("../data/keyboards/ca.yaml")),
    ("ca_wide", include_str!("../data/keyboards/ca_wide.yaml")),
    
    ("ch+fr", include_str!("../data/keyboards/ch+fr.yaml")),
    ("ch+fr_wide", include_str!("../data/keyboards/ch+fr_wide.yaml")),
    ("ch+de", include_str!("../data/keyboards/ch+de.yaml")),
    ("ch+de_wide", include_str!("../data/keyboards/ch+de_wide.yaml")),
    ("ch", include_str!("../data/keyboards/ch.yaml")),
    ("ch_wide", include_str!("../data/keyboards/ch_wide.yaml")),

    ("de", include_str!("../data/keyboards/de.yaml")),
    ("de_wide", include_str!("../data/keyboards/de_wide.yaml")),

    ("cz", include_str!("../data/keyboards/cz.yaml")),
    ("cz_wide", include_str!("../data/keyboards/cz_wide.yaml")),

    ("cz+qwerty", include_str!("../data/keyboards/cz+qwerty.yaml")),
    ("cz+qwerty_wide", include_str!("../data/keyboards/cz+qwerty_wide.yaml")),

    ("dk", include_str!("../data/keyboards/dk.yaml")),
    ("dk_wide", include_str!("../data/keyboards/dk_wide.yaml")),

    ("epo", include_str!("../data/keyboards/epo.yaml")),
    ("epo_wide", include_str!("../data/keyboards/epo_wide.yaml")),

    ("es", include_str!("../data/keyboards/es.yaml")),
    ("es_wide", include_str!("../data/keyboards/es_wide.yaml")),
    ("es+cat", include_str!("../data/keyboards/es+cat.yaml")),
    ("es+cat_wide", include_str!("../data/keyboards/es+cat_wide.yaml")),

    ("fi", include_str!("../data/keyboards/fi.yaml")),
    ("fi_wide", include_str!("../data/keyboards/fi_wide.yaml")),

    ("fr", include_str!("../data/keyboards/fr.yaml")),
    ("fr_wide", include_str!("../data/keyboards/fr_wide.yaml")),
    ("fr+bepo", include_str!("../data/keyboards/fr+bepo.yaml")),
    ("fr+bepo_wide", include_str!("../data/keyboards/fr+bepo_wide.yaml")),

    ("ge", include_str!("../data/keyboards/ge.yaml")),
    ("ge_wide", include_str!("../data/keyboards/ge_wide.yaml")),

    ("gr", include_str!("../data/keyboards/gr.yaml")),
    ("gr_wide", include_str!("../data/keyboards/gr_wide.yaml")),
    ("gr+polytonic", include_str!("../data/keyboards/gr+polytonic.yaml")),
    ("gr+polytonic_wide", include_str!("../data/keyboards/gr+polytonic_wide.yaml")),

    ("hu", include_str!("../data/keyboards/hu.yaml")),
    ("hu_wide", include_str!("../data/keyboards/hu_wide.yaml")),

    ("il", include_str!("../data/keyboards/il.yaml")),
    ("il_wide", include_str!("../data/keyboards/il_wide.yaml")),
    
    ("ir", include_str!("../data/keyboards/ir.yaml")),
    ("ir_wide", include_str!("../data/keyboards/ir_wide.yaml")),

    ("it", include_str!("../data/keyboards/it.yaml")),
    ("it_wide", include_str!("../data/keyboards/it_wide.yaml")),
    ("it+fur", include_str!("../data/keyboards/it+fur.yaml")),
    ("it+fur_wide", include_str!("../data/keyboards/it+fur_wide.yaml")),

    ("jp+kana", include_str!("../data/keyboards/jp+kana.yaml")),
    ("jp+kana_wide", include_str!("../data/keyboards/jp+kana_wide.yaml")),

    ("no", include_str!("../data/keyboards/no.yaml")),
    ("no_wide", include_str!("../data/keyboards/no_wide.yaml")),

    ("pl", include_str!("../data/keyboards/pl.yaml")),
    ("pl_wide", include_str!("../data/keyboards/pl_wide.yaml")),

    ("pt", include_str!("../data/keyboards/pt.yaml")),
    ("pt_wide", include_str!("../data/keyboards/pt_wide.yaml")),

    ("ro", include_str!("../data/keyboards/ro.yaml")),
    ("ro_wide", include_str!("../data/keyboards/ro_wide.yaml")),

    ("ru", include_str!("../data/keyboards/ru.yaml")),
    ("ru_wide", include_str!("../data/keyboards/ru_wide.yaml")),

    ("se", include_str!("../data/keyboards/se.yaml")),
    ("se_wide", include_str!("../data/keyboards/se_wide.yaml")),

    ("si", include_str!("../data/keyboards/si.yaml")),
    ("si_wide", include_str!("../data/keyboards/si_wide.yaml")),

    ("th", include_str!("../data/keyboards/th.yaml")),
    ("th_wide", include_str!("../data/keyboards/th_wide.yaml")),

    ("tr", include_str!("../data/keyboards/tr.yaml")),
    ("tr_wide", include_str!("../data/keyboards/tr_wide.yaml")),
    ("tr+f", include_str!("../data/keyboards/tr+f.yaml")),
    ("tr+f_wide", include_str!("../data/keyboards/tr+f_wide.yaml")),

    ("ua", include_str!("../data/keyboards/ua.yaml")),
    ("ua_wide", include_str!("../data/keyboards/ua_wide.yaml")),

    ("us+colemak", include_str!("../data/keyboards/us+colemak.yaml")),
    ("us+colemak_wide", include_str!("../data/keyboards/us+colemak_wide.yaml")),

    ("us+dvorak", include_str!("../data/keyboards/us+dvorak.yaml")),
    ("us+dvorak_wide", include_str!("../data/keyboards/us+dvorak_wide.yaml")),

    // Email
    ("email/us", include_str!("../data/keyboards/email/us.yaml")),
    ("email/us_wide", include_str!("../data/keyboards/email/us_wide.yaml")),

    // URL
    ("url/us", include_str!("../data/keyboards/url/us.yaml")),
    ("url/us_wide", include_str!("../data/keyboards/url/us_wide.yaml")),

    // Others
    ("number/us", include_str!("../data/keyboards/number/us.yaml")),
    ("number/us_wide", include_str!("../data/keyboards/number/us_wide.yaml")),
    ("pin/us", include_str!("../data/keyboards/pin/us.yaml")),
    ("pin/us_wide", include_str!("../data/keyboards/pin/us_wide.yaml")),

    // Terminal
    ("terminal/de", include_str!("../data/keyboards/terminal/de.yaml")),
    ("terminal/de_wide",   include_str!("../data/keyboards/terminal/de_wide.yaml")),

    ("terminal/es", include_str!("../data/keyboards/terminal/es.yaml")),
    ("terminal/es_wide",   include_str!("../data/keyboards/terminal/es_wide.yaml")),

    ("terminal/fr", include_str!("../data/keyboards/terminal/fr.yaml")),
    ("terminal/fr_wide", include_str!("../data/keyboards/terminal/fr_wide.yaml")),

    ("terminal/us", include_str!("../data/keyboards/terminal/us.yaml")),
    ("terminal/us_wide",   include_str!("../data/keyboards/terminal/us_wide.yaml")),
    ("terminal/us+dvorak", include_str!("../data/keyboards/terminal/us+dvorak.yaml")),
    ("terminal/us+dvorak_wide",   include_str!("../data/keyboards/terminal/us+dvorak_wide.yaml")),

    // Overlays
    ("emoji/us", include_str!("../data/keyboards/emoji/us.yaml")),
    ("emoji/us_wide", include_str!("../data/keyboards/emoji/us_wide.yaml")),
];

pub fn get_keyboard(needle: &str) -> Option<&'static str> {
    KEYBOARDS.iter().find(|(name, _)| *name == needle).map(|(_, layout)| *layout)
}

static OVERLAY_NAMES: &[&'static str] = &[
    "emoji",
    "terminal",
];

pub fn get_overlays() -> Vec<&'static str> {
    OVERLAY_NAMES.to_vec()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_overlays_present() {
        for name in get_overlays() {
            assert!(get_keyboard(&format!("{}/us", name)).is_some());
        }
    }
}

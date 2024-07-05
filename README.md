*squeekboard* - a Wayland on-screen keyboard
========================================

*Squeekboard* is the on-screen keyboard for Phosh. It is primarily designed for smartphones, tablet-PCs, and other devices with touchscreens.

It squeaks because some Rust got inside.

Features
--------

### Present

- GTK3
- Custom keyboard layouts defined in yaml
- Input purpose dependent keyboard layouts
- DBus interface to show and hide
- Use Wayland input method protocol to submit text
- Use Wayland virtual keyboard protocol

### TODO

- [Port to GTK4 / GTK4-Layer-Shell](https://gitlab.gnome.org/World/Phosh/squeekboard/-/issues/64)
- [Text prediction/correction](https://gitlab.gnome.org/World/Phosh/squeekboard/-/issues/54)
- Use preedit
- Submit actions like "next field" using a future Wayland protocol
- Pick up DBus interface files from /usr/share

Creating layouts
-------------------

If you want to work on layouts, check out the [guide](doc/tutorial.md).

Building
--------

### Dependencies

See `.gitlab-ci.yml` or run `apt-get build-dep .`

### Build from git repo

```sh
$ git clone https://gitlab.gnome.org/World/Phosh/squeekboard.git
$ cd squeekboard
$ mkdir _build
$ meson _build/
$ cd _build
$ ninja
```

To run tests use `ninja test`. To install squeekboard run `ninja install`.

Running
-------

```sh
$ phoc # if no compatible Wayland compositor is running yet
$ cd ../build/
$ src/squeekboard
```

Squeekboard's panel will appear whenever a compatible application requests an input method. Click a text field in any GTK application, like `python3 ./tools/entry.py`.

Squeekboard honors the gnome "screen-keyboard-enabled" setting. Either enable this through gnome-settings under accessibility or run:

```sh
$ gsettings set org.gnome.desktop.a11y.applications screen-keyboard-enabled true
```

Alternatively, force panel visibility manually with:

```sh
$ busctl call --user sm.puri.OSK0 /sm/puri/OSK0 sm.puri.OSK0 SetVisible b true
```

### What the compositor has to support

A compatible compositor has to support the protocols:

- layer-shell
- virtual-keyboard-v1

It's strongly recommended to support:

- input-method-v2

Developing
----------

See [`doc/hacking.md`](doc/hacking.md) for this copy, or the [official documentation](https://world.pages.gitlab.gnome.org/Phosh/squeekboard) for the current release.

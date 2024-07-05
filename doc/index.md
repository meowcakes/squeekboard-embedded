Welcome to squeekboard's documentation!
=======================================

Introduction
------------

*Squeekboard* is the on-screen keyboard for Phosh. It is primarily designed for smartphones, tablet-PCs, and other devices with touchscreens.

Building
--------

For information about building Squeekboard, read the [README](README.md).

Layouts
-------

Squeekboard can [load user-provided keyboard-layouts](layouts.md#using-custom-layouts). Those can be created and used without recompiling the keyboard code. The [tutorial](tutorial.md) explains the process in detail.

Layouts are created using a [text-based format, based on YAML](layouts.md).

### Views

Squeekboard layouts are separated into *views* and use a *room metaphor* to [switch views](views.md).

Contributions
-------------

Anyone is free to modify *squeekboard*. See the [contributing document](hacking.md).

### Code documentation

To expose the structure of Squeekboard in detail, there's a [code reference](https://world.pages.gitlab.gnome.org/Phosh/squeekboard/doc/rs/).

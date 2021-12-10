# Program for switching between windows on [swaywm](https://swaywm.org)

Sway doesn't support switching between recent windows out-of-box

This project is good solution, if you need this feature for using sway

## Non standard keys

You can choose whats keys will be used

supported flags:

- `--key-alt` - name of keyboard key with behavior of `Alt` key (by default
  `KEY_LEFTALT`)
- `--key-shift` - name of keyboard key with behavior of `Shift` key (by default
  `KEY_LEFTSHIFT`)
- `--key-tab` - name of key with behavior of `Tab` key (by default `KEY_TAB`)

All names of keyboard keys you can see in
[libevdev](https://www.freedesktop.org/software/libevdev/doc/1.1/kernel_header.html#:~:text=*-,Keys%20and%20buttons,-*)
library headers

Only one key set is supported at now

## Custom event devices

You can choose event device for processing from it

- `--device` - path to event device (by default detect automatically)

Only one event device at once is supported at now

> **_Note_**: You need read permissions to files in `/dev/input/` directory or
> you can set sticky bit to `sway-alttab` executable file (it's tries
> [`setgid`](https://man7.org/linux/man-pages/man2/setgid.2.html) at first and
> then [`setuid`](https://man7.org/linux/man-pages/man2/setuid.2.html) if it
> needed)

## Storing windows in order of last access

`sway-alttab` is remembering in what order you opening windows and switching
between them

## Preview Mode

When you press Alt-Tab `sway-alttab` doesn't consider this as access while Alt
is pressed. Multi pressing on Tab while Alt is pressed is switching between
windows without remembering that. Only after release Alt key `sway-alttab` will
move up current focused window to top of the list (other windows will be
untouched)

## Building

- Install
  [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- Clone project `git clone https://github.com/x1b6e6/sway-alttab`
- Run `cargo build` in cloned directory

## Similar projects

[sway-alttab](https://github.com/reisub0/sway-alttab)

# Program for switching between windows on [swaywm](https://swaywm.org)

## Building

- Install
  [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- Clone project `git clone https://github.com/x1b6e6/sway-alttab`
- Run `cargo build` in cloned directory

## Usage

```bash
sway-alttab --device <DEVICE> [--key-alt <KEY_ALT>] [--key-shift <KEY_SHIFT>] [--key-tab <KEY_TAB>]
```

- `<DEVICE>` - keyboard event device (`/dev/input/eventX` where `X` is number of
  device)
- `<KEY_ALT>` - name of keyboard key with behavior of `Alt` key (by default
  `KEY_LEFTALT`)
- `<KEY_SHIFT>` - name of keyboard key with behavior of `Shift` key (by default
  `KEY_LEFTSHIFT`)
- `<KEY_TAB>` - name of key with behavior of `Tab` key (by default `KEY_TAB`)

All names of keyboard keys you can see in
[libevdev](https://www.freedesktop.org/software/libevdev/doc/1.1/kernel_header.html#:~:text=*-,Keys%20and%20buttons,-*)
library headers

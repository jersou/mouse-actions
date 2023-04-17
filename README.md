# Mouse actions

![mouse_actions_logo.svg](mouse_actions_logo.svg)

mouse_actions allows to execute some commands from mouse events such as:

* clicks / wheel on the side / corners of the screen,
* or drawing shapes.

It's a mix between [Easystroke](https://github.com/thjaeger/easystroke)
and [Compiz edge
commands](http://wiki.compiz.org/CCSM#Mouse_Buttons).

For instance, you can configure:

* a click on the top left corner of the screen to go to the first desktop,
* a middle click on the top side of the screen to play/pause the media,
* scroll from the left side to increase/decrease the brightness of the
  screen,
* scroll from the top-left corner to increase/decrease the volume,
* draw a `T` with the mouse right button pressed to open a terminal,
* draw a `G` with the mouse right button pressed to open a text editor (gedit),

![mouse_actions_logo.gif](mouse_actions_logo.gif)

The GUI to configure the application :
![Mouse Action Configuration Editor](config-editor/mace.png)

## Features

Bind command execution with mouse button/wheel events (this conditions bellow
are optional):

* shape drawing with the mouse (like Easystroke)
* press/release only or click (don't propagate the press & release event)
* with some modifiers : shift/Ctrl/Alt...
* with screen edge : Top/Left...
* auto reload config on changes

## Project status

**⚠️ Alpha version ⚠️**

My feedback on Linux/X11 : after 10 month of daily use (since 15/05/2022) and
300'000 triggers, it's works well and X11 has not crashed (Unlike Easystroke
which made X11 crash every day before on my laptop).
With my usage, mouse_actions triggers commands about once/twice per minute, and
half of which by shape bindings.

## Known bugs

* when a device (like mouse or bluetooth earphone) is added, the mouse/keyboard
  modifier are locked : if Ctrl is pressed during this plug, Ctrl keep pressed.
  A workaround for me is to switch : Ctrl+Alt+F1 & Ctrl+Alt+F7.
* when a device (like mouse or bluetooth earphone ) is added, the cursor freeze
  while 2 seconds.

## Install / run

[Download the release](https://github.com/jersou/mouse-actions/releases), the 2
release binaries `mouse-actions` and `mouse-actions-gui` are standalone,
the avantage of GUI less version is the RAM usage : 6.6 Mo vs 34 Mo.

The gui unbundled release need this packages :

* Debian/Ubuntu : libwebkit2gtk-4.0-37, libgtk-3-0
* Arch : webkit2gtk, gtk3
* Fedora : webkit2gtk3, gtk3

### Build

* GUI less version : `cargo build --release`
* GUI version : `cd config-editor && npm i && npm run tauri-build`

### Requirement :

To use the main feature "grab event", you need to have the read&write permission
on `/dev/input/event*`. Check the group of `/dev/input/event*` files :

```bash
ls -al /dev/input/event*
# > crw-rw---- 1 root input /dev/input/event5
#                     ^^^^^
```

You need to add the current user to this group, usually `input` or `plugdev` :

```bash
sudo usermod -a -G plugdev $USER
# or
sudo usermod -a -G input $USER
```

Furthermore, you must have the read&write permission on `/dev/uinput`, you can
check with:

```bash
getfacl /dev/uinput
# ...
# user:<the current user>:rw-
# ...
```

If this permission is not available on the user, to add it
temporary : `sudo setfacl -m u:$USER:rw /dev/uinput` or persistent :

```bash
sudo tee /etc/udev/rules.d/80-mouse-actions.rules <<<'KERNEL=="uinput", SUBSYSTEM=="misc", TAG+="uaccess", OPTIONS+="static_node=uinput"'
```

You need to restart your desktop session to apply these changes.

To check the user groups and the ACL after the session restart or the reboot:

```bash
$ groups
... input ...
$ getfacl /dev/uinput
# ...
# user:<the current user>:rw-
# ...
```

### Platform compatibility

I only tested on Linux & X11, but it should work on Mac, Windows as well as
Linux+Wayland (with --no-listen option for Wayland).

The `grab` feature from rdev give an inaccurate mouse position, so I used
the `listen` feature from rdev. This function not works on Wayland, but the
mouse shape detection should work (with little modification of code), the listen
feature is used to detect edge of screen clik.

## Configuration

### config editor

Run `mouse-actions-gui` to init the configuration.

### Configuration file format

The config file default path `~/.config/mouse-actions.json`

#### Structure

* `shape_button`: the mouse button to use to draw shapes :
  `Left` | `Right` | `Middle` | `Side` | `Extra` | `Forward` | `Back`
  | `Task` | `Trigger` | `Thumb` | `Thumb2` | `WheelUp` | `WheelDown`
  | `Unknown` | `None`

* `bindings` : array of binding :
    * `cmd` : command line to exec, as array of string
    * `event`: object :
        * `button`: `Left` | `Right` | `Middle` | `Side` | `Extra` | `Forward`
          | `Back`
          | `Task` | `Trigger` | `Thumb` | `Thumb2` | `WheelUp` | `WheelDown`
          | `Unknown` | `None`
        * `edges`: array of : `Top`,`Right`,`Bottom`,`Left`,
        * `event_type`: `Press` | `Release`| `Click`
        * `modifiers`: array of :  `ShiftLeft`, `ShiftRight`, `ControlLeft`,
          `ControlRight`, `MetaLeft`, `Alt`, `AltGr`
        * `shapes_xy`: the shapes, array of arrays of coordinates. The best
          shape match will be used.

## CLI usage

```
Use RUST_LOG to set the log level : error, warn, info, debug, trace,
Example : RUST_BACKTRACE=1 RUST_LOG=debug ./mouse_actions

Usage: mouse_actions [OPTIONS] [COMMAND]

Commands:
  show-gui        Default command with mouse-actions-gui, show Mouse Actions Config Editor
  start           Default command with mouse-actions, Start mouse_actions bindings
  trace           Trace events
  record          Start record mode to add some mouse bindings
  list-bindings   List the current config bindings
  grab-one-event  Grab one event, print it and exit
  stop            Stop mouse action
  status          Get mouse action status : exit 0 if running
  show-config     print the json config
  set-config      set the json config from stdin
  help            Print this message or the help of the given subcommand(s)

Options:
  -n, --no-listen                  don't run the listen thread (for Wayland), the edge bindings might not work
  -c, --config-path <CONFIG_PATH>  config path, default : ~/.config/mouse-actions.json
  -v, --version                    print version
  -h, --help                       Print help
```

## Exemple : big config

* mouse button bindings:
    * Super+Left click → screenshot script
    * Side click → script: Alt + Left mouse down
    * Extra click → script: Alt + Tab

* edges and corners bindings:
    * Middle click in the top left corner → script: key ² → open Tilda
    * Middle click in the top right corner → script: lock the screen
    * Middle click in the top edge → script: play/pause
    * Right click in the top left corner → script: go to the top left desktop
    * Right click in the top right corner → script: go to the top right desktop
    * Right click in the bottom left corner → script: go to the bottom left
      desktop
    * Right click in the bottom right corner → script: go to the bottom right
      desktop
    * Wheel up in the top left corner → script: increase volume
    * Wheel up in the top right corner → script: increase volume
    * Wheel up in the bottom left corner → script: increase volume
    * Wheel up in the bottom right corner → script: increase volume
    * Wheel down in the top left corner → script: decrease volume
    * Wheel down in the top right corner → script: decrease volume
    * Wheel down in the bottom left corner → script: decrease volume
    * Wheel down in the bottom right corner → script: decrease volume
    * Ctrl + Wheel up in the top edge → script: audio next
    * Ctrl + Wheel up in the top edge → script: audio previous
    * Wheel up in the left edge → script: increase brightness 1%
    * Ctrl + Wheel up in the top edge → script: increase brightness 10%
    * Wheel down in the left edge → script: decrease brightness 1%
    * Ctrl + Wheel down in the left edge → script: decrease brightness 10%
    * Right click in the left edge → script: go to desktop on the left
    * Right click in the top edge → script: go to desktop on the top
    * Right click in the right edge → script: go to desktop on the right
    * Right click in the bottom edge → script: go to desktop on the bottom

* Shape biding with the right button :
    * Draw G shape → launch gedit (text editor)
    * Draw T shape → launch the terminal
    * Draw C shape → key Ctrl+C (Copy)
    * Draw V shape → key Ctrl+V (Paste)
    * Draw ↑ (vertical line to the top) shape → go to the desktop on the top
    * Draw ↓ (vertical line to the bottom) shape → go to the desktop on the top
    * Draw → (horizontal line to the right) shape → go to the desktop on the
      right
    * Draw ← (horizontal line to the left) shape → go to the desktop on the left
    * Draw N shape → open the Note tool
    * Draw ↗ (line to the top right) shape → F2 key (rename)
    * Draw ↖ (line to the top left) shape → F2 key (rename)
    * Draw ↙ (line to the left bottom) shape → Alt+Tab key
    * Draw n shape → launch nemo (file explorer)
    * Draw m shape → launch nautilus (file explorer)
    * Draw ↘ (line to the bottom right) shape → Alt+F8 key (resize the window)
    * Draw S shape → Ctrl+S key (save)
    * Draw ∝ (alpha) shape → Ctrl+X key (cut)
    * Draw ɣ (gamma) shape → Ctrl+X key (cut)
    * Draw ↵ (bottom then left) shape → Ctrl+X key (cut)
    * Draw ↶ (reverse n) shape → show/hide hamster time tracker
    * Draw Z shape → Ctrl+Z key (undo)
    * Draw F shape → Ctrl+F key (search)
    * Draw H shape → Ctrl+H key (toggle hide)
    * Draw D shape → Ctrl+Alt+D key (show the window on all desktops)
    * Draw B shape → script to remove the window decoration
    * Draw 2 shape → Shift+F9 key clear draw on screen (Gromit-MPX)
    * Draw 𝛥 shape (↗↘←) → F9 key toggle draw on screen (Gromit-MPX)

## Development

This project use [rdev crate](https://crates.io/crates/rdev) that
use [Evdev](https://en.wikipedia.org/wiki/Evdev) to grab mouse Event.

### Motivations

* I used [Easystroke](https://github.com/thjaeger/easystroke) a lot but its
  development stopped in 2014, and it causes my system to crash regularly.
* Besides, I was also using a lot Compiz screen corner command bindings, and I
  wanted to have these bindings without necessarily using compiz.

The goal of this project is then to have these 2 features without having
OS crash (X11 crash).

CCSM screenshot (Compiz Config Setting Manager) :
![ccsm.png](ccsm.png)

Easystoke screenshot :
![easystroke.png](easystroke.png)

### Dev notes : Shape recognition

Shape recognition : compare angles, get the average of the angles differences :

![shape-recognition.svg](shape-recognition.svg)

The calculated difference is approximately the area between the 2 curves of
angles (mod 2𝜋) visible on the right of the above image.

Get the minimum difference by shifting a curve horizontally: try removing the
beginning or the end, by +/- 10 % max offset (max 20 try).

### upgrade

```bash
cargo update
cargo audit
cargo test
cargo build --release
```

## TODO

### High

* fix rdev
    * fix rdev devices delete/update: the FIXME "inotify CREATE but not DELETE
      in grab::inotify_devices()" in rdev/src/linux/grab.rs:493
    * pull request/contribute/modify rdev without checkout it in this repo (
      mouse btn add & fix devices setup/Delete notify)
* reset the modifiers/button state at root loop restart
* cancel shape if no move after few ms (400 ms ?)

### Medium

* Shape event use also the screen edges/corners : adapt the filter or the GUI (
  bug or feature ?)
* create ~/.config if it doesn't exist
* fix exec cmd
  error `Err(Os { code: 2, kind: NotFound, message: "No such file or directory" })`
* check $XDG_SESSION_TYPE == "x11"/"wayland" to trace/enable --no-listen option
* process TODO and FIXME
* github actions : tests, build
* merge the config editor and the main binary : add config-gui subcommand ?

### Low

* use https://github.com/hoodie/notify-rust
* a better Readme
* improve shape recognition
* refactor
    * don't use arrayvec ?
    * reduce clone() usages
    * handle errors correctly : remove panic, reduce unwrap
    * refactor Arc/Mutex usages
    * refactor/change the pressState usage
    * dev doc, tests

### Maybe

* add "enable" in bindings
* add config file version
* release a debug version and a gui-less version ?
* options
    * dry-run option
    * min diff shape config option
    * min score shape config option
* find a better project name and icon
* support Wayland & Windows & macOS (get the mouse position on wayland
  is impossible ?)
* notif/sound/cursor change on action trigger success/failure (
  configurable) ? https://crates.io/crates/rodio
* mouse move edge event ?
* use rdev send() ? → cmd OR sendKeys in bindings (or autopilot-rs) :  trigger
  keyboard event as action (avoid xdotool usage in
  cmd) : https://github.com/Narsil/rdev#sending-some-events
* change config : if shape → no need button
* move shape_button from root config to bindings
* hide/freeze cursor while shape drawing ?
* Ctrl alias for ControlLeft & ControlRight, Shift for ShiftLeft & ShiftRight,

# Mouse actions

![mouse_actions_logo.svg](mouse_actions_logo.svg)

mouse_actions allows to execute some commands from mouse events such as
clicks/wheel on the side / corners of the screen, or drawing shapes. It's a mix
between [Easystroke](https://github.com/thjaeger/easystroke) and [Compiz edge
commands](http://wiki.compiz.org/CCSM#Mouse_Buttons).

You can click on the top left corder of the screen to go to the first desktop,
or scroll from the top corner to increase/decrease the brightness of the screen
or change the volume... Or draw a T with the mouse right button pressed to
open a terminal, ...

## Features

Bind command execution with mouse button/wheel events (this conditions bellow
are optional):

* shape drawing with the mouse (like Easystroke)
* Press/Release only or click (don't propagate the press & release event)
* with some modifiers : shift/Ctrl/...
* with screen edge : Top/Left ...
* auto reload config on changes

## Project status

**/!\ Alpha version !**

It's works (tested on Linux/X11) but there is no GUI to configure the bindings
for now, you add to write the json config yourself or
use `mouse_actions record`.

I have been using mouse_actions for several days (since 15/05/2022) and X11 has
not crashed (Unlike Easystroke which made X11 crash every day before on my
laptop).

With my usage, mouse_actions triggers commands about once/twice per minute,
and half of which by shape bindings.

My feedback : after 10 month of daily use and 300'000 triggers,
it's works well.

### Known bugs

* when a device (like mouse or bluetooth earphone ) is added, the mouse/keyboard
  modifier are locked : if Ctrl is pressed during this plug, Ctrl keep pressed.
  A workaround for me is to switch : Ctrl+Alt+F1 & Ctrl+Alt+F7.
* when a device (like mouse or bluetooth earphone ) is added, the cursor freeze
  2 seconds.

## Install

Download the release, or with Cargo, run directly :

```bash
cargo run
```

or build the binary:

```bash
cargo build --release
```

### Requirement :

To use the main feature "grab event", you need to add the current user
to `input` & `plugdev` group :

```
sudo usermod -a -G plugdev $USER
sudo usermod -a -G input $USER
```

You need to restart your desktop session to apply this user changes.

To check the user groups:

```bash
$ groups
... plugdev ... input ...
```

### Config

Run `mouse_actions record` to init the configuration.

### Configuration

The config file path `~/.config/mouse-actions.json`

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

#### Get shape values

To get a shape angles values, run :

```
mouse_actions trace
```

#### Exemple

```json
{
  "comment": "Extra click → script: Alt + Tab",
  "shape_button": "Right",
  "bindings": [
    {
      "cmd": [
        "xdotool",
        "key",
        "Alt+Tab"
      ],
      "event": {
        "button": "Extra",
        "event_type": "Click"
      }
    },
    {
      "comment": "Right click in the top right corner → script: go to the top right desktop",
      "event": {
        "button": "Right",
        "edges": [
          "Right",
          "Top"
        ],
        "event_type": "Click"
      },
      "cmd": [
        "wmctrl",
        "-s",
        "1"
      ]
    },
    {
      "comment": "Draw T shape with the right button → launch the terminal",
      "event": {
        "button": "Right",
        "edges": [],
        "modifiers": [],
        "event_type": "Click",
        "shapes_xy": [
          0,
          0,
          500,
          0,
          1000,
          0,
          500,
          0,
          500,
          1000
        ]
      },
      "cmd": [
        "gnome-terminal"
      ]
    }
  ]
}
```

## CLI usage

```
Use RUST_LOG to set the log level : error, warn, info, debug, trace,
Example : RUST_BACKTRACE=1 RUST_LOG=debug ./mouse_actions

Usage: mouse_actions [OPTIONS] [COMMAND]

Commands:
  start           Default command, use mouse_actions bindings
  open-config     Open the config file (xdg-open)
  trace           Trace events
  record          Start record mode to add some mouse bindings
  list-bindings   List the current config bindings
  grab-one-event  Grab one event, print it and exit
  stop            Stop mouse action
  status          Get mouse action status : exit 0 if running
  help            Print this message or the help of the given subcommand(s)

Options:
  -n, --no-listen                  don't run the listen thread (for Wayland), the edge bindings might not work
  -c, --config-path <CONFIG_PATH>  config path, default : ~/.config/mouse-actions.json
  -h, --help                       Print help
  -V, --version                    Print version

```

## Development

This project use [rdev crate](https://crates.io/crates/rdev) that
use [Evdev](https://en.wikipedia.org/wiki/Evdev) to grab mouse Event.

### Platform compatibility

I only tested on Linux & X11 but it should work on Mac, Windows as well as
Linux+Wayland (with --no-listen option for Wayland).

The `grab` function from rdev give an inaccurate mouse position, so I used
the `listen` function from rdev. This function not works on Wayland, but the
mouse shape detection should work (with little modification of code), the listen
function is used to detect edge of screen click.

## Motivations

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
    * Wheel down in the top edge → script: decrease brightness 1%
    * Ctrl + Wheel down in the top edge → script: decrease brightness 10%
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

## Dev notes

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

----

## TODO

### High

* fix #1: permission denied on Xubuntu 22.04 without sudo
* fix exec cmd error `Err(Os { code: 2, kind: NotFound, message: "No such file or directory" })`
* a release 0.3.0
* fix rdev
    * fix rdev devices delete/update: the FIXME "inotify CREATE but not DELETE
      in grab::inotify_devices()" in rdev/src/linux/grab.rs:493
    * pull request/contribute/modify rdev without checkout it in this repo (
      mouse btn add & fix devices setup/Delete notify)
* reset the modifiers/button state at root loop restart
* cancel shape if no move after few ms (400 ms ?)
* Add group check on Linux and error
  message `sudo usermod -a -G plugdev $USER && sudo usermod -a -G input $USER && restart session`
* add config file version
* create ~/.config if it doesn't exist
* check $XDG_SESSION_TYPE == "x11"/"wayland" to trace/enable --no-listen option

### Medium

* POC : config editor server with deno
* use https://github.com/hoodie/notify-rust
* don't use arrayvec ?
* process TODO and FIXME
* refactor
    * reduce clone() usages
    * handle errors correctly : remove panic, reduce unwrap
    * refactor Arc/Mutex usages
    * refactor/change the pressState usage
    * dev doc, tests

### Low

* a better Readme

### Maybe

* find a better project name and icon
* GUI (Tauri ?)
* support Wayland & Windows & macOS (get the mouse position on wayland
  impossible ?)
* notif/sound/cursor change on action trigger (configurable) ?
* mouse move edge event ?
* use rdev send() ? → cmd OR sendKeys in bindings (or autopilot-rs) :  trigger
  keyboard event as action (avoid xdotool usage in
  cmd) : https://github.com/Narsil/rdev#sending-some-events
* change config : if shape → no need button
* move shape_button from root config to bindings
* hide/freeze cursor while shape drawing ?
* Ctrl alias for ControlLeft & ControlRight, Shift for ShiftLeft & ShiftRight,

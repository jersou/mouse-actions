# Mouse actions

## Description

mouse_actions allows to execute some command from mouse events such as
clicks/wheel on the side / corners of the screen, or drawing shapes. It's a mix
between Easystroke and Compiz edge commands.

You can click on the top left corder of the screen to go to the first desktop,
or scroll on the top corner to increase/decrease the brightness of the screen,
change the volume... Or draw a T with the right button of the mouse down to
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
for
now, you add to write the json config yourself or use `mouse_actions record`.

I have been using mouse_actions for several days (since 15/05/2022) and X11 has
not crashed (Unlike Easystroke which made X11 crash every day before on my l
aptop), but
actually, mouse_actions exit on error when a device is added or removed.

With my usage, mouse_actions triggers commands about once per minute, and half
of which by form shape bindings.

The shape recognition is still not as good as Easystroke's, it can be improved.

## Install

### Requirement :

Add the current user to `input` & `plugdev` group :

```
sudo usermod -a -G plugdev $USER
sudo usermod -a -G input $USER
```

## Run

`cargo run`

## Build

`cargo build --release`

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
        * `shape`: array of number, the shape angles (radian)...

#### Get shape values

To get a shape angles values, run :

```
mouse_actions trace
```

#### Exemple

```json
{
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
        "edges": [],
        "event_type": "Click",
        "modifiers": [],
        "shape": []
      }
    },
    {
      "event": {
        "button": "Right",
        "edges": [
          "Right",
          "Top"
        ],
        "modifiers": [],
        "event_type": "Click",
        "shape": []
      },
      "cmd": [
        "wmctrl",
        "-s",
        "1"
      ]
    },
    {
      "event": {
        "button": "Right",
        "edges": [],
        "modifiers": [],
        "event_type": "Click",
        "shape": [
          0.04,
          0.11,
          -0.08,
          -1.64,
          -2.20,
          -1.96,
          -1.65
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
USAGE:
    mouse_actions [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -h, --help         Print help information
    -n, --no-listen    don't run the listen thread (for Wayland), the edge bindings might not work
    -V, --version      Print version information

SUBCOMMANDS:
    help           Print this message or the help of the given subcommand(s)
    open-config    Open the config file (xdg-open)
    record         Start record mode to add some mouse bindings
    start          Default command, use mouse_actions bindings
    trace          Trace events
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

## TODO

### next

* reset modifier at start ?
* enhance shape detection
    * use coef for shape.len() : bigger shape disadvantage ?
    * several patterns for a binding ?
* fix exit on laptop sleep/device add/remove (loop+sleep inc), detect/reboot
  app : `Grab Error: IoError(Os { code: 13, kind: PermissionDenied, message: "Permission denied" })`
* record : fix cmd split
* record : limit float precision
* refactor
    * remove panic
    * refactor/use Rust best practices
    * refactor Arc/Mutex usages
    * refactor/change the pressState usage
    * dev doc, tests

### maybe

* change config : if shape → no need button
* cancel shape if no move after few ms (400 ms ?)
* trigger keyboard event as action (avoid xdotool usage in
  cmd) : https://github.com/Narsil/rdev#sending-some-events
* vendors (in a separate branch)
* hide/freeze cursor while shape drawing ?
* better Readme
* find better a project name
* remove shape_button from config : filter the config bindings to set this value
* build & test on Windows
* GUI

### abandoned

* get the mouse position on wayland → impossible ?
* notif/sound/cursor change on action trigger (configurable) ?
* detach subprocess (close mouse_actions must not close sub process)
    * "cmd": [ "bash", "-c",  "nohup gedit &" ]
* mouse move edge event ?



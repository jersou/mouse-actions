# Changelog

## [0.4.5] - 2024-08-10

* update dependencies

## [0.4.4] - 2023-07-07

### Bug Fixes

* fix app crash or hang on new device event : the new device detection is now
  disabled (a better fix might be found later)

## [0.4.3] - 2023-07-02

### Features

* use command as string in config #7 : the configuration is upgrade at the first
  start of version > 0.4.2

BREAKING config file format : config from < v0.4.3 will be automatically upgrade
to v0.4.3 but mouse-actions program < v0.4.3 will not be able to open this
v0.4.3 config format.

## [0.4.2] - 2023-06-18

### Features

* force --no-listen option if Wayland is detected
* add --log-level option

### Bug Fixes

* forward GUI args to mouse action daemon (the start button)

## [0.4.1] - 2023-04-21

### Bug Fixes

- config editor: fix shape button select
- config editor: default binding: no screen part, add Ctrl modifiers
- config editor: show button select for shape type
- don't compare edges for shape events

## [0.4.0] - 2023-04-18

### Features

- **Config editor with Tauri**
- Add commit & build date to version
- Backup config before save
- Add ShowConfig subcommand
- save config (from stdin) subcommand

## [0.3.1] - 2023-04-10

### Bug Fixes

- Add doc to set rw permission on /dev/uinput #1

## [0.3.0] - 2023-04-09

### Features

- Single instance: use pid file and kill the old instance
- Detach subprocess
- Add logo/icon
- Add new commands : grab-one-event, stop, status
- Config path cli option
- Add shape_xy to config: remove shape_angles
- Several shapes in binding
- Add user to pid file path
- Err msg if GrabError::IoError::PermissionDenied

### Breaking

- change config file format : shape[] (angles) → [shape_xy[]]

## [0.1.0] - 2022-05-22

### Features

- first release, main features : shape recognition & side/corner click events

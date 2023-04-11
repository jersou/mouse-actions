# Changelog

## [unreleased]

### Features

- Add commit & build date to version
- Backup config before save
- Add ShowConfig subcommand
- Config editor: init

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

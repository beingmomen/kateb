#!/bin/bash
exec env -i \
  HOME="$HOME" \
  USER="$USER" \
  PATH="$HOME/.nvm/versions/node/v24.13.0/bin:$HOME/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin" \
  DISPLAY="${DISPLAY}" \
  DBUS_SESSION_BUS_ADDRESS="${DBUS_SESSION_BUS_ADDRESS}" \
  XDG_RUNTIME_DIR="${XDG_RUNTIME_DIR}" \
  WAYLAND_DISPLAY="${WAYLAND_DISPLAY:-}" \
  TERM="${TERM:-xterm-256color}" \
  pnpm tauri dev

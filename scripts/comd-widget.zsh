#!/usr/bin/env zsh
# Run comd from a ZLE key binding (default: Ctrl+E).
#
# The line shows `comd` (not an internal helper name). A shell function `comd` runs the real binary
# via `command`, sets COMD_ZSH_BUFFER_FILE, then inserts the result with print -z.
#
# Optional:
#   export COMD_CMD='comd'     # binary or full command line (zsh-split)
#   export COMD_ARGS='--foo'   # extra args (zsh-split)

comd-widget() {
  emulate -L zsh
  setopt localoptions pipefail no_aliases 2>/dev/null

  if ! [[ -o interactive ]] || ! [[ -o zle ]]; then
    print -u2 "comd-widget: needs an interactive zsh with ZLE"
    return 1
  fi

  local tmp bin
  tmp=$(mktemp) || return 1
  if [[ ! -x "comd" ]] && ! command -v -- "comd" &>/dev/null; then
    print -u2 "comd-widget: command not found: comd"
    command rm -f "$tmp"
    return 1
  fi

  COMD_ZSH_BUFFER_FILE=$tmp command -- "comd"
  local r=$?

  if [[ -s $tmp ]]; then
    print -z "$(<$tmp)"
    zle .accept-line
  fi
  command rm -f "$tmp"
  return r
}

zle -N comd-widget
# Replaces default emacs ^E (end-of-line); use another key if you rely on that.
bindkey -M emacs '^E' comd-widget
bindkey -M vicmd '^E' comd-widget
bindkey -M viins '^E' comd-widget

#!/usr/bin/env zsh
# Run comd from a ZLE binding (default: Ctrl+E).
#
# Uses push-line + BUFFER=comd + accept-line so comd runs as a normal command with a real TTY.

comd() {
  emulate -L zsh
  setopt localoptions pipefail no_aliases

  if ! command -v comd &>/dev/null; then
    print -u2 "comd: command not found"
    return 1
  fi

  local tmp
  tmp=$(mktemp) || return 1

  COMD_ZSH_BUFFER_FILE=$tmp command comd
  local r=$?

  if [[ -s $tmp ]]; then
    print -z "$(<$tmp)"
  fi
  command rm -f "$tmp"
  return r
}

comd-widget() {
  emulate -L zsh
  setopt localoptions no_aliases

  if ! [[ -o interactive ]] || ! [[ -o zle ]]; then
    print -u2 "comd-widget: needs an interactive zsh with ZLE"
    return 1
  fi

  zle .push-line
  BUFFER=comd
  zle .accept-line
}

zle -N comd-widget
# Replaces default emacs ^E (end-of-line); use another key if you rely on that.
bindkey -M emacs '^h' comd-widget
bindkey -M vicmd '^h' comd-widget
bindkey -M viins '^h' comd-widget

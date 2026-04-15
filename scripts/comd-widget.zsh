#!/usr/bin/env zsh
# Run comd from a ZLE key binding (default: Ctrl+E).
__comd_into_prompt() {
  emulate -L zsh
  setopt localoptions pipefail no_aliases 2>/dev/null

  local tmp bin
  tmp=$(mktemp) || return 1
  bin=${COMD_CMD:-comd}
  local cmdword="${bin%% *}"
  if [[ ! -x ${cmdword} ]] && ! command -v -- "$cmdword" &>/dev/null; then
    print -u2 "comd-widget: command not found: $cmdword (set COMD_CMD?)"
    command rm -f "$tmp"
    return 1
  fi

  local -a run
  run=(${(z)bin})
  [[ -n ${COMD_ARGS-} ]] && run+=(${(z)COMD_ARGS})

  COMD_ZSH_BUFFER_FILE=$tmp command -- "${run[@]}"
  local r=$?

  if [[ -s $tmp ]]; then
    print -z "$(<$tmp)"
  fi
  command rm -f "$tmp"
  return r
}

comd-widget() {
  emulate -L zsh
  setopt localoptions no_aliases 2>/dev/null

  if ! [[ -o interactive ]] || ! [[ -o zle ]]; then
    print -u2 "comd-widget: needs an interactive zsh with ZLE"
    return 1
  fi

  local bin=${COMD_CMD:-comd}
  local cmdword="${bin%% *}"
  if [[ ! -x ${cmdword} ]] && ! command -v -- "$cmdword" &>/dev/null; then
    print -u2 "comd-widget: command not found: $cmdword (set COMD_CMD?)"
    return 1
  fi

  zle .push-line
  BUFFER=__comd_into_prompt
  zle .accept-line
}

zle -N comd-widget
# Replaces default emacs ^E (end-of-line); use another key if you rely on that.
bindkey -M emacs '^E' comd-widget
bindkey -M vicmd '^E' comd-widget
bindkey -M viins '^E' comd-widget

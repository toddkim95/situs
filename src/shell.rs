pub(crate) fn print_zsh_init() {
    let bindkey = crate::config::read_configured_bindkey()
        .ok()
        .flatten()
        .unwrap_or_else(|| "^G".to_string());
    let mut init = ZSH_INIT.replace("SITUS_BINDKEY:=^G", &format!("SITUS_BINDKEY:={}", bindkey));
    if bindkey == "None" {
        init = init.replace(
            "bindkey \"$SITUS_BINDKEY\" _situs_accept_from_history",
            "# bindkey disabled",
        );
    }
    print!("{init}");
}

pub(crate) fn print_bash_init() {
    let bindkey = crate::config::read_configured_bindkey()
        .ok()
        .flatten()
        .unwrap_or_else(|| "^G".to_string());
    let mut init = BASH_INIT.replace("SITUS_BINDKEY:=^G", &format!("SITUS_BINDKEY:={}", bindkey));
    if bindkey == "None" {
        init = init.replace(
            "bind -x \"\\\"$SITUS_BINDKEY\\\": _situs_bash_widget\"",
            "# bind disabled",
        );
    }
    print!("{init}");
}

pub(crate) fn print_fish_init() {
    let bindkey = crate::config::read_configured_bindkey()
        .ok()
        .flatten()
        .unwrap_or_else(|| "^G".to_string());
    let init = if bindkey == "None" {
        let init_temp = FISH_INIT.replace(
            "bind $SITUS_BINDKEY __situs_choose_widget",
            "# bind disabled",
        );
        init_temp.replace("SITUS_BINDKEY \\cg", "SITUS_BINDKEY None")
    } else {
        let fish_key = if bindkey.starts_with('^') && bindkey.len() == 2 {
            format!(
                "\\c{}",
                bindkey.chars().nth(1).unwrap().to_ascii_lowercase()
            )
        } else {
            bindkey.clone()
        };
        FISH_INIT.replace("SITUS_BINDKEY \\cg", &format!("SITUS_BINDKEY {}", fish_key))
    };
    print!("{init}");
}

const BASH_INIT: &str = r#"# situs-cli bash integration
# Add this to ~/.bashrc:
#   eval "$(situs init bash)"

_situs_bash_precmd() {
  local status=$?
  local last_cmd
  last_cmd=$(history 1 | sed -e 's/^[ ]*[0-9]*[ ]*//')
  if [[ -n "$last_cmd" ]]; then
    if [[ "$last_cmd" != "$_situs_last_recorded" ]]; then
      command situs record --cwd "$PWD" --status "$status" -- "$last_cmd" >/dev/null 2>&1
      _situs_last_recorded="$last_cmd"
    fi
  fi
}

if [[ "$PROMPT_COMMAND" != *_situs_bash_precmd* ]]; then
  if [[ -n "$PROMPT_COMMAND" ]]; then
    PROMPT_COMMAND="_situs_bash_precmd; $PROMPT_COMMAND"
  else
    PROMPT_COMMAND="_situs_bash_precmd"
  fi
fi

_situs_bash_widget() {
  local selected selected_action selected_dir selected_cmd selected_query selected_rest
  selected="$(command situs choose --print-widget-selection --command "$READLINE_LINE")"
  local situs_choose_status=$?

  if [[ $situs_choose_status -eq 0 && -n "$selected" ]]; then
    selected_action="${selected%%$'\n'*}"
    selected_rest="${selected#*$'\n'}"
    selected_dir="${selected_rest%%$'\n'*}"
    selected_rest="${selected_rest#*$'\n'}"
    selected_cmd="${selected_rest%%$'\n'*}"
    selected_query="${selected_rest#*$'\n'}"

    if [[ "$selected_action" == "cd" ]]; then
      if builtin cd -- "$selected_dir"; then
        READLINE_LINE="$selected_query"
        READLINE_POINT=${#READLINE_LINE}
      fi
    elif [[ "$selected_action" == "run" ]]; then
      if builtin cd -- "$selected_dir"; then
        READLINE_LINE="cd -- $(printf %q "$selected_dir") && $selected_cmd"
        READLINE_POINT=${#READLINE_LINE}
      fi
    fi
  fi
}

: "${SITUS_BINDKEY:=^G}"
bind -x "\"$SITUS_BINDKEY\": _situs_bash_widget"

: "${SITUS_ALIAS:=st}"
if [[ "$SITUS_ALIAS" != "off" ]]; then
  alias "$SITUS_ALIAS"=situs
fi
"#;

const FISH_INIT: &str = r#"# situs-cli fish integration
# Add this to ~/.config/fish/config.fish:
#   situs init fish | source

function __situs_record_handler --on-event fish_postexec
  command situs record --cwd (pwd) --status $status -- $argv[1] >/dev/null 2>&1
end

function __situs_choose_widget
  set -l cmd (commandline)
  set -l selected (command situs choose --print-widget-selection --command "$cmd")
  if test $status -eq 0; and test -n "$selected"
    set -l lines (string split "\n" "$selected")
    set -l selected_action $lines[1]
    set -l selected_dir $lines[2]
    set -l selected_cmd $lines[3]
    set -l selected_query $lines[4]

    if test "$selected_action" = "cd"
      if cd "$selected_dir"
        commandline -r "$selected_query"
        commandline -f repaint
      end
    else if test "$selected_action" = "run"
      if cd "$selected_dir"
        commandline -r "$selected_cmd"
        commandline -f execute
      end
    end
  end
end

if not set -q SITUS_BINDKEY
  set -g SITUS_BINDKEY \cg
end

bind $SITUS_BINDKEY __situs_choose_widget

if not set -q SITUS_ALIAS
  set -g SITUS_ALIAS st
end
if test "$SITUS_ALIAS" != "off"
  alias $SITUS_ALIAS=situs
end
"#;

const ZSH_INIT: &str = r#"# situs-cli zsh integration
# Add this to ~/.zshrc:
#   eval "$(situs init zsh)"
#
# Default key: Ctrl-G.
# Override before eval if you want another key, for example:
#   export SITUS_BINDKEY='^G'
#
# Execution mode:
#   stay    cd into the selected directory and stay there after the command
#   restore run in the selected directory, then return to the original directory
# Override before eval if you want restore mode:
#   export SITUS_MODE='restore'

autoload -Uz add-zsh-hook

_situs_preexec() {
  if [[ -n "${_situs_override_command-}" && -n "${_situs_override_pwd-}" ]]; then
    typeset -g _situs_last_command="$_situs_override_command"
    typeset -g _situs_last_pwd="$_situs_override_pwd"
    unset _situs_override_command _situs_override_pwd
  else
    typeset -g _situs_last_command="$1"
    typeset -g _situs_last_pwd="$PWD"
  fi
}

_situs_precmd() {
  local situs_status=$?

  if [[ -n "${_situs_last_command-}" && -n "${_situs_last_pwd-}" ]]; then
    command situs record --cwd "$_situs_last_pwd" --status "$situs_status" -- "$_situs_last_command" >/dev/null 2>&1
  fi

  unset _situs_last_command _situs_last_pwd
  return $situs_status
}

_situs_restore_run() {
  local selected_dir="$1"
  local selected_cmd="$2"
  local situs_original_pwd="$PWD"
  local situs_status=1

  if cd -- "$selected_dir"; then
    eval "$selected_cmd"
    situs_status=$?
    cd -- "$situs_original_pwd"
  fi

  return $situs_status
}

_situs_accept_from_history() {
  local cmd="$BUFFER"

  local selected selected_action selected_dir selected_cmd selected_query selected_rest
  zle -I
  selected="$(command situs choose --print-widget-selection --command "$cmd")"
  local situs_choose_status=$?

  if [[ $situs_choose_status -eq 0 && -n "$selected" ]]; then
    selected_action="${selected%%$'\n'*}"
    selected_rest="${selected#*$'\n'}"
    selected_dir="${selected_rest%%$'\n'*}"
    selected_rest="${selected_rest#*$'\n'}"
    selected_cmd="${selected_rest%%$'\n'*}"
    selected_query="${selected_rest#*$'\n'}"

    case "$selected_action" in
      cd)
        if builtin cd -- "$selected_dir"; then
          if (( $+functions[chpwd] )); then
            chpwd
          fi
          local hook
          for hook in "${chpwd_functions[@]}"; do
            if (( $+functions[$hook] )); then
              "$hook"
            fi
          done
          for hook in "${precmd_functions[@]}"; do
            if (( $+functions[$hook] )); then
              "$hook" >/dev/null 2>&1
            fi
          done
          BUFFER="$selected_query"
          CURSOR=${#BUFFER}
          if (( $+functions[p10k] )); then
            p10k display -r
          fi
          zle reset-prompt
        else
          zle -M "situs: failed to cd to $selected_dir"
        fi
        ;;
      run)
        typeset -g _situs_override_command="$selected_cmd"
        typeset -g _situs_override_pwd="$selected_dir"
        case "${SITUS_MODE:-stay}" in
          stay|"")
            BUFFER="cd -- ${(q)selected_dir} && $selected_cmd"
            ;;
          restore)
            BUFFER="_situs_restore_run ${(q)selected_dir} ${(q)selected_cmd}"
            ;;
          *)
            zle -M "situs: unknown SITUS_MODE ${SITUS_MODE}; expected stay or restore"
            zle redisplay
            return
            ;;
        esac
        CURSOR=${#BUFFER}
        zle reset-prompt
        zle accept-line
        return
        ;;
      *)
        zle -M "situs: unknown selection action $selected_action"
        zle redisplay
        return
        ;;
    esac
  fi

  zle redisplay
}

add-zsh-hook -d preexec _situs_preexec 2>/dev/null
add-zsh-hook -d precmd _situs_precmd 2>/dev/null
add-zsh-hook preexec _situs_preexec
add-zsh-hook precmd _situs_precmd

zle -N situs-accept-from-history _situs_accept_from_history
: "${SITUS_BINDKEY:=^G}"
bindkey "$SITUS_BINDKEY" situs-accept-from-history

: "${SITUS_ALIAS:=st}"
if [[ "$SITUS_ALIAS" != "off" ]]; then
  alias "$SITUS_ALIAS"=situs
fi
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zsh_widget_uses_widget_selection_protocol() {
        assert!(ZSH_INIT.contains("situs choose --print-widget-selection"));
        assert!(ZSH_INIT.contains("selected_action"));
    }

    #[test]
    fn zsh_widget_tab_cd_keeps_query_without_accepting_line() {
        let cd_branch = ZSH_INIT
            .split("cd)")
            .nth(1)
            .and_then(|rest| rest.split(";;").next())
            .unwrap();

        assert!(cd_branch.contains("builtin cd -- \"$selected_dir\""));
        assert!(cd_branch.contains("BUFFER=\"$selected_query\""));
        assert!(cd_branch.contains("CURSOR=${#BUFFER}"));
        assert!(cd_branch.contains("zle reset-prompt"));
        assert!(!cd_branch.contains("zle accept-line"));
    }

    #[test]
    fn zsh_widget_run_accepts_selected_history_command() {
        let run_branch = ZSH_INIT
            .split("\n      run)")
            .nth(1)
            .and_then(|rest| rest.split("\n      *)").next())
            .unwrap();

        assert!(run_branch.contains("BUFFER=\"cd -- ${(q)selected_dir} && $selected_cmd\""));
        assert!(run_branch.contains("zle reset-prompt\n        zle accept-line\n        return"));
    }

    #[test]
    fn bash_integration_contains_bindkey_and_record() {
        assert!(BASH_INIT.contains("situs choose --print-widget-selection"));
        assert!(BASH_INIT.contains("bind -x"));
        assert!(BASH_INIT.contains("situs record"));
    }

    #[test]
    fn fish_integration_contains_bind_and_postexec() {
        assert!(FISH_INIT.contains("situs choose --print-widget-selection"));
        assert!(FISH_INIT.contains("bind $SITUS_BINDKEY"));
        assert!(FISH_INIT.contains("fish_postexec"));
    }
}

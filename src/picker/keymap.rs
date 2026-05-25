use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum KeyIntent {
    Run,
    Cancel,
    CdOnly,
    ToggleHelp,
    CycleSource,
    CycleContext,
    MoveSelectionUp,
    MoveSelectionDown,
    MoveSelectionPageUp,
    MoveSelectionPageDown,
    MoveQueryLeft,
    MoveQueryRight,
    MoveQueryHome,
    MoveQueryEnd,
    Backspace,
    Delete,
    ToggleFailures,
    ToggleInspect,
    ClearQuery,
    CopySelected,
    DeleteSelected,
    InsertChar(char),
    Ignore,
}

pub(super) fn key_intent(key: KeyEvent) -> KeyIntent {
    match key.code {
        KeyCode::Enter => KeyIntent::Run,
        KeyCode::Esc => KeyIntent::Cancel,
        KeyCode::Tab => KeyIntent::CdOnly,
        KeyCode::BackTab => KeyIntent::Ignore,
        KeyCode::F(1) => KeyIntent::ToggleHelp,
        KeyCode::F(2) => KeyIntent::CycleSource,
        KeyCode::F(3) => KeyIntent::CycleContext,
        KeyCode::PageUp => KeyIntent::MoveSelectionPageUp,
        KeyCode::PageDown => KeyIntent::MoveSelectionPageDown,
        KeyCode::Left => KeyIntent::MoveQueryLeft,
        KeyCode::Right => KeyIntent::MoveQueryRight,
        KeyCode::Up => KeyIntent::MoveSelectionUp,
        KeyCode::Down => KeyIntent::MoveSelectionDown,
        KeyCode::Home => KeyIntent::MoveQueryHome,
        KeyCode::End => KeyIntent::MoveQueryEnd,
        KeyCode::Backspace => KeyIntent::Backspace,
        KeyCode::Delete => KeyIntent::Delete,
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => KeyIntent::Cancel,
        KeyCode::Char('/') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            KeyIntent::ToggleHelp
        }
        KeyCode::Char('_') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            KeyIntent::ToggleHelp
        }
        KeyCode::Char('a') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            KeyIntent::MoveQueryHome
        }
        KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            KeyIntent::DeleteSelected
        }
        KeyCode::Char('e') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            KeyIntent::MoveQueryEnd
        }
        KeyCode::Char('f') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            KeyIntent::ToggleFailures
        }
        KeyCode::Char('o') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            KeyIntent::ToggleInspect
        }
        KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            KeyIntent::ClearQuery
        }
        KeyCode::Char('y') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            KeyIntent::CopySelected
        }
        KeyCode::Char(character)
            if key.modifiers.is_empty() || key.modifiers == KeyModifiers::SHIFT =>
        {
            KeyIntent::InsertChar(character)
        }
        _ => KeyIntent::Ignore,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_tab_and_enter_to_picker_intents() {
        assert!(matches!(
            key_intent(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE)),
            KeyIntent::CdOnly
        ));
        assert!(matches!(
            key_intent(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)),
            KeyIntent::Run
        ));
    }

    #[test]
    fn maps_navigation_and_editing_keys_to_picker_intents() {
        assert!(matches!(
            key_intent(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE)),
            KeyIntent::MoveSelectionUp
        ));
        assert!(matches!(
            key_intent(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE)),
            KeyIntent::MoveQueryRight
        ));
        assert!(matches!(
            key_intent(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE)),
            KeyIntent::InsertChar('x')
        ));
    }
}

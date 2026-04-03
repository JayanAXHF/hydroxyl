use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::action::Action;

pub fn action_from_key(key: KeyEvent, editing: bool) -> Action {
    if editing {
        return match key.code {
            KeyCode::Enter => Action::ConfirmEdit,
            KeyCode::Esc => Action::CancelEdit,
            KeyCode::Backspace => Action::Backspace,
            KeyCode::Char(value) => Action::InputChar(value),
            _ => Action::Noop,
        };
    }

    if key.modifiers.contains(KeyModifiers::CONTROL) && matches!(key.code, KeyCode::Char('c')) {
        return Action::RequestQuit;
    }

    if key.modifiers.contains(KeyModifiers::CONTROL) && matches!(key.code, KeyCode::Char('w')) {
        return Action::CloseActiveTab;
    }

    match key.code {
        KeyCode::Char('q') => Action::RequestQuit,
        KeyCode::Char('s') => Action::SaveActive,
        KeyCode::Char('e') | KeyCode::Enter => Action::StartEdit,
        KeyCode::Char('o') => Action::OpenSelected,
        KeyCode::Tab => Action::FocusNext,
        KeyCode::BackTab => Action::PreviousTab,
        KeyCode::Char('[') => Action::PreviousTab,
        KeyCode::Char(']') => Action::NextTab,
        KeyCode::Left | KeyCode::Char('h') => Action::MoveLeft,
        KeyCode::Right | KeyCode::Char('l') => Action::MoveRight,
        KeyCode::Up | KeyCode::Char('k') => Action::MoveUp,
        KeyCode::Down | KeyCode::Char('j') => Action::MoveDown,
        _ => Action::Noop,
    }
}

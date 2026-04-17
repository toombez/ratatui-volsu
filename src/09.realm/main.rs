pub mod components;
pub mod model;

use tuirealm::application::PollStrategy;
use tuirealm::{AttrValue, Attribute, Update};

use model::Model;

/// Сообщения, которыми обмениваются компоненты
#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    ClockTick,
    DigitCounterChanged(isize),
    DigitCounterBlur,
    LetterCounterChanged(isize),
    LetterCounterBlur,
}

/// Идентификаторы компонентов в приложении
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    Clock,
    DigitCounter,
    LetterCounter,
    Label,
}

fn main() {
    let mut model = Model::default();
    let _ = model.terminal.enter_alternate_screen();
    let _ = model.terminal.enable_raw_mode();

    while !model.quit {
        match model.app.tick(PollStrategy::Once) {
            Err(err) => {
                let _ = model.app.attr(
                    &Id::Label,
                    Attribute::Text,
                    AttrValue::String(format!("Error: {err}")),
                );
            }
            Ok(messages) if !messages.is_empty() => {
                model.redraw = true;
                for msg in messages {
                    let mut msg = Some(msg);
                    while msg.is_some() {
                        msg = model.update(msg);
                    }
                }
            }
            _ => {}
        }
        if model.redraw {
            model.view();
            model.redraw = false;
        }
    }

    let _ = model.terminal.leave_alternate_screen();
    let _ = model.terminal.disable_raw_mode();
    let _ = model.terminal.clear_screen();
}

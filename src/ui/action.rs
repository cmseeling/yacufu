use serde::{Deserialize, Serialize};
use strum::Display;

use super::app::Mode;

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
pub enum ListAction {
    SelectNext,
    SelectPrev,
    SelectFirst,
    SelectLast,
    SelectNone,
    MarkSelection,
}

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
pub enum Action {
    Tick,
    Render,
    Resize(u16, u16),
    Suspend,
    Resume,
    Quit,
    ClearScreen,
    Error(String),
    Help,
    ListAction(ListAction),
    ChangeMode(Mode),
}

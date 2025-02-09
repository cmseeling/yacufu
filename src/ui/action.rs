use serde::{Deserialize, Serialize};
use strum::Display;

use super::ViewState;

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
pub enum ListAction {
    SelectNext,
    SelectPrev,
    SelectFirst,
    SelectLast,
    SelectNone,
    MakeSelection,
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
    UpdateViewState(ViewState),
    FocusMainMenu,
    NextMode,
    PrevMode,
}

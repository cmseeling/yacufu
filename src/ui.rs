use serde::{Deserialize, Serialize};

pub(crate) mod action;
pub mod app;
mod components;
pub(crate) mod tui;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[default]
    System,
    PackageSources,
    InstalledPackages,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Focus {
    #[default]
    MainMenu,
    Page,
}

use std::fmt;

use serde::{Deserialize, Serialize};

pub(crate) mod action;
pub mod app;
mod components;
pub(crate) mod tui;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mode {
    Universal,
    List,
    Tabs,
    #[default]
    MainMenu,
    System,
    PackageSourceTabs,
    PackageSourceList,
    InstalledPackageTabs,
    InstalledPackageList,
    ConfirmationPopup,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Page {
    #[default]
    System,
    PackageSources,
    InstalledPackages,
    ConfirmationPopup,
}

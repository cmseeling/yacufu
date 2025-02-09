use std::collections::HashMap;

use color_eyre::Result;
use lazy_static::lazy_static;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, List, ListState},
    Frame,
};
use tracing::info;

use crate::ui::{
    action::{Action, ListAction},
    Mode, Page, ViewState,
};

use super::Component;

lazy_static! {
    static ref MENU_OPTIONS: Vec<&'static str> =
        vec!["System",
        // "Installed Packages",
        "Package Sources"];
}

#[derive(Default)]
pub struct MainMenu {
    state: ListState,
}

impl MainMenu {
    pub fn new() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        Self { state }
    }

    fn change_page(&self) -> Option<Action> {
        if let Some(selected) = self.state.selected() {
            let page = match MENU_OPTIONS[selected] {
                "System" => Page::System,
                "Installed Packages" => Page::InstalledPackages,
                "Package Sources" => Page::PackageSources,
                _ => Page::System,
            };
            Some(Action::UpdateViewState(ViewState::new(
                Mode::MainMenu,
                page,
            )))
        } else {
            None
        }
    }

    fn next_option(&mut self) -> Option<Action> {
        let selected = self.state.selected().unwrap_or(0);
        // info!("index is {:?}", selected);
        if selected < MENU_OPTIONS.len() - 1 {
            self.state.select_next();
        } else {
            self.state.select_first();
        }
        self.change_page()
    }

    fn prev_option(&mut self) -> Option<Action> {
        let selected = self.state.selected().unwrap_or(0);
        if selected > 0 {
            self.state.select_previous();
        } else {
            self.state.select(Some(MENU_OPTIONS.len() - 1));
        }
        self.change_page()
    }

    fn first_option(&mut self) -> Option<Action> {
        self.state.select_first();
        self.change_page()
    }

    fn last_option(&mut self) -> Option<Action> {
        self.state.select(Some(MENU_OPTIONS.len() - 1));
        self.change_page()
    }

    fn focus_page(&self, view_state: ViewState) -> Option<Action> {
        match view_state.page {
            Page::System => Some(Action::UpdateViewState(ViewState::new(
                Mode::System,
                view_state.page,
            ))),
            Page::InstalledPackages => Some(Action::UpdateViewState(ViewState::new(
                Mode::InstalledPackageTabs,
                view_state.page,
            ))),
            Page::PackageSources => Some(Action::UpdateViewState(ViewState::new(
                Mode::PackageSourceTabs,
                view_state.page,
            ))),
            _ => None,
        }
    }
}

impl Component for MainMenu {
    fn update(&mut self, action: Action, view_state: ViewState) -> Result<Option<Action>> {
        match action {
            Action::ListAction(list_action) => {
                if view_state.mode == Mode::MainMenu {
                    info!("MainMenu handling action: {list_action:?}");
                    match list_action {
                        ListAction::SelectNext => Ok(self.next_option()),
                        ListAction::SelectPrev => Ok(self.prev_option()),
                        ListAction::SelectFirst => Ok(self.first_option()),
                        ListAction::SelectLast => Ok(self.last_option()),
                        ListAction::SelectNone => {
                            self.state.select(None);
                            Ok(None)
                        }
                        ListAction::MakeSelection => Ok(self.focus_page(view_state)),
                        _ => Ok(None),
                    }
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
        }
    }

    fn draw(
        &mut self,
        view_state: ViewState,
        frame: &mut Frame,
        areas: &HashMap<&str, Rect>,
    ) -> Result<()> {
        let area = areas.get("menu").unwrap();
        let border_style = match view_state.mode {
            Mode::MainMenu => Style::default().fg(Color::Blue),
            _ => Style::default(),
        };
        let list = List::new(MENU_OPTIONS.clone())
            .block(
                Block::bordered()
                    .title("Main Menu")
                    .border_style(border_style),
            )
            .highlight_style(Style::new().bg(Color::Blue).add_modifier(Modifier::BOLD))
            .highlight_symbol(">");
        frame.render_stateful_widget(list, area.clone(), &mut self.state);
        Ok(())
    }
}

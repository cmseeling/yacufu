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
    app::Mode,
};

use super::Component;

lazy_static! {
    static ref MENU_OPTIONS: Vec<&'static str> =
        vec!["System", "Installed Packages", "Package Sources"];
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

    fn get_mode_action(&self) -> Option<Action> {
        if let Some(selected) = self.state.selected() {
            let mode = match MENU_OPTIONS[selected] {
                "System" => Mode::System,
                "Installed Packages" => Mode::InstalledPackages,
                "Package Sources" => Mode::PackageSources,
                _ => Mode::System,
            };
            Some(Action::ChangeMode(mode))
        } else {
            None
        }
    }

    fn next_option(&mut self) -> Option<Action> {
        self.state.select_next();
        self.get_mode_action()
    }

    fn prev_option(&mut self) -> Option<Action> {
        self.state.select_previous();
        self.get_mode_action()
    }
}

impl Component for MainMenu {
    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::ListAction(list_action) => {
                info!("MainMenu handling action: {list_action:?}");
                match list_action {
                    ListAction::SelectNext => Ok(self.next_option()),
                    ListAction::SelectPrev => Ok(self.prev_option()),
                    ListAction::SelectFirst => {
                        self.state.select_first();
                        Ok(None)
                    }
                    ListAction::SelectLast => {
                        self.state.select_last();
                        Ok(None)
                    }
                    ListAction::SelectNone => {
                        self.state.select(None);
                        Ok(None)
                    }
                    // ListAction::MarkSelection => todo!(),
                    _ => Ok(None),
                }
            }
            _ => Ok(None),
        }
    }

    fn draw(&mut self, frame: &mut Frame, areas: &HashMap<&str, Rect>) -> Result<()> {
        let area = areas.get("menu").unwrap();
        let list = List::new(MENU_OPTIONS.clone())
            .block(Block::bordered().title("Main Menu"))
            .highlight_style(Style::new().bg(Color::Blue).add_modifier(Modifier::BOLD))
            .highlight_symbol(">");
        frame.render_stateful_widget(list, area.clone(), &mut self.state);
        Ok(())
    }
}

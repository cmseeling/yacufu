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
    Focus, Mode,
};

use super::Component;

lazy_static! {
    static ref MENU_OPTIONS: Vec<&'static str> =
        vec!["System", "Installed Packages", "Package Sources"];
}

#[derive(Default)]
pub struct MainMenu {
    focused: bool,
    state: ListState,
}

impl MainMenu {
    pub fn new() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        Self {
            focused: true,
            state,
        }
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
        let selected = self.state.selected().unwrap_or(0);
        info!("index is {:?}", selected);
        if selected < MENU_OPTIONS.len() - 1 {
            self.state.select_next();
        } else {
            self.state.select_first();
        }
        self.get_mode_action()
    }

    fn prev_option(&mut self) -> Option<Action> {
        let selected = self.state.selected().unwrap_or(0);
        if selected > 0 {
            self.state.select_previous();
        } else {
            self.state.select(Some(MENU_OPTIONS.len() - 1));
        }
        self.get_mode_action()
    }

    fn first_option(&mut self) -> Option<Action> {
        self.state.select_first();
        self.get_mode_action()
    }

    fn last_option(&mut self) -> Option<Action> {
        self.state.select(Some(MENU_OPTIONS.len() - 1));
        self.get_mode_action()
    }
}

impl Component for MainMenu {
    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::ChangeFocus(focus_action) => match focus_action {
                Focus::MainMenu => {
                    self.focused = true;
                    Ok(None)
                }
                _ => {
                    self.focused = false;
                    Ok(None)
                }
            },
            Action::ListAction(list_action) => {
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
                    ListAction::MarkSelection => Ok(Some(Action::ChangeFocus(Focus::Page))),
                    _ => Ok(None),
                }
            }
            _ => Ok(None),
        }
    }

    fn draw(&mut self, frame: &mut Frame, areas: &HashMap<&str, Rect>) -> Result<()> {
        let area = areas.get("menu").unwrap();
        let border_style = match self.focused {
            true => Style::default().fg(Color::Blue),
            false => Style::default(),
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

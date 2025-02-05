use std::collections::HashMap;

use color_eyre::Result;
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListState, Tabs},
    Frame,
};
use tracing::info;

use crate::{
    repositories::{apt::AptRepositories, Repository},
    ui::{
        action::{Action, ListAction},
        Focus, Mode,
    },
};

use super::Component;

#[derive(Default)]
pub struct PackageSources {
    show: bool,
    focused: bool,
    is_enabled: bool,
    repositories: AptRepositories,
    state: ListState,
}

impl PackageSources {
    pub fn new() -> Self {
        let repositories = AptRepositories::default();
        Self {
            show: false,
            focused: false,
            is_enabled: repositories.check_for_repository(),
            repositories,
            state: ListState::default(),
        }
    }
}

impl Component for PackageSources {
    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::ChangeMode(mode) => {
                match mode {
                    Mode::PackageSources => self.show = true,
                    _ => self.show = false,
                };
                self.focused = false;
            }
            Action::ChangeFocus(focus) => match focus {
                Focus::Page => {
                    if self.show {
                        self.focused = true
                    } else {
                        self.focused = false
                    }
                }
                _ => self.focused = false,
            },
            Action::ListAction(list_action) => {
                info!("PackageSources handling action: {list_action:?}");
                match list_action {
                    // ListAction::SelectNext => self.state.select_next(),
                    // ListAction::SelectPrev => self.state.select_previous(),
                    // ListAction::SelectFirst => self.state.select_first(),
                    // ListAction::SelectLast => self.state.select_last(),
                    // ListAction::SelectNone => self.state.select(None),
                    // ListAction::MarkSelection => todo!(),
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, areas: &HashMap<&str, Rect>) -> Result<()> {
        if self.show && self.is_enabled {
            let area = areas.get("page").unwrap();
            let border_style = match self.focused {
                true => Style::default().fg(Color::Blue),
                false => Style::default(),
            };
            let block = Block::bordered()
                .title("Package Sources")
                .border_style(border_style);
            let inner = block.inner(*area);

            frame.render_widget(block, *area);

            let [tabs_area, page] =
                Layout::vertical([Constraint::Length(2), Constraint::Min(0)]).areas(inner);
            let [_, centered, _] =
                Layout::horizontal([Constraint::Fill(1), Constraint::Min(0), Constraint::Fill(1)])
                    .flex(Flex::Center)
                    .areas(tabs_area);

            let tabs = Tabs::new(vec!["Apt", "Flatpak", "Homebrew"])
                .highlight_style(
                    Style::new()
                        .fg(Color::Green)
                        .add_modifier(Modifier::UNDERLINED),
                )
                .select(0)
                .padding("", "")
                .divider(" ");
            frame.render_widget(tabs, centered);

            if !self.repositories.initialized {
                self.repositories.load_repository_list()?;
            }

            let list = List::new(self.repositories.get_repository_list())
                .block(Block::bordered().borders(Borders::TOP))
                .highlight_style(Style::new().bg(Color::Blue).add_modifier(Modifier::BOLD))
                .highlight_symbol(">");
            frame.render_stateful_widget(list, page, &mut self.state);
        }
        Ok(())
    }
}

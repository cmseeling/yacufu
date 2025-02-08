use std::{
    collections::HashMap,
    fmt::{self, format},
};

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
        Mode, Page,
    },
};

use super::Component;

#[derive(Debug, Default, PartialEq)]
enum PageFocus {
    #[default]
    Tabs,
    List,
}

#[derive(Default)]
pub struct PackageSources {
    show: bool,
    focused: bool,
    page_focus: PageFocus,
    is_enabled: bool,
    repositories: AptRepositories,
    selected_tab: usize,
    list_state: ListState,
}

impl PackageSources {
    pub fn new() -> Self {
        let repositories = AptRepositories::default();
        Self {
            show: false,
            focused: false,
            page_focus: PageFocus::Tabs,
            is_enabled: repositories.check_for_repository(),
            repositories,
            selected_tab: 0,
            list_state: ListState::default(),
        }
    }

    fn get_view_state_debug(&self) -> String {
        format!(
            "PackageSources: {{ show: {:?}, focused: {:?}, page_focus: {:?} }}",
            self.show, self.focused, self.page_focus
        )
    }

    fn handle_list_action(&mut self, list_action: ListAction) -> Result<Option<Action>> {
        if self.show && self.focused {
            info!("PackageSources handling action: {list_action:?}");
            info!("{}", self.get_view_state_debug());
            if self.page_focus == PageFocus::Tabs {
                self.handle_tab_movement(list_action)
            } else {
                self.handle_list_movement(list_action)
            }
        } else {
            Ok(None)
        }
    }

    fn handle_tab_movement(&mut self, list_action: ListAction) -> Result<Option<Action>> {
        match list_action {
            ListAction::SelectNext => self.next_tab_item(),
            ListAction::SelectPrev => self.prev_tab_item(),
            _ => Ok(None),
        }
    }

    fn next_tab_item(&mut self) -> Result<Option<Action>> {
        let selected = self.list_state.selected().unwrap_or(0);
        if selected < 2 {
            self.selected_tab += 1;
        } else {
            self.selected_tab = 0;
        }
        Ok(None)
    }

    fn prev_tab_item(&mut self) -> Result<Option<Action>> {
        let selected = self.list_state.selected().unwrap_or(0);
        if selected > 0 {
            self.selected_tab -= 1;
        } else {
            self.selected_tab = 2;
        }
        Ok(None)
    }

    fn handle_list_movement(&mut self, list_action: ListAction) -> Result<Option<Action>> {
        match list_action {
            ListAction::SelectNext => self.next_list_item(),
            ListAction::SelectPrev => self.prev_list_item(),
            ListAction::SelectFirst => self.first_list_item(),
            ListAction::SelectLast => self.last_list_item(),
            ListAction::SelectNone => self.clear_list_item(),
            ListAction::MarkSelection => Ok(None), //TODO: implement selection popup
            _ => Ok(None),
        }
    }

    fn clear_list_item(&mut self) -> Result<Option<Action>> {
        self.list_state.select(None);
        Ok(None)
    }

    fn next_list_item(&mut self) -> Result<Option<Action>> {
        let selected = self.list_state.selected().unwrap_or(0);
        if selected < self.repositories.get_repository_list().len() - 1 {
            self.list_state.select_next();
        } else {
            self.list_state.select_first();
        }
        Ok(None)
    }

    fn prev_list_item(&mut self) -> Result<Option<Action>> {
        let selected = self.list_state.selected().unwrap_or(0);
        if selected > 0 {
            self.list_state.select_previous();
        } else {
            self.list_state
                .select(Some(self.repositories.get_repository_list().len() - 1));
        }
        Ok(None)
    }

    fn first_list_item(&mut self) -> Result<Option<Action>> {
        self.list_state.select_first();
        Ok(None)
    }

    fn last_list_item(&mut self) -> Result<Option<Action>> {
        self.list_state
            .select(Some(self.repositories.get_repository_list().len() - 1));
        Ok(None)
    }
}

impl Component for PackageSources {
    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::ChangePage(page) => {
                match page {
                    Page::PackageSources => self.show = true,
                    _ => self.show = false,
                };
                self.focused = false;
                Ok(None)
            }
            Action::FocusPage => {
                if self.show {
                    self.focused = true
                } else {
                    self.focused = false
                }
                self.page_focus = PageFocus::Tabs;
                Ok(None)
            }
            Action::FocusMainMenu => {
                self.focused = false;
                Ok(None)
            }
            Action::ChangeMode(mode) => {
                match mode {
                    Mode::PackageSourceTabs => {
                        if self.show {
                            self.focused = true
                        } else {
                            self.focused = false
                        }
                        self.page_focus = PageFocus::Tabs
                    }
                    Mode::PackageSourceList => {
                        if self.show {
                            self.focused = true
                        } else {
                            self.focused = false
                        }
                        self.page_focus = PageFocus::List
                    }
                    _ => self.focused = false,
                };
                Ok(None)
            }
            Action::ListAction(list_action) => self.handle_list_action(list_action),
            _ => Ok(None),
        }
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
                .select(self.selected_tab)
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
            frame.render_stateful_widget(list, page, &mut self.list_state);
        }
        Ok(())
    }
}

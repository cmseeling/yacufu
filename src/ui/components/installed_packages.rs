use std::collections::HashMap;

use color_eyre::Result;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::Block,
    Frame,
};
use tracing::info;

use crate::ui::{action::Action, Mode, Page, ViewState};

use super::Component;

#[derive(Default)]
enum PageFocus {
    #[default]
    Tabs,
    List,
}

#[derive(Default)]
pub struct InstalledPackages {
    // show: bool,
    // focused: bool,
    // page_focus: PageFocus,
}

impl InstalledPackages {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for InstalledPackages {
    fn update(&mut self, action: Action, view_state: ViewState) -> Result<Option<Action>> {
        match action {
            Action::ListAction(list_action) => {
                match view_state.mode {
                    Mode::InstalledPackageList => {
                        info!("InstalledPackages handling action: {list_action:?}");
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
                    Mode::InstalledPackageTabs => {} //TODO: handle tabs
                    _ => {}
                }
            }
            // Action::NextMode => {
            //     if view_state.mode == Mode::InstalledPackageTabs {
            //         Ok(Some(Action::UpdateViewState(ViewState::new(
            //             Mode::InstalledPackageList,
            //             view_state.page,
            //         ))))
            //     } else {
            //         Ok(None)
            //     }
            // }
            // Action::PrevMode => {
            //     if view_state.mode == Mode::InstalledPackageList {
            //         Ok(Some(Action::UpdateViewState(ViewState::new(
            //             Mode::InstalledPackageTabs,
            //             view_state.page,
            //         ))))
            //     } else {
            //         Ok(None)
            //     }
            // }
            _ => {}
        }
        Ok(None)
    }

    fn draw(
        &mut self,
        view_state: ViewState,
        frame: &mut Frame,
        areas: &HashMap<&str, Rect>,
    ) -> Result<()> {
        if view_state.page == Page::InstalledPackages {
            let area = areas.get("page").unwrap();
            let border_style = match view_state.mode {
                Mode::InstalledPackageTabs | Mode::InstalledPackageList => {
                    Style::default().fg(Color::Blue)
                }
                _ => Style::default(),
            };
            let block = Block::bordered()
                .title("Installed Packages")
                .border_style(border_style);
            // let inner = block.inner(*area);

            frame.render_widget(block, *area);

            // let [tabs_area, page] =
            //     Layout::vertical([Constraint::Length(2), Constraint::Min(0)]).areas(inner);
            // let [_, centered, _] =
            //     Layout::horizontal([Constraint::Fill(1), Constraint::Min(0), Constraint::Fill(1)])
            //         .flex(Flex::Center)
            //         .areas(tabs_area);

            // let tabs = Tabs::new(vec!["Apt", "Flatpak", "Homebrew"])
            //     .highlight_style(
            //         Style::new()
            //             .fg(Color::Green)
            //             .add_modifier(Modifier::UNDERLINED),
            //     )
            //     .select(0)
            //     .padding("", "")
            //     .divider(" ");
            // frame.render_widget(tabs, centered);

            // if !self.repositories.initialized {
            //     self.repositories.load_repository_list()?;
            // }

            // let list = List::new(self.repositories.get_repository_list())
            //     .block(Block::bordered().borders(Borders::TOP))
            //     .highlight_style(Style::new().bg(Color::Blue).add_modifier(Modifier::BOLD))
            //     .highlight_symbol(">");
            // frame.render_stateful_widget(list, page, &mut self.state);
        }
        Ok(())
    }
}

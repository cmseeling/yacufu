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

use crate::action::{Action, ListAction};

use super::Component;

lazy_static! {
    static ref MENU_OPTIONS: Vec<String> = vec![
        String::from("System"),
        String::from("Installed Packages"),
        String::from("Package Sources")
    ];
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
}

impl Component for MainMenu {
    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::ListAction(list_action) => {
                info!("MainMenu handling action: {list_action:?}");
                match list_action {
                    ListAction::SelectNext => self.state.select_next(),
                    ListAction::SelectPrev => self.state.select_previous(),
                    ListAction::SelectFirst => self.state.select_first(),
                    ListAction::SelectLast => self.state.select_last(),
                    ListAction::SelectNone => self.state.select(None),
                    // ListAction::MarkSelection => todo!(),
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(None)
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

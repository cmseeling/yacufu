use color_eyre::Result;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{action::Action, config::Config};

use super::{main_menu::MainMenu, right_panel::RightPanel, Component};

#[derive(Default)]
pub struct RootLayout {
    main_menu: Box<MainMenu>,
    right_panel: Box<RightPanel>,
}

impl RootLayout {
    pub fn new() -> Self {
        Self {
            main_menu: Box::new(MainMenu::new()),
            right_panel: Box::new(RightPanel::new()),
        }
    }
}

impl Component for RootLayout {
    fn register_action_handler(&mut self, tx: &UnboundedSender<Action>) -> Result<()> {
        self.main_menu.register_action_handler(tx)?;
        self.right_panel.register_action_handler(tx)?;
        Ok(())
    }

    fn register_config_handler(&mut self, config: &Config) -> Result<()> {
        self.main_menu.register_config_handler(config)?;
        self.right_panel.register_config_handler(config)?;
        Ok(())
    }

    fn init(&mut self, area: ratatui::prelude::Size) -> Result<()> {
        self.main_menu.init(area)?;
        self.right_panel.init(area)?;
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::ListAction(_) => self.main_menu.update(action.clone()),
            _ => self.right_panel.update(action.clone()),
        }
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
            .split(area);

        let _ = self.main_menu.draw(frame, chunks[0]);
        let _ = self.right_panel.draw(frame, chunks[1]);

        Ok(())
    }
}

use color_eyre::Result;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{action::Action, config::Config};

use super::{fps::FpsCounter, home::Home, main_menu::MainMenu, Component};

#[derive(Default)]
pub struct RightPanel {
    home: Box<Home>,
    fps: Box<FpsCounter>,
}

impl RightPanel {
    pub fn new() -> Self {
        Self {
            home: Box::new(Home::new()),
            fps: Box::new(FpsCounter::new()),
        }
    }
}

impl Component for RightPanel {
    fn register_action_handler(&mut self, tx: &UnboundedSender<Action>) -> Result<()> {
        self.home.register_action_handler(tx)?;
        self.fps.register_action_handler(tx)?;
        Ok(())
    }

    fn register_config_handler(&mut self, config: &Config) -> Result<()> {
        self.home.register_config_handler(config)?;
        self.fps.register_config_handler(config)?;
        Ok(())
    }

    fn init(&mut self, area: ratatui::prelude::Size) -> Result<()> {
        self.home.init(area)?;
        self.fps.init(area)?;
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => self.fps.update(action.clone()),
            Action::Render => self.fps.update(action.clone()),
            _ => self.home.update(action.clone()),
        }
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let _ = self.home.draw(frame, chunks[0]);
        let _ = self.fps.draw(frame, chunks[1]);

        Ok(())
    }
}

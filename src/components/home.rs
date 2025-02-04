use std::collections::HashMap;

use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::{action::Action, config::Config};

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
}

impl Home {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for Home {
    fn register_action_handler(&mut self, tx: &UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx.clone());
        Ok(())
    }

    fn register_config_handler(&mut self, config: &Config) -> Result<()> {
        self.config = config.clone();
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {
                // add any logic here that should run on every tick
            }
            Action::Render => {
                // add any logic here that should run on every render
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, areas: &HashMap<&str, Rect>) -> Result<()> {
        let area = areas.get("page").unwrap();
        frame.render_widget(
            Paragraph::new("hello world").block(Block::bordered().title("Page Area")),
            area.clone(),
        );
        Ok(())
    }
}

use std::collections::HashMap;

use color_eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Constraint, Layout},
    prelude::Rect,
};
use tokio::sync::mpsc;
use tracing::{debug, info};

use crate::{
    config::Config,
    ui::action::Action,
    ui::components::{
        installed_packages::InstalledPackages, main_menu::MainMenu,
        package_sources::PackageSources, system_page::SystemPage, Component,
    },
    ui::tui::{Event, Tui},
};

use super::{Mode, Page};

pub struct App {
    config: Config,
    tick_rate: f64,
    frame_rate: f64,
    components: Vec<Box<dyn Component>>,
    should_quit: bool,
    should_suspend: bool,
    mode: Mode,
    page: Page,
    last_tick_key_events: Vec<KeyEvent>,
    action_tx: mpsc::UnboundedSender<Action>,
    action_rx: mpsc::UnboundedReceiver<Action>,
}

impl App {
    pub fn new(tick_rate: f64, frame_rate: f64) -> Result<Self> {
        let (action_tx, action_rx) = mpsc::unbounded_channel();
        Ok(Self {
            tick_rate,
            frame_rate,
            components: vec![
                Box::new(InstalledPackages::new()),
                Box::new(MainMenu::new()),
                Box::new(PackageSources::new()),
                Box::new(SystemPage::new()),
            ],
            should_quit: false,
            should_suspend: false,
            config: Config::new()?,
            mode: Mode::MainMenu,
            page: Page::System,
            last_tick_key_events: Vec::new(),
            action_tx,
            action_rx,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut tui = Tui::new()?
            // .mouse(true) // uncomment this line to enable mouse support
            .tick_rate(self.tick_rate)
            .frame_rate(self.frame_rate);
        tui.enter()?;

        for component in self.components.iter_mut() {
            component.register_action_handler(&self.action_tx)?;
        }
        for component in self.components.iter_mut() {
            component.register_config_handler(&self.config)?;
        }
        for component in self.components.iter_mut() {
            component.init(tui.size()?)?;
        }

        let action_tx = self.action_tx.clone();
        loop {
            self.handle_events(&mut tui).await?;
            self.handle_actions(&mut tui)?;
            if self.should_suspend {
                tui.suspend()?;
                action_tx.send(Action::Resume)?;
                action_tx.send(Action::ClearScreen)?;
                // tui.mouse(true);
                tui.enter()?;
            } else if self.should_quit {
                tui.stop()?;
                break;
            }
        }
        tui.exit()?;
        Ok(())
    }

    async fn handle_events(&mut self, tui: &mut Tui) -> Result<()> {
        let Some(event) = tui.next_event().await else {
            return Ok(());
        };
        let action_tx = self.action_tx.clone();
        match event {
            Event::Quit => action_tx.send(Action::Quit)?,
            Event::Tick => action_tx.send(Action::Tick)?,
            Event::Render => action_tx.send(Action::Render)?,
            Event::Resize(x, y) => action_tx.send(Action::Resize(x, y))?,
            Event::Key(key) => self.handle_key_event(key)?,
            _ => {}
        }
        for component in self.components.iter_mut() {
            if let Some(action) = component.handle_events(Some(event.clone()))? {
                action_tx.send(action)?;
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        let action_tx = self.action_tx.clone();
        info!("Getting keymap for {:?}", self.mode);
        let keymap = {
            let Some(universal_keybinds) = self.config.keybindings.get(&Mode::Universal) else {
                return Ok(());
            };
            let list_keymap =
                if self.mode.to_string().ends_with("List") || self.mode == Mode::MainMenu {
                    match self.config.keybindings.get(&Mode::List) {
                        Some(l_map) => l_map,
                        None => &HashMap::new(),
                    }
                } else {
                    &HashMap::new()
                };
            let tabs_keymap = if self.mode.to_string().ends_with("Tabs") {
                match self.config.keybindings.get(&Mode::Tabs) {
                    Some(t_map) => t_map,
                    None => &HashMap::new(),
                }
            } else {
                &HashMap::new()
            };
            let mode_keymap = match self.config.keybindings.get(&self.mode) {
                Some(m_map) => m_map,
                None => &HashMap::new(),
            };
            universal_keybinds
                .iter()
                .chain(list_keymap.iter())
                .chain(tabs_keymap.iter())
                .chain(mode_keymap.iter())
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect::<HashMap<_, _>>()
        };
        match keymap.get(&vec![key]) {
            Some(action) => {
                info!("Got action: {action:?}");
                action_tx.send(action.clone())?;
            }
            _ => {
                // If the key was not handled as a single key action,
                // then consider it for multi-key combinations.
                self.last_tick_key_events.push(key);

                // Check for multi-key combinations
                if let Some(action) = keymap.get(&self.last_tick_key_events) {
                    info!("Got action: {action:?}");
                    action_tx.send(action.clone())?;
                }
            }
        }
        Ok(())
    }

    fn handle_actions(&mut self, tui: &mut Tui) -> Result<()> {
        while let Ok(action) = self.action_rx.try_recv() {
            if action != Action::Tick && action != Action::Render {
                debug!("{action:?}");
            }
            match action {
                Action::Tick => {
                    self.last_tick_key_events.drain(..);
                }
                Action::Quit => self.should_quit = true,
                Action::Suspend => self.should_suspend = true,
                Action::Resume => self.should_suspend = false,
                Action::ClearScreen => tui.terminal.clear()?,
                Action::Resize(w, h) => self.handle_resize(tui, w, h)?,
                Action::Render => self.render(tui)?,
                Action::ChangeMode(mode) => self.mode = mode,
                Action::ChangePage(page) => self.page = page,
                Action::FocusMainMenu => self.mode = Mode::MainMenu,
                _ => {}
            }
            for component in self.components.iter_mut() {
                if let Some(action) = component.update(action.clone())? {
                    self.action_tx.send(action)?
                };
            }
        }
        Ok(())
    }

    fn handle_resize(&mut self, tui: &mut Tui, w: u16, h: u16) -> Result<()> {
        tui.resize(Rect::new(0, 0, w, h))?;
        self.render(tui)?;
        Ok(())
    }

    fn render(&mut self, tui: &mut Tui) -> Result<()> {
        tui.draw(|frame| {
            let [main, help] =
                Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).areas(frame.area());
            let [menu, page] =
                Layout::horizontal([Constraint::Percentage(20), Constraint::Percentage(80)])
                    .areas(main);

            let layout_areas = HashMap::from([
                ("main", main),
                ("help", help),
                ("menu", menu),
                ("page", page),
            ]);

            for component in self.components.iter_mut() {
                if let Err(err) = component.draw(frame, &layout_areas) {
                    let _ = self
                        .action_tx
                        .send(Action::Error(format!("Failed to draw: {:?}", err)));
                }
            }
        })?;
        Ok(())
    }
}

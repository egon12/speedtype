use crate::event::{AppEvent, Event, EventHandler};
use crate::sentences::{TypingSession, SAMPLE_SENTENCES};
use crate::my_logger::MyLogger;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};
use rand::Rng;
use std::io;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Current typing session
    pub typing_session: TypingSession,
    /// Event handler.
    pub events: EventHandler,

    pub logger: MyLogger,
}

impl Default for App {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let sentence_index = rng.gen_range(0..SAMPLE_SENTENCES.len());
        let sentence = SAMPLE_SENTENCES[sentence_index].to_string();
        
        
        Self {
            running: true,
            typing_session: TypingSession::new(sentence),
            events: EventHandler::new(),
            logger: MyLogger::new(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event) => self.handle_key_events(key_event)?,
                    _ => {}
                },
                Event::App(app_event) => match app_event {
                    AppEvent::Quit => self.quit(),
                },
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> io::Result<()> {
        match key_event.code {
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            KeyCode::Char(ch) => {
                // Handle regular character input
                if ch == ' ' {
                    self.typing_session.handle_space();
                } else {
                    self.typing_session.add_character(ch);
                }
            }
            KeyCode::Backspace => {
                // Handle backspace
                self.typing_session.handle_backspace();
            }
            KeyCode::Enter => {
                // Restart with new sentence if current one is completed
                if self.typing_session.is_completed() {
                    self.restart_typing();
                }
            }
            KeyCode::Tab => {
                // Restart typing session
                self.restart_typing();
            }
            _ => {}
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Restart the typing session with a new random sentence
    pub fn restart_typing(&mut self) {
        let mut rng = rand::thread_rng();
        let sentence_index = rng.gen_range(0..SAMPLE_SENTENCES.len());
        let sentence = SAMPLE_SENTENCES[sentence_index].to_string();
        self.typing_session = TypingSession::new(sentence);
    }

    /// Get the current typing session
    pub fn get_typing_session(&self) -> &TypingSession {
        &self.typing_session
    }
}

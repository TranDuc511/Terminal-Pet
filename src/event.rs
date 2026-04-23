// event.rs — Unified event loop: keyboard input + tick timer
//
// Spawns a background thread that reads crossterm events and sends them
// alongside periodic Tick events through an mpsc channel to the main loop.

use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};

/// Events that the application can receive.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum AppEvent {
    /// A keyboard key was pressed.
    Key(KeyEvent),
    /// A mouse event occurred (reserved for future use).
    Mouse(MouseEvent),
    /// The terminal was resized (reserved for future use).
    Resize(u16, u16),
    /// A periodic game tick (driven by `tick_rate`).
    Tick,
}

/// Handles event polling and tick generation.
pub struct EventHandler {
    /// Receiving end of the event channel.
    pub receiver: mpsc::Receiver<AppEvent>,
}

impl EventHandler {
    /// Create a new `EventHandler` that ticks every `tick_rate` milliseconds.
    pub fn new(tick_rate_ms: u64) -> Self {
        let (sender, receiver) = mpsc::channel();
        let tick_rate = Duration::from_millis(tick_rate_ms);

        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                // How long until the next tick?
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or(Duration::ZERO);

                // Poll for a crossterm event within the remaining tick window.
                if event::poll(timeout).unwrap_or(false) {
                    match event::read() {
                        Ok(CrosstermEvent::Key(key)) => {
                            let _ = sender.send(AppEvent::Key(key));
                        }
                        Ok(CrosstermEvent::Mouse(mouse)) => {
                            let _ = sender.send(AppEvent::Mouse(mouse));
                        }
                        Ok(CrosstermEvent::Resize(w, h)) => {
                            let _ = sender.send(AppEvent::Resize(w, h));
                        }
                        _ => {}
                    }
                }

                // If a full tick has elapsed, send the Tick event.
                if last_tick.elapsed() >= tick_rate {
                    let _ = sender.send(AppEvent::Tick);
                    last_tick = Instant::now();
                }
            }
        });

        Self { receiver }
    }

    /// Receive the next event (blocks until one is available).
    pub fn next(&self) -> Result<AppEvent, mpsc::RecvError> {
        self.receiver.recv()
    }
}

//! Manages scheduled command execution with async timer support.

use std::pin::Pin;
use std::time::Duration;

use tokio::time::Sleep;

use crate::command_stream::Command;

/// Tracks a pending fire event and executes it after a configured delay.
#[derive(Default)]
pub struct Scheduler {
    pending_fire: Option<Pin<Box<Sleep>>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler { pending_fire: None }
    }

    pub fn clear_pending(&mut self) {
        self.pending_fire = None;
    }

    pub fn is_empty(&self) -> bool {
        self.pending_fire.is_none()
    }

    /// Handles a command by scheduling, canceling, or ignoring it.
    pub fn process_command(&mut self, command: Command) {
        match command {
            Command::Schedule(delay) => {
                self.pending_fire = Some(Box::pin(tokio::time::sleep(Duration::from_secs(delay))));
            }
            Command::Cancel => {
                self.clear_pending();
            }
            Command::Quit => {
                // No action needed here for Quit in Scheduler
            }
        }
    }

    /// Awaits the next scheduled fire event, or blocks indefinitely if none is pending.
    pub async fn next_fire(&mut self) {
        if let Some(ref mut timer) = self.pending_fire.take() {
            timer.await;
        } else {
            std::future::pending().await
        }
    }
}

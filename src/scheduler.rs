use std::pin::Pin;
use std::time::Duration;

use tokio::time::Sleep;

use crate::command_stream::Command;

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

    pub async fn next_fire(&mut self) {
        if let Some(ref mut timer) = self.pending_fire {
            timer.await;
            self.clear_pending();
        } else {
            std::future::pending().await
        }
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

//! Scheduler module for managing timed actuations.
//! Handles scheduling, canceling, and firing of actuations based on event stream commands.

use crate::event_stream::{Command, Event};

/// Represents a scheduled actuation with its firing time
struct Actuation {
    fire_at: u64,
}

/// Manages scheduled actuations, ensuring at most one actuation is pending at any time.
/// New schedules automatically cancel any pending actuation.
pub struct Scheduler {
    pending_actuation: Option<Actuation>,
}

impl Scheduler {
    /// Creates a new Scheduler with no pending actuations.
    pub fn new() -> Self {
        Scheduler {
            pending_actuation: None,
        }
    }

    /// Processes an event, handling Schedule and Cancel commands.
    /// Schedule commands set a new pending actuation, canceling any existing one.
    /// Cancel commands clear any pending actuation.
    pub fn process_event(&mut self, event: &Event) {
        match &event.command {
            Some(Command::Schedule(fire_after)) => {
                if let Some(pending_actuation) = &self.pending_actuation {
                    println!(
                        "[{}] cancel pending firing at {}",
                        event.time, pending_actuation.fire_at
                    );
                }

                println!("[{}] schedule firing in {}", event.time, fire_after);
                self.pending_actuation = Some(Actuation {
                    fire_at: event.time + *fire_after,
                });
            }
            Some(Command::Cancel) => {
                println!("[{}] cancel any pending firing", event.time);
                self.pending_actuation = None;
            }
            None => {
                // No command, just a time tick
            }
        }
    }

    /// Fires any pending actuation if its scheduled time matches the current time.
    /// Clears the pending actuation after firing.
    pub fn fire_ready_actuations(&mut self, current_time: u64) {
        if let Some(actuation) = &self.pending_actuation
            && actuation.fire_at == current_time
        {
            println!("[{}] firing now!", current_time);
            self.pending_actuation = None;
        }
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

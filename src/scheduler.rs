use crate::event_stream::{Command, Event};

/// Represents a scheduled actuation with its firing time
struct Actuation {
    fire_at: u64,
}

pub struct Scheduler {
    pending_actuation: Option<Actuation>,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            pending_actuation: None,
        }
    }

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
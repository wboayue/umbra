pub mod event_stream;
pub mod scheduler;

use event_stream::{Command, EventStream};

/// Represents a scheduled actuation with its firing time
struct Actuation {
    fire_at: u64,
}

// scheduler 

/// Processes events from the stream, managing scheduled actuations.
/// * Handles Schedule commands to set firing times
/// * Handles Cancel commands to clear pending actuations
/// * Fires when the event time matches the scheduled time
fn main() {
    let mut pending_actuation: Option<Actuation> = None;
    let event_stream = EventStream::new();

    for event in event_stream {
        // Process incoming commands
        match &event.command {
            Some(Command::Schedule(fire_after)) => {
                if let Some(pending_actuation) = &pending_actuation {
                    println!(
                        "[{}] cancel pending firing at {}",
                        event.time, pending_actuation.fire_at
                    );
                }

                println!("[{}] schedule firing in {}", event.time, fire_after);
                pending_actuation = Some(Actuation {
                    fire_at: event.time + *fire_after,
                });
            }
            Some(Command::Cancel) => {
                println!("[{}] cancel any pending firing", event.time);
                pending_actuation = None;
            }
            None => {
                // No command, just a time tick
            }
        }

        // Check if it's time to fire the pending actuation
        if let Some(actuation) = &pending_actuation
            && actuation.fire_at == event.time
        {
            println!("[{}] firing now!", event.time);
            pending_actuation = None;
        }
    }
}

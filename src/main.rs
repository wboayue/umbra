pub mod event_stream;
pub mod scheduler;

use std::io;

use event_stream::{EventStream};
use scheduler::Scheduler;

/// Processes events from the stream, managing scheduled actuations.
/// * Handles Schedule commands to set firing times
/// * Handles Cancel commands to clear pending actuations
/// * Fires when the event time matches the scheduled time
fn main() {
    let stdin = io::stdin().lock();
    let event_stream = EventStream::new(stdin);
    let mut scheduler = Scheduler::new();
    
    for event in event_stream {
        scheduler.process_event(&event);
        scheduler.fire_ready_actuations(event.time);
    }
}

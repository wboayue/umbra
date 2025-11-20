//! Event stream processing module that reads timed commands from stdin
//! and generates a continuous stream of time-stepped events.

use std::io::{self, BufRead};

/// Command types that can be issued at specific times
#[derive(PartialEq, Debug)]
pub enum Command {
    /// Schedule an actuation to fire after the specified duration
    Schedule(u64),
    /// Cancel any pending actuation
    Cancel,
}

/// An event at a specific time, optionally containing a command
pub struct Event {
    pub time: u64,
    pub command: Option<Command>,
}

/// Iterator that reads commands from stdin and produces time-stepped events.
/// Input format: `time\tfire_after` where negative fire_after indicates Cancel.
/// Fills in time ticks between command events up to the last scheduled time.
pub struct EventStream {
    stdin: io::Stdin,
    pending_event: Option<Event>,
    last_time: u64,
    current_time: u64,
}

impl EventStream {
    pub fn new() -> Self {
        EventStream {
            stdin: io::stdin(),
            last_time: 0,
            current_time: 0,
            pending_event: None,
        }
    }

    /// Parses a tab-separated line into an Event.
    /// Format: `time\tfire_after` where negative fire_after means Cancel.
    fn parse_event(&mut self, line: &str) -> Option<Event> {
        let parts: Vec<&str> = line.splitn(2, '\t').collect();
        if parts.len() != 2 {
            return None;
        }
        let time = parts[0].parse::<u64>().ok()?;
        let fire_after = parts[1].parse::<i64>().ok()?;

        let command = if fire_after < 0 {
            Command::Cancel
        } else {
            Command::Schedule(fire_after as u64)
        };

        Some(Event {
            time,
            command: Some(command),
        })
    }

    /// Processes an event, filling in time ticks if needed before the event time.
    /// Updates last_time to track when scheduled actuations should complete.
    fn process_event(&mut self, tick: Event) -> Event {
        // Fill in time ticks if event is in the future
        if self.current_time < tick.time {
            return self.advance_time(Some(tick));
        }

        self.current_time = tick.time + 1;

        // Track the latest scheduled firing time
        if let Some(Command::Schedule(fire_after)) = &tick.command {
            self.last_time = tick.time + *fire_after;
        }

        tick
    }

    /// Advances time by one tick, returning an event with no command.
    /// Stores the pending event for processing in the next iteration.
    fn advance_time(&mut self, pending: Option<Event>) -> Event {
        self.pending_event = pending;
        let time = self.current_time;
        self.current_time += 1;

        Event {
            time,
            command: None,
        }
    }
}

impl Default for EventStream {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for EventStream {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        // Process any pending event first
        if let Some(pending) = self.pending_event.take() {
            if self.current_time < pending.time {
                return Some(self.advance_time(Some(pending)));
            } else {
                return Some(self.process_event(pending));
            }
        }

        // Read next line from stdin
        let mut stdin = self.stdin.lock();
        let mut buf = String::new();
        match stdin.read_line(&mut buf) {
            Ok(0) => {
                // EOF: continue time ticks until last scheduled time
                if self.current_time <= self.last_time {
                    Some(self.advance_time(None))
                } else {
                    None
                }
            }
            Ok(_) => {
                let event = self.parse_event(buf.trim())?;
                // Stops on error or invalid line. Could skip invalid lines instead.
                Some(self.process_event(event))
            }
            Err(err) => {
                eprintln!("Error reading stdin: {}", err);
                None
            }
        }
    }
}

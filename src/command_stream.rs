//! Parses commands from an async input stream.

use tokio::io::{AsyncBufRead, AsyncBufReadExt, Lines};

/// A scheduler command parsed from input.
pub enum Command {
    /// Schedule a fire event after the given delay in seconds.
    Schedule(u64),
    /// Cancel any pending fire event.
    Cancel,
    /// Signal to terminate the scheduler.
    Quit,
}

/// Reads lines from an async reader and parses them into commands.
pub struct CommandStream<R> {
    lines: Lines<R>,
}

impl<R: AsyncBufRead + Unpin> CommandStream<R> {
    pub fn new(reader: R) -> Self {
        CommandStream {
            lines: reader.lines(),
        }
    }

    /// Parses the next command from input. Returns `Quit` on EOF.
    pub async fn next_command(&mut self) -> Command {
        loop {
            match self.lines.next_line().await {
                Ok(Some(line)) => {
                    let trimmed = line.trim();
                    let delay = if let Ok(delay) = trimmed.parse::<i64>() {
                        delay
                    } else {
                        continue;
                    };

                    if delay >= 0 {
                        return Command::Schedule(delay as u64);
                    }

                    if delay == -1 {
                        return Command::Cancel;
                    }

                    // Invalid input, ignore and wait for the next command
                }
                Ok(None) => return Command::Quit,
                Err(_) => continue,
            }
        }
    }
}

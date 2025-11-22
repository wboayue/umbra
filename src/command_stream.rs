use tokio::io::{AsyncBufRead, AsyncBufReadExt, Lines};

pub enum Command {
    Schedule(u64),
    Cancel,
    Quit,
}

pub struct CommandStream<R> {
    lines: Lines<R>,
}

impl<R: AsyncBufRead + Unpin> CommandStream<R> {
    pub fn new(reader: R) -> Self {
        CommandStream {
            lines: reader.lines(),
        }
    }

    pub async fn next_command(&mut self) -> Command {
        loop {
            match self.lines.next_line().await {
                Ok(Some(line)) => {
                    let trimmed = line.trim();
                    if let Ok(delay) = trimmed.parse::<i64>() {
                        if delay < 0 {
                            return Command::Cancel;
                        } else {
                            return Command::Schedule(delay as u64);
                        }
                    }
                    // Invalid input, ignore and wait for the next command
                }
                Ok(None) => return Command::Quit,
                Err(_) => continue,
            }
        }
    }
}

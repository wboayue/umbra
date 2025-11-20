use std::io::{self, BufRead};

struct Tick {
    time: u64,
    fire_in: Option<i64>,
}

struct EventStream {
    stdin: io::Stdin,
    pending_event: Option<Tick>,
    last_time: u64,
    current_time: u64,
}

impl EventStream {
    fn new() -> Self {
        EventStream {
            stdin: io::stdin(),
            last_time: 0,
            current_time: 0,
            pending_event: None,
        }
    }

    fn parse_event(&mut self, line: &str) -> Option<Tick> {
        let parts: Vec<&str> = line.splitn(2, '\t').collect();
        if parts.len() != 2 {
            return None;
        }
        let time = parts[0].parse::<u64>().ok()?;
        let fire_id = parts[1].parse::<i64>().ok()?;
        Some(Tick {
            time,
            fire_in: Some(fire_id),
        })
    }

    fn process_event(&mut self, tick: Tick) -> Tick {
        if self.current_time < tick.time {
            return self.advance_time(Some(tick));
        }

        self.current_time = tick.time + 1;

        if let Some(fire_in) = tick.fire_in
            && fire_in != -1
        {
            self.last_time = tick.time + fire_in as u64;
        }

        Tick {
            time: tick.time,
            fire_in: tick.fire_in,
        }
    }

    fn advance_time(&mut self, pending: Option<Tick>) -> Tick {
        self.pending_event = pending;
        let time = self.current_time;
        self.current_time += 1;

        return Tick {
            time,
            fire_in: None,
        };
    }
}

impl Iterator for EventStream {
    type Item = Tick;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pending) = self.pending_event.take() {
            if self.current_time < pending.time {
                return Some(self.advance_time(Some(pending)));
            } else {
                return Some(self.process_event(pending));
            }
        }

        let mut stdin = self.stdin.lock();
        let mut buf = String::new();
        match stdin.read_line(&mut buf) {
            Ok(0) => {
                if self.current_time <= self.last_time {
                    Some(self.advance_time(None))
                } else {
                    None
                }
            } // EOF
            Ok(_) => {
                let event = self.parse_event(buf.trim())?;
                // stops on error or invalid line. could skip invalid lines instead.
                Some(self.process_event(event))
            }
            Err(err) => {
                eprintln!("Error reading stdin: {}", err);
                None
            }
        }
    }
}

struct Command {
    fire_at: u64,
}

fn main() {
    let mut pending_command: Option<Command> = None;
    let event_stream = EventStream::new();

    for tick in event_stream {
        if let Some(cmd) = &tick.fire_in {
            if *cmd == -1 {
                println!("{} Cancel at {} ticks", tick.time, cmd);
                pending_command = None;
            } else {
                println!("{} Scheduling command to fire in {} ticks", tick.time, cmd);
                if pending_command.is_some() {
                    println!("{} Cancel at {} ticks", tick.time, pending_command.unwrap().fire_at);
                }
                pending_command = Some(Command { fire_at: tick.time + *cmd as u64 });
            }
        }

        if let Some(cmd) = &pending_command && cmd.fire_at == tick.time {
            println!("Firing command at time {}", tick.time);
            pending_command = None;
        }
    }
}

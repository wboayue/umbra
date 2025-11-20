use std::io::{self, BufRead};

struct Tick {
    time: u64,
    fire_in: Option<u64>,
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
        let fire_id = parts[1].parse::<u64>().ok()?;
        Some(Tick {
            time,
            fire_in: Some(fire_id),
        })
    }

    fn process_event(&mut self, tick: Tick) -> Tick {
        if self.current_time < tick.time {
            self.pending_event = Some(tick);
            let time = self.current_time;
            self.current_time += 1;

            return Tick {
                time,
                fire_in: None,
            };
        }
        self.current_time = tick.time + 1;
        self.last_time = tick.time + tick.fire_in.unwrap_or(0);

        Tick {
            time: tick.time,
            fire_in: tick.fire_in,
        }
    }
}

impl Iterator for EventStream {
    type Item = Tick;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pending) = self.pending_event.take() {
            if self.current_time < pending.time {
                self.pending_event = Some(pending);
                let time = self.current_time;
                self.current_time += 1;
                return Some(Tick {
                    time,
                    fire_in: None,
                });
            } else {
                return Some(self.process_event(pending));
            }
        }

        let mut stdin = self.stdin.lock();
        let mut buf = String::new();
        match stdin.read_line(&mut buf) {
            Ok(0) => {
                if self.current_time <= self.last_time {
                    let time = self.current_time;
                    self.current_time += 1;
                    return Some(Tick {
                        time,
                        fire_in: None,
                    });
                } else {
                    return None;
                }
            } // EOF
            Ok(_) => {
                let event = self.parse_event(buf.trim())?;
                // stops on error or invalid line. could skip invalid lines instead.
                return Some(self.process_event(event));
            }
            Err(err) => {
                eprintln!("Error reading stdin: {}", err);
                return None;
            }
        }
    }
}

fn main() {
    let event_stream = EventStream::new();
    for tick in event_stream {
        println!("Time: {}, Command: {:?}", tick.time, tick.fire_in);
    }
}

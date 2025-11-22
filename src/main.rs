use std::pin::Pin;
use std::time::Duration;

use tokio::{io::BufReader, time::Sleep};
use tokio::io::AsyncBufReadExt;

 #[tokio::main]
async fn main() {
    let stdin = BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();
    let mut pending_fire: Option<Pin<Box<Sleep>>> = None;

    loop {
        tokio::select! {
            // Branch 1: New input received
            line = lines.next_line() => {
                match line {
                    Ok(Some(text)) => {
                        match parse_command(&text) {
                            Some(delay) if delay < 0 => {
                                // Cancel
                                pending_fire = None;
                            }
                            Some(delay) => {
                                // Schedule (overwrites any existing)
                                pending_fire = Some(Box::pin(
                                    tokio::time::sleep(Duration::from_secs(delay as u64))
                                ));
                            }
                            None => {} // Invalid input, ignore
                        }
                    }
                    Ok(None) => break, // EOF
                    Err(_) => break,   // Error
                }
            }

            // Branch 2: Timer expired
            _ = async {
                if let Some(ref mut timer) = pending_fire {
                    timer.await
                } else {
                    std::future::pending().await
                }
            } => {
                println!("firing now!");
                pending_fire = None;
            }
        }
    }
}

fn parse_command(line: &str) -> Option<i64> {
    line.trim().parse::<i64>().ok()
}

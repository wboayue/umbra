//! Async flight computer scheduler that reads commands from stdin and executes them at scheduled times.

use crate::scheduler::Scheduler;

pub mod command_stream;
pub mod scheduler;

use command_stream::{Command, CommandStream};

#[tokio::main]
async fn main() {
    let reader = tokio::io::BufReader::new(tokio::io::stdin());
    let mut command_stream = CommandStream::new(reader);
    
    let mut scheduler = Scheduler::default();

    loop {
        tokio::select! {
            command = command_stream.next_command() => {
                match command {
                    Command::Quit => {
                        if scheduler.is_empty() {
                            break;
                        }
                    }
                    _ => {
                        scheduler.process_command(command);
                    }
                }
            }

            _ = scheduler.next_fire() => {
                println!("firing now!");
            }
        }
    }
}

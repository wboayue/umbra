use crate::scheduler::Scheduler;

pub mod command_stream;
pub mod scheduler;

use command_stream::{Command, CommandStream};

#[tokio::main]
async fn main() {
    let mut scheduler = Scheduler::new();
    let mut command_stream = CommandStream::new(tokio::io::BufReader::new(tokio::io::stdin()));

    loop {
        tokio::select! {
            command = command_stream.next_command() => {
                match command {
                    Command::Quit => {
                        break;
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

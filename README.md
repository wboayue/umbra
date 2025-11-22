Solution by [Wil Boayue](https://wboayue.com) for the Umbra coding problem.

## Quickstart

Tested with `rustc 1.87.0`.

```bash
cargo run
```

The program reads delay values from stdin and fires after the scheduled delay elapses.

## Design Decisions

### Runtime

Uses the tokio async runtime to handle concurrent scheduling and input processing with `tokio::select!`.

## Architecture

The solution consists of three components:

- **CommandStream**: Asynchronously reads commands from stdin, parsing each line as an integer. Positive values create schedule commands, negative values create cancel commands.

- **Scheduler**: Maintains at most one pending actuation at a time using tokio timers. Processes schedule and cancel commands, and exposes a future that completes when the scheduled time arrives.

- **Main loop**: Uses `tokio::select!` to concurrently wait for either new commands from stdin or the pending timer to fire, processing whichever occurs first.
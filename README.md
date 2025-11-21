Solution by [Wil Boayue](https://wboayue.com) for the Umbra coding problem.

## Quickstart

Tested with `rustc 1.87.0`.

```bash
cargo run < events.csv
```

You should get output similar to:

```text
[0] schedule firing in 15
[2] cancel pending firing at 15
[2] schedule firing in 30
[32] firing now!
```

## Design Decisions

### Runtime

No need for an async runtime, so I went with the standard library and no additional dependencies.

### Input

I primarily considered two input formats:

1. **`command` only per line**: Simple format supporting infinite time steps, but could result in many empty lines for time steps without commands. Less readable for humans.

2. **`time_step,command` per line**: More compact and readable, but requires slightly more parsing work. Risk of events being specified out of order.

I chose the `time_step,command` format for readability.

```csv
0,15
2,30
```

### Output

I made the scheduling messages more descriptive, added a cancel message, and included time steps in all messages.

## Architecture

The solution consists of three components:

- **EventStream**: Reads commands from stdin and produces a continuous stream of time-stepped events. It generates synthetic time ticks to fill gaps between commands and continues generating events until all scheduled actuations have fired.

- **Scheduler**: Maintains at most one pending actuation at a time. Processes schedule and cancel commands, automatically canceling any existing actuation when a new one is scheduled, and fires actuations when their scheduled time arrives.

- **Main loop**: Pulls events from the EventStream and forwards them to the Scheduler for processing.
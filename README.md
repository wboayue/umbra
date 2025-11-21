Solution by [Wil Boayue](https://wboayue.com) for the Umbra coding problem.

## Quickstart

Tested with `rustc 1.87.0`.

```bash
cargo run < events.csv
```

You should get output similar to

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

### Output

I made the scheduling messages more descriptive, added a cancel message, and included time steps in all messages.

## Architecture

The solution consists of three components:

- **EventStream**: Reads from stdin and generates a stream of events for each time step until there is no pending actuation. Events always contain a time step and may contain an actuation or cancel command.

- **Scheduler**: Processes actuation and cancel commands and fires actuations for the given time step.

- **Main control loop**: Ties everything together by instantiating the EventStream and Scheduler, pulling events from the stream, and forwarding them to the Scheduler.
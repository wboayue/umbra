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

No need for async runtime so went with standard library and no additional dependencies.

### Input

Primary considered two input formats. One with `time_step,command` per line and only `command` per line.

Only a `command` per line is a simple format supporting infinite time steps. Downside is file could potentially have many empty line for time steps without commands. Less readable for human.

With a `time_step,command` per line you end up with a compact readable file but have a little more work parsing the file. Cons are limiting the number of time steps that can be specified and opening yourself to errors file events being specified out of order.

### Output

I made the scheduling messages more descriptive, added a cancel message, and included time steps in all messages.

## Architecture

The solution consist of 3 components. An EventStream, a Scheduler and the control loop.

The EventStream reads from stdin and generate a stream of events for each time step until there are no longer pending actuation. Events always contain a time step and may contain an Actuation or cancel command.

The Scheduler processes actuation and cancel commands and fire actuation commands for the given timestep.

The main control loop ties every things together. Instantiating the eventsteam and scheduler, pulling events from the stream and forwarding them to the scheduler.
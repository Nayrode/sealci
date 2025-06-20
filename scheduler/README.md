# Scheduler technical documentation

## Dependencies

- `sealcid_traits` to implement the `App` trait common to SealCI services.
- `tonic` for gRPC and `prost` for protobuf.
- `tokio` packages for asynchronicity.
- `tracing` for textual logging.
- `clap` for the CLI.
- `http`, only used to parse Agent addresses into valid URIs for Tonic.

This microservice uses Tonic. Tonic (gRPC) makes use of Prost (protobuf), and is built on top of the tokio stack.
We also use the `tracing` package for logging.

## Commands

Place yourself in the scheduler service's directory (`sealci/scheduler/`)

To start the Scheduler service:

```bash
cargo run
```

With 'debug' logging level

```bash
RUST_LOG=debug cargo run
```

More logging levels (by order of increasing verbosity): 'error', 'warn', 'info', 'debug', 'trace'.

To launch integration tests

```bash
cargo test
```

You can also launch them individually:

```bash
cargo test --test name_of_test
```

The command used to build (or run) the server for production is:

```bash
RUST_LOG=info cargo build -r
```

> Note: `--release` is the same as `-r`. It builds the binary with optimizations.

### With Docker Compose

Place yourself in the project root directory (`sealci/`)

Building the image and starting the container:

```bash
docker compose up --build -d
```

As per the `docker-compose.yml` file:

- The container will automatically restart (`restart: always` policy).
- The container will run on `0.0.0.0:5005`. That is equivalent to `[::]:5005` or `[::0]:5005` in IPv6.

You can then follow the logs:

```bash
docker compose logs -f
```

## Testing with grpcurl

`grpcurl` is a command-line tool that lets you interact with gRPC servers.

Testing Agent registration locally:

```bash
$ grpcurl -plaintext -d '{
  "health": {
    "cpu_avail": 4,
    "memory_avail": 8192
  },
  "hostname": {
    "host": "127.0.0.1",
    "port": 50051
  }
}' localhost:50051 scheduler.Agent/RegisterAgent

{
  "id": 1
}
```

Testing Agent health status report locally:

```bash
$ grpcurl -d @ -plaintext [::1]:50051 scheduler.Agent.ReportHealthStatus <<EOM
{
  "agent_id": 1,
  "health": {
    "cpu_avail": 70,
    "memory_avail": 2048
  }
}
{
  "agent_id": 3,
  "health": {
    "cpu_avail": 65,
    "memory_avail": 1980
  }
}
EOM

{}
```

## File structure and modules

Explanations of the Scheduler implementation architecture.

### .gitignore

Contains

```.gitignore
/target
```

As to not commit compiled binaries, dependencies, and other artifacts into version control.

### target/

This directory is created by `cargo` during `cargo run`
Cargo is Rust's build system and package manager.
*It does not appear if you have not run the project*

> Tip: you can use `cargo clean` to clean remove artifacts generated by cargo from the target directory.

### Cargo.lock

This file contains the state of resolved dependencies. It is generated automatically.

### Cargo.toml

This file describes our dependencies, and our binary targets.
The binary targets are the programs we can run.
If you try to execute `cargo run`, the following (or similar) will display:

```bash
$ cargo run
error: `cargo run` could not determine which binary to run. Use the `--bin` option to specify a binary, or the `default-run` manifest key.
available binaries: server
```

It shows the available target binaries to run. You can run them with `cargo run --bin <name_of_bin>`.

### build.rs

This script is compiled and executed by Cargo before building the package.
Notably, we point to the .proto files in this script.

### src/main.rs

This file contains the server.
It calls the code generated by Tonic from the .proto, as well as the code for the gRPC `interfaces/`.

### tests/

Contains integration tests to test mock data against the gRPC interfaces.
Each of these tests runs the gRPC server and then runs its tests against it. To launch the server, they import modules from the `scheduler` package, made available by lib.rs
You can run the integration tests with `cargo test`.

### lib.rs

This file defines the modules that are publicly available / can be imported this package.
They can be called using `use scheduler::...`, "scheduler" being the name of our crate.

The `interfaces` and `proto` modules were made public to be imported outside the crate, as they are needed in the integration tests in `tests/` to launch a mock instance of the scheduler server.

### src/proto/mod.rs

This module definition imports the code generated from the gRPC protos as a module to make it easier to import and use in the crate or externally.

### src/interfaces/

Contains the gRPC interfaces implementation.
That is, handling requests, responses, streams...
There is no scheduler logic within that source code, only gRPC implementations, gRPC errors handling.

The only context known by this code is gRPC.
As such, this code should not handle any other errors than gRPC errors.

This code calls the Scheduler logic implementation defined in `src/logic/*`
All context relative to the Scheduler logic implementation (such as inputs from gRPC requests) is passed down to this code.

### src/logic/

Contains the Scheduler logic implementation.

The only context known by this code is the Scheduler logic. That is its procedures, data structures...
As such, this code should be handling only errors relative to the Scheduler implementation.

This code is called by the gRPC interfaces implementation defined in `src/interfaces/*`

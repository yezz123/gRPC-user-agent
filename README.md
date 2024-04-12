# User Agent Analyzer

This project implements a gRPC server and client in Rust that can analyze HTTP user agent strings and make a decision whether to block or allow the request.

## Usage

### Running the server

To run the server, execute:

```sh
cargo run --bin server
```

The server will start listening on port 50051.

### Running the client

To run the client, execute:

```sh
cargo run --bin client -- "<user_agent>"
```

Replace `<user_agent>` with the user agent string you want to analyze.

The client will send the user agent string to the server and print the decision to the console.

### Testing

To run the tests, execute:

```sh
cargo test
```

The tests will automatically read user agent strings from `data/user_agents.txt`.

# Actix simple API

Kickoff to actix.


## Previous steps

Check your rust version, the minimum required is 1.42:
```sh
rustc --version
```

Install mysql client DEV, i.e. `sudo apt-get install libmysqlclient-dev` or the linux package manager that your distro uses.

Install Diesel cli: `cargo install diesel_cli --no-default-features --features mysql`

Copy `.env.dist` to `.env` an customize the parameters.

Run `diesel migration run`.

## Run

```sh
cargo build
./target/debug/hello_cargo
```

..or its all-in-one command:
```sh
cargo run
```

Navigate to http://localhost:8080/ .



## Creating entities

Create a new Queryable and a new Insertable in `src/models.rs`.
Run `diesel print-schema > src/schema.rs`

## Troubleshooting

```
error: linking with `cc` failed: exit status: 1
```
Check if you have installed the correct database client in your system (for mysql see intructions in https://github.com/sgrif/mysqlclient-sys).


Other issues: try running
```
cargo clean
cargo update
cargo build --all-features
```

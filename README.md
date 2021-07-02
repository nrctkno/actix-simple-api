# Actix simple API

Kickoff to actix.


## Previous steps

Check your rust version, the minimum required is 1.42:
```sh
rustc --version
```

Install mysql client DEV, i.e. `sudo apt-get install libmysqlclient-dev` or the linux package manager that your distro uses.

Copy `.env.dist` to `.env` an customize the parameters.

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


## Install aditional tools

Cargo: `cargo install diesel_cli --no-default-features --features mysql`


## Troubleshooting

```
error: linking with `cc` failed: exit status: 1
```
Run:
```
cargo clean
cargo update
cargo build --all-features
```



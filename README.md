Rusty Rubik
==

**Rusty Rubik** is a Rubik's Cube solving program implemented in Rust. 


Downloading
===
You can get this repo via cloning through Git:
```
git clone https://github.com/esqu1/Rusty-Rubik
```

Running
===
It is preferable to run the program in release mode, since this will dramatically speed up the search time for a solution:
```
cargo run --release
```
If you don't want to run it in release mode, simply leave out the `--release` flag.

To just build the project:
```
cargo build --release
```

Documentation
===
You can build the documentation for this project via:
```
cargo doc --no-deps --open
```
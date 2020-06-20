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
It is preferable to run the program in release mode, since this will dramatically speed up the search time for a solution. First, build the project:
```
cargo build --release
```

If you don't want to run it in release mode, simply leave out the `--release` flag. You will first need to generate the pruning tables necessary for the IDA solver to run:
```
cargo run --release -- -p
```
This will create pruning tables `corners.pt`, `edges_o.pt`, and `edges_p.pt` in the root directory of the project. To verify that these were generated correctly, run the verification script:
```
./scripts/verify_checksum.sh
```

Then you can run the executable in `target/release/rusty-rubik`. You can see the available options using the `--help` flag:
```
./target/release/rusty-rubik --help
```


Documentation
===
You can build the documentation for this project via:
```
cargo doc --no-deps --open
```

Rusty Rubik
==
[![esqu1](https://circleci.com/gh/esqu1/Rusty-Rubik.svg?style=svg)](https://circleci.com/gh/esqu1/Rusty-Rubik)

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

On my CPU (AMD Ryzen 5 3600 @ 3.6 GHz, 6 cores) the pruning tables take about 10 minutes to generate, so I'm expecting most modern processors should take around 15-20 minutes to finish.  

Then you can run the executable in `target/release/rusty-rubik`. You can see the available options using the `--help` flag:
```
./target/release/rusty-rubik --help
```


Documentation
===
The document requires the KaTeX header file to be built with it to
render properly. You can build the documentation for this project via:
```
RUSTDOCFLAGS="--html-in-header header.html" cargo doc --no-deps --open
```

## An Implementation of the _Java Discrete Events Simulation_ Lab in Rust

### Why Rust?
In short the Java implementation is not restrictive enough in access to the Simulation by invoked events, stop condition etc.
- Event `invoke` can call `simulate()`.
- The Simulation `stop()` method can access `simulate()`, `schedule(...)` and has unrestricted access to the state of the class it is overriden in.
- The simulation is still usable after `simulate()` is complete, though the simulation may be in an unusable state (e.g nonempty queue but stop completed)
- Any method can be overriden, so any child class implementation can cause bugs through `getCurrentTime()`, `simulate()`, `stop()`, etc (classic inheritance pitfalls)
- Getting arguments in a type safe & failure handled way is complex

In Rust we can avoid these by:
- Being compelled to use composition instead of inheritance
- Using ownership to restrict access to public methods (e.g a method can take `&self`, `&mut self`, or `self`)
- Eliminating potential bugs (passing null references)
- Easy argument parsing with clap

### Project Structure
```bash
.
├── Cargo.toml          # Project Configuration (dependencies and )
├── Cargo.lock
├── rust-toolchain.toml # Toolchain Config (rust version)
├── README.md           # This readme
|
├── src
│   ├── main.rs         
│   ├── print3.rs
│   ├── simulation.rs
│   ├── ssq.rs
│   └── ticks.rs
|
└── target # Generated on build
    ├── debug    
    ├── doc      
    └── release
```


### How to build?
Firstly [get Rust](https://rustup.rs/), I recommend using Rust-Analyser with VSCode.

To just run:
```bash
cargo run -- --help
```

To read the documentation:
```bash
cargo doc

# Can now open in the browser
firefox target/doc/rust_discrete_event_simulation/index.html
```

To test:
```bash
cargo test
```

To build:
```bash
cargo build           # debug build
cargo build --release 

# can now run binary
./target/debug/rust_discrete_event_simulation <args here>
./target/release/rust_discrete_event_simulation <args here>
```

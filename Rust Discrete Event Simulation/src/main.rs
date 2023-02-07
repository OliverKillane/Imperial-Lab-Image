//! ## Rust Discrete Event Simulation
//! A basic implementation of the _Java Discrete Event Simulation_ lab in Rust.
//!
//! Differences in language design make this translation difficult:
//! - No inheritance
//! - `f64` (`double`) does not implement `Ord` (can be a `NaN` -> `Ord` cannot fail)
//!
//! I have cheated and used a u32 instead, alternatively I could implement
//! Ord (some ordering, or a panic for NaN), or as a in the BinaryHeap compare.
//!
//! However there are some significant benefits:
//! - Cannot fail anywhere but assertions, not exceptions
//! - Can limit access to methods through references (e.g an event can only invoke, but cannot simulate)
//! - Can consume (after `.simulate()` the Simulation is destroyed and usage after will not compile)
//!
//! I have chosen to implement this using generics rather than `dyn`.
//!
//! `Haskell Typeclass <--> Rust Trait <--------> Java Interface`
//!
//! In java all methods are virtual, and all objects are boxed (pointer to heap
//! allocation). In Rust (and C++) either the compiler needs to be able to deduce
//! the one type
//!
//! Advantage:    Can send anything that is object (or impl that trait)
//! Disadvantage: Performance
//!
//! | Java    | Rust              |
//! |---------|-------------------|
//! | `Event` |  `Box<dyn Event>` |
//!
//! In Rust we can have the compiler work out the type at compile time, and monomorphise.
//!
//! Advantage:    Performance
//! Disadvantage: Monomorphic (Cannot have multiple different types event in a single simulation)
//!
//! | Java    | Rust    |
//! |---------|---------|
//! | nothing | `Event` |
//!
//! There are fewer places to test here as the type system eliminates much of the potentially buggy
//! behaviour that could be implemented in java. (e.g `stop()` should not change time, or checking
//! arguments are not null, or checking arguments are not negative).

mod print3;
mod simulation;
mod ssq;
mod ticks;

use clap::{command, Parser, Subcommand};
use simulation::Time;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    sim: RunSim,
}

#[derive(Subcommand, Debug)]
enum RunSim {
    Print3,
    Ticks { stop_time: Time },
    Ssq { stop_time: Time },
}

fn main() {
    match Cli::parse().sim {
        RunSim::Print3 => print3::sim(),
        RunSim::Ticks { stop_time } => ticks::sim(stop_time),
        RunSim::Ssq { stop_time } => ssq::sim(stop_time),
    }
}

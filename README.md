# Minos Clap 

Simple clap commands for minos containers.

## Usage: 
```rust
use std::{env, path::PathBuf};

use clap::Parser;
use minos::{engine::StaticContainer, Container};
use minos_clap::MinosCommand;

fn path() -> PathBuf {
    env::current_dir()
        .expect("Error running pwd")
        .join(".policies")
}

fn container() -> Container<StaticContainer> {
    Container::new(
        "EXAMPLE_MINOS_CLI".to_string(),
        "Example container for minos cli test".to_string(),
        vec![path()],
    )
    .load()
    .expect("Error loading container")
}

fn main() {
    // Parse commands
    let minos = MinosCommand::parse();

    // load container
    let container = container();


    // run 
    minos.run(&container);
}

 ```
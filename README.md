# rust-log
A simple log using standard libraries

# Usage:

- Step 1: Add log dependencies to your project as follows:

Cargo.toml:
````
[dependencies]
rust-log = { git = "https://github.com/gobkc/rust-log" }
````

- Step 2: run `cargo update`
- Step 3: now you can use log the library

# Example

````
use rust_log::{info, log};

fn main() {
    log::set_prefix("test");
    info!("{} world", "hello");
}
````

output:

````
2024-03-01 08:21:35 [test-info] hello world
````

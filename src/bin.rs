
use strix_roost::dependency::cli::{Print, Runnable};

fn main() {
    Print::try_run().unwrap_or_else(|e| {
        panic!("{}", e);
    });
}

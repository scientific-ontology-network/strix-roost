
use strix_roost::dependency::cli::{DependencyWriter, Runnable};

fn main() {
    DependencyWriter::try_run().unwrap_or_else(|e| {
        panic!("{}", e);
    });
}

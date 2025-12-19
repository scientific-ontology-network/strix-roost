
use clap::{Parser, Subcommand};
use strix_roost::dependency::cli::{DependencyWriter, Runnable};
use strix_roost::ontology::io::load_set_ontology;
use strix_roost::ontology::cli::{AnnotationWriter};

#[derive(Parser)]
#[command(name = "strix-roost")]
#[command(about = "A command line tool for ontology use-cases")]
struct Cli {
    #[arg(short, long)]
    in_path: std::path::PathBuf,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Dependency(DependencyWriter),
    Annotations(AnnotationWriter),
}

fn main() {
    match Cli::try_parse() {
        Ok(cli) => {
            let onto =
                load_set_ontology(cli.in_path.to_str().unwrap()).expect("Failed to load ontology");
            match cli.command {
                Command::Dependency(c) => {
                    c.run(onto);
                },
                Command::Annotations(c) => {
                    c.run(onto);
                },
            }
        }
        Err(err) => {panic!("{err}")}
    }
}

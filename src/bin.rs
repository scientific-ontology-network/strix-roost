use clap::{Parser, Subcommand};
use horned_owl::model::{ArcStr, ForIRI};
use horned_owl::ontology::set::SetOntology;
use strix_roost::dependency::cli::{DependencyWriter, Runnable};
use strix_roost::ontology::cli::AnnotationWriter;
use strix_roost::ontology::io::load_set_ontology;

#[derive(Parser)]
#[command(name = "strix-roost")]
#[command(about = "A command line tool for ontology use-cases")]
struct Cli {
    #[arg(short, long)]
    in_path: std::path::PathBuf,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
enum Command {
    Dependency(DependencyWriter),

    Annotations(AnnotationWriter),
}

impl Runnable<ArcStr> for Command {
    fn run(&self, onto: SetOntology<ArcStr>) {
        match self {
            Command::Dependency(c) => c.run(onto),
            Command::Annotations(c) => c.run(onto),
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let time = std::time::Instant::now();
    let onto = load_set_ontology(cli.in_path.to_str().unwrap()).expect("Failed to load ontology");
    cli.command.run(onto);
    println!("Finished in {:?}", time.elapsed().as_millis());
}

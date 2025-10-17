
use strix_roost::dependency::base::DependencyBuilder;
use strix_roost::dependency::growth::GrowthDependency;
use strix_roost::ontology::io::load_set_ontology;


fn main() {

    let path = "/home/glauer/Downloads/oeo-full.owl";
    let onto = load_set_ontology(path);
    let set_index = onto.i();
    let dependencies = GrowthDependency::build_dependencies(set_index.iter());
    let cleaned_dependencies = GrowthDependency::remove_supers(dependencies, set_index.iter());
    for (a,b) in cleaned_dependencies{
        println!("{:?} -> {:?}",a,b)
    }
}

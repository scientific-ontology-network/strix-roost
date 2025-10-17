
use strix_roost::dependency::base::DependencyBuilder;
use strix_roost::dependency::growth::GrowthDependency;
use strix_roost::dependency::base::load_rdf_ontology;


fn main() {

    let path = "/home/glauer/Downloads/oeo-full.owl";
    let onto = match load_rdf_ontology(path) {
        Ok(onto) => onto,
        Err(e) => panic!("{:?}", e),
    };

    let (set_index,_,_) = onto.index();
    let dependencies = GrowthDependency::build_dependencies(set_index.iter().map(|arc|&**arc));
    let cleaned_dependencies = GrowthDependency::remove_supers(dependencies, set_index.iter().map(|arc|&**arc));
    for (a,b) in cleaned_dependencies{
        println!("{:?} -> {:?}",a,b)
    }
}

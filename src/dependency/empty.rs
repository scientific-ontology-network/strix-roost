use std::collections::{HashMap, HashSet};
use horned_owl::model::{AnnotatedComponent, Build, Class, ClassExpression, Component, ForIRI, MutableOntology, ObjectPropertyExpression, ObjectPropertyRange, SubClassOf};
use horned_owl::ontology::set::SetOntology;
use horned_owl::vocab::OWL;
use indicatif::ProgressIterator;
use crate::dependency::base::{DependencyBuilder, DependencyMap};
use crate::dependency::symbol::{Term, Symbol};
use whelk::whelk::owl::translate_ontology;
use whelk::whelk::reasoner::assert;
use crate::dependency::syntax_based::SyntaxBasedDependency;

pub struct SemanticEmptinessDependency {}


impl<T:ForIRI> DependencyBuilder<T> for SemanticEmptinessDependency {
    fn build_dependencies<'a> (
        ontology_iter: impl Iterator<Item = &'a AnnotatedComponent<T>>,
    ) -> DependencyMap<T, HashSet<&'a Component<T>>>
    {
        let axioms : Vec<_> = ontology_iter.collect();

        let mut declared_classes = HashSet::new();
        let mut declared_roles = HashSet::new();
        for ac in axioms.iter() {
            match &ac.component {
                Component::DeclareClass(dc) => {
                    declared_classes.insert(&dc.0.0);
                },
                Component::DeclareObjectProperty(dop) => {
                    declared_roles.insert(&dop.0.0);
                },
                _ => {}
            }
        }
        let builder = Build::<T>::new();
        let mut dependencies = HashMap::new();
        let mut ontology : SetOntology<T> = SetOntology::from_iter(axioms.into_iter().cloned());
        for c in declared_classes.into_iter().progress() {
            let ax = AnnotatedComponent { component: Component::SubClassOf(SubClassOf{sub: ClassExpression::Class(Class(c.clone())), sup:ClassExpression::Class(Class(builder.iri(OWL::Nothing))) }), ann: Default::default() };
            ontology.insert(ax.clone());
            let whelk_axioms = translate_ontology(&ontology);
            let whelk = assert(&whelk_axioms);
            for (sub, sup) in whelk.named_subsumptions() {
                if (*sup).id == OWL::Nothing.to_string().as_str() {
                    let l = Symbol::Class(c.underlying());
                    let r_iri = builder.iri((*sub).id.clone());
                    let r = Symbol::Class(r_iri.underlying());
                    if !dependencies.contains_key(&l) {
                        dependencies.insert(l.clone(), HashMap::new());
                    }
                    dependencies.get_mut(&l).unwrap().insert(r.clone(), HashSet::new());
                }
            }
            ontology.remove(&ax);
        }
        dependencies.into_iter().collect()
    }
}

pub struct SyntacticEmptinessDependency {}

impl<T:ForIRI> DependencyBuilder<T> for SyntacticEmptinessDependency {
    fn build_dependencies<'a>(ontology_iter: impl Iterator<Item=&'a AnnotatedComponent<T>>) -> HashMap<Symbol<T>, HashMap<Symbol<T>, HashSet<&'a Component<T>>>>
    where
        T: 'a
    {
        Self::derive_from_axioms(ontology_iter)
    }
}

impl<T:ForIRI> SyntaxBasedDependency<T> for SyntacticEmptinessDependency {
    
    fn dependencies_from_object_property_expression(ope: &ObjectPropertyExpression<T>) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        match ope {
            ObjectPropertyExpression::ObjectProperty(_op) => { HashSet::new() },
            ObjectPropertyExpression::InverseObjectProperty(_op) => {
                println!("Inverse object properties are not supported in syntactic emptiness dependency yet. Skipping!");
                HashSet::new() },
        }
    }

    fn dependencies_from_object_intersection_of<'a>(x: &'a ClassExpression<T>, ces: &'a Vec<ClassExpression<T>>) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        ces.into_iter()
            .flat_map(|ce2| {
                [(Term::CE(x), Term::CE(ce2))]
                    .into_iter()
                    .chain(Self::dependencies_from_class_expression(ce2))
            })
            .collect()
    }

    fn dependencies_from_object_union_of<'a>(x: &'a ClassExpression<T>, ces: &'a Vec<ClassExpression<T>>) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        ces.into_iter()
            .flat_map(|ce2| {
                [(Term::CE(ce2), Term::CE(x))]
                    .into_iter()
                    .chain(Self::dependencies_from_class_expression(ce2))
            })
            .collect()
    }

    fn dependencies_from_object_some_values_from<'a>(x: &'a ClassExpression<T>, ope: &'a ObjectPropertyExpression<T>, bce: &'a ClassExpression<T>) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        [
            (Term::CE(x), Term::CE(bce)),
            (Term::CE(x), Term::Role(ope)),
        ]
            .into_iter()
            .chain(Self::dependencies_from_class_expression(bce))
            .chain(Self::dependencies_from_object_property_expression(ope))
            .collect()
    }

    // range(r) <= C
    fn dependency_from_object_property_range(_opr: &ObjectPropertyRange<T>) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        [
            (Term::Role(&_opr.ope), Term::CE(&_opr.ce)) // r -> X, X -> C
        ]
            .into_iter()
            .chain(Self::dependencies_from_class_expression(&_opr.ce))
            .chain(Self::dependencies_from_object_property_expression(&_opr.ope))
            .collect()
    }
}
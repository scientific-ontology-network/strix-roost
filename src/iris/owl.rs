/// OWL (http://www.w3.org/2002/07/owl#) vocabulary IRIs as Rust constants.
pub const NS: &str = "http://www.w3.org/2002/07/owl#";

pub const THING: &str = "http://www.w3.org/2002/07/owl#Thing";
pub const NOTHING: &str = "http://www.w3.org/2002/07/owl#Nothing";

pub const CLASS: &str = "http://www.w3.org/2002/07/owl#Class";
pub const COMPLEMENT_OF: &str = "http://www.w3.org/2002/07/owl#complementOf";
pub const INTERSECTION_OF: &str = "http://www.w3.org/2002/07/owl#intersectionOf";
pub const UNION_OF: &str = "http://www.w3.org/2002/07/owl#unionOf";
pub const ONE_OF: &str = "http://www.w3.org/2002/07/owl#oneOf";
pub const ENUMERATION: &str = ONE_OF; // alias

pub const RESTRICTION: &str = "http://www.w3.org/2002/07/owl#Restriction";
pub const ON_PROPERTY: &str = "http://www.w3.org/2002/07/owl#onProperty";
pub const SOME_VALUES_FROM: &str = "http://www.w3.org/2002/07/owl#someValuesFrom";
pub const ALL_VALUES_FROM: &str = "http://www.w3.org/2002/07/owl#allValuesFrom";
pub const HAS_VALUE: &str = "http://www.w3.org/2002/07/owl#hasValue";
pub const MIN_CARDINALITY: &str = "http://www.w3.org/2002/07/owl#minCardinality";
pub const MAX_CARDINALITY: &str = "http://www.w3.org/2002/07/owl#maxCardinality";
pub const CARDINALITY: &str = "http://www.w3.org/2002/07/owl#cardinality";

pub const MIN_QUALIFIED_CARDINALITY: &str = "http://www.w3.org/2002/07/owl#minQualifiedCardinality";
pub const MAX_QUALIFIED_CARDINALITY: &str = "http://www.w3.org/2002/07/owl#maxQualifiedCardinality";
pub const QUALIFIED_CARDINALITY: &str = "http://www.w3.org/2002/07/owl#qualifiedCardinality";

pub const OBJECT_PROPERTY: &str = "http://www.w3.org/2002/07/owl#ObjectProperty";
pub const DATA_PROPERTY: &str = "http://www.w3.org/2002/07/owl#DatatypeProperty";
pub const ANNOTATION_PROPERTY: &str = "http://www.w3.org/2002/07/owl#AnnotationProperty";

pub const TRANSITIVE_PROPERTY: &str = "http://www.w3.org/2002/07/owl#TransitiveProperty";
pub const SYMMETRIC_PROPERTY: &str = "http://www.w3.org/2002/07/owl#SymmetricProperty";
pub const ASYMMETRIC_PROPERTY: &str = "http://www.w3.org/2002/07/owl#AsymmetricProperty";
pub const REFLEXIVE_PROPERTY: &str = "http://www.w3.org/2002/07/owl#ReflexiveProperty";
pub const IRREFLEXIVE_PROPERTY: &str = "http://www.w3.org/2002/07/owl#IrreflexiveProperty";
pub const FUNCTIONAL_PROPERTY: &str = "http://www.w3.org/2002/07/owl#FunctionalProperty";
pub const INVERSE_FUNCTIONAL_PROPERTY: &str = "http://www.w3.org/2002/07/owl#InverseFunctionalProperty";

pub const INVERSE_OF: &str = "http://www.w3.org/2002/07/owl#inverseOf";
pub const SAME_AS: &str = "http://www.w3.org/2002/07/owl#sameAs";
pub const DIFFERENT_FROM: &str = "http://www.w3.org/2002/07/owl#differentFrom";
pub const DISTINCT_MEMBERS: &str = "http://www.w3.org/2002/07/owl#distinctMembers";
pub const ALL_DISJOINT_CLASSES: &str = "http://www.w3.org/2002/07/owl#AllDisjointClasses";
pub const ALL_DISJOINT_PROPERTIES: &str = "http://www.w3.org/2002/07/owl#AllDisjointProperties";
pub const MEMBERS: &str = "http://www.w3.org/2002/07/owl#members";

pub const PROPERTY_CHAIN_AXIOM: &str = "http://www.w3.org/2002/07/owl#propertyChainAxiom";
pub const PROPERTY_DISJOINT_WITH: &str = "http://www.w3.org/2002/07/owl#propertyDisjointWith";

pub const TOP_OBJECT_PROPERTY: &str = "http://www.w3.org/2002/07/owl#topObjectProperty";
pub const BOTTOM_OBJECT_PROPERTY: &str = "http://www.w3.org/2002/07/owl#bottomObjectProperty";
pub const TOP_DATA_PROPERTY: &str = "http://www.w3.org/2002/07/owl#topDataProperty";
pub const BOTTOM_DATA_PROPERTY: &str = "http://www.w3.org/2002/07/owl#bottomDataProperty";

pub const HAS_KEY: &str = "http://www.w3.org/2002/07/owl#hasKey";

pub const ANNOTATION: &str = "http://www.w3.org/2002/07/owl#Annotation";
pub const ANNOTATED_SOURCE: &str = "http://www.w3.org/2002/07/owl#annotatedSource";
pub const ANNOTATED_PROPERTY: &str = "http://www.w3.org/2002/07/owl#annotatedProperty";
pub const ANNOTATED_TARGET: &str = "http://www.w3.org/2002/07/owl#annotatedTarget";

pub const AXIOM: &str = "http://www.w3.org/2002/07/owl#Axiom";

pub const ON_DATATYPE: &str = "http://www.w3.org/2002/07/owl#onDatatype";
pub const ON_DATA_RANGE: &str = "http://www.w3.org/2002/07/owl#onDataRange";
pub const ON_CLASS: &str = "http://www.w3.org/2002/07/owl#onClass";
pub const WITH_RESTRICTIONS: &str = "http://www.w3.org/2002/07/owl#withRestrictions";
pub const ON_DATATYPE_ALIAS: &str = ON_DATATYPE;

pub const SOURCE_INDIVIDUAL: &str = "http://www.w3.org/2002/07/owl#sourceIndividual";
pub const TARGET_INDIVIDUAL: &str = "http://www.w3.org/2002/07/owl#targetIndividual";
pub const TARGET_VALUE: &str = "http://www.w3.org/2002/07/owl#targetValue";
pub const ASSERTION_PROPERTY: &str = "http://www.w3.org/2002/07/owl#assertionProperty";

pub const NEGATIVE_PROPERTY_ASSERTION: &str = "http://www.w3.org/2002/07/owl#NegativePropertyAssertion";
pub const NEGATIVE_OBJECT_PROPERTY_ASSERTION: &str = "http://www.w3.org/2002/07/owl#NegativeObjectPropertyAssertion";
pub const NEGATIVE_DATA_PROPERTY_ASSERTION: &str = "http://www.w3.org/2002/07/owl#NegativeDataPropertyAssertion";

pub const PRIOR_VERSION: &str = "http://www.w3.org/2002/07/owl#priorVersion";
pub const VERSION_INFO: &str = "http://www.w3.org/2002/07/owl#versionInfo";
pub const BACKWARD_COMPATIBLE_WITH: &str = "http://www.w3.org/2002/07/owl#backwardCompatibleWith";
pub const INCOMPATIBLE_WITH: &str = "http://www.w3.org/2002/07/owl#incompatibleWith";

pub const ONTOLOGY: &str = "http://www.w3.org/2002/07/owl#Ontology";
pub const IMPORTS: &str = "http://www.w3.org/2002/07/owl#imports";
pub const DEPRECATED: &str = "http://www.w3.org/2002/07/owl#deprecated";
pub const DEPRECATED_CLASS: &str = "http://www.w3.org/2002/07/owl#DeprecatedClass";
pub const DEPRECATED_PROPERTY: &str = "http://www.w3.org/2002/07/owl#DeprecatedProperty";

pub const ONTOLOGY_PROPERTY: &str = "http://www.w3.org/2002/07/owl#OntologyProperty"; // deprecated
pub const ANTI_SYMMETRIC_PROPERTY: &str = "http://www.w3.org/2002/07/owl#AntisymmetricProperty"; // deprecated alias

// A few legacy / backwards-compatibility names found in vocab lists:
pub const SUBJECT: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#subject"; // RDF legacy
pub const PREDICATE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#predicate";
pub const OBJECT: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#object";

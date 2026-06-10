use std::collections::HashSet;

use padmet::spec::PadmetSpec;


/// Sorted list of reactions from a PADMet object
pub fn padmet_reaction_order(pathway_id: &String, padmet_object: &PadmetSpec) -> Option<Vec<String>> {
    if let Some(node) = padmet_object.dic_of_nodes.get(pathway_id) {
        if let Some(order) = node.node_misc.get("REACTION-ORDER") {
            let order = &order[0];
            return Some(order.split(",").map(|e| e.to_owned()).collect());
        }
    } 
    None
}


/// Parent pathway ontology classes
/// 
/// get all target of PADMet `is_a` relations, coming from the pathway node pathway_id
pub fn padmet_pathway_ontology(pathway_id: &String, padmet_object: &PadmetSpec) -> HashSet<String> {
    padmet_object.get_relations_type_id_in(&"is_a".to_owned(), pathway_id).iter().map(
        |relation| relation.id_out.clone()
    ).collect()
}

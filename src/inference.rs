//! # A metabolic pathway inference decision algorithm

/* std use */
use std::collections::HashSet;
use std::collections::HashMap;

/* crate use */

/* project use */


/// Identify the missing reactions of a metabolic pathway
/// ## Arguments
/// - pathway : a pathway identifier
/// - reactome : a set of reaction of the reactome (known to be catalyzed in the organism)
/// - pathway_to_reactions : a mapping from pathway identifier to a set of its reactions
///     that are required to consider it complete (may/should ignore both non-spontaneous and non-orphan reactions of the metabolic pathway)
pub fn get_missing_reactions(pathway_reactions: &HashSet<String>,
                         reactome: &HashSet<String>)
                          -> Vec<String> {
    let missing_reactions = pathway_reactions.difference(reactome);
    let missing_reactions: Vec<String> = missing_reactions.into_iter().cloned().collect();
    missing_reactions
}

/// Try to find some reason to reject or keep a metabolic pathway
/// in the set of predicted metabolic pathways of an organism.
/// Returns a Boolean stating whether we consider
/// the metabolic pathway to be expected in the organism metabolism.
pub fn metabolic_pathway_prediction(pathway: &String,
                                    reactome: &HashSet<String>,
                                    pathway_to_reactions: &HashMap<String, HashSet<String>>,
                                    taxon_id: Option<u32>, 
                                    pathway_ontology_classes: &HashMap<String,HashSet<String>>,
) -> bool {
    if let Some(pathway_reactions) = pathway_to_reactions.get(pathway) {
        let missing_reactions = get_missing_reactions(pathway_reactions, reactome);
        // accept the pathway if it is complete
        if missing_reactions.is_empty() {
            return true;
        }
        if let Some(pathway_classes) = pathway_ontology_classes.get(pathway) {
            // reject the pathway if it a biosynthesis pathway and it lacks the last reaction
            let sources
             = 
            if pathway_classes.contains("Biosynthesis") {

            }
            
            // reject the pathway if it is an energy metabolism pathway and it lacks half its reactions.
            if pathway_classes.contains("Energy-Metabolism") && missing_reactions.len() < pathway_reactions.len() / 2 {
                return false;
            } 
        }

        return false; // by default we reject the pathway
    } else {
        // reject the pathway if there is no reactions in the pathway
        return false;
    }
}
    
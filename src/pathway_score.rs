//! # Scoring the likelihood of a pathway

use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;

/* crate use */
use padmet::spec::PadmetSpec;

/* project use */
use crate::padmet::padmet_count_pathways_with_reaction;
use crate::padmet::padmet_pathway_key_reactions;

pub struct PathwayScore {
    pub pathway_id: &String,
    pub pathway_reactions: &HashSet<String>,
    pub padmet_object: &PadmetSpec,
    pub reactome: &HashSet<String>,
    pub pathway_key_reactions: HashSet<String>,
}



impl PathwayScore {

    pub fn new(pathway_id: &String, pathway_reactions: &HashSet<String>, padmet_object: &PadmetSpec, reactome: &HashSet<String>) -> Self {
        let key_reactions: HashSet<String> = padmet_pathway_key_reactions(pathway_id, padmet_object).unwrap_or(Vec::new()).collect();
        PathwayScore { pathway_id, pathway_reactions, padmet_object, reactome, pathway_key_reactions: key_reactions }
    }
   
    pub fn presence_score(&self, reaction_id: &String) -> f64 {
        if reactome.contains(reaction_id) {
            0.2
        } else {
            0.0
        }
    }
    
    pub fn uniqueness_score(&self, reaction_id: &String) -> f64 {
        (-(padmet_count_pathways_with_reaction(reaction_id, self.padmet_object) as f64) / 10.0).exp()
    }

    pub fn key_reaction_score(&self, reaction_id: &String) -> f64 {
        if self.pathway_key_reactions.contains(reaction_id) {
            0.5
        } else {
            0.0
        }
    }
    
    pub fn reaction_score(&self, reaction_id: &String) -> f64 {
        if reactome.contains(reaction_id) {
            self.presence_score(reaction_id) + self.uniqueness_score(reaction_id) + self.key_reaction_score(reaction_id)
        } else {
            0.0
        }
    }


    pub fn pathway_score(&self) -> f64 {
        if self.pathway_reactions.len() == 0 {
            0.0
        } else {
            self.pathway_reactions.map(|reaction_id| self.reaction_score(reaction_id)).sum() / self.pathway_reactions.len()
        }
    }

}
//! predict metabolic pathways

#![warn(missing_docs)]

/* std use */
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, Error, Write};
use std::path::Path;


/* crate use */
use clap::Parser as _;
use rule_kit::RuleEngineBuilder;
use anyhow::Context as _;
use padmet::spec::PadmetSpec;

/* project use */
use pan2met::{cli, inference};
use pan2met::error;
use pan2met::inference_rules::{self, PathwayInferenceRule};

use pan2met::padmet::{padmet_pathway_ontology, padmet_reaction_order};

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_set<P>(filename: P) -> io::Result<HashSet<String>>
where
    P: AsRef<Path>,
{
    let lines = read_lines(filename)?;
    let mut set: HashSet<String> = HashSet::new();
    for line in lines.map_while(Result::ok) {
        set.insert(line);
    }
    Ok(set)
}

/// Run the inference
fn pathway_inference(reactome: &HashSet<String>, padmet_object: &PadmetSpec, rules: &[inference_rules::PathwayInferenceRule]) -> Vec<String> {
    let mut infered: Vec<String> = Vec::new();
    for (pathway, pathway_reactions) in padmet_object.get_pathways_reactions() {

        let pathway_classes = padmet_pathway_ontology(&pathway, padmet_object);
        let reaction_order = padmet_reaction_order(&pathway, padmet_object);
        let missing_reactions = inference::get_missing_reactions(&pathway_reactions, reactome);

        let mut inference = inference_rules::PathwayInference::new(
            &pathway,
            &pathway_classes,
            &pathway_reactions,
            &missing_reactions,
            None, // taxon_id
            reaction_order,
        );
        let mut engine = RuleEngineBuilder::new()
            .with_rules(rules.to_vec())
            .priority_asc()
            .build();

        let res = engine.evaluate_all(&mut inference);
        if res.is_ok() {
            if inference.decision.unwrap_or(inference_rules::Decision::Reject) == inference_rules::Decision::Accept {
                infered.push(pathway);
            }
        }
    }
    infered
}

#[hotpath::main]
fn main() -> error::Result<()> {
    // Parse argument
    let arguments = cli::Arguments::parse();

    // Setup logger
    stderrlog::new()
        .module(module_path!())
        .quiet(arguments.quiet())
        .verbosity(arguments.verbosity())
        .timestamp(arguments.timestamp())
        .init()
        .context("stderrlog already create a logger")?;

    log::info!("Loading catalyzed reactions.");
    let reactome: HashSet<String> =
        read_set(arguments.reactions())?;
    log::info!("Loading PADMet reference.");
    let padmet_object: PadmetSpec =
        PadmetSpec::from_file(arguments.padmet())?;

    log::info!("Start a metabolic pathway prediction.");
    let rules = PathwayInferenceRule::all();
    let infered = pathway_inference(&reactome, &padmet_object, &rules);

    let mut output_file = File::create(arguments.output())?;
    for pathway in infered {
        writeln!(output_file, "{pathway:#}")?;
    }

    log::info!("End a metabolic pathway prediction.");

    Ok(())
}

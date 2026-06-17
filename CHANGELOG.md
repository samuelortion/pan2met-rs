# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- use padmet-rs crate to read [PADMet](https://github.com/AuReMe/padmet.git) files

#### Rules

- reject if pathway outside taxonomic range, from NCBI-Taxonomy reference
- reject if pathway is missing a key reaction
- reject if pathway either signaling or transport pathway
- reject if no pathway reaction is catalyzed
- reject if a synthesis pathway is missing last reaction
- reject if a catabolysis pathway is missing its first first reaction
- 
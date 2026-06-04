<!--
SPDX-License-Identifier: MPL-2.0
Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
-->
# Changelog

All notable changes to `proof-of-work` will be documented in this file.

This file is generated from conventional commits by the
[`changelog-reusable.yml`](https://github.com/hyperpolymath/standards/blob/main/.github/workflows/changelog-reusable.yml)
workflow (`hyperpolymath/standards#206`). Adopt the workflow in this repo's CI to keep this file in sync automatically — see
[`templates/cliff.toml`](https://github.com/hyperpolymath/standards/blob/main/templates/cliff.toml)
for the canonical config.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/);
this project aims to follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- feat(abi): I5 DifficultyInRange half discharged Rust-side (#68)
- feat(abi): discharge I3 placePreservesWF — real proof, no postulate (#128) (#60)
- feat(abi): commit Idris2 seam + stance; discharge I5 (verified) (#56)
- feat(crg): add crg-grade and crg-badge justfile recipes
- feat: add stapeln.toml layer-based container definition\n\nConverted from existing Containerfile to stapeln format.\nIncludes Chainguard base, security hardening, SBOM generation.\n\nCo-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>
- feat: deploy UX Manifesto infrastructure
- feat: add CLADE.a2ml — clade taxonomy declaration

### Fixed

- fix(verification): adapt to z3 0.19 API drift (#67)
- fix(abi): unhang Foreign.idr under idris2 0.8.0 (#128) (#63)
- fix(abi): make the proof-of-work SPARK/ABI seam actually buildable (P0) (#58)
- fix(licence): #3 isolated — clear scaffold-placeholder leak (proof-of-work) (#57)
- fix(ci): bump a2ml/k9-validate-action pins to canonical (#53)
- fix(ci): sync hypatia-scan.yml to canonical (#52)
- fix(ci): repair YAML block-scalar in workflow-linter Check Permissions step (#51)
- fix(ci): complete Ast import removal in verification/mod.rs and cargo fmt across crate (#44)
- fix(verify): drop unused Ast import to unbreak Rust CI
- fix(bench): replace fraudulent black_box(42) benchmarks with real game logic

### Changed

- refactor: migrate 6SCM → 6A2 (.scm → .a2ml format)

### Documentation

- docs(abi): annotate Rust call sites with PROOF-OBLIGATION refs to Invariants.idr (#66)
- docs(proof-needs): sync I3/I7 status to actual code state (#65)
- docs(flake): annotate KEEP+DEP rationale (standards#102) (#61)
- docs(governance): CRG v2.0 STRICT audit — C (declared) -> D (honest)
- docs: substantive CRG C annotation (EXPLAINME.adoc)
- docs: add EXPLAINME.adoc — prove-it file backing README claims
- docs: update SCM files with project information

### CI

- ci(rust): convert rust-ci.yml to thin wrapper (standards#174) (#72)
- ci: redistribute concurrency-cancel guard to read-only check workflows (#55)
- ci(workflow): adopt hardened hypatia-scan from hyperpolymath/hypatia#237 (#49)
- ci(dependabot): restore cargo PR limit so security PRs flow (#45)
- ci: SHA-pin hyperpolymath validate-actions in dogfood-gate

## Pre-history

Prior commits to this file's introduction are recorded in git history but not formally classified into Keep-a-Changelog sections. To backfill, run `git cliff -o CHANGELOG.md` locally using the canonical [`cliff.toml`](https://github.com/hyperpolymath/standards/blob/main/templates/cliff.toml) — this is one-shot mechanical work.

---

<!-- This file was seeded by the 2026-05-26 estate tech-debt audit follow-up (Row-2 Phase 3); see [`hyperpolymath/standards/docs/audits/2026-05-26-estate-documentation-debt.md`](https://github.com/hyperpolymath/standards/blob/main/docs/audits/2026-05-26-estate-documentation-debt.md). -->

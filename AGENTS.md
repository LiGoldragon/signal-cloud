# signal-cloud — Agent Instructions

Read `~/primary/AGENTS.md`, then this repository's `INTENT.md`,
`ARCHITECTURE.md`, and this file.

This repository is a pure Signal contract crate. It declares the ordinary
cloud-provider wire vocabulary and contains no daemon, storage, actors,
provider clients, credentials, or runtime policy.

Do not add meta-policy mutation or secret-bearing records here. Provider
credentials, external-account authority, and live plan application belong in
`meta-signal-cloud`.

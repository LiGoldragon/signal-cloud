# signal-cloud — Agent Instructions

Read this repository's `INTENT.md`, `ARCHITECTURE.md`, and this file before
editing.

This repository is a pure Signal contract crate. It declares the ordinary
cloud-provider wire vocabulary and contains no daemon, storage, actors,
provider clients, credentials, or runtime policy.

Do not add meta-policy mutation or secret-bearing records here. Provider
credentials, external-account authority, and live plan application belong in
`meta-signal-cloud`.

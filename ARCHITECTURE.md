# signal-cloud Architecture

`signal-cloud` is the ordinary Signal contract for the `cloud` component.
It lets peers observe provider capabilities and state, validate desired
provider-neutral cloud state, and observe daemon-held plans.

## Direction

`signal-cloud` is the **ordinary peer-callable wire contract** for the `cloud` component. It carries the read-and-validate surface: peers observe provider capabilities, zones, records, redirects, and daemon-held plans, and validate desired provider-neutral state without preparing or applying any mutation. Plan preparation and plan application — which write daemon plan state and mutate external provider accounts — live in `meta-signal-cloud`.

The contract names cloud-provider concepts without binding the domain model to any one provider. Cloudflare, Google Cloud, and Hetzner are provider variants, not separate operation roots. This is a workspace generalization: a component whose state surface is a reflected external resource exposes its read surface on the ordinary contract and its mutation surface on the meta contract. The `cloud` daemon is the first worked example, per Spirit records 311 and 325 (Maximum certainty, 2026-05-23).

## Boundary

The contract names cloud-provider concepts without binding the domain model
to any one provider. Cloudflare, Google Cloud, and Hetzner are provider
variants; they are not separate operation roots.

The ordinary surface does not prepare or apply plans. Preparing a plan writes
daemon plan state, and applying a plan mutates external provider accounts; both
belong to `meta-signal-cloud` until Criome-mediated authorization is in place.

## Public Operations

- `Observe(Observation)` reads provider capabilities, zones, records,
  redirects, or daemon-held plan state.
- `Validate(DesiredState)` checks a desired provider-neutral state without
  preparing or applying a mutation.

There is no public `Assert`, `Mutate`, `Retract`, `Match`, `Subscribe`, or
`Validate` Sema root. Sema classification is daemon-local and projected after
the public operation has been lowered into a component command.

## Ordinary vs meta split

Per Spirit records 311 and 325 (Maximum certainty, 2026-05-23), the cloud
surface splits Mutate-class verbs onto `meta-signal-cloud` (privileged) and
Query-class verbs onto `signal-cloud` (public). Cloudflare and other provider
states are treated as **external state that the cloud daemon reflects**:
refresh-by-querying is public; daemon-internal mutations such as preparing
plans, registering accounts, or applying plans require meta authority.

This is a workspace generalization: a component whose state surface is a
reflected external resource exposes its read surface on the ordinary contract
and its mutation surface on the meta contract. The cloud daemon is the first
worked example.

The previous shape held `Plan(DesiredState)` on this ordinary contract. The
new shape moves plan preparation to `meta-signal-cloud` as
`PreparePlan(PlanPreparation)`; reading a prepared plan stays here as an
`Observation::Plan(PlanQuery)` variant since plan reads are Query-class.

## Owns

- Provider variants.
- Capability variants.
- Provider-neutral domain-name-system records and redirect rules.
- Desired-state, validation, and observable plan records.
- Ordinary observation and validation replies.
- Typed unsupported-provider and rejected-request replies.

## Does Not Own

- Provider credentials.
- Secret bytes.
- Meta policy.
- Live provider mutation.
- Runtime actors, sockets, databases, or rate-limit state.
- The Criome domain registry. That belongs to `domain-criome`.

## Constraints

- Depend on `signal-frame`, not deprecated `signal-core`.
- Keep records runtime-free.
- Use typed variants instead of strings for providers and capabilities.
- Secret values never cross this ordinary contract.

## Schema-emission status

**Status:** partial. `schema/lib.schema` is present and `build.rs` runs
`schema-rust-next`'s `GenerationPlan::wire_contract`, emitting the
checked-in witness at `src/schema/lib.rs`.

The crate's public top-level API still comes from the hand-written
`src/lib.rs` types and `signal_channel!` invocation. The remaining schema
cutover is to replace that duplicate hand-written surface with generated
re-exports from `src/schema/lib.rs`, then update downstream imports and tests.
Daemon Nexus and SEMA schemas live in `cloud/schema/` and import this contract
through Cargo schema metadata.

Regenerate the checked-in witness with
`SIGNAL_CLOUD_UPDATE_SCHEMA_ARTIFACTS=1 cargo build --all-features` after
schema edits. Ordinary builds use the generator's freshness check.

**Per-component concerns:** Per `primary-kbmi.1`. The ordinary cloud contract is
paired with `meta-signal-cloud`; both contracts stay separate from the cloud
daemon's Nexus and SEMA runtime schemas.

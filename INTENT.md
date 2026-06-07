# INTENT — signal-cloud

*The ordinary peer-callable wire contract for the `cloud` component.
Companion to `ARCHITECTURE.md` and `Cargo.toml`.
Maintenance: `primary/skills/repo-intent.md`.*

## Repo-scope only

This file carries only the intent that is FOR this `signal-cloud`
contract. Workspace-shape intent stays in the primary workspace
`primary/INTENT.md`. Component daemon intent stays in `cloud/INTENT.md`.
Owner/meta policy intent stays in `meta-signal-cloud/INTENT.md`.

## Why this repo exists

`signal-cloud` is the **ordinary peer-callable wire contract** for the
`cloud` component. It carries the read-and-validate surface: peers
observe provider capabilities, zones, records, redirects, and
daemon-held plans, and validate a desired provider-neutral cloud state
without preparing or applying any mutation. Plan preparation and plan
application — which write daemon plan state and mutate external provider
accounts — live in `meta-signal-cloud`; runtime actors, sockets, storage,
and rate-limit state live in `cloud`.

## Provider-neutral by construction

The contract names cloud-provider concepts without binding the domain
model to any one provider. Cloudflare, Google Cloud, and Hetzner are
typed provider *variants*, not separate operation roots. A component
whose state surface is a reflected external resource exposes its read
surface on the ordinary contract and its mutation surface on the owner
contract; `cloud` is the first worked example (per Spirit records 311
and 325, Maximum certainty, 2026-05-23).

## The channel shape

The ordinary cloud channel carries:

- **Requests:** `Observe(Observation)` reads provider capabilities,
  zones, records, redirects, or daemon-held plan state;
  `Validate(DesiredState)` checks a desired provider-neutral state
  without preparing or applying a mutation.
- **Replies:** ordinary observation and validation replies, plus typed
  unsupported-provider and rejected-request replies.

Reading a prepared plan stays here as an `Observation::Plan(PlanQuery)`
variant because plan reads are Query-class; preparing the plan moved to
`meta-signal-cloud` as `PreparePlan(PlanPreparation)`.

## Channels are closed, boundaries are named

- Wire enums are closed. No `Unknown` escape hatch.
- Provider and capability values are typed variants, never strings.
- There is no public `Assert`, `Mutate`, `Retract`, `Match`,
  `Subscribe`, or `Validate` Sema root. Sema classification is
  daemon-local and projected after the public operation lowers into a
  component command.
- Secret values never cross this ordinary contract.

## Wire vocabulary discipline

Per `primary/skills/contract-repo.md` §"Public contracts use
contract-local operation verbs":

- Operation roots are domain verbs in verb form: `Observe`, `Validate`.
- Payload records are domain nouns: `Observation`, `DesiredState`,
  `PlanQuery`.
- Refresh-by-querying provider state is public; daemon-internal
  mutations (preparing plans, registering accounts, applying plans)
  require owner authority and live in `meta-signal-cloud`.

## Constraints

- This crate carries only typed wire vocabulary, NOTA codecs, and
  round-trip witnesses. No runtime code.
- Depend on `signal-frame`, not deprecated `signal-core`.
- Every operation and reply variant round-trips through both rkyv frames
  and NOTA text.
- This contract's hand-written `signal_channel!` invocation is scheduled
  to convert to this repo's `schema/lib.schema` as the ordinary Signal
  contract; the daemon's Nexus and SEMA schemas live in `cloud/schema/`
  and import this contract through Cargo schema metadata.

## Non-ownership

This crate does not own:

- provider credentials, secret bytes, owner policy, or live provider
  mutation;
- `cloud` daemon runtime actors, sockets, databases, or rate-limit
  state;
- the Criome domain registry — that belongs to `domain-criome`.

## See also

- `ARCHITECTURE.md` — detailed channel shape, ordinary/owner split, and
  closed-enum discipline.
- `../cloud/INTENT.md` — daemon-side intent when it lands.
- `../meta-signal-cloud/INTENT.md` — owner meta-signal policy contract.
- `primary/skills/contract-repo.md` — contract repo discipline and
  naming rules.
- `primary/skills/component-triad.md` — repo triad structure and wire
  layers.

# signal-cloud Architecture

`signal-cloud` is the ordinary Signal contract for the `cloud` component.
It lets peers observe provider capabilities and state, validate desired
provider-neutral cloud state, and observe daemon-held plans.

## Boundary

The contract names cloud-provider concepts without binding the domain model
to any one provider. Cloudflare, Google Cloud, and Hetzner are provider
variants; they are not separate operation roots.

The ordinary surface does not prepare or apply plans. Preparing a plan writes
daemon plan state, and applying a plan mutates external provider accounts; both
belong to `owner-signal-cloud` until Criome-mediated authorization is in place.

## Public Operations

- `Observe(Observation)` reads provider capabilities, zones, records,
  redirects, or daemon-held plan state.
- `Validate(DesiredState)` checks a desired provider-neutral state without
  preparing or applying a mutation.

There is no public `Assert`, `Mutate`, `Retract`, `Match`, `Subscribe`, or
`Validate` Sema root. Sema classification is daemon-local and projected after
the public operation has been lowered into a component command.

## Ordinary vs owner split

Per Spirit records 311 and 325 (Maximum certainty, 2026-05-23), the cloud
surface splits Mutate-class verbs onto `owner-signal-cloud` (privileged) and
Query-class verbs onto `signal-cloud` (public). Cloudflare and other provider
states are treated as **external state that the cloud daemon reflects**:
refresh-by-querying is public; daemon-internal mutations such as preparing
plans, registering accounts, or applying plans require owner authority.

This is a workspace generalization: a component whose state surface is a
reflected external resource exposes its read surface on the ordinary contract
and its mutation surface on the owner contract. The cloud daemon is the first
worked example.

The previous shape held `Plan(DesiredState)` on this ordinary contract. The
new shape moves plan preparation to `owner-signal-cloud` as
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
- Owner policy.
- Live provider mutation.
- Runtime actors, sockets, databases, or rate-limit state.
- The Criome domain registry. That belongs to `domain-criome`.

## Constraints

- Depend on `signal-frame`, not deprecated `signal-core`.
- Keep records runtime-free.
- Use typed variants instead of strings for providers and capabilities.
- Secret values never cross this ordinary contract.

## Pending schema-engine upgrade

**Status:** scheduled for migration to schema-language-based contract per `reports/designer/326-v13-spirit-complete-schema-vision.md` + `reports/designer/324-migration-mvp-spirit-handover-re-specification.md`.

**Target:** this contract's hand-written `signal_channel!` invocation converts to a single `cloud/cloud.schema` file (shared with the `cloud` daemon's repository). The brilliant macro library (`primary-ezqx.1`) reads the schema + emits this crate's wire types + ShortHeader projection + dispatcher binding + VersionProjection impls.

**Sequence:** per `primary-kbmi.1`. Spirit is the MVP pilot landing first via `primary-ezqx.1`; this contract's schema cutover coordinates with cloud daemon implementation.

**Per-component concerns:** Per `primary-kbmi.1`. The ordinary cloud contract is paired with `owner-signal-cloud`; both legs of the policy-vs-working split appear in the shared `cloud.schema` file per the schema-language's separation discipline.

**References:**
- `reports/designer/326-v13-spirit-complete-schema-vision.md` — uniform header form + schema-language design
- `reports/designer/324-migration-mvp-spirit-handover-re-specification.md` — migration MVP + handover state
- `reports/designer/322-spirit-mvp-positional-schema-worked-example.md` — Spirit MVP worked example
- `reports/operator/174-schema-import-header-design-critique-2026-05-24.md` — header/body/feature separation + lowering rules

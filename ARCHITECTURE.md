# signal-cloud Architecture

`signal-cloud` is the ordinary Signal contract for the `cloud` component.
It lets peers observe provider capabilities and state, validate desired
provider-neutral cloud state, and ask the daemon to prepare provider-specific
plans.

## Boundary

The contract names cloud-provider concepts without binding the domain model
to any one provider. Cloudflare, Google Cloud, and Hetzner are provider
variants; they are not separate operation roots.

The ordinary surface does not apply plans. Applying a plan mutates external
provider accounts and therefore belongs to `owner-signal-cloud` until
Criome-mediated authorization is in place.

## Public Operations

- `Observe(Observation)` reads provider capabilities, zones, records,
  redirects, or daemon-held plan state.
- `Validate(DesiredState)` checks a desired provider-neutral state without
  preparing or applying a mutation.
- `Plan(DesiredState)` asks the daemon to compute a concrete provider plan.

There is no public `Assert`, `Mutate`, `Retract`, `Match`, `Subscribe`, or
`Validate` Sema root. Sema classification is daemon-local and projected after
the public operation has been lowered into a component command.

## Owns

- Provider variants.
- Capability variants.
- Provider-neutral domain-name-system records and redirect rules.
- Desired-state and plan records.
- Ordinary observation, validation, and planning replies.
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

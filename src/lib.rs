//! Ordinary Signal contract for the cloud component.
//!
//! This crate carries peer-callable provider observation and validation
//! records. Owner-only credentials, plan preparation, and live mutation live
//! in `owner-signal-cloud`.

use nota_codec::{NotaEnum, NotaRecord, NotaTransparent};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;

macro_rules! string_newtype {
    ($name:ident) => {
        #[derive(
            Archive,
            RkyvSerialize,
            RkyvDeserialize,
            NotaTransparent,
            Debug,
            Clone,
            PartialEq,
            Eq,
            Hash,
        )]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

string_newtype!(DomainName);
string_newtype!(RecordValue);
string_newtype!(UniformResourceLocator);
string_newtype!(PlanIdentifier);
string_newtype!(ProviderAccount);
string_newtype!(ZoneIdentifier);
string_newtype!(RuleIdentifier);

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum Provider {
    Cloudflare,
    GoogleCloud,
    Hetzner,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum Capability {
    DomainNameSystemRecords,
    RedirectRules,
    CloudHosts,
    Networks,
    Firewalls,
    LoadBalancers,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum CapabilityState {
    Compiled,
    Configured,
    Authorized,
    Unsupported,
    Unauthorized,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum RecordKind {
    AddressV4,
    AddressV6,
    CanonicalName,
    Text,
    MailExchange,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum ProxyMode {
    Direct,
    ProviderProxy,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum RedirectStatus {
    Permanent,
    Temporary,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum PathTreatment {
    Preserve,
    Replace,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct CapabilityQuery {
    pub provider: Option<Provider>,
    pub capability: Option<Capability>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct CapabilityObservation {
    pub provider: Provider,
    pub capability: Capability,
    pub state: CapabilityState,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct CapabilityReport {
    pub capabilities: Vec<CapabilityObservation>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ZoneQuery {
    pub provider: Option<Provider>,
    pub account: Option<ProviderAccount>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Zone {
    pub provider: Provider,
    pub account: ProviderAccount,
    pub identifier: ZoneIdentifier,
    pub name: DomainName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ZoneListing {
    pub zones: Vec<Zone>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DomainNameSystemRecord {
    pub name: DomainName,
    pub kind: RecordKind,
    pub value: RecordValue,
    pub proxy_mode: ProxyMode,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RecordQuery {
    pub provider: Provider,
    pub zone: DomainName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RecordListing {
    pub records: Vec<DomainNameSystemRecord>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RedirectRule {
    pub source: DomainName,
    pub target: UniformResourceLocator,
    pub status: RedirectStatus,
    pub path_treatment: PathTreatment,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RedirectQuery {
    pub provider: Provider,
    pub zone: DomainName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RedirectListing {
    pub rules: Vec<RedirectRule>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DesiredState {
    pub provider: Provider,
    pub zone: DomainName,
    pub records: Vec<DomainNameSystemRecord>,
    pub redirects: Vec<RedirectRule>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Validation {
    pub desired_state: DesiredState,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Plan {
    pub identifier: PlanIdentifier,
    pub provider: Provider,
    pub zone: DomainName,
    pub records_to_create: Vec<DomainNameSystemRecord>,
    pub records_to_update: Vec<DomainNameSystemRecord>,
    pub record_names_to_delete: Vec<DomainName>,
    pub redirects_to_create: Vec<RedirectRule>,
    pub redirects_to_update: Vec<RedirectRule>,
    pub redirect_sources_to_delete: Vec<DomainName>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct PlanQuery {
    pub identifier: PlanIdentifier,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq)]
pub enum Observation {
    Capabilities(CapabilityQuery),
    Zones(ZoneQuery),
    Records(RecordQuery),
    Redirects(RedirectQuery),
    Plan(PlanQuery),
}

impl Observation {
    pub fn kind(&self) -> ObservationKind {
        match self {
            Self::Capabilities(_) => ObservationKind::Capabilities,
            Self::Zones(_) => ObservationKind::Zones,
            Self::Records(_) => ObservationKind::Records,
            Self::Redirects(_) => ObservationKind::Redirects,
            Self::Plan(_) => ObservationKind::Plan,
        }
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum ObservationKind {
    Capabilities,
    Zones,
    Records,
    Redirects,
    Plan,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq)]
pub enum ObservationResult {
    Capabilities(CapabilityReport),
    Zones(ZoneListing),
    Records(RecordListing),
    Redirects(RedirectListing),
    Plan(Plan),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum FindingSeverity {
    Notice,
    Warning,
    Error,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ValidationFinding {
    pub severity: FindingSeverity,
    pub message: String,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ValidationReport {
    pub findings: Vec<ValidationFinding>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum UnsupportedReason {
    ProviderNotCompiled,
    ProviderNotConfigured,
    CapabilityNotCompiled,
    CapabilityNotConfigured,
    CapabilityUnauthorized,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RequestUnsupported {
    pub operation: OperationKind,
    pub provider: Option<Provider>,
    pub capability: Option<Capability>,
    pub reason: UnsupportedReason,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum RejectionReason {
    InvalidDesiredState,
    PlanExpired,
    ProviderRateLimited,
    ProviderUnavailable,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RequestRejected {
    pub operation: OperationKind,
    pub reason: RejectionReason,
}

signal_channel! {
    channel Cloud {
        operation Observe(Observation),
        operation Validate(Validation),
    }
    reply Reply {
        Observed(ObservationResult),
        Validated(ValidationReport),
        RequestUnsupported(RequestUnsupported),
        RequestRejected(RequestRejected),
    }
}

pub type ChannelRequest = signal_frame::Request<Operation>;
pub type ChannelReply = signal_frame::Reply<Reply>;

impl Operation {
    pub fn operation_kind(&self) -> OperationKind {
        self.kind()
    }
}

//! Authority-seated identities for the strict capability Interface.
//!
//! These opaque identities and canonical-order values are minted state. None
//! is derived from spelling, source position, or Rust representation.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthoritySeat {
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}

impl AuthoritySeat {
    pub const fn new(spelling: &'static str, local: u16, canonical: u64) -> Self {
        Self {
            spelling,
            local,
            canonical,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeclarationSeat {
    pub owner_local: Option<u16>,
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}

impl DeclarationSeat {
    pub const fn new(
        owner_local: Option<u16>,
        spelling: &'static str,
        local: u16,
        canonical: u64,
    ) -> Self {
        Self {
            owner_local,
            spelling,
            local,
            canonical,
        }
    }
}

pub const AUTHORITY_IDENTITY: [u8; 32] = [
    51, 165, 31, 15, 212, 184, 159, 223, 239, 116, 190, 2, 80, 95, 65, 202, 224, 146, 78, 202, 156,
    152, 44, 152, 243, 104, 135, 46, 86, 128, 176, 25,
];
pub const AUTHORITY_REVISION: u64 = 1;
pub const GRAMMAR_DOCUMENT_LOCAL: u16 = 6279;
pub const GRAMMAR_SYNTAX_LOCAL: u16 = 36990;

pub const INTERFACE_SEAT: AuthoritySeat =
    AuthoritySeat::new("Interface", 19572, 0x643ac7ddaee60480);
pub const NEXUS_SEAT: AuthoritySeat = AuthoritySeat::new("Nexus", 17804, 0x78cc3576603db1fe);
pub const SEMA_SEAT: AuthoritySeat = AuthoritySeat::new("Sema", 34297, 0x415c054e0d5bcf60);
pub const INPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Input", 22647, 0x203f476f808f901d);
pub const OUTPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Output", 3734, 0x85983599620a5927);
pub const REFUSAL_SEAT: AuthoritySeat = AuthoritySeat::new("Refusal", 18397, 0xe329e0aede92df92);
pub const STRING_SEAT: AuthoritySeat = AuthoritySeat::new("String", 33026, 0x6efcebb5291baacd);
pub const INTEGER_SEAT: AuthoritySeat = AuthoritySeat::new("Integer", 6525, 0x962be719e9bd6957);
pub const BOOLEAN_SEAT: AuthoritySeat = AuthoritySeat::new("Boolean", 28590, 0x1dfb013641f72a83);
pub const UNIT_SEAT: AuthoritySeat = AuthoritySeat::new("Unit", 13857, 0xb9cc904f40367d24);
pub const VECTOR_SEAT: AuthoritySeat = AuthoritySeat::new("Vector", 46717, 0x21e2f47e1db2b3f3);
pub const OPTION_SEAT: AuthoritySeat = AuthoritySeat::new("Option", 24397, 0xd1cb8ed3672acb7c);
pub const MAP_SEAT: AuthoritySeat = AuthoritySeat::new("Map", 19255, 0x7f37653381b290c8);
pub const RESULT_SEAT: AuthoritySeat = AuthoritySeat::new("Result", 49400, 0x028b33056d3655f3);
pub const STREAM_SEAT: AuthoritySeat = AuthoritySeat::new("Stream", 29110, 0x6e4ded7af5064cfd);
pub const STREAMIDENTITY_SEAT: AuthoritySeat =
    AuthoritySeat::new("StreamIdentity", 60788, 0x92c3708e7af4bae4);

pub const RUST_VOCABULARY_LOCALS: [u16; 10] = [
    37769, 61673, 64176, 16719, 16803, 52139, 13965, 64644, 44793, 4179,
];

pub const PROVIDER_LOCAL: u16 = 44270;
pub const CAPABILITY_LOCAL: u16 = 51375;
pub const CAPABILITY_STATE_LOCAL: u16 = 62193;

pub const DECLARATION_SEATS: &[DeclarationSeat] = &[
    DeclarationSeat::new(None, "Provider", PROVIDER_LOCAL, 0x6320abf24e87cfc9),
    DeclarationSeat::new(
        Some(PROVIDER_LOCAL),
        "Cloudflare",
        24388,
        0x897ee65ba05bc8fe,
    ),
    DeclarationSeat::new(
        Some(PROVIDER_LOCAL),
        "GoogleCloud",
        40816,
        0xa0a8e54981018bb1,
    ),
    DeclarationSeat::new(Some(PROVIDER_LOCAL), "Hetzner", 39771, 0xb0b96cb0c01512a4),
    DeclarationSeat::new(
        Some(PROVIDER_LOCAL),
        "DigitalOcean",
        1239,
        0x4fb80c93090ea8c6,
    ),
    DeclarationSeat::new(None, "Capability", CAPABILITY_LOCAL, 0x8dedb02df2f6e9a3),
    DeclarationSeat::new(
        Some(CAPABILITY_LOCAL),
        "DomainNameSystemRecords",
        64478,
        0xdd139d8524fb4408,
    ),
    DeclarationSeat::new(
        Some(CAPABILITY_LOCAL),
        "RedirectRules",
        58383,
        0xe055fc2cb561b38f,
    ),
    DeclarationSeat::new(
        Some(CAPABILITY_LOCAL),
        "CloudHosts",
        32151,
        0x77ab078776ebc74e,
    ),
    DeclarationSeat::new(
        Some(CAPABILITY_LOCAL),
        "Networks",
        55676,
        0xd615f357b44729ee,
    ),
    DeclarationSeat::new(
        Some(CAPABILITY_LOCAL),
        "Firewalls",
        10236,
        0x38e5cb1cd43af55b,
    ),
    DeclarationSeat::new(
        Some(CAPABILITY_LOCAL),
        "LoadBalancers",
        28929,
        0x852c85e4ef4b2369,
    ),
    DeclarationSeat::new(
        None,
        "CapabilityState",
        CAPABILITY_STATE_LOCAL,
        0x04363009c43568e6,
    ),
    DeclarationSeat::new(
        Some(CAPABILITY_STATE_LOCAL),
        "NotBuilt",
        48054,
        0xe001337c7a6ad47c,
    ),
    DeclarationSeat::new(
        Some(CAPABILITY_STATE_LOCAL),
        "Compiled",
        13081,
        0x7132d3e9892895e8,
    ),
    DeclarationSeat::new(
        Some(CAPABILITY_STATE_LOCAL),
        "Configured",
        23846,
        0x397146e185cc212d,
    ),
    DeclarationSeat::new(
        Some(CAPABILITY_STATE_LOCAL),
        "Authorized",
        30242,
        0xdf660b50d9c33e9c,
    ),
    DeclarationSeat::new(
        Some(CAPABILITY_STATE_LOCAL),
        "Unsupported",
        24107,
        0xe8c1bfb1e6b827c9,
    ),
    DeclarationSeat::new(
        Some(CAPABILITY_STATE_LOCAL),
        "Unauthorized",
        35132,
        0x0a72da854814c0c2,
    ),
    DeclarationSeat::new(None, "CapabilityQuery", 35442, 0xa0a4a824164c2e31),
    DeclarationSeat::new(None, "CapabilityObservation", 2960, 0x1dff488f98e21dc2),
    DeclarationSeat::new(None, "CapabilityReport", 29899, 0xaa137a7772a94816),
];

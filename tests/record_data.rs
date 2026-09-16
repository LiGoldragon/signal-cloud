//! One concrete text example and one round trip for every record kind.
//!
//! The `nexus` skill requires each record kind to land as a concrete text
//! example with a round-trip test before its type is final. The example text
//! is the falsifiable specification: it is asserted literally, then parsed
//! back and compared to the value it came from.

use nota::{NotaEncode, NotaSource};
use signal_cloud::{
    DomainName, DomainNameSystemRecord, IpV4Address, IpV6Address, MailExchangeRecord, Port,
    Priority, ProxyMode, RecordData, RecordKind, RecordTarget, Seconds, ServiceRecord, TextValue,
    TimeToLive, Weight,
};

/// Asserts the exact text of one payload and that the text parses back to it.
fn round_trips(data: RecordData, kind: RecordKind, expected_text: &str) {
    assert_eq!(data.kind(), kind, "payload kind");
    let text = data.to_nota();
    assert_eq!(text, expected_text, "record payload text");
    let decoded = NotaSource::new(&text)
        .parse::<RecordData>()
        .expect("record payload decodes");
    assert_eq!(decoded, data);
}

#[test]
fn address_v4_round_trips_through_its_text_example() {
    round_trips(
        RecordData::AddressV4(IpV4Address::new("203.0.113.8")),
        RecordKind::AddressV4,
        "(AddressV4 203.0.113.8)",
    );
}

#[test]
fn address_v6_round_trips_through_its_text_example() {
    round_trips(
        RecordData::AddressV6(IpV6Address::new("2001:db8::8")),
        RecordKind::AddressV6,
        "(AddressV6 2001:db8::8)",
    );
}

#[test]
fn canonical_name_round_trips_through_its_text_example() {
    round_trips(
        RecordData::CanonicalName(RecordTarget::new("xmpp.goldragon.criome.net")),
        RecordKind::CanonicalName,
        "(CanonicalName xmpp.goldragon.criome.net)",
    );
}

#[test]
fn text_round_trips_through_its_text_example() {
    round_trips(
        RecordData::Text(TextValue::new("v=spf1 -all")),
        RecordKind::Text,
        "(Text [v=spf1 -all])",
    );
}

#[test]
fn mail_exchange_round_trips_through_its_text_example() {
    round_trips(
        RecordData::MailExchange(MailExchangeRecord {
            priority: Priority::new(10),
            record_target: RecordTarget::new("mail.criome.net"),
        }),
        RecordKind::MailExchange,
        "(MailExchange (10 mail.criome.net))",
    );
}

#[test]
fn name_server_round_trips_through_its_text_example() {
    round_trips(
        RecordData::NameServer(RecordTarget::new("ns1.criome.net")),
        RecordKind::NameServer,
        "(NameServer ns1.criome.net)",
    );
}

#[test]
fn pointer_round_trips_through_its_text_example() {
    round_trips(
        RecordData::Pointer(RecordTarget::new("xmpp.goldragon.criome.net")),
        RecordKind::Pointer,
        "(Pointer xmpp.goldragon.criome.net)",
    );
}

#[test]
fn service_round_trips_through_its_text_example() {
    round_trips(
        RecordData::Service(ServiceRecord {
            priority: Priority::new(10),
            weight: Weight::new(5),
            port: Port::new(5222),
            record_target: RecordTarget::new("xmpp.goldragon.criome.net"),
        }),
        RecordKind::Service,
        "(Service (10 5 5222 xmpp.goldragon.criome.net))",
    );
}

#[test]
fn certificate_authority_authorization_round_trips_through_its_text_example() {
    round_trips(
        RecordData::CertificateAuthorityAuthorization(TextValue::new("0 issue letsencrypt.org")),
        RecordKind::CertificateAuthorityAuthorization,
        "(CertificateAuthorityAuthorization [0 issue letsencrypt.org])",
    );
}

#[test]
fn secure_shell_fingerprint_round_trips_through_its_text_example() {
    round_trips(
        RecordData::SecureShellFingerprint(TextValue::new("4 2 abcdef")),
        RecordKind::SecureShellFingerprint,
        "(SecureShellFingerprint [4 2 abcdef])",
    );
}

#[test]
fn transport_layer_security_authentication_round_trips_through_its_text_example() {
    round_trips(
        RecordData::TransportLayerSecurityAuthentication(TextValue::new("3 1 1 abcdef")),
        RecordKind::TransportLayerSecurityAuthentication,
        "(TransportLayerSecurityAuthentication [3 1 1 abcdef])",
    );
}

#[test]
fn uniform_resource_identifier_round_trips_through_its_text_example() {
    round_trips(
        RecordData::UniformResourceIdentifier(TextValue::new("10 1 https://criome.net/")),
        RecordKind::UniformResourceIdentifier,
        "(UniformResourceIdentifier [10 1 https://criome.net/])",
    );
}

#[test]
fn service_binding_round_trips_through_its_text_example() {
    round_trips(
        RecordData::ServiceBinding(TextValue::new("1 . alpn=h2")),
        RecordKind::ServiceBinding,
        "(ServiceBinding [1 . alpn=h2])",
    );
}

#[test]
fn https_binding_round_trips_through_its_text_example() {
    round_trips(
        RecordData::HttpsBinding(TextValue::new("1 . alpn=h3")),
        RecordKind::HttpsBinding,
        "(HttpsBinding [1 . alpn=h3])",
    );
}

#[test]
fn location_round_trips_through_its_text_example() {
    round_trips(
        RecordData::Location(TextValue::new("51 30 0.000 N 0 7 0.000 W 0.00m")),
        RecordKind::Location,
        "(Location [51 30 0.000 N 0 7 0.000 W 0.00m])",
    );
}

#[test]
fn provider_automatic_time_to_live_round_trips_through_its_text_example() {
    let ttl = TimeToLive::ProviderAutomatic;
    let text = ttl.to_nota();
    assert_eq!(text, "ProviderAutomatic");
    assert_eq!(
        NotaSource::new(&text)
            .parse::<TimeToLive>()
            .expect("decode"),
        ttl
    );
}

#[test]
fn fixed_time_to_live_round_trips_through_its_text_example() {
    let ttl = TimeToLive::Fixed(Seconds::new(300));
    let text = ttl.to_nota();
    assert_eq!(text, "(Fixed 300)");
    assert_eq!(
        NotaSource::new(&text)
            .parse::<TimeToLive>()
            .expect("decode"),
        ttl
    );
}

#[test]
fn a_service_record_round_trips_whole_through_its_text_example() {
    let record = DomainNameSystemRecord {
        name: DomainName::new("_xmpp-client._tcp.xmpp.goldragon.criome.net"),
        data: RecordData::Service(ServiceRecord {
            priority: Priority::new(10),
            weight: Weight::new(5),
            port: Port::new(5222),
            record_target: RecordTarget::new("xmpp.goldragon.criome.net"),
        }),
        proxy_mode: ProxyMode::Direct,
        time_to_live: TimeToLive::ProviderAutomatic,
    };
    assert_eq!(record.kind(), RecordKind::Service);
    let text = record.to_nota();
    assert_eq!(
        text,
        "(_xmpp-client._tcp.xmpp.goldragon.criome.net (Service (10 5 5222 xmpp.goldragon.criome.net)) Direct ProviderAutomatic)"
    );
    let decoded = NotaSource::new(&text)
        .parse::<DomainNameSystemRecord>()
        .expect("decode");
    assert_eq!(decoded, record);
}

#[test]
fn presentation_form_matches_the_rfc_single_line_shape() {
    assert_eq!(
        RecordData::Service(ServiceRecord {
            priority: Priority::new(10),
            weight: Weight::new(5),
            port: Port::new(5222),
            record_target: RecordTarget::new("xmpp.goldragon.criome.net"),
        })
        .presentation()
        .as_str(),
        "10 5 5222 xmpp.goldragon.criome.net"
    );
    assert_eq!(
        RecordData::MailExchange(MailExchangeRecord {
            priority: Priority::new(10),
            record_target: RecordTarget::new("mail.criome.net"),
        })
        .presentation()
        .as_str(),
        "10 mail.criome.net"
    );
}

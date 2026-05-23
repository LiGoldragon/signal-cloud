use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use signal_cloud::{
    Capability, CapabilityQuery, DesiredState, DomainName, DomainNameSystemRecord, Observation,
    ObservationResult, Operation, OperationKind, PathTreatment, Plan, PlanIdentifier, Provider,
    RecordKind, RecordValue, RedirectRule, RedirectStatus, Reply, ReplyKind, RequestUnsupported,
    UniformResourceLocator, UnsupportedReason, Validation,
};
use signal_frame::{
    ExchangeFrame, ExchangeFrameBody, ExchangeIdentifier, ExchangeLane, LaneSequence,
    RequestPayload, SessionEpoch,
};

fn encode_to_text<T: NotaEncode>(value: &T) -> String {
    let mut encoder = Encoder::new();
    value.encode(&mut encoder).expect("encode");
    encoder.into_string()
}

fn desired_state() -> DesiredState {
    DesiredState {
        provider: Provider::Cloudflare,
        zone: DomainName::new("goldragon.criome"),
        records: vec![DomainNameSystemRecord {
            name: DomainName::new("goldragon.criome"),
            kind: RecordKind::AddressV4,
            value: RecordValue::new("203.0.113.10"),
            proxy_mode: signal_cloud::ProxyMode::ProviderProxy,
        }],
        redirects: vec![RedirectRule {
            source: DomainName::new("goldragon.criome"),
            target: UniformResourceLocator::new("https://linktr.ee/example"),
            status: RedirectStatus::Permanent,
            path_treatment: PathTreatment::Preserve,
        }],
    }
}

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

#[test]
fn operations_are_contract_local_without_sema_roots() {
    let operation = Operation::Validate(Validation {
        desired_state: desired_state(),
    });

    assert_eq!(operation.operation_kind(), OperationKind::Validate);
    assert_eq!(
        <Operation as signal_frame::SignalOperationHeads>::HEADS,
        &["Observe", "Validate"]
    );
}

#[test]
fn capability_observation_round_trips_through_nota() {
    let operation = Operation::Observe(Observation::Capabilities(CapabilityQuery {
        provider: Some(Provider::Cloudflare),
        capability: Some(Capability::RedirectRules),
    }));

    let text = encode_to_text(&operation);
    assert_eq!(
        text,
        "(Observe (Capabilities ((Some Cloudflare) (Some RedirectRules))))"
    );

    let mut decoder = Decoder::new(&text);
    let decoded = Operation::decode(&mut decoder).expect("decode");
    assert_eq!(decoded, operation);
}

#[test]
fn request_frame_round_trips_with_generated_operation() {
    let request = Operation::Validate(Validation {
        desired_state: desired_state(),
    })
    .into_request();
    let frame = signal_cloud::Frame::new(ExchangeFrameBody::Request {
        exchange: exchange(),
        request: request.clone(),
    });

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded =
        ExchangeFrame::<Operation, Reply>::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        ExchangeFrameBody::Request {
            exchange: decoded_exchange,
            request: decoded_request,
        } => {
            assert_eq!(decoded_exchange, exchange());
            assert_eq!(decoded_request, request);
        }
        _ => panic!("expected request frame"),
    }
}

#[test]
fn unsupported_provider_reply_round_trips_through_nota() {
    let reply = Reply::RequestUnsupported(RequestUnsupported {
        operation: OperationKind::Validate,
        provider: Some(Provider::Hetzner),
        capability: Some(Capability::RedirectRules),
        reason: UnsupportedReason::CapabilityNotCompiled,
    });

    assert_eq!(reply.kind(), ReplyKind::RequestUnsupported);

    let text = encode_to_text(&reply);
    let mut decoder = Decoder::new(&text);
    let decoded = Reply::decode(&mut decoder).expect("decode");
    assert_eq!(decoded, reply);
}

#[test]
fn plan_observation_reply_round_trips_through_nota() {
    let reply = Reply::Observed(ObservationResult::Plan(Plan {
        identifier: PlanIdentifier::new("plan-one"),
        provider: Provider::Cloudflare,
        zone: DomainName::new("goldragon.criome"),
        records_to_create: vec![],
        records_to_update: vec![],
        record_names_to_delete: vec![],
        redirects_to_create: vec![],
        redirects_to_update: vec![],
        redirect_sources_to_delete: vec![],
    }));

    let text = encode_to_text(&reply);
    let mut decoder = Decoder::new(&text);
    let decoded = Reply::decode(&mut decoder).expect("decode");
    assert_eq!(decoded, reply);
}

#[test]
fn contract_does_not_depend_on_deprecated_signal_core() {
    let manifest = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"))
        .expect("manifest");
    assert!(!manifest.contains("signal-core"));
}

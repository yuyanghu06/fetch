use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

#[derive(Deserialize, Serialize)]
struct RawEnvelope {
    result: Box<RawValue>,
}

#[test]
fn canonical_raw_payload_preserves_unknown_fields_numbers_and_media() {
    let raw = r#"{ "unknownFutureField": {"counter":184467440737095516160000000001,"decimal":1.234567890123456789e-300}, "content":[{"type":"image","data":"AAEC","mimeType":"image/png","future":true}], "structuredContent":{"precise":9007199254740993}, "isError":false }"#;
    let envelope: RawEnvelope = serde_json::from_str(&format!(r#"{{"result":{raw}}}"#)).unwrap();
    assert_eq!(envelope.result.get(), raw);
    let serialized = serde_json::to_string(&envelope).unwrap();
    assert!(serialized.contains(raw));
    let projected: serde_json::Value = serde_json::from_str(raw).unwrap();
    assert_eq!(
        projected["unknownFutureField"]["counter"].to_string(),
        "184467440737095516160000000001"
    );
    assert_eq!(
        projected["structuredContent"]["precise"].to_string(),
        "9007199254740993"
    );
}

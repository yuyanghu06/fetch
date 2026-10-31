use super::{CredentialError, CredentialStore, KeyProvider, SecretDocument};
use super::document::{FieldMapping, OAuthFields, OAuthPolicy, map_fields, normalize_oauth, update_section};
use serde_json::{Map, Value, json};
use std::sync::{Arc, Barrier};
use zeroize::Zeroizing;

struct FixtureKeys;
impl KeyProvider for FixtureKeys {
    fn key_for_server(&self, _: &str) -> Result<Zeroizing<[u8; 32]>, CredentialError> {
        Ok(Zeroizing::new([93; 32]))
    }
}

fn policy() -> OAuthPolicy {
    OAuthPolicy { require_new_refresh: false, require_expiry: false }
}

fn initial() -> Value { json!({"manual":{},"oauth":null}) }

fn fields(value: Value) -> Map<String, Value> { value.as_object().unwrap().clone() }

#[test]
fn named_values_can_be_read_from_different_nested_json_shapes() {
    let mapping = [
        FieldMapping { name: "session", pointer: "/credentials/access", required: true },
        FieldMapping { name: "workspace", pointer: "/credentials/team/id", required: true },
        FieldMapping { name: "optional", pointer: "/credentials/unused", required: false },
        FieldMapping { name: "escaped", pointer: "/a~1b/~0key", required: true },
    ];
    let result = map_fields(&json!({
        "credentials":{"access":"fixture-session","team":{"id":"fixture-team"},"ignored":"fixture-never-map"},
        "a/b":{"~key":"fixture-escaped"}
    }), &mapping).unwrap();
    assert_eq!(result, fields(json!({"session":"fixture-session","workspace":"fixture-team","escaped":"fixture-escaped"})));
    let flat = map_fields(&json!({"api_value":"fixture-other","browser_cookie":"fixture-cookie"}), &[
        FieldMapping { name: "arbitrary-token", pointer: "/api_value", required: true },
        FieldMapping { name: "arbitrary-cookie", pointer: "/browser_cookie", required: true },
    ]).unwrap();
    assert_eq!(flat.len(), 2);
    assert_eq!(flat["arbitrary-cookie"], "fixture-cookie");
}

#[test]
fn invalid_or_ambiguous_mappings_fail_before_any_values_are_returned() {
    let source = json!({"one":"fixture-secret","two":"fixture-second"});
    for mapping in [
        vec![FieldMapping { name:"value",pointer:"/missing",required:true }],
        vec![FieldMapping { name:"value",pointer:"one",required:true }],
        vec![FieldMapping { name:"value",pointer:"/one~2",required:false }],
        vec![FieldMapping { name:"",pointer:"/one",required:true }],
        vec![FieldMapping { name:"same",pointer:"/one",required:true }, FieldMapping { name:"same",pointer:"/two",required:true }],
        vec![FieldMapping { name:"first",pointer:"/one",required:true }, FieldMapping { name:"second",pointer:"/one",required:true }],
    ] {
        let error = map_fields(&source, &mapping).unwrap_err();
        assert!(!format!("{error:?}").contains("fixture-secret"));
    }
    assert!(map_fields(&json!({"one":null}), &[FieldMapping { name:"value",pointer:"/one",required:true }]).is_err());
    assert!(map_fields(&json!([]), &[FieldMapping { name:"value",pointer:"/one",required:false }]).is_err());
}

#[test]
fn partial_section_updates_preserve_other_credentials_and_login_metadata() {
    let mut document = json!({"manual":{"replace":"old","keep":"fixture-keep","remove":"obsolete"},
        "oauth":{"refresh_token":"fixture-refresh"},"client":{"id":"existing"},"future":{"field":42}});
    update_section(&mut document, "manual", &fields(json!({"replace":"fixture-new","pair":"fixture-pair"})), &["remove".into()]).unwrap();
    assert_eq!(document["manual"], json!({"replace":"fixture-new","keep":"fixture-keep","pair":"fixture-pair"}));
    assert_eq!(document["oauth"]["refresh_token"], "fixture-refresh");
    assert_eq!(document["client"]["id"], "existing");
    assert_eq!(document["future"]["field"], 42);
    let optional = map_fields(&json!({"new":"fixture-update"}), &[
        FieldMapping { name:"replace",pointer:"/new",required:true },
        FieldMapping { name:"keep",pointer:"/absent",required:false },
    ]).unwrap();
    update_section(&mut document, "manual", &optional, &[]).unwrap();
    assert_eq!(document["manual"]["keep"], "fixture-keep");
}

#[test]
fn rejected_section_update_never_partly_changes_the_document() {
    let original = json!({"manual":{"keep":"fixture-original"},"oauth":null});
    for (set, remove) in [
        (json!({"new":"fixture-new"}), vec!["new".to_owned()]),
        (json!({"new":"fixture-new"}), vec!["keep".to_owned(), "keep".to_owned()]),
        (json!({"":"fixture-new"}), vec![]),
    ] {
        let mut document = original.clone();
        assert!(update_section(&mut document, "manual", &fields(set), &remove).is_err());
        assert_eq!(document, original);
    }
    let mut malformed = json!({"manual":["fixture-original"]});
    let original = malformed.clone();
    assert!(update_section(&mut malformed, "manual", &fields(json!({"new":"fixture-new"})), &[]).is_err());
    assert_eq!(malformed, original);
}

#[test]
fn common_oauth_update_preserves_or_replaces_refresh_tokens_as_returned() {
    let token = json!({"access_token":"fixture-new-access","token_type":"bEaReR","expires_in":90});
    let result = normalize_oauth(&token, &OAuthFields::STANDARD, Some("fixture-prior-refresh"), 1000, policy()).unwrap();
    assert_eq!(result["access_token"], "fixture-new-access");
    assert_eq!(result["refresh_token"], "fixture-prior-refresh");
    assert_eq!(result["expires_at"], 1090);
    assert_eq!(result["verified"], false);
    let mut rotating = token.clone();
    rotating["refresh_token"] = json!("fixture-rotated-refresh");
    let result = normalize_oauth(&rotating, &OAuthFields::STANDARD, Some("fixture-prior-refresh"), 1000, policy()).unwrap();
    assert_eq!(result["refresh_token"], "fixture-rotated-refresh");
    let result = normalize_oauth(&token, &OAuthFields::STANDARD, None, 1000, policy()).unwrap();
    assert!(result["refresh_token"].is_null());
}

#[test]
fn oauth_field_names_and_required_rotation_are_not_tied_to_a_service_name() {
    let custom = OAuthFields { access_token:"/session/key",refresh_token:"/renewal/value",token_type:"/kind",expires_in:"/duration" };
    let source = json!({"session":{"key":"fixture-custom-access"},"renewal":{"value":"fixture-custom-refresh"},"kind":"Bearer","duration":300});
    let strict = OAuthPolicy { require_new_refresh:true, require_expiry:true };
    let result = normalize_oauth(&source, &custom, Some("fixture-old-refresh"), 50, strict).unwrap();
    assert_eq!(result["refresh_token"], "fixture-custom-refresh");
    assert_eq!(result["expires_at"], 350);
    assert!(normalize_oauth(&json!({"access_token":"fixture-access","token_type":"Bearer","expires_in":60}), &OAuthFields::STANDARD,
        Some("fixture-prior-refresh"), 0, OAuthPolicy { require_new_refresh:true,require_expiry:false }).is_err());
    assert!(normalize_oauth(&json!({"access_token":"fixture-access","token_type":"Bearer","refresh_token":"fixture-new-refresh"}), &OAuthFields::STANDARD,
        None, 0, OAuthPolicy { require_new_refresh:false,require_expiry:true }).is_err());
}

#[test]
fn malformed_supplied_oauth_values_cannot_silently_reuse_an_old_value() {
    let valid = json!({"access_token":"fixture-access","token_type":"Bearer","expires_in":60});
    for (name, value) in [
        ("access_token", json!("")), ("access_token", json!("fixture\r\nheader")),
        ("token_type", json!("Basic")), ("refresh_token", json!("")),
        ("refresh_token", json!(false)), ("refresh_token", Value::Null), ("refresh_token", json!("fixture\nrefresh")),
        ("expires_in", json!(0)), ("expires_in", Value::Null), ("expires_in", json!("60")), ("expires_in", json!(u64::MAX)),
    ] {
        let mut token = valid.clone();
        token[name] = value;
        let error = normalize_oauth(&token, &OAuthFields::STANDARD, Some("fixture-old-refresh"), 10, policy()).unwrap_err();
        assert!(!format!("{error:?}").contains("fixture"));
    }
}

#[test]
fn json_updates_return_encrypted_readback_and_do_not_rewrite_unchanged_values() {
    let directory = tempfile::tempdir().unwrap();
    let store = CredentialStore::new(directory.path().join("state"), Arc::new(FixtureKeys));
    let saved = store.update_json("first", &initial(), |document| {
        update_section(document, "manual", &fields(json!({"access":"fixture-encrypted-access"})), &[])
            .map_err(|_| CredentialError::InvalidDocument)
    }).unwrap();
    assert!(saved.changed);
    assert_eq!(store.read_json("first").unwrap().unwrap(), saved.document);
    assert_eq!(saved.document["manual"]["access"], "fixture-encrypted-access");
    assert!(!format!("{saved:?}").contains("fixture-encrypted-access"));
    let path = directory.path().join("state/servers").join(crate::storage::server_directory_name("first")).join("credentials.enc");
    let before = std::fs::read(&path).unwrap();
    assert!(!before.windows(b"fixture-encrypted-access".len()).any(|bytes| bytes == b"fixture-encrypted-access"));
    let unchanged = store.update_json("first", &initial(), |_| Ok(())).unwrap();
    assert!(!unchanged.changed);
    assert_eq!(before, std::fs::read(path).unwrap());
    assert!(store.read_json("other").unwrap().is_none());
    let created = store.update_json("other", &initial(), |_| Ok(())).unwrap();
    assert!(created.changed);
    assert_eq!(store.read_json("other").unwrap().unwrap(), created.document);
}

#[test]
fn invalid_json_or_failed_updates_preserve_the_previous_encrypted_file() {
    let directory = tempfile::tempdir().unwrap();
    let store = CredentialStore::new(directory.path().join("state"), Arc::new(FixtureKeys));
    store.save("first", &SecretDocument::new(b"{\"manual\":{\"keep\":\"fixture-original\"}}".to_vec())).unwrap();
    let before = store.read("first").unwrap().unwrap();
    let error = store.update_json("first", &initial(), |document| {
        document["manual"]["keep"] = json!("fixture-never-save");
        Err(CredentialError::InvalidDocument)
    }).unwrap_err();
    assert_eq!(error, CredentialError::InvalidDocument);
    assert_eq!(store.read("first").unwrap().unwrap().expose(), before.expose());
    for bytes in [b"not-json".as_slice(), b"[]".as_slice()] {
        store.save("malformed", &SecretDocument::new(bytes.to_vec())).unwrap();
        let before = store.read("malformed").unwrap().unwrap();
        assert!(store.read_json("malformed").is_err());
        assert!(store.update_json("malformed", &initial(), |_| Ok(())).is_err());
        assert_eq!(store.read("malformed").unwrap().unwrap().expose(), before.expose());
    }
}

#[test]
fn oversized_updates_and_tampered_ciphertext_cannot_replace_saved_credentials() {
    let directory = tempfile::tempdir().unwrap();
    let store = CredentialStore::new(directory.path().join("state"), Arc::new(FixtureKeys));
    store.update_json("first", &initial(), |document| {
        document["manual"]["keep"] = json!("fixture-original");
        Ok(())
    }).unwrap();
    let before = store.read("first").unwrap().unwrap();
    let error = store.update_json("first", &initial(), |document| {
        document["manual"]["huge"] = json!("x".repeat(1024 * 1024));
        Ok(())
    }).unwrap_err();
    assert_eq!(error, CredentialError::TooLarge);
    assert_eq!(store.read("first").unwrap().unwrap().expose(), before.expose());
    let path = directory.path().join("state/servers").join(crate::storage::server_directory_name("first")).join("credentials.enc");
    let mut tampered = std::fs::read(&path).unwrap();
    *tampered.last_mut().unwrap() ^= 1;
    std::fs::write(&path, &tampered).unwrap();
    assert_eq!(store.update_json("first", &initial(), |_| Ok(())).unwrap_err(), CredentialError::IntegrityFailed);
    assert_eq!(std::fs::read(path).unwrap(), tampered);
}

#[test]
fn concurrent_updates_to_one_server_keep_every_saved_value_and_separate_servers() {
    let directory = tempfile::tempdir().unwrap();
    let store = Arc::new(CredentialStore::new(directory.path().join("state"), Arc::new(FixtureKeys)));
    let barrier = Arc::new(Barrier::new(8));
    let threads: Vec<_> = (0..8).map(|index| {
        let store = store.clone();
        let barrier = barrier.clone();
        std::thread::spawn(move || {
            barrier.wait();
            store.update_json("same", &initial(), |document| {
                document["manual"][format!("value-{index}")] = json!(format!("fixture-{index}"));
                Ok(())
            }).unwrap();
        })
    }).collect();
    for thread in threads { thread.join().unwrap(); }
    let saved = store.read_json("same").unwrap().unwrap();
    assert_eq!(saved["manual"].as_object().unwrap().len(), 8);
    for index in 0..8 { assert_eq!(saved["manual"][format!("value-{index}")], format!("fixture-{index}")); }
    store.update_json("separate", &initial(), |document| {
        document["manual"]["value-0"] = json!("fixture-separate");
        Ok(())
    }).unwrap();
    assert_eq!(store.read_json("same").unwrap().unwrap()["manual"]["value-0"], "fixture-0");
}

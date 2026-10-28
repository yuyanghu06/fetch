use fetch_local::application::{
    app_port_for_build, data_root_for_home, default_app_port, default_data_root,
};
use std::path::{Path, PathBuf};

#[test]
fn development_and_release_state_are_separate_siblings_for_each_user_home() {
    let fixture = tempfile::tempdir().unwrap();
    let homes = [
        fixture.path().join("first-user"),
        fixture.path().join("another-location/second-user"),
    ];
    let mut resolved_roots = Vec::new();

    for home in &homes {
        let development = data_root_for_home(home, true).unwrap();
        let release = data_root_for_home(home, false).unwrap();
        let support = home.join("Library/Application Support");

        assert_eq!(development, support.join("Fetch-Development"));
        assert_eq!(release, support.join("Fetch"));
        assert_eq!(development.parent(), release.parent());
        assert!(!development.starts_with(&release));
        assert!(!release.starts_with(&development));
        assert!(
            !home.exists(),
            "Resolving defaults must not create user state"
        );
        resolved_roots.push((development, release));
    }

    assert_ne!(resolved_roots[0].0, resolved_roots[1].0);
    assert_ne!(resolved_roots[0].1, resolved_roots[1].1);
}

#[test]
fn development_and_release_have_distinct_fixed_listeners() {
    assert_eq!(app_port_for_build(true), 53672);
    assert_eq!(app_port_for_build(false), 53671);
}

#[test]
fn actual_defaults_follow_the_current_home_and_build() {
    let expected_port = if cfg!(debug_assertions) { 53672 } else { 53671 };
    assert_eq!(default_app_port(), expected_port);

    let current_home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .filter(|home| home.is_absolute());
    match current_home {
        Some(home) => {
            let directory = if cfg!(debug_assertions) {
                "Fetch-Development"
            } else {
                "Fetch"
            };
            assert_eq!(
                default_data_root().unwrap(),
                home.join("Library/Application Support").join(directory)
            );
        }
        None => assert!(default_data_root().is_err()),
    }
}

#[test]
fn missing_relative_or_unexpanded_home_paths_are_rejected() {
    for home in [Path::new(""), Path::new("relative-home"), Path::new("~/")] {
        assert!(data_root_for_home(home, true).is_err());
        assert!(data_root_for_home(home, false).is_err());
    }
}

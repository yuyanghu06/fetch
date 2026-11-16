//! Core project records and preparation use disposable files only. No APIs,
//! authentication, downloads, executable MCPs or personal state are exercised.
use fetch_local::{
    discovery::DiscoveryService,
    installation::Installer,
    projects::{Project, ProjectRegistry, visible_server_ids},
    storage::ServerRepository,
};
use serde_json::{Value, json};
use std::{collections::BTreeSet, fs, path::Path, sync::Arc, time::Duration};

fn record(id:&str, project:Option<&str>) -> Value {
    let mut value=json!({
        "server_id":id,"name":"Database","runtime":{"status":"stopped","error":null},
        "authentication":{"status":"unknown","error":null},"restart_required":false,"restart":null,
        "config":{"kind":"remote","package":null,"executable":null,"args":[],"working_directory":null,
            "environment":[],"connection":{"url":"https://never-used.fixture.invalid/mcp","transport":"streamable_http","headers":[]}}
    });
    if let Some(project)=project {value["config"]["scope"]=json!({"project_id":project});}
    value
}
fn ids(repository:&ServerRepository,project:Option<&str>) -> BTreeSet<String> {
    visible_server_ids(repository,project).unwrap().into_iter().collect()
}

#[test]
fn canonical_projects_persist_and_shared_visibility_does_not_rewrite_legacy_servers() {
    let directory=tempfile::tempdir().unwrap();let root=directory.path().join("fetch-state");
    let a_folder=directory.path().join("project-a");let b_folder=directory.path().join("project-b");
    fs::create_dir_all(&a_folder).unwrap();fs::create_dir_all(&b_folder).unwrap();
    let registry=ProjectRegistry::new(root.clone());
    assert!(registry.list().unwrap().is_empty());assert!(!root.exists(),"reading an empty registry creates no state");
    let (a,created)=registry.register(a_folder.to_str().unwrap(),Some("A")).unwrap();assert!(created);
    assert_eq!(a.folder_path,a_folder.canonicalize().unwrap());
    let canonical=a_folder.canonicalize().unwrap();
    let (again,created)=registry.register(canonical.to_str().unwrap(),Some("Do not rename A")).unwrap();
    assert!(!created);assert_eq!(again,a);
    #[cfg(unix)] {
        let alias=directory.path().join("project-a-alias");std::os::unix::fs::symlink(&a_folder,&alias).unwrap();
        let (through_alias,created)=registry.register(alias.to_str().unwrap(),None).unwrap();
        assert!(!created);assert_eq!(through_alias,a);
    }
    let (b,_)=registry.register(b_folder.to_str().unwrap(),Some("B")).unwrap();
    let reopened=ProjectRegistry::new(root.clone());assert_eq!(reopened.get(&a.id).unwrap(),Some(a.clone()));assert_eq!(reopened.list().unwrap().len(),2);

    let repository=ServerRepository::new(root);
    let legacy=record("legacy-shared",None);repository.save("legacy-shared",&legacy).unwrap();
    let mut shared=record("explicit-shared",None);shared["config"]["scope"]=json!({"project_id":null});repository.save("explicit-shared",&shared).unwrap();
    repository.save("a-server",&record("a-server",Some(&a.id))).unwrap();
    repository.save("b-server",&record("b-server",Some(&b.id))).unwrap();
    let shared_ids=BTreeSet::from(["legacy-shared".into(),"explicit-shared".into()]);
    assert_eq!(ids(&repository,None),shared_ids);
    let mut a_ids=shared_ids.clone();a_ids.insert("a-server".into());assert_eq!(ids(&repository,Some(&a.id)),a_ids);
    let mut b_ids=shared_ids;b_ids.insert("b-server".into());assert_eq!(ids(&repository,Some(&b.id)),b_ids);
    assert_eq!(repository.get("legacy-shared").unwrap().unwrap()["config"]["scope"]["project_id"],Value::Null);
    assert_eq!(repository.get_saved("legacy-shared").unwrap().unwrap(),legacy,"default Shared projection leaves saved legacy configuration untouched");
}

async fn prepare(installer:&Arc<Installer>,input:Value,project:Option<Project>) -> Value {
    let accepted=installer.submit_scoped(input,project).await.unwrap();let id=accepted["installation_id"].as_str().unwrap();
    tokio::time::timeout(Duration::from_secs(3),async{
        loop {let progress=installer.progress(id).await.unwrap();if progress["status"]!="installing"{return progress;}tokio::task::yield_now().await;}
    }).await.expect("local fixture preparation needs no runtime execution")
}

#[tokio::test]
async fn project_preparation_keeps_reuse_cwd_discovery_and_progress_in_the_selected_scope() {
    let directory=tempfile::tempdir().unwrap();let root=directory.path().join("fetch-state");
    let a_folder=directory.path().join("project-a");let b_folder=directory.path().join("project-b");let home=directory.path().join("empty-home");
    for path in [&a_folder,&b_folder,&home]{fs::create_dir_all(path).unwrap();}
    let registry=ProjectRegistry::new(root.clone());let a=registry.register(a_folder.to_str().unwrap(),None).unwrap().0;let b=registry.register(b_folder.to_str().unwrap(),None).unwrap().0;
    let program=directory.path().join("unexecuted-fixture-program");fs::write(&program,"#!/bin/sh\nexit 91\n").unwrap();
    let explicit=a.folder_path.join("explicit-cwd");fs::create_dir_all(&explicit).unwrap();
    let source=a.folder_path.join(".mcp.json");
    fs::write(&source,json!({"mcpServers":{"Database":{"command":program},"Explicit":{"command":program,"cwd":explicit}}}).to_string()).unwrap();
    let discovery=DiscoveryService::new(home,None);let repository=ServerRepository::new(root.clone());let installer=Installer::new(root,discovery.clone());
    let selection=json!({"servers":[{"config_path":source,"server_name":"Database"}]});
    let first=prepare(&installer,selection.clone(),Some(a.clone())).await;assert_eq!(first["status"],"succeeded");
    let a_id=first["servers"][0]["server_id"].as_str().unwrap();
    let group=first["installation_id"].as_str().unwrap();
    let default=repository.get_saved(a_id).unwrap().unwrap();assert_eq!(default["config"]["scope"]["project_id"],a.id);
    assert_eq!(Path::new(default["config"]["working_directory"].as_str().unwrap()),a.folder_path);
    let listed=discovery.scan_scoped(None,&repository,Some(&a)).unwrap();
    let row=listed["configs"].as_array().unwrap().iter().flat_map(|config|config["servers"].as_array().unwrap()).find(|row|row["name"]=="Database").unwrap();
    assert_eq!(row["installed"],true);assert_eq!(row["installed_server_id"],a_id);
    let b_before=discovery.scan_scoped(Some(&source),&repository,Some(&b)).unwrap();
    let row=b_before["configs"].as_array().unwrap().iter().flat_map(|config|config["servers"].as_array().unwrap()).find(|row|row["name"]=="Database").unwrap();assert_eq!(row["installed"],false);

    let again=prepare(&installer,selection.clone(),Some(a.clone())).await;assert_eq!(again["servers"][0]["server_id"],a_id);
    let second=prepare(&installer,selection.clone(),Some(b.clone())).await;assert_eq!(second["status"],"succeeded");
    let b_id=second["servers"][0]["server_id"].as_str().unwrap();assert_ne!(a_id,b_id);
    let saved=repository.get_saved(b_id).unwrap().unwrap();assert_eq!(saved["config"]["working_directory"],b.folder_path.to_str().unwrap());
    let global=prepare(&installer,selection,None).await;let shared_id=global["servers"][0]["server_id"].as_str().unwrap();assert_ne!(shared_id,a_id);assert_ne!(shared_id,b_id);
    assert!(repository.get_saved(shared_id).unwrap().unwrap()["config"]["working_directory"].is_null());
    let preserved=prepare(&installer,json!({"servers":[{"config_path":source,"server_name":"Explicit"}]}),Some(b.clone())).await;
    let saved=repository.get_saved(preserved["servers"][0]["server_id"].as_str().unwrap()).unwrap().unwrap();assert_eq!(saved["config"]["working_directory"],explicit.to_str().unwrap());
    assert_eq!(installer.progress_scoped(group,Some(&a.id)).await.unwrap()["status"],"succeeded");
    assert_eq!(installer.progress_scoped(group,Some(&b.id)).await.unwrap_err().http_status,404);
    assert_eq!(installer.progress_scoped(group,None).await.unwrap_err().http_status,404);
    assert_eq!(fs::read_to_string(&program).unwrap(),"#!/bin/sh\nexit 91\n");
    installer.shutdown();
}

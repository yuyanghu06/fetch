//! Actual native controls with API replies and dropped Tasks: no accounts or files are touched.
use super::{Message,Page,auth_tests::desktop_fixture,neumorphic_tests::ViewFixture,project_scope::{Choice,Event},workspace};
use iced::Size;
use serde_json::json;

#[tokio::test]
async fn registering_a_workspace_folder_keeps_server_install_and_connection_choices_independent(){
    let (_directory,mut desktop)=desktop_fixture().await;
    desktop.page=Page::Workspace;
    desktop.workspace.folder_path=Some("/fixture/project-a".into());
    desktop.workspace.folder_input="/fixture/project-a".into();
    let project=json!({"id":"project-aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","name":"Project A","folder_path":"/fixture/project-a","mcp_url":"https://127.0.0.1/mcp/projects/project-aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"});
    let mut view=ViewFixture::new(&desktop,Size::new(874.0,750.0)).await;
    let task=desktop.update(Message::Workspace(workspace::Event::Discovered(0,Ok(json!({"folder_path":"/fixture/project-a","files":[],"agent_directories":[]})))));
    assert!(task.units()>0,"successful folder opening automatically submits registration");drop(task);
    assert!(!view.survey(&desktop).text.iter().any(|(text,_)|text.contains("Register project")||text=="Project registered"));
    // Inject the returned native API result, without executing a request.
    drop(desktop.update(Message::Projects(Event::Registered(Ok(json!({"project":project}))))));
    assert_eq!(desktop.projects.records.len(),1);
    assert_eq!(desktop.projects.server_filter,Choice::All);
    assert_eq!(desktop.projects.install,Choice::Shared);
    assert_eq!(desktop.projects.connection,Choice::Shared);
    let a=Choice::Project{id:"project-aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into(),name:"Project A".into()};
    drop(desktop.update(Message::Projects(Event::Connection(a.clone()))));
    drop(desktop.update(Message::Projects(Event::Install(a.clone()))));
    drop(desktop.update(Message::Projects(Event::Filter(Choice::Shared))));
    view.relayout(&desktop);
    drop(desktop.update(Message::Workspace(workspace::Event::FolderInput(String::new()))));
    drop(desktop.update(Message::Workspace(workspace::Event::OpenFolder)));
    assert!(desktop.workspace.folder_path.is_none());
    assert_eq!(desktop.projects.connection,a);
    assert_eq!(desktop.projects.install,a);
    assert_eq!(desktop.projects.server_filter,Choice::Shared);
    drop(desktop.update(Message::Workspace(workspace::Event::FolderInput("/fixture/project-b".into()))));
    drop(desktop.update(Message::Workspace(workspace::Event::OpenFolder)));
    assert_eq!(desktop.projects.connection,a,"browsing B must leave connected A unchanged");
    desktop.page=Page::Connect;
    let command=desktop.connector_command("codex");
    let parsed:toml::Value=toml::from_str(&command).expect("project Codex snippet is native TOML");
    assert!(parsed["mcp_servers"]["fetch"]["args"].as_array().unwrap().iter().any(|arg|arg.as_str()==a.project_id()));
    assert!(desktop.connector_command("claude-code").contains("--scope project"));
    view.relayout(&desktop);view.export(&desktop,"project-connect-a");
    desktop.page=Page::Install;view.relayout(&desktop);view.export(&desktop,"project-install-a");
    desktop.page=Page::Servers;view.relayout(&desktop);view.export(&desktop,"project-server-shared");
}

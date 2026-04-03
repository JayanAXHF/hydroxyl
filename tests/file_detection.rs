use std::fs;

use hydroxyl::services::workspace_service::WorkspaceService;

#[test]
fn workspace_service_reads_server_properties_and_file_lists() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    let world = root.join("survival");

    fs::create_dir_all(world.join("playerdata")).unwrap();
    fs::create_dir_all(world.join("stats")).unwrap();
    fs::create_dir_all(world.join("advancements")).unwrap();
    fs::write(
        root.join("server.properties"),
        "online-mode=true\nlevel-name=survival\n",
    )
    .unwrap();
    fs::write(world.join("playerdata").join("abc.dat"), b"test").unwrap();
    fs::write(world.join("stats").join("abc.json"), b"{}").unwrap();
    fs::write(world.join("advancements").join("abc.json"), b"{}").unwrap();

    let workspace = WorkspaceService.load(root).unwrap();

    assert!(workspace.online_mode);
    assert_eq!(workspace.level_name, "survival");
    assert_eq!(workspace.player_files.len(), 1);
    assert_eq!(workspace.stats_files.len(), 1);
    assert_eq!(workspace.advancements_files.len(), 1);
    assert_eq!(workspace.player_entries.len(), 1);
    assert_eq!(workspace.stats_entries.len(), 1);
    assert_eq!(workspace.advancements_entries.len(), 1);
}

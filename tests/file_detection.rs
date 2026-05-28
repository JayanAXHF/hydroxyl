use std::fs;

use hydroxyl::{
    app::context::WorldFileStructure,
    domain::files::{detect::detect_file_kind, kind::FileKind},
    services::workspace_service::WorkspaceService,
};

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

    let workspace = WorkspaceService
        .load(root, WorldFileStructure::Legacy)
        .unwrap();

    assert!(workspace.online_mode);
    assert_eq!(workspace.level_name, "survival");
    assert_eq!(workspace.player_files.len(), 1);
    assert_eq!(workspace.stats_files.len(), 1);
    assert_eq!(workspace.advancements_files.len(), 1);
    assert_eq!(workspace.player_entries.len(), 1);
    assert_eq!(workspace.stats_entries.len(), 1);
    assert_eq!(workspace.advancements_entries.len(), 1);
}

#[test]
fn workspace_service_reads_new_file_structure_file_lists() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    let world = root.join("survival");

    fs::create_dir_all(world.join("players").join("data")).unwrap();
    fs::create_dir_all(world.join("players").join("stats")).unwrap();
    fs::create_dir_all(world.join("players").join("advancements")).unwrap();
    fs::create_dir_all(world.join("dimensions").join("minecraft").join("overworld")).unwrap();
    fs::write(
        root.join("server.properties"),
        "online-mode=true\nlevel-name=survival\n",
    )
    .unwrap();
    fs::write(world.join("players").join("data").join("abc.dat"), b"test").unwrap();
    fs::write(world.join("players").join("stats").join("abc.json"), b"{}").unwrap();
    fs::write(
        world.join("players").join("advancements").join("abc.json"),
        b"{}",
    )
    .unwrap();

    let workspace = WorkspaceService
        .load(root, WorldFileStructure::New)
        .unwrap();

    assert!(workspace.online_mode);
    assert_eq!(workspace.level_name, "survival");
    assert_eq!(workspace.player_files.len(), 1);
    assert_eq!(workspace.stats_files.len(), 1);
    assert_eq!(workspace.advancements_files.len(), 1);
    assert_eq!(workspace.player_entries.len(), 1);
    assert_eq!(workspace.stats_entries.len(), 1);
    assert_eq!(workspace.advancements_entries.len(), 1);
    assert!(workspace.player_files[0].ends_with("players/data/abc.dat"));
    assert!(workspace.stats_files[0].ends_with("players/stats/abc.json"));
    assert!(workspace.advancements_files[0].ends_with("players/advancements/abc.json"));
}

#[test]
fn file_detection_supports_layout_specific_player_data_paths() {
    let legacy = std::path::Path::new("world/playerdata/abc.dat");
    let new = std::path::Path::new("world/players/data/abc.dat");
    let stats = std::path::Path::new("world/players/stats/abc.json");
    let advancements = std::path::Path::new("world/players/advancements/abc.json");

    assert_eq!(
        detect_file_kind(legacy, WorldFileStructure::Legacy),
        FileKind::PlayerData
    );
    assert_eq!(
        detect_file_kind(new, WorldFileStructure::Legacy),
        FileKind::Nbt
    );
    assert_eq!(
        detect_file_kind(new, WorldFileStructure::New),
        FileKind::PlayerData
    );
    assert_eq!(
        detect_file_kind(stats, WorldFileStructure::New),
        FileKind::Stats
    );
    assert_eq!(
        detect_file_kind(advancements, WorldFileStructure::New),
        FileKind::Advancements
    );
}

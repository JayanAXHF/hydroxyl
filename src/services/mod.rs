pub mod advancements_service;
pub mod backup_service;
pub mod cache_service;
pub mod document_service;
pub mod mojang_api;
pub mod nbt_service;
pub mod player_service;
pub mod save_service;
pub mod skin_service;
pub mod stats_service;
pub mod workspace_service;

use crate::util::result::Result;

use self::{
    advancements_service::AdvancementsService, backup_service::BackupService,
    nbt_service::NbtService, player_service::PlayerService, save_service::SaveService,
    skin_service::SkinService, stats_service::StatsService, workspace_service::WorkspaceService,
};

pub struct AppServices {
    pub workspace: WorkspaceService,
    pub player: PlayerService,
    pub nbt: NbtService,
    pub stats: StatsService,
    pub advancements: AdvancementsService,
    pub backup: BackupService,
    pub save: SaveService,
    pub skins: SkinService,
}

impl AppServices {
    pub fn new() -> Result<Self> {
        Ok(Self {
            workspace: WorkspaceService,
            player: PlayerService,
            nbt: NbtService,
            stats: StatsService,
            advancements: AdvancementsService,
            backup: BackupService,
            save: SaveService,
            skins: SkinService::new()?,
        })
    }
}

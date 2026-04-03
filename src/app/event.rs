use crossterm::event::KeyEvent;
use uuid::Uuid;

use crate::domain::minecraft::profile::Face8x8;

#[derive(Debug, Clone)]
pub enum AppEvent {
    Key(KeyEvent),
    Tick,
    SkinFetched(SkinUpdate),
}

#[derive(Debug, Clone)]
pub struct SkinUpdate {
    pub uuid: Uuid,
    pub result: Result<Face8x8, String>,
    pub resolved_name: Option<String>,
    pub skin_url: Option<String>,
}

use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum AsyncCommand {
    FetchSkin(Uuid),
}

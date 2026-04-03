#[derive(Debug, Clone)]
pub struct PlayerAbilities {
    pub flying: bool,
    pub may_fly: bool,
    pub instabuild: bool,
    pub invulnerable: bool,
    pub may_build: bool,
    pub walk_speed: f32,
    pub fly_speed: f32,
}

impl Default for PlayerAbilities {
    fn default() -> Self {
        Self {
            flying: false,
            may_fly: false,
            instabuild: false,
            invulnerable: false,
            may_build: true,
            walk_speed: 0.1,
            fly_speed: 0.05,
        }
    }
}

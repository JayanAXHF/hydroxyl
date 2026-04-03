#[derive(Debug, Clone)]
pub struct PlayerAttributes {
    pub health: f32,
    pub food_level: i32,
    pub food_saturation: f32,
    pub xp_level: i32,
    pub xp_progress: f32,
    pub xp_total: i32,
    pub air: i32,
}

impl Default for PlayerAttributes {
    fn default() -> Self {
        Self {
            health: 20.0,
            food_level: 20,
            food_saturation: 5.0,
            xp_level: 0,
            xp_progress: 0.0,
            xp_total: 0,
            air: 300,
        }
    }
}

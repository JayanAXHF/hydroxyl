use ratatui::style::Color;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PlayerIdentity {
    pub uuid: Option<Uuid>,
    pub name: Option<String>,
    pub skin_url: Option<String>,
}

impl Default for PlayerIdentity {
    fn default() -> Self {
        Self {
            uuid: None,
            name: None,
            skin_url: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SkinState {
    NotRequested,
    Loading,
    Ready(Face8x8),
    Unavailable(String),
}

impl Default for SkinState {
    fn default() -> Self {
        Self::NotRequested
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FacePixel {
    pub color: Color,
}

#[derive(Debug, Clone)]
pub struct Face8x8 {
    pub pixels: [[FacePixel; 8]; 8],
}

impl Face8x8 {
    pub fn placeholder() -> Self {
        let skin = [
            [229, 194, 159],
            [229, 194, 159],
            [229, 194, 159],
            [70, 42, 31],
            [70, 42, 31],
            [229, 194, 159],
            [229, 194, 159],
            [229, 194, 159],
        ];

        let mut pixels = [[FacePixel {
            color: Color::Rgb(0, 0, 0),
        }; 8]; 8];
        for (y, row) in pixels.iter_mut().enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                let rgb = if y < 2 {
                    [92, 61, 46]
                } else if y == 3 && (x == 2 || x == 5) {
                    [255, 255, 255]
                } else if y == 4 && (x == 2 || x == 5) {
                    [50, 50, 50]
                } else {
                    skin[x]
                };
                pixel.color = Color::Rgb(rgb[0], rgb[1], rgb[2]);
            }
        }
        Self { pixels }
    }
}

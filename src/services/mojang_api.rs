use base64::{Engine as _, engine::general_purpose::STANDARD};
use image::{Rgba, RgbaImage};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    domain::minecraft::profile::{Face8x8, FacePixel},
    util::{error::HydroxylError, result::Result},
};

#[derive(Clone)]
pub struct MojangApi {
    client: reqwest::blocking::Client,
}

impl MojangApi {
    pub fn new() -> Result<Self> {
        let client = reqwest::blocking::Client::builder()
            .user_agent("hydroxyl/0.1.0")
            .timeout(std::time::Duration::from_secs(10))
            .build()?;
        Ok(Self { client })
    }

    pub fn fetch_profile(&self, uuid: Uuid) -> Result<ResolvedProfile> {
        let profile_url = format!(
            "https://sessionserver.mojang.com/session/minecraft/profile/{}",
            uuid.as_simple()
        );
        let response = self.client.get(profile_url).send()?.error_for_status()?;
        let profile: MojangProfileResponse = response.json()?;
        let textures = profile
            .properties
            .iter()
            .find(|property| property.name == "textures")
            .ok_or_else(|| HydroxylError::invalid_data("missing Mojang textures property"))?;

        let decoded = STANDARD.decode(textures.value.as_bytes())?;
        let payload: TexturesPayload = serde_json::from_slice(&decoded)?;
        let skin_url = payload.textures.skin.map(|skin| skin.url);

        Ok(ResolvedProfile {
            name: profile.name,
            skin_url,
        })
    }

    pub fn fetch_face(&self, skin_url: &str) -> Result<Face8x8> {
        let bytes = self
            .client
            .get(skin_url)
            .send()?
            .error_for_status()?
            .bytes()?;
        let image = image::load_from_memory(bytes.as_ref())?.to_rgba8();
        Ok(extract_face(&image))
    }

    pub fn fetch_skin(&self, uuid: Uuid) -> Result<ResolvedSkin> {
        let profile = self.fetch_profile(uuid)?;
        let Some(skin_url) = profile.skin_url.clone() else {
            return Err(HydroxylError::invalid_data("missing Mojang skin URL"));
        };

        Ok(ResolvedSkin {
            name: Some(profile.name),
            skin_url: Some(skin_url.clone()),
            face: self.fetch_face(&skin_url)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ResolvedProfile {
    pub name: String,
    pub skin_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ResolvedSkin {
    pub name: Option<String>,
    pub skin_url: Option<String>,
    pub face: Face8x8,
}

#[derive(Debug, Deserialize)]
struct MojangProfileResponse {
    name: String,
    properties: Vec<MojangProperty>,
}

#[derive(Debug, Deserialize)]
struct MojangProperty {
    name: String,
    value: String,
}

#[derive(Debug, Deserialize)]
struct TexturesPayload {
    textures: Textures,
}

#[derive(Debug, Deserialize)]
struct Textures {
    #[serde(rename = "SKIN")]
    skin: Option<SkinTexture>,
}

#[derive(Debug, Deserialize)]
struct SkinTexture {
    url: String,
}

fn extract_face(image: &RgbaImage) -> Face8x8 {
    if image.width() < 48 || image.height() < 16 {
        return Face8x8::placeholder();
    }

    let mut pixels = [[FacePixel {
        color: ratatui::style::Color::Rgb(0, 0, 0),
    }; 8]; 8];
    for y in 0..8 {
        for x in 0..8 {
            let mut pixel = composite_over_background(*image.get_pixel(8 + x, 8 + y));
            let overlay = image.get_pixel(40 + x, 8 + y).0;
            if overlay[3] > 0 {
                pixel = blend(pixel, overlay);
            }
            pixels[y as usize][x as usize] = FacePixel {
                color: ratatui::style::Color::Rgb(pixel[0], pixel[1], pixel[2]),
            };
        }
    }
    Face8x8 { pixels }
}

fn composite_over_background(pixel: Rgba<u8>) -> [u8; 4] {
    if pixel[3] == 255 {
        return pixel.0;
    }

    let background = [24, 24, 24, 255];
    blend(background, pixel.0)
}

fn blend(base: [u8; 4], overlay: [u8; 4]) -> [u8; 4] {
    let alpha = overlay[3] as f32 / 255.0;
    let mix = |index: usize| {
        ((overlay[index] as f32 * alpha) + (base[index] as f32 * (1.0 - alpha))) as u8
    };
    [mix(0), mix(1), mix(2), 255]
}

impl From<Rgba<u8>> for FacePixel {
    fn from(value: Rgba<u8>) -> Self {
        Self {
            color: ratatui::style::Color::Rgb(value[0], value[1], value[2]),
        }
    }
}

#[cfg(test)]
mod tests {
    use image::{Rgba, RgbaImage};
    use ratatui::style::Color;

    use super::extract_face;

    #[test]
    fn extract_face_blends_hat_layer() {
        let mut image = RgbaImage::from_pixel(64, 64, Rgba([0, 0, 0, 0]));
        image.put_pixel(8, 8, Rgba([10, 20, 30, 255]));
        image.put_pixel(40, 8, Rgba([250, 150, 50, 128]));

        let face = extract_face(&image);
        let Color::Rgb(red, green, blue) = face.pixels[0][0].color else {
            panic!("expected rgb color")
        };

        assert!(red > 10);
        assert!(green > 20);
        assert!(blue > 30);
    }
}

use macroquad::{
    color::WHITE, file::load_string, math::Rect, texture::{draw_texture, load_texture, FilterMode, Texture2D}
};

use macroquad_tiled::{self as tiled, Map};
use crate::constants::*;

pub struct GameMap {
    name: String,
    background_texture: Texture2D,
    map: Map,
    layers: Vec<String>,
}

impl GameMap {
    pub async fn new(
        name: String,
        background_texture_path: String,
        tileset_texture_path: String,
        tileset_json_path: String,
        tileset_json_tileset_name: String,
        layers: Vec<String>,
    ) -> GameMap {
        let background_texture = load_texture(&background_texture_path).await.unwrap();
        let tileset_texture = load_texture(&tileset_texture_path).await.unwrap();

        tileset_texture.set_filter(FilterMode::Nearest);

        let tiled_map_json = load_string(&tileset_json_path).await.unwrap();
        let map = tiled::load_map(&tiled_map_json, &[(&tileset_json_tileset_name, tileset_texture.clone())], &[]).unwrap();

        GameMap {
            name,
            background_texture,
            map,
            layers,
        }
    }

    pub fn draw_map(&self) {
        let background = &self.background_texture;

        draw_texture(background, 0., 0., WHITE);

        for layer in &self.layers {
            self.map.draw_tiles(layer, Rect::new(0.0, 0.0, WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32), None);
        }
    }

    pub fn get_name(&self) -> String { self.name.clone() }
}

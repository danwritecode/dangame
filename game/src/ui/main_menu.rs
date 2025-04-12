use std::cell::RefCell;
use std::rc::Rc;

use macroquad::prelude::*;

use macroquad::ui::{
    hash, root_ui,
    widgets::{self, Group},
};

use crate::maps::map::GameMap;

pub enum GameMode {
    SinglePlayer,
    Multiplayer,
}

#[derive(Clone)]
pub enum CharacterSelection {
    Character1,
    Character2,
    Character3,
}

pub struct MenuState {
    pub game_mode: Option<GameMode>,
    pub character_selection: Option<CharacterSelection>,
    pub map_selection: Option<usize>,
    pub connect_pressed: bool,
}

impl MenuState {
    pub fn new() -> Self {
        Self {
            game_mode: None,
            character_selection: None,
            map_selection: None,
            connect_pressed: false,
        }
    }
    
    /// Go back one step in the menu
    pub fn back(&mut self) {
        if self.map_selection.is_some() {
            self.map_selection = None;
        } else if self.connect_pressed {
            self.connect_pressed = false;
        } else if self.character_selection.is_some() {
            self.character_selection = None;
        } else if self.game_mode.is_some() {
            self.game_mode = None;
        }
    }
}

pub async fn draw_menu(background: &Texture2D, maps: &Vec<GameMap>, menu_state: &mut MenuState, callback: Rc<RefCell<impl FnMut(usize)>>) {
    draw_texture(&background, 0., 0., WHITE);

    // Game Mode Selection Window
    if menu_state.game_mode.is_none() {
        widgets::Window::new(hash!(), vec2(500., 600.), vec2(310., 180.))
            .label("Game Mode")
            .titlebar(true)
            .ui(&mut *root_ui(), |ui| {
                Group::new(hash!("game-mode"), Vec2::new(300., 120.)).ui(ui, |ui| {
                    ui.label(Vec2::new(10., 10.), "Select Game Mode:");
                    
                    if ui.button(Vec2::new(40., 40.), "Single Player") {
                        menu_state.game_mode = Some(GameMode::SinglePlayer);
                    }
                    
                    if ui.button(Vec2::new(40., 80.), "Multiplayer") {
                        menu_state.game_mode = Some(GameMode::Multiplayer);
                    }
                });
            });
    }
    // Character Selection Window
    else if menu_state.character_selection.is_none() {
        widgets::Window::new(hash!(), vec2(500., 600.), vec2(310., 260.))
            .label("Character Selection")
            .titlebar(true)
            .ui(&mut *root_ui(), |ui| {
                Group::new(hash!("character-select"), Vec2::new(300., 200.)).ui(ui, |ui| {
                    ui.label(Vec2::new(10., 10.), "Select Character:");
                    
                    if ui.button(Vec2::new(40., 40.), "Character 1") {
                        menu_state.character_selection = Some(CharacterSelection::Character1);
                    }
                    
                    if ui.button(Vec2::new(40., 80.), "Character 2") {
                        menu_state.character_selection = Some(CharacterSelection::Character2);
                    }
                    
                    if ui.button(Vec2::new(40., 120.), "Character 3") {
                        menu_state.character_selection = Some(CharacterSelection::Character3);
                    }
                    
                    // Back button
                    if ui.button(Vec2::new(40., 160.), "Back") {
                        menu_state.back();
                    }
                });
            });
    }
    // Multiplayer Connect Button (if in multiplayer mode)
    else if matches!(menu_state.game_mode, Some(GameMode::Multiplayer)) && !menu_state.connect_pressed {
        widgets::Window::new(hash!(), vec2(500., 600.), vec2(310., 160.))
            .label("Connect")
            .titlebar(true)
            .ui(&mut *root_ui(), |ui| {
                Group::new(hash!("connect"), Vec2::new(300., 100.)).ui(ui, |ui| {
                    ui.label(Vec2::new(10., 10.), "Ready to connect:");
                    
                    if ui.button(Vec2::new(40., 40.), "Connect") {
                        menu_state.connect_pressed = true;
                    }
                    
                    // Back button
                    if ui.button(Vec2::new(40., 80.), "Back") {
                        menu_state.back();
                    }
                });
            });
    }
    // Map Selection Window (after completing previous steps)
    else if menu_state.map_selection.is_none() {
        widgets::Window::new(hash!(), vec2(500., 600.), vec2(310., 450.))
            .label("Select Map")
            .titlebar(true)
            .ui(&mut *root_ui(), |ui| {
                for (i, m) in maps.iter().enumerate() {
                    let callback = Rc::clone(&callback);
                    let mut callback = callback.borrow_mut();

                    Group::new(hash!("map-select", i), Vec2::new(300., 50.)).ui(ui, |ui| {
                        ui.label(Vec2::new(10., 10.), &format!("Map: {}", m.get_name()));

                        if ui.button(Vec2::new(240., 10.), "Select") {
                            menu_state.map_selection = Some(i);
                            callback(i);
                        }
                    });
                }
                
                // Back button at the bottom of the maps list
                Group::new(hash!("map-back"), Vec2::new(300., 50.)).ui(ui, |ui| {
                    if ui.button(Vec2::new(120., 10.), "Back") {
                        menu_state.back();
                    }
                });
            });
    }
}

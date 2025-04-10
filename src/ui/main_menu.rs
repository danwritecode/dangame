use std::cell::RefCell;
use std::rc::Rc;

use macroquad::prelude::*;

use macroquad::ui::{
    hash, root_ui,
    widgets::{self, Group},
};

use crate::maps::map::GameMap;

pub async fn draw_menu(maps: &Vec<GameMap>, callback: Rc<RefCell<impl FnMut(usize)>>) {
    widgets::Window::new(hash!(), vec2(400., 200.), vec2(310., 400.))
        .label("Select Map")
        .titlebar(true)
        .ui(&mut *root_ui(), |ui| {
            for (i, m) in maps.iter().enumerate() {
                let callback = Rc::clone(&callback);
                let mut callback = callback.borrow_mut();

                Group::new(hash!("map-select", i), Vec2::new(300., 50.)).ui(ui, |ui| {
                    ui.label(Vec2::new(10., 10.), &format!("Map: {}", m.get_name()));

                    if ui.button(Vec2::new(240., 10.), "Select") {
                        callback(i);
                    }
                });
            }
        });
}

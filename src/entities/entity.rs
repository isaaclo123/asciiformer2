// use crate::linedraw::plot_line;
use super::Direction;
use crate::map::Map;
use crate::textures::Texture;
use crate::vectors::Vector;
use std::cell::RefCell;
use std::rc::Rc;
use termion::color;

pub trait Entity {
    fn get_texture(&self) -> Texture;
    fn get_point(&self) -> Vector<f32>;

    fn get_draw_point(&self) -> Vector<u16> {
        self.get_point().floor_int()
    }

    fn to_string(&self) -> &str {
        "Entity"
    }

    fn get_color(&self) -> Option<&dyn color::Color> {
        None
    }

    fn should_remove(&self) -> bool {
        false
    }

    fn update(&mut self) {}

    fn action(&mut self, direction: Direction) {}

    fn collide(&mut self, map: &Rc<RefCell<Map>>) {}
}

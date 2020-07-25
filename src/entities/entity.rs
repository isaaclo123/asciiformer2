// use crate::linedraw::plot_line;
use crate::debug;
use crate::map::Map;
use crate::textures::{AirTextures, Texture};
use crate::vectors::Vector;
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;
use termion::{color, cursor};

pub trait Entity<'a> {
    fn update(&mut self);
    // fn get_x(&mut self) -> u16;
    // fn get_y(&mut self) -> u16;
    // fn draw(&mut self, stdout: &'a mut impl Write);
    fn collide(&mut self, entity: &'a mut impl Entity<'a>);

    // fn should_draw(&self) -> bool;

    fn to_string(&self) -> &'a str;
    fn get_texture(&self) -> Texture;
    fn get_color(&self) -> Option<&'a dyn color::Color>;
    fn get_point(&self) -> Vector<u16>;

    fn clear(&self, stdout: &mut impl Write, origin: Vector<u16>, map: Rc<RefCell<Map>>) {
        // if !self.should_draw() {
        //     return;
        // }

        let Vector {
            x: point_x,
            y: point_y,
        } = self.get_point();
        let draw_pt = origin + self.get_point();
        let Vector {
            x: draw_x,
            y: draw_y,
        } = draw_pt;

        let texture = self.get_texture();

        for texture_y in 0..texture.len() {
            for texture_x in 0..texture[texture_y].len() {
                let (tex_x, tex_y) = (
                    point_x as i16 + texture_x as i16,
                    point_y as i16 + texture_y as i16,
                );
                let my_map = map.borrow();
                let bg_texture = my_map.get(tex_x, tex_y);

                let sym = if let Some(e) = bg_texture {
                    // e.get_texture()[0][0]
                    debug::write(&format!(
                        "TEXTURE ({}, {} '{}')",
                        tex_x,
                        tex_y,
                        e.get_texture()[0][0]
                    ));
                    e.get_texture()[0][0]
                } else {
                    AirTextures::AIR[0][0]
                };

                writeln!(
                    stdout,
                    "{goto}{sym}",
                    goto = cursor::Goto(
                        draw_x + 1 + texture_x as u16,
                        draw_y as u16 + 1 + texture_y as u16
                    ),
                    sym = sym
                )
                .unwrap();
            }
        }

        // TODO overflow issue

        write!(stdout, "{}", color::Fg(color::Reset)).unwrap();
    }

    fn draw(
        &self,
        stdout: &mut impl Write,
        origin: Vector<u16>,
        // point: Vector<i16>,
        // fg_opt: Option<impl color::Color>,
    ) {
        // if !self.should_draw() {
        //     return;
        // }

        // match self.get_texture() {
        //     PlayerTextures::NO_EXTEND => debug::write(stdout, "NO_EXTEND"),
        //     PlayerTextures::Y_EXTEND => debug::write(stdout, "Y_EXTEND"),
        //     PlayerTextures::X_EXTEND => debug::write(stdout, "X_EXTEND"),
        //     PlayerTextures::X_Y_EXTEND => debug::write(stdout, "X_Y_EXTEND"),
        //     _ => (),
        // };

        if let Some(c) = self.get_color() {
            write!(stdout, "{}", color::Fg(c)).unwrap();
        }

        let draw_pt = origin + self.get_point();
        let Vector {
            x: draw_x,
            y: draw_y,
        } = draw_pt;

        let texture = self.get_texture();

        // debug::write(
        //     stdout,
        //     &format!("texture_len x {} y {}", texture[0].len(), texture.len()),
        // );

        for y in 0..texture.len() {
            for x in 0..texture[y].len() {
                writeln!(
                    stdout,
                    "{goto}{sym}",
                    goto = cursor::Goto(draw_x + 1 + x as u16, draw_y as u16 + 1 + y as u16),
                    sym = texture[y][x]
                )
                .unwrap();
            }
        }

        // TODO overflow issue

        write!(stdout, "{}", color::Fg(color::Reset)).unwrap();
    }
}

use crate::consts::{EntityType, TEXTURE_MAP};
use crate::textures::{PlayerTextures, Texture};
use crate::vectors::Vector;
use std::io::Write;
use termion::{clear, color, cursor};

pub trait Entity<'a> {
    // fn update(&mut self);
    // fn get_x(&mut self) -> u16;
    // fn get_y(&mut self) -> u16;
    // fn draw(&mut self, stdout: &'a mut impl Write);
    fn collide(&mut self, entity: &'a mut impl Entity<'a>);

    fn to_string(&self) -> &'a str;
    fn get_texture(&self) -> Texture;
    fn get_color(&self) -> Option<&'a dyn color::Color>;
    fn get_point(&self) -> Vector<u16>;

    fn draw(
        &self,
        stdout: &mut impl Write,
        origin: Vector<u16>,
        // point: Vector<i16>,
        // fg_opt: Option<impl color::Color>,
    ) {
        match self.get_texture() {
            PlayerTextures::NO_EXTEND => {
                writeln!(stdout, "{}NO_EXTEND", cursor::Goto(1, 1)).unwrap()
            }
            PlayerTextures::Y_EXTEND => writeln!(stdout, "{}Y_EXTEND", cursor::Goto(1, 1)).unwrap(),
            PlayerTextures::X_EXTEND => writeln!(stdout, "{}X_EXTEND", cursor::Goto(1, 1)).unwrap(),
            PlayerTextures::X_Y_EXTEND => {
                writeln!(stdout, "{}X_Y_EXTEND", cursor::Goto(1, 1)).unwrap()
            }
            _ => (),
        };

        if let Some(c) = self.get_color() {
            write!(stdout, "{}", color::Fg(c)).unwrap();
        }

        let draw_pt = origin + self.get_point();
        let Vector {
            x: draw_x,
            y: draw_y,
        } = draw_pt;

        let texture = self.get_texture();

        writeln!(
            stdout,
            "{}xlen {} ylen {}",
            cursor::Goto(1, 2),
            texture[0].len(),
            texture.len()
        )
        .unwrap();

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

// TODO remove pub
pub struct Player<'a> {
    pub point: Vector<f32>,
    pub velocity: Vector<f32>,
    pub name: &'a str,
}

impl<'a> Entity<'a> for Player<'a> {
    fn get_texture(&self) -> Texture {
        let floor_pt = self.point.floor_int();
        let round_pt = self.point.round_int();

        let texture = match (floor_pt.x == round_pt.x, floor_pt.y == round_pt.y) {
            (true, true) => PlayerTextures::NO_EXTEND,
            (true, false) => PlayerTextures::Y_EXTEND,
            (false, true) => PlayerTextures::X_EXTEND,
            (false, false) => PlayerTextures::X_Y_EXTEND,
        };

        texture
    }

    fn get_color(&self) -> Option<&'a dyn color::Color> {
        Some(&color::Red)
    }

    fn get_point(&self) -> Vector<u16> {
        self.point.floor_int()
    }

    fn collide(&mut self, entity: &'a mut impl Entity<'a>) {}

    fn to_string(&self) -> &'a str {
        self.name
    }
    // fn update(&mut self) {}
}

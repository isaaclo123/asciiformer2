use super::debug;
use super::entities::{Entity, EntitySync};
use super::genindex::GenIndexSync;
use super::helpers::{unlock, wrap};
use super::map::MapSync;
use super::textures::AirTextures;
use super::vectors::Vector;
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;
use termion::{color, cursor};

use std::sync::{Arc, Mutex, MutexGuard};

pub fn mark_hitbox(
    // ref_obj: &EntitySync,
    obj: &dyn Entity,
    map: &MapSync,
) {
    if obj.get_id().is_none() {
        return;
    }

    let mut my_map = unlock(map);

    let pt = obj.get_draw_point();
    let Vector { x: pt_x, y: pt_y } = pt;

    let texture = obj.get_texture();

    for y in 0..texture.len() {
        for x in 0..texture[y].len() {
            my_map.set(
                (pt_x + x as u16) as i16,
                (pt_y + y as u16) as i16,
                obj.get_id().unwrap(),
            );
        }
    }
}

pub fn clear_hitbox(obj: &dyn Entity, map: &MapSync) {
    if obj.get_id().is_none() {
        return;
    }

    let mut my_map = unlock(map);

    let pt = obj.get_draw_point();
    let Vector { x: pt_x, y: pt_y } = pt;

    let texture = obj.get_texture();

    for y in 0..texture.len() {
        for x in 0..texture[y].len() {
            my_map.del((pt_x + x as u16) as i16, (pt_y + y as u16) as i16);
        }
    }
}

pub fn clear(
    obj: &dyn Entity,
    stdout: &mut impl Write,
    origin: Vector<u16>,
    map: &MapSync,
    gen_index: &GenIndexSync<EntitySync>,
) {
    // if !obj.should_draw() {
    //     return;
    // }
    // let obj = unlock(ref_obj);

    let Vector {
        x: point_x,
        y: point_y,
    } = obj.get_draw_point();
    let draw_pt = origin + obj.get_draw_point();
    let Vector {
        x: draw_x,
        y: draw_y,
    } = draw_pt;

    let texture = obj.get_texture();

    let my_map = unlock(map);

    for texture_y in 0..texture.len() {
        for texture_x in 0..texture[texture_y].len() {
            let (tex_x, tex_y) = (
                point_x as i16 + texture_x as i16,
                point_y as i16 + texture_y as i16,
            );
            let bg_texture = my_map.get(tex_x, tex_y);

            let sym = if let Some(e) = bg_texture {
                // e.get_texture()[0][0]
                // debug::write(&format!(
                //     "TEXTURE ({}, {} '{}')",
                //     tex_x,
                //     tex_y,
                //     e.borrow().get_texture()[0][0]
                //
                // ));
                let entity_locked = gen_index.get(*e as usize).unwrap();
                let entity = unlock(&entity_locked);
                entity.get_texture()[0][0]
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
                // sym = sym
                sym = sym
            )
            .unwrap();
        }
    }

    // TODO overflow issue

    write!(stdout, "{}", color::Fg(color::Reset)).unwrap();
}

pub fn draw(
    // ref_obj: &EntitySync,
    obj: &dyn Entity,
    stdout: &mut impl Write,
    origin: Vector<u16>,
) {
    // let obj = unlock(ref_obj);
    // if !obj.should_draw() {
    //     return;
    // }

    // match obj.get_texture() {
    //     PlayerTextures::NO_EXTEND => debug::write(stdout, "NO_EXTEND"),
    //     PlayerTextures::Y_EXTEND => debug::write(stdout, "Y_EXTEND"),
    //     PlayerTextures::X_EXTEND => debug::write(stdout, "X_EXTEND"),
    //     PlayerTextures::X_Y_EXTEND => debug::write(stdout, "X_Y_EXTEND"),
    //     _ => (),
    // };

    if let Some(c) = obj.get_color() {
        write!(stdout, "{}", color::Fg(c)).unwrap();
    }

    let draw_pt = origin + obj.get_draw_point();
    let Vector {
        x: draw_x,
        y: draw_y,
    } = draw_pt;

    let texture = obj.get_texture();

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

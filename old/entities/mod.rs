pub mod bullet;
pub mod directions;
pub mod entity;
pub mod linedraw;
pub mod player;
pub mod wall;

pub use self::bullet::Bullet;
pub use self::directions::Direction;
pub use self::entity::Entity;
pub use self::entity::EntitySync;
pub use self::linedraw::plot_line;
pub use self::player::Player;
pub use self::wall::Wall;

// #[derive(Clone, Copy)]
// pub enum EntityInstance<'a> {
//     Bullet(Bullet),
//     Player(Player<'a>),
//     Wall(Wall),
// }

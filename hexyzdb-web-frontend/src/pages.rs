//! Pages

mod enemies;
mod enemy;
mod home;
mod item;
mod items;
mod not_found;
mod weapon;
mod weapons;

pub use self::{
	enemies::enemies,
	enemy::enemy,
	home::home,
	item::item,
	items::items,
	not_found::not_found,
	weapon::weapon,
	weapons::weapons,
};

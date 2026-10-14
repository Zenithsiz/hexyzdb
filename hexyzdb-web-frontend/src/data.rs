//! Data

use {
	app_error::{AppError, Context},
	dhexyzf_build_data::{enemies::Enemies, items::Items, weapons::Weapons},
	dynatos_loadable::LoadableSignal,
	dynatos_reactive::{SignalBorrow, async_signal::Loader},
	dynatos_web::DynatosWebCtx,
	dynatos_web_router::LocationSignal,
	zutil_cloned::cloned,
};

#[derive(Debug)]
pub struct Data {
	pub weapons: LoadableSignal<WeaponsLoadable>,
	pub items:   LoadableSignal<ItemsLoadable>,
	pub enemies: LoadableSignal<EnemiesLoadable>,
}

impl Data {
	#[define_opaque(WeaponsLoadable, ItemsLoadable, EnemiesLoadable)]
	pub fn new(ctx: &DynatosWebCtx) -> Self {
		#[cloned(ctx)]
		let weapons = LoadableSignal::new(move || {
			let ctx = ctx.clone();
			async move { self::load_weapons(&ctx).await }
		});

		#[cloned(ctx)]
		let items = LoadableSignal::new(move || {
			let ctx = ctx.clone();
			async move { self::load_items(&ctx).await }
		});

		#[cloned(ctx)]
		let enemies = LoadableSignal::new(move || {
			let ctx = ctx.clone();
			async move { self::load_enemies(&ctx).await }
		});

		Self {
			weapons,
			items,
			enemies,
		}
	}
}

pub type WeaponsLoadable = impl Loader<Output = Result<Weapons, AppError>>;
pub type ItemsLoadable = impl Loader<Output = Result<Items, AppError>>;
pub type EnemiesLoadable = impl Loader<Output = Result<Enemies, AppError>>;

async fn load_weapons(ctx: &DynatosWebCtx) -> Result<Weapons, AppError> {
	let location = ctx.store().get::<LocationSignal>();
	let url = location
		.borrow_no_dep()
		.join("static/weapons.json")
		.context("Unable to build url")?;
	let weapons = reqwest::get(url)
		.await
		.context("Unable to get weapons")?
		.error_for_status()
		.context("Server returned an error")?
		.json()
		.await
		.context("Unable to parse weapons")?;

	Ok(weapons)
}

async fn load_items(ctx: &DynatosWebCtx) -> Result<Items, AppError> {
	let location = ctx.store().get::<LocationSignal>();
	let url = location
		.borrow_no_dep()
		.join("static/items.json")
		.context("Unable to build url")?;
	let items = reqwest::get(url)
		.await
		.context("Unable to get items")?
		.error_for_status()
		.context("Server returned an error")?
		.json()
		.await
		.context("Unable to parse items")?;

	Ok(items)
}

async fn load_enemies(ctx: &DynatosWebCtx) -> Result<Enemies, AppError> {
	let location = ctx.store().get::<LocationSignal>();
	let url = location
		.borrow_no_dep()
		.join("static/enemies.json")
		.context("Unable to build url")?;
	let enemies = reqwest::get(url)
		.await
		.context("Unable to get enemies")?
		.error_for_status()
		.context("Server returned an error")?
		.json()
		.await
		.context("Unable to parse enemies")?;

	Ok(enemies)
}

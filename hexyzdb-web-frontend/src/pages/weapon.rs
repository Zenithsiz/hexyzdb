//! Weapon page

use {
	crate::{components::loadable_wrapper::NodeWithDynLoadableChild, data::Data, util::NodeWithCssLink},
	app_error::AppError,
	dynatos_loadable::Loadable,
	dynatos_reactive::{SignalBorrow, SignalGetCloned},
	dynatos_web::{DynatosWebCtx, ElementWithClass, NodeWithChildren, NodeWithText, html, html_file},
	dynatos_web_router::{QuerySignal, SingleQuery},
	dynatos_web_title::ObjectWithTitle,
	zutil_cloned::cloned,
};

pub fn href(idx: usize) -> String {
	format!("/weapon?idx={idx}")
}

pub fn weapon(ctx: &DynatosWebCtx) -> web_sys::HtmlElement {
	let idx = QuerySignal::new(ctx, SingleQuery::<usize>::new(ctx, "idx"));

	let weapons = ctx.store().with_expect(|data: &Data| data.weapons.clone());

	#[cloned(idx, weapons)]
	let weapon = move || {
		let idx = idx.get_cloned()?;
		let weapons = weapons.get_cloned()?;
		let weapon = weapons.weapons.get(idx)?.clone();

		Loadable::<_, AppError>::Loaded(weapon)
	};

	#[cloned(ctx, idx)]
	let prev = move || {
		let idx = idx.get_cloned().loaded()?;
		let prev = idx.checked_sub(1)?;

		Some(dynatos_web_router::anchor(&ctx, self::href(prev)).with_text("Prev"))
	};

	#[cloned(ctx, idx)]
	let next = move || {
		let idx = idx.get_cloned().loaded()?;
		let weapons = weapons.borrow().loaded()?;
		let next = idx.checked_add(1)?;
		if next >= weapons.weapons.len() {
			return None;
		}

		Some(dynatos_web_router::anchor(&ctx, self::href(next)).with_text("Next"))
	};

	#[cloned(ctx)]
	let weapon = move || {
		let weapon = weapon()?;

		let prev = prev();
		let next = next();
		let skills = html::ol(&ctx).with_children(
			weapon
				.skills
				.iter()
				.map(|skill| html::li(&ctx).with_text(format!("{skill}")))
				.collect::<Vec<_>>(),
		);
		let weapon = html_file!("hexyzdb-web-frontend/html/pages/weapon/weapon.html");

		Loadable::Loaded(weapon)
	};

	let weapons_el = html::ul(ctx).with_dyn_loadable_children(ctx, weapon);

	html::div(ctx)
		.with_class("weapons-page")
		.with_css_link(ctx, "css/pages/weapons.css")
		.with_title(ctx, "Weapon | Hexyz Db")
		.with_child(weapons_el)
}

//! Enemy page

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
	format!("/enemy?idx={idx}")
}

pub fn enemy(ctx: &DynatosWebCtx) -> web_sys::HtmlElement {
	let idx = QuerySignal::new(ctx, SingleQuery::<usize>::new(ctx, "idx"));

	let enemies = ctx.store().with_expect(|data: &Data| data.enemies.clone());

	#[cloned(idx, enemies)]
	let enemy = move || {
		let idx = idx.get_cloned()?;
		let enemies = enemies.get_cloned()?;
		let enemy = enemies.enemies.get(idx)?.clone();

		Loadable::<_, AppError>::Loaded(enemy)
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
		let enemies = enemies.borrow().loaded()?;
		let next = idx.checked_add(1)?;
		if next >= enemies.enemies.len() {
			return None;
		}

		Some(dynatos_web_router::anchor(&ctx, self::href(next)).with_text("Next"))
	};

	#[cloned(ctx)]
	let enemy = move || {
		let enemy = enemy()?;

		let prev = prev();
		let next = next();
		let enemy = html_file!("hexyzdb-web-frontend/html/pages/enemy/enemy.html");

		Loadable::Loaded(enemy)
	};

	let enemies_el = html::ul(ctx).with_dyn_loadable_children(ctx, enemy);

	html::div(ctx)
		.with_class("enemies-page")
		.with_css_link(ctx, "css/pages/enemies.css")
		.with_title(ctx, "Enemy | Hexyz Db")
		.with_child(enemies_el)
}

//! Item page

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
	format!("/item?idx={idx}")
}

pub fn item(ctx: &DynatosWebCtx) -> web_sys::HtmlElement {
	let idx = QuerySignal::new(ctx, SingleQuery::<usize>::new(ctx, "idx"));

	let items = ctx.store().with_expect(|data: &Data| data.items.clone());

	#[cloned(idx, items)]
	let item = move || {
		let idx = idx.get_cloned()?;
		let items = items.get_cloned()?;
		let item = items.items.get(idx)?.clone();

		Loadable::<_, AppError>::Loaded(item)
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
		let items = items.borrow().loaded()?;
		let next = idx.checked_add(1)?;
		if next >= items.items.len() {
			return None;
		}

		Some(dynatos_web_router::anchor(&ctx, self::href(next)).with_text("Next"))
	};

	#[cloned(ctx)]
	let item = move || {
		let item = item()?;

		let prev = prev();
		let next = next();
		let item = html_file!("hexyzdb-web-frontend/html/pages/item/item.html");

		Loadable::Loaded(item)
	};

	let items_el = html::ul(ctx).with_dyn_loadable_children(ctx, item);

	html::div(ctx)
		.with_class("items-page")
		.with_css_link(ctx, "css/pages/items.css")
		.with_title(ctx, "Item | Hexyz Db")
		.with_child(items_el)
}

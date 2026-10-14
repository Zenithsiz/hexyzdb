//! Items page

use {
	super::item,
	crate::{components::loadable_wrapper::NodeWithDynLoadableChild, data::Data, util::NodeWithCssLink},
	dynatos_loadable::{IntoLoaded, Loadable},
	dynatos_reactive::SignalBorrow,
	dynatos_web::{DynatosWebCtx, ElementWithAttr, ElementWithClass, NodeWithChildren, NodeWithText, html},
	dynatos_web_title::ObjectWithTitle,
	zutil_cloned::cloned,
};

pub fn items(ctx: &DynatosWebCtx) -> web_sys::HtmlElement {
	let items = ctx.store().with_expect(|data: &Data| data.items.clone());

	#[cloned(ctx)]
	let items = move || {
		let items = items
			.borrow()?
			.items
			.iter()
			.enumerate()
			.map(|(idx, item)| {
				let link = dynatos_web_router::anchor(&ctx, item::href(idx)).with_text(item.name.as_str());
				html::li(&ctx)
					.with_child(
						html::img(&ctx).with_attr("src", format!("/static/images/icons/items/{}.png", item.icon)),
					)
					.with_child(link)
			})
			.collect::<Vec<_>>()
			.into_loaded();
		Loadable::Loaded(items)
	};

	let items_el = html::ul(ctx).with_dyn_loadable_children(ctx, items);

	html::div(ctx)
		.with_class("items-page")
		.with_css_link(ctx, "css/pages/items.css")
		.with_title(ctx, "Items | Hexyz Db")
		.with_child(items_el)
}

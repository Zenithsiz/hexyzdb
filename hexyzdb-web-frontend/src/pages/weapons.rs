//! Weapons page

use {
	super::weapon,
	crate::{components::loadable_wrapper::NodeWithDynLoadableChild, data::Data, util::NodeWithCssLink},
	dynatos_loadable::{IntoLoaded, Loadable},
	dynatos_reactive::SignalBorrow,
	dynatos_web::{DynatosWebCtx, ElementWithAttr, ElementWithClass, NodeWithChildren, NodeWithText, html},
	dynatos_web_title::ObjectWithTitle,
	zutil_cloned::cloned,
};

pub fn weapons(ctx: &DynatosWebCtx) -> web_sys::HtmlElement {
	let weapons = ctx.store().with_expect(|data: &Data| data.weapons.clone());

	#[cloned(ctx)]
	let weapons = move || {
		let weapons = weapons
			.borrow()?
			.weapons
			.iter()
			.enumerate()
			.map(|(idx, weapon)| {
				let link = dynatos_web_router::anchor(&ctx, weapon::href(idx)).with_text(weapon.name.as_str());
				html::li(&ctx)
					.with_child(html::img(&ctx).with_attr(
						"src",
						format!("/static/images/icons/items/{}.png", 75 + weapon.weapon_type),
					))
					.with_child(link)
			})
			.collect::<Vec<_>>()
			.into_loaded();
		Loadable::Loaded(weapons)
	};

	let weapons_el = html::ul(ctx).with_dyn_loadable_children(ctx, weapons);

	html::div(ctx)
		.with_class("weapons-page")
		.with_css_link(ctx, "css/pages/weapons.css")
		.with_title(ctx, "Weapons | Hexyz Db")
		.with_child(weapons_el)
}

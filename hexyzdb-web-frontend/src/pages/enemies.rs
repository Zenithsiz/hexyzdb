//! Enemies page

use {
	super::{enemy, item},
	crate::{components::loadable_wrapper::NodeWithDynLoadableChild, data::Data, util::NodeWithCssLink},
	dynatos_loadable::{IntoLoaded, Loadable},
	dynatos_reactive::SignalBorrow,
	dynatos_web::{DynatosWebCtx, ElementWithAttr, ElementWithClass, NodeWithChildren, NodeWithText, html},
	dynatos_web_title::ObjectWithTitle,
	zutil_cloned::cloned,
};

#[expect(clippy::too_many_lines, reason = "TODO: Split it up")]
pub fn enemies(ctx: &DynatosWebCtx) -> web_sys::HtmlElement {
	let enemies = ctx.store().with_expect(|data: &Data| data.enemies.clone());
	let items = ctx.store().with_expect(|data: &Data| data.items.clone());

	#[cloned(ctx)]
	let enemies = move || {
		let items = items.borrow()?;

		let enemies = enemies
			.borrow()?
			.enemies
			.iter()
			.enumerate()
			.map(|(idx, enemy)| {
				let icon = dynatos_web_router::anchor(&ctx, enemy::href(idx))
					.with_child(html::img(&ctx).with_attr("src", format!("/static/images/icons/enemy/{idx}.png")));
				let link = dynatos_web_router::anchor(&ctx, enemy::href(idx)).with_text(enemy.name.as_str());

				let stats = [
					("Hp", enemy.hp),
					("Atk", enemy.atk),
					("Strength", enemy.strength),
					("Defense", enemy.defense),
					("Force", enemy.force),
					("Spirit", enemy.spirit),
					("Dexterity", enemy.dexterity),
					("Agility", enemy.agility),
					("EXP", enemy.exp),
					("FP", enemy.fp),
				];

				let stats = stats
					.into_iter()
					.map(|(name, value)| {
						html::div(&ctx)
							.with_class("stat")
							.with_child(html::p(&ctx).with_class("name").with_text(name))
							.with_child(html::p(&ctx).with_class("value").with_text(value.to_string()))
					})
					.collect::<Vec<_>>();

				let res = [
					("Fire", "fire", enemy.res_fire),
					("Water", "water", enemy.res_water),
					("Wind", "wind", enemy.res_wind),
					("Earth", "earth", enemy.res_earth),
					("Bolt", "bolt", enemy.res_bolt),
					("Light", "light", enemy.res_light),
					("Dark", "dark", enemy.res_dark),
				];

				let element_icon_name = |element, name| {
					html::div(&ctx)
						.with_class("icon-name")
						.with_child(
							html::img(&ctx)
								.with_class("icon")
								.with_attr("src", format!("/static/images/icons/elements/{element}.png")),
						)
						.with_child(html::p(&ctx).with_class("name").with_text(name))
				};

				let res = res
					.into_iter()
					.map(|(name, element, value)| {
						html::div(&ctx)
							.with_class("res")
							.with_child(element_icon_name(element, name))
							.with_child(html::p(&ctx).with_class("value").with_text(match value == 0 {
								true => "---".to_owned(),
								false => value.to_string(),
							}))
					})
					.collect::<Vec<_>>();

				let regular_drops = [
					("Fire", "fire", enemy.drop_fire),
					("Water", "water", enemy.drop_water),
					("Wind", "wind", enemy.drop_wind),
					("Earth", "earth", enemy.drop_earth),
					("Bolt", "bolt", enemy.drop_bolt),
					("Light", "light", enemy.drop_light),
					("Dark", "dark", enemy.drop_dark),
					("Void", "void", enemy.drop_void),
				];

				let element_drop = |name, element, item, quantity_or_chance| {
					html::div(&ctx)
						.with_class("drop")
						.with_child(element_icon_name(element, name))
						.with_child(
							html::div(&ctx)
								.with_class("item")
								.with_child(
									html::div(&ctx)
										.with_class("name-icon")
										.with_child(match usize::try_from(item) {
											Ok(item_idx) => dynatos_web_router::anchor(&ctx, item::href(item_idx))
												.with_class("name")
												.with_text(items.items[item_idx].name.as_str()),
											Err(_) => html::p(&ctx).with_class("name").with_text("---"),
										})
										.with_children(
											usize::try_from(item)
												.ok()
												.map(|item_idx| {
													let icon_idx = items.items[item_idx].icon;
													html::img(&ctx).with_class("icon").with_attr(
														"src",
														format!("/static/images/icons/items/{icon_idx}.png"),
													)
												})
												.as_slice(),
										),
								)
								.with_child(html::p(&ctx).with_class("quantity").with_text(match item != -1 {
									true => quantity_or_chance,
									false => String::new(),
								})),
						)
				};

				let mut drops = regular_drops
					.into_iter()
					.map(|(name, element, drop)| {
						element_drop(name, element, drop.item, match (drop.min, drop.max) {
							(0, 0) => "0".to_owned(),
							(0, _) => format!("≤{}", drop.max),
							(min, max) if min == max => format!("{}", drop.min),

							_ => format!("≥{} ≤{}", drop.min, drop.max),
						})
					})
					.collect::<Vec<_>>();

				drops.push(element_drop(
					"Rare",
					"rare",
					enemy.drop_rare.item,
					format!("{}%", enemy.drop_rare.chance),
				));

				html::tr(&ctx)
					.with_child(html::td(&ctx).with_class("icon").with_child(icon))
					.with_child(html::td(&ctx).with_class("name").with_child(link))
					.with_child(
						html::td(&ctx)
							.with_class("stats")
							.with_children(html::div(&ctx).with_children(stats)),
					)
					.with_child(
						html::td(&ctx)
							.with_class("res")
							.with_children(html::div(&ctx).with_children(res)),
					)
					.with_child(
						html::td(&ctx)
							.with_class("drops")
							.with_children(html::div(&ctx).with_children(drops)),
					)
					.with_child(
						html::td(&ctx)
							.with_class("description")
							.with_text(enemy.description.as_str()),
					)
			})
			.collect::<Vec<_>>()
			.into_loaded();
		Loadable::Loaded(enemies)
	};

	let enemies_el = html::table(ctx)
		.with_child(html::thead(ctx).with_child(html::tr(ctx).with_children([
			html::th(ctx).with_class("icon").with_text("Icon"),
			html::th(ctx).with_class("name").with_text("Name"),
			html::th(ctx).with_class("stats").with_text("Stats"),
			html::th(ctx).with_class("res").with_text("Res"),
			html::th(ctx).with_class("drops").with_text("Drops"),
			html::th(ctx).with_class("description").with_text("Description"),
		])))
		.with_child(html::tbody(ctx).with_dyn_loadable_children(ctx, enemies));

	html::div(ctx)
		.with_class("enemies-page")
		.with_css_link(ctx, "css/pages/enemies.css")
		.with_title(ctx, "Enemies | Hexyz Db")
		.with_child(enemies_el)
}

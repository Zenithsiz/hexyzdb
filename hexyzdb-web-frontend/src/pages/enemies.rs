//! Enemies page

use {
	super::{enemy, item},
	crate::{
		components::loadable_wrapper::NodeWithDynLoadableChild,
		data::{Data, ItemsLoader},
		util::NodeWithCssLink,
	},
	dhexyzf_build_data::{enemies::Enemy, items::Items},
	dynatos_loadable::{IntoLoaded, Loadable, LoadableSignal},
	dynatos_reactive::SignalBorrow,
	dynatos_web::{DynatosWebCtx, ElementWithAttr, ElementWithClass, NodeWithChildren, NodeWithText, html},
	dynatos_web_title::ObjectWithTitle,
	zutil_cloned::cloned,
};

pub fn enemies(ctx: &DynatosWebCtx) -> web_sys::HtmlElement {
	let (enemies, items) = ctx
		.store()
		.with_expect(|data: &Data| (data.enemies.clone(), data.items.clone()));

	#[cloned(ctx)]
	let enemies = move || {
		let enemies = enemies
			.borrow()?
			.enemies
			.iter()
			.enumerate()
			.map(|(idx, enemy)| {
				let icon = dynatos_web_router::anchor(&ctx, enemy::href(idx))
					.with_child(html::img(&ctx).with_attr("src", format!("/static/images/icons/enemy/{idx}.png")));
				let link = dynatos_web_router::anchor(&ctx, enemy::href(idx)).with_text(enemy.name.as_str());

				let column = |class, el| html::td(&ctx).with_class(class).with_child(el);
				html::tr(&ctx)
					.with_child(column("icon", icon))
					.with_child(column("name", link))
					.with_child(column("stats", self::stats(&ctx, enemy)))
					.with_child(column("res", self::res(&ctx, enemy)))
					.with_child(column("drops", self::drops(&ctx, &items, enemy)))
					.with_child(column(
						"description",
						html::p(&ctx).with_text(enemy.description.as_str()),
					))
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

fn stats(ctx: &DynatosWebCtx, enemy: &Enemy) -> web_sys::HtmlElement {
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
			html::div(ctx)
				.with_class("stat")
				.with_child(html::p(ctx).with_class("name").with_text(name))
				.with_child(html::p(ctx).with_class("value").with_text(value.to_string()))
		})
		.collect::<Vec<_>>();

	html::div(ctx).with_children(stats)
}

fn res(ctx: &DynatosWebCtx, enemy: &Enemy) -> web_sys::HtmlElement {
	let res = [
		("Fire", "fire", enemy.res_fire),
		("Water", "water", enemy.res_water),
		("Wind", "wind", enemy.res_wind),
		("Earth", "earth", enemy.res_earth),
		("Bolt", "bolt", enemy.res_bolt),
		("Light", "light", enemy.res_light),
		("Dark", "dark", enemy.res_dark),
	];

	let res = res
		.into_iter()
		.map(|(name, element, value)| {
			html::div(ctx)
				.with_class("res")
				.with_child(self::element_icon_name(ctx, element, name))
				.with_child(html::p(ctx).with_class("value").with_text(match value == 0 {
					true => "---".to_owned(),
					false => value.to_string(),
				}))
		})
		.collect::<Vec<_>>();


	html::div(ctx).with_children(res)
}

fn drops(ctx: &DynatosWebCtx, items: &LoadableSignal<ItemsLoader>, enemy: &Enemy) -> web_sys::HtmlElement {
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

	let mut drops = regular_drops
		.into_iter()
		.map(|(name, element, drop)| {
			self::drop_element(ctx, name, element, items, drop.item, || match (drop.min, drop.max) {
				(0, 0) => "0".to_owned(),
				(0, _) => format!("≤{}", drop.max),
				(min, max) if min == max => format!("{}", drop.min),

				_ => format!("≥{} ≤{}", drop.min, drop.max),
			})
		})
		.collect::<Vec<_>>();

	drops.push(self::drop_element(
		ctx,
		"Rare",
		"rare",
		items,
		enemy.drop_rare.item,
		|| format!("{}%", enemy.drop_rare.chance),
	));

	html::div(ctx).with_children(drops)
}

fn drop_element(
	ctx: &DynatosWebCtx,
	name: &str,
	element: &str,
	items: &LoadableSignal<ItemsLoader>,
	item: i16,
	value: impl FnOnce() -> String,
) -> web_sys::HtmlElement {
	let item_quantity = html::div(ctx)
		.with_class("item")
		.with_dyn_loadable_child(
			ctx,
			#[cloned(ctx, items)]
			move || try {
				let items = items.borrow().result()?;
				self::drop_name_icon(&ctx, items.as_deref(), item)
			},
		)
		.with_child(html::p(ctx).with_class("value").with_text(match item != -1 {
			true => value(),
			false => String::new(),
		}));

	html::div(ctx)
		.with_class("drop")
		.with_child(self::element_icon_name(ctx, element, name))
		.with_child(item_quantity)
}

fn drop_name_icon(ctx: &DynatosWebCtx, items: Option<&Items>, item: i16) -> web_sys::HtmlElement {
	let name;
	let icon;

	match usize::try_from(item) {
		Ok(item_idx) => match items {
			Some(items) => {
				name = dynatos_web_router::anchor(ctx, item::href(item_idx))
					.with_class("name")
					.with_text(items.items[item_idx].name.as_str());

				let icon_idx = items.items[item_idx].icon;
				icon = html::img(ctx)
					.with_class("icon")
					.with_attr("src", format!("/static/images/icons/items/{icon_idx}.png"))
					.into();
			},
			None => {
				name = html::p(ctx).with_class("name").with_text(format!("#{item}"));
				icon = html::template(ctx);
			},
		},
		Err(_) => {
			name = html::p(ctx).with_class("name").with_text("---");
			icon = html::template(ctx);
		},
	}

	html::div(ctx).with_class("name-icon").with_child(name).with_child(icon)
}

fn element_icon_name(ctx: &DynatosWebCtx, element: &str, name: &str) -> web_sys::HtmlElement {
	html::div(ctx)
		.with_class("icon-name")
		.with_child(
			html::img(ctx)
				.with_class("icon")
				.with_attr("src", format!("/static/images/icons/elements/{element}.png")),
		)
		.with_child(html::p(ctx).with_class("name").with_text(name))
}

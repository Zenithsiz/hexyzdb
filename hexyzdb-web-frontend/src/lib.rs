//! hexyzdb web

#![feature(stmt_expr_attributes, proc_macro_hygiene, type_alias_impl_trait)]

mod components;
mod data;
mod pages;
mod util;

use {
	self::{data::Data, util::NodeWithCssLink},
	dynatos_reactive::SignalGetCloned,
	dynatos_web::{DynatosWebCtx, ElementWithClass, NodeAddChildren, NodeWithChildren, NodeWithText, html},
	dynatos_web_reactive::NodeWithDynChild,
	dynatos_web_router::LocationSignal,
	dynatos_web_title::ObjectWithTitle,
	std::rc::Rc,
	url::Url,
	zutil_cloned::cloned,
};

#[derive(Clone)]
#[derive(derive_more::Deref)]
#[deref(forward)]
struct BackendUrl(Rc<Url>);

pub fn attach(ctx: &DynatosWebCtx) {
	let location = LocationSignal::new(ctx);

	// Build the backend url
	// TODO: Should this be reactive?
	let backend_url = location
		.get_cloned_no_dep()
		.join("backend/")
		.expect("Backend url was invalid");
	let backend_url = BackendUrl(backend_url.into());
	ctx.store().set(location);
	ctx.store().set(backend_url);

	let data = Data::new(ctx);
	ctx.store().set(data);

	ctx.head().add_children([
		html!(r#"<link href="/css/default.css" rel="stylesheet" />"#),
		html!(r#"<meta charset="UTF-8" />"#),
		html!(r#"<meta name="viewport" content="width=device-width, initial-scale=1.0" />"#),
		html!(r#"<title>Hexyz Db</title>"#),
	]);

	// And attach our app to the body
	let content = html::div(ctx).with_class("content").with_dyn_child(
		ctx,
		#[cloned(ctx)]
		move || self::render_route(&ctx),
	);

	let sidebar_links = [
		("Home", "/"),
		("Weapons", "/weapons"),
		("Items", "/items"),
		("Enemies", "/enemies"),
	];
	let sidebar_links = sidebar_links
		.map(|(name, href)| html::li(ctx).with_child(dynatos_web_router::anchor(ctx, href).with_text(name)));

	let app = dynatos_web::html_file!("hexyzdb-web-frontend/html/app.html")
		.with_css_link(ctx, "css/app.css")
		.with_title(ctx, "Hexyz Db");

	ctx.body().with_child(app);
}


fn render_route(ctx: &DynatosWebCtx) -> web_sys::HtmlElement {
	let location = ctx.store().get::<LocationSignal>().get_cloned();

	tracing::debug!(%location, "Rendering route");
	match location.path().trim_end_matches('/') {
		"" => pages::home(ctx),
		"/weapons" => pages::weapons(ctx),
		"/weapon" => pages::weapon(ctx),
		"/items" => pages::items(ctx),
		"/item" => pages::item(ctx),
		"/enemies" => pages::enemies(ctx),
		"/enemy" => pages::enemy(ctx),
		page => pages::not_found(ctx, page),
	}
}

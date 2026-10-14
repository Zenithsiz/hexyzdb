//! `/home` page

use {crate::util::NodeWithCssLink, dynatos_web::DynatosWebCtx, dynatos_web_title::ObjectWithTitle};

pub fn home(ctx: &DynatosWebCtx) -> web_sys::HtmlElement {
	dynatos_web::html_file!("hexyzdb-web-frontend/html/pages/home.html")
		.with_css_link(ctx, "css/pages/home.css")
		.with_title(ctx, "Home | Hexyz Db")
}

//! Not found page

use {
	dynatos_web::{DynatosWebCtx, NodeWithText, html},
	dynatos_web_title::ObjectWithTitle,
};

pub fn not_found(ctx: &DynatosWebCtx, page: &str) -> web_sys::HtmlElement {
	html::p(ctx)
		.with_title(ctx, "Not found | Hexyz Db")
		.with_text(format!("Unknown page: {page:?}"))
}

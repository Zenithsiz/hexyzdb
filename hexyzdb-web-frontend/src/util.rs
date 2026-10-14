//! Utilities

use dynatos_web::{DynatosWebCtx, ElementWithAttr, NodeAddChildren, html};


/// Returns a `<link href="<href>" rel="stylesheet"/>`
#[must_use]
pub fn css_link(ctx: &DynatosWebCtx, href: &str) -> web_sys::HtmlElement {
	html::link(ctx).with_attr("href", href).with_attr("rel", "stylesheet")
}

#[extend::ext(name = NodeAddCssLink)]
pub impl web_sys::Node {
	fn add_css_link(&self, ctx: &DynatosWebCtx, href: &str) {
		self.add_child(self::css_link(ctx, href));
	}
}

#[extend::ext(name = NodeWithCssLink)]
pub impl<N: AsRef<web_sys::Node>> N {
	fn with_css_link(self, ctx: &DynatosWebCtx, href: &str) -> Self {
		self.as_ref().add_css_link(ctx, href);
		self
	}
}

//! Wrapper over a `Loadable`.
// TODO: Better name?

// Imports
use {
	app_error::AppError,
	dynatos_loadable::Loadable,
	dynatos_web::{DynatosWebCtx, ElementWithClass, html},
	dynatos_web_reactive::{NodeDynChild, NodeDynChildren, WithDynNodes},
	zutil_cloned::cloned,
};

/// Loadable wrapper
pub fn loadable_wrapper<L>(ctx: &DynatosWebCtx, loadable: &L) -> web_sys::HtmlElement
where
	L: ToLoadableChild,
{
	loadable.with_loadable(|loadable| match loadable {
		Loadable::Empty => html::template(ctx).with_class("loadable-loading"),
		Loadable::Err(err) => {
			tracing::error!("{err:?}");
			html::template(ctx).with_class("loadable-error")
		},
		Loadable::Loaded(view) => view,
	})
}

/// Extension trait to add a reactive `LoadableWrapper` child to a node.
#[extend::ext(name = NodeDynLoadableChild)]
pub impl web_sys::Node {
	/// Adds a loadable child to this element
	#[track_caller]
	fn add_dyn_loadable_child<F>(&self, ctx: &DynatosWebCtx, loadable: F)
	where
		F: ToLoadableChild + 'static,
	{
		self.add_dyn_child(
			ctx,
			#[cloned(ctx)]
			move || self::loadable_wrapper(&ctx, &loadable),
		);
	}

	/// Adds loadable children to this element
	#[track_caller]
	fn add_dyn_loadable_children<F>(&self, ctx: &DynatosWebCtx, loadables: F)
	where
		F: ToLoadableChildren + 'static,
	{
		struct Adapter<F> {
			ctx:       DynatosWebCtx,
			loadables: F,
		}

		impl<F: ToLoadableChildren> WithDynNodes for Adapter<F> {
			fn with_nodes(&self, mut f: impl FnMut(web_sys::Node)) {
				self.loadables
					.with_loadables(|loadable| f(self::loadable_wrapper(&self.ctx, &loadable).into()));
			}
		}

		self.add_dyn_children(ctx, Adapter {
			ctx: ctx.clone(),
			loadables,
		});
	}
}

/// Extension trait to add a reactive `LoadableWrapper` child to a node.
#[extend::ext(name = NodeWithDynLoadableChild)]
pub impl<E> E
where
	E: AsRef<web_sys::Node>,
{
	/// Adds a loadable child to this element.
	///
	/// Returns the element, for chaining
	#[track_caller]
	fn with_dyn_loadable_child<F>(self, ctx: &DynatosWebCtx, loadable: F) -> Self
	where
		F: ToLoadableChild + 'static,
	{
		self.as_ref().add_dyn_loadable_child(ctx, loadable);
		self
	}

	/// Adds loadable children to this element.
	///
	/// Returns the element, for chaining
	#[track_caller]
	fn with_dyn_loadable_children<F>(self, ctx: &DynatosWebCtx, loadables: F) -> Self
	where
		F: ToLoadableChildren + 'static,
	{
		self.as_ref().add_dyn_loadable_children(ctx, loadables);
		self
	}
}

/// Trait for values accepted by [`LoadableWrapper`] and [`NodeWithDynLoadableChild`].
///
/// This allows it to work with the following types:
/// - `impl Fn() -> N`
/// - `N`
/// - `Loadable<N>`
///
/// Where `N` is an html element type.
pub trait ToLoadableChild {
	/// Uses the inner loadable type
	fn with_loadable<O>(&self, f: impl FnOnce(Loadable<web_sys::HtmlElement, AppError>) -> O) -> O;
}

impl<F, N> ToLoadableChild for F
where
	F: Fn() -> N,
	N: ToLoadableChild,
{
	fn with_loadable<O>(&self, f: impl FnOnce(Loadable<web_sys::HtmlElement, AppError>) -> O) -> O {
		self().with_loadable(f)
	}
}

// TODO: Impl for `impl AsRef<web_sys::HtmlElement>` if we can get rid of
//       the conflict with the function impl
impl ToLoadableChild for web_sys::HtmlElement {
	fn with_loadable<O>(&self, f: impl FnOnce(Loadable<Self, AppError>) -> O) -> O {
		f(Loadable::Loaded(self.clone()))
	}
}

impl<N> ToLoadableChild for Loadable<N, AppError>
where
	N: ToLoadableChild,
{
	fn with_loadable<O>(&self, f: impl FnOnce(Loadable<web_sys::HtmlElement, AppError>) -> O) -> O {
		match self.as_ref() {
			Loadable::Empty => f(Loadable::Empty),
			Loadable::Err(err) => f(Loadable::Err(err)),
			Loadable::Loaded(value) => value.with_loadable(f),
		}
	}
}

/// Types that may be used for [`NodeWithDynLoadableChild`]
pub trait ToLoadableChildren {
	/// Uses the inner loadable types
	fn with_loadables(&self, f: impl FnMut(Loadable<web_sys::HtmlElement, AppError>));
}

impl<F, N> ToLoadableChildren for F
where
	F: Fn() -> N,
	N: ToLoadableChildren,
{
	fn with_loadables(&self, f: impl FnMut(Loadable<web_sys::HtmlElement, AppError>)) {
		self().with_loadables(f);
	}
}

// TODO: Impl for `impl AsRef<web_sys::HtmlElement>` if we can get rid of
//       the conflict with the function impl
impl ToLoadableChildren for web_sys::HtmlElement {
	fn with_loadables(&self, mut f: impl FnMut(Loadable<Self, AppError>)) {
		f(Loadable::Loaded(self.clone()));
	}
}

impl<N> ToLoadableChildren for Loadable<N, AppError>
where
	N: ToLoadableChildren,
{
	fn with_loadables(&self, mut f: impl FnMut(Loadable<web_sys::HtmlElement, AppError>)) {
		match self.as_ref() {
			Loadable::Empty => f(Loadable::Empty),
			Loadable::Err(err) => f(Loadable::Err(err)),
			Loadable::Loaded(value) => value.with_loadables(f),
		}
	}
}

impl ToLoadableChildren for () {
	fn with_loadables(&self, _f: impl FnMut(Loadable<web_sys::HtmlElement, AppError>)) {}
}

impl<C> ToLoadableChildren for [C]
where
	C: ToLoadableChildren + 'static,
{
	fn with_loadables(&self, mut f: impl FnMut(Loadable<web_sys::HtmlElement, AppError>)) {
		for child in self {
			child.with_loadables(&mut f);
		}
	}
}

impl<C, const N: usize> ToLoadableChildren for [C; N]
where
	C: ToLoadableChildren + 'static,
{
	fn with_loadables(&self, f: impl FnMut(Loadable<web_sys::HtmlElement, AppError>)) {
		self.as_slice().with_loadables(f);
	}
}

impl<C> ToLoadableChildren for Vec<C>
where
	C: ToLoadableChildren + 'static,
{
	fn with_loadables(&self, f: impl FnMut(Loadable<web_sys::HtmlElement, AppError>)) {
		self.as_slice().with_loadables(f);
	}
}

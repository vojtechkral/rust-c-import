use syntax::ext::base::ExtCtxt;
use syntax::codemap::Span;


/// Compile error
pub enum CError
{
	NotAModule,
	InvalidArgs,
	ErrOther(String),
}

impl CError
{
	pub fn spit(&self, cx: &mut ExtCtxt, sp: Span)
	{
		use self::CError::*;
		match *self
		{
			NotAModule => cx.span_err(sp, "`c_import` can only be declared on a module"),
			InvalidArgs => cx.span_err(sp, "`c_import`: Invalid arguments"),
			ErrOther(ref s) => cx.span_err(sp, &format!("`c_import`: Error: {}", s)),
		};
	}

	pub fn spit_ret<T>(&self, cx: &mut ExtCtxt, sp: Span, ret: T) -> T
	{
		self.spit(cx, sp);
		ret
	}
}

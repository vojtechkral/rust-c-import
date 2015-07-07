use syntax::codemap::{Span, Spanned};
use syntax::ast;
use syntax::ast::{Name, Ident, Expr, Item, Visibility};
use syntax::ext::base::ExtCtxt;
use syntax::ext::build::AstBuilder;
use syntax::ptr::P;

use rustc_serialize::json::{Json, Decoder};
use rustc_serialize::Decodable;


pub fn hack_vis(item: &mut P<Item>, vis: Visibility)
{
	let item_mut = unsafe
	{
		&mut *::std::mem::transmute::<_, *mut Item>(&**item)
	};

	item_mut.vis = vis;
}


pub trait Def
{
	fn make_item(&self, cx: &mut ExtCtxt, sp: Span, name: Name) -> P<Item>;
}

pub type Defs = Vec<Box<Def>>;

pub fn def_from_json(json: &Json) -> Option<Box<Def>>
{
	if !json.is_object() { return None; }

	if let Some(cnst) = json.find("const")
	{
		if let Some(const_int) = cnst.find("int")
		{
			let mut decoder = Decoder::new(const_int.clone());
			return Some(Box::new(DefConstInt::decode(&mut decoder).unwrap()));
		}
	}

	None
}

#[derive(RustcDecodable)]
struct DefConstInt
{
	bits: i32,
	value: i64,
}

impl Def for DefConstInt
{
	fn make_item(&self, cx: &mut ExtCtxt, sp: Span, name: Name) -> P<Item>
	{
		use syntax::ast::Expr_::ExprLit;
		use syntax::ast::Lit_::LitInt;
		use syntax::ast::LitIntType::UnsuffixedIntLit;
		use syntax::ast::Sign::*;

		let sign = if self.value >= 0 { Plus } else { Minus };

		let ty = match self.bits
		{
			8 => quote_ty!(cx, i8),
			16 => quote_ty!(cx, i16),
			32 => quote_ty!(cx, i32),
			_ => quote_ty!(cx, i64),
		};

		let value = P(Expr{
			id: ast::DUMMY_NODE_ID,
			node: ExprLit(P(Spanned{
				node: LitInt(self.value as u64, UnsuffixedIntLit(sign)),
				span: sp,
			})),
			span: sp,
		});

		let mut item = cx.item_const(sp, Ident::new(name), ty, value);
		hack_vis(&mut item, Visibility::Public);
		item
	}
}

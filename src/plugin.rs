use syntax::codemap::Span;
use syntax::parse::token::intern;
use syntax::ext::base::ExtCtxt;
use syntax::ext::base::{MultiItemModifier, Annotatable};
use syntax::ast::{Item, MetaItem, Mod};
use syntax::ptr::P;

use error::CError;
use error::CError::*;
use backend::*;

use def::*;


pub struct CImportModifier;

impl CImportModifier
{
	pub fn new() -> CImportModifier { CImportModifier }

	fn get_args<'a>(&'a self, cx: &mut ExtCtxt, mi: &'a MetaItem) -> Option<(&str, Span, &Vec<P<MetaItem>>)>
	{
		use syntax::ast::MetaItem_::*;
		use syntax::ast::Lit_::*;

		match mi.node
		{
			MetaList(ref _name, ref list) =>
			{
				match list.get(0)
				{
					Some(hi) =>
					{
						match hi.node
						{
							MetaNameValue(ref name, ref value) if *name == "h" =>
							{
								match value.node
								{
									LitStr(ref s, ref _style) =>
									{
										Some((&**s, hi.span, list))
									},
									_ => InvalidArgs.spit_ret(cx, value.span, None),
								}
							},
							_ => CError::InvalidArgs.spit_ret(cx, hi.span, None),
						}
					},
					None => CError::InvalidArgs.spit_ret(cx, mi.span, None),
				}
			},
			_ => CError::InvalidArgs.spit_ret(cx, mi.span, None),
		}
	}

	fn get_module<'a>(&'a self, cx: &mut ExtCtxt, sp: Span, item: &'a Annotatable) -> Option<(&Item, Mod)>
	{
		use syntax::ast::Item_::*;
		use syntax::ext::base::Annotatable::*;

		match *item
		{
			Item(ref i) => match i.node
			{
				ItemMod(ref m) =>
				{
					Some((i, m.clone()))
				},
				_ => NotAModule.spit_ret(cx, sp, None),
			},
			_ => NotAModule.spit_ret(cx, sp, None),
		}
	}

	fn load_requests(&self, cx: &mut ExtCtxt, sp: Span, args: &Vec<P<MetaItem>>, rqs: &mut Requests) -> bool
	{
		use syntax::ast::MetaItem_::*;

		if args.len() < 2
		{
			return InvalidArgs.spit_ret(cx, sp, false)
		}

		for item in args.iter().skip(1)
		{
			if let MetaWord(ref word) = item.node
			{
				rqs.push(intern(word));
			}
			else
			{
				return InvalidArgs.spit_ret(cx, item.span, false);
			}
		}

		true
	}

	fn do_import(&self, cx: &mut ExtCtxt, sp: Span, header: &str, rqs: &Requests, defs: &mut Defs) -> bool
	{
		let backend = Backend::new(sp, header, rqs);
		backend.import(cx, defs);
		true
	}

	fn push_ast(&self, cx: &mut ExtCtxt, sp: Span, items: &mut Vec<P<Item>>, rqs: &Requests, defs: &Defs) -> bool
	{
		for (rq, def) in rqs.iter().zip(defs)   // Here it's relied on the order of rqs->defs being preserved
		{
			items.push(def.make_item(cx, sp, *rq));
		}
		true
	}
}

impl MultiItemModifier for CImportModifier
{
	fn expand(&self, cx: &mut ExtCtxt, sp: Span, mi: &MetaItem, item: Annotatable) -> Annotatable
	{
		use syntax::ast::Item_::ItemMod;

		if let Some((item, mut module)) = self.get_module(cx, sp, &item)
		{
			if let Some((header, h_span, args)) = self.get_args(cx, mi)
			{
				let mut rqs = Requests::new();
				let mut defs = Defs::new();

				if self.load_requests(cx, mi.span, args, &mut rqs)
					&& self.do_import(cx, h_span, header, &rqs, &mut defs)
					&& self.push_ast(cx, module.inner, &mut module.items, &rqs, &defs)
				{
					return Annotatable::Item(P(Item{
						ident: item.ident,
						attrs: item.attrs.clone(),
						id: item.id,
						node: ItemMod(module),
						vis: item.vis,
						span: item.span,
					}))
				}
			}
		}

		item.clone()  // Something went awry, lad, but not to worry, errors were spat.
	}
}

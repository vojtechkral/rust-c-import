#![crate_name = "c_import"]
#![crate_type = "dylib"]
#![feature(plugin_registrar, rustc_private, quote, convert, std_misc)]

extern crate syntax;
extern crate rustc;
extern crate rustc_serialize;

use rustc::plugin::Registry;
use syntax::ext::base::SyntaxExtension::MultiModifier;
use syntax::parse::token::intern;

mod plugin;
mod error;
mod backend;
mod def;

use plugin::CImportModifier;


#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry)
{
	reg.register_syntax_extension(intern("c_import"), MultiModifier(Box::new(CImportModifier::new())));
}

#![feature(plugin, custom_attribute)]
#![plugin(c_import)]


#[c_import(h = "limits.h", INT_MAX)]
mod limits
{
	// This module is empty now, but the plugin will import
	// requested constants in here at compile time.
}

fn main()
{
	println!("Yay, got INT_MAX: {}", limits::INT_MAX);
}

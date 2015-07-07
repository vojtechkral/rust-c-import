use std::result::Result;
use std::ops::Deref;
use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::process::{Command, Stdio, ChildStdin};

use syntax::ast::Name;
use syntax::codemap::Span;
use syntax::ext::base::ExtCtxt;

use rustc_serialize::json::Json;

use def::*;


const BACKEND_NAME: &'static str = "clang-rust-import";

pub type Requests = Vec<Name>;

pub struct Backend<'a>
{
	sp: Span,
	header: &'a str,
	rqs: &'a Requests,
}

impl<'a> Backend<'a>
{
	pub fn new(sp: Span, header: &'a str, requests: &'a Requests) -> Backend<'a>
	{
		Backend{
			sp: sp,
			header: header,
			rqs: requests,
		}
	}

	fn write_stdin(&self, stdin: &mut ChildStdin) -> io::Result<()>
	{
		try!(write!(stdin, "[\"{}\"", self.rqs.first().unwrap().deref()));  // Number of rqs checked by rq loader
		for rq in self.rqs.iter().skip(1)
		{
			try!(write!(stdin, ", \"{}\"", rq.deref()));
		}
		try!(write!(stdin, "]\n"));

		Ok(())
	}

	fn read_defs(&self, cx: &mut ExtCtxt, defs: &mut Defs, json_str: &String)
	{
		let json = match Json::from_str(&json_str)
		{
			Ok(json) => json,
			Err(err) => cx.span_fatal(self.sp, &format!("Backend error: {}", err.description())),
		};

		let json_defs = match json.as_array()
		{
			Some(array) => array,
			None => cx.span_fatal(self.sp, &format!("Backend error: JSON not an array: {}", &json_str[..50])),
		};

		for def in json_defs.iter()
		{
			let def = match def_from_json(&def)
			{
				Some(def) => def,
				None => cx.span_fatal(self.sp, "Backend error: Could not get defs"),
			};
			defs.push(def);
		}
	}

	pub fn import(&self, cx: &mut ExtCtxt, defs: &mut Defs)
	{
		let mut ps = match Command::new(BACKEND_NAME)
			.stdin(Stdio::piped())
			.stdout(Stdio::piped())
			.arg(self.header)
			.spawn()
		{
			Err(err) => cx.span_fatal(self.sp, &format!("Backend error: {}", err.description())),
			Ok(p) => p,
		};

		{
			let stdin = ps.stdin.as_mut().unwrap();
			if let Err(err) = self.write_stdin(stdin)
			{
				cx.span_fatal(self.sp, &format!("Backend error: {}", err.description()));
			}
		}

		let out = match ps.wait_with_output()
		{
			Ok(out) => out,
			Err(err) => cx.span_fatal(self.sp, &format!("Backend error: {}", err.description())),
		};

		if !out.status.success()
		{
			cx.span_fatal(self.sp, &format!("Backend error: {}", out.status));
		}

		let json_str = match String::from_utf8(out.stdout)
		{
			Ok(s) => s,
			Err(err) => cx.span_fatal(self.sp, &format!("Backend error: {}", err.description())),
		};

		self.read_defs(cx, defs, &json_str)
	}
}

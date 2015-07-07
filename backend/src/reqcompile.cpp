#include "reqcompile.h"

using std::string;


ReqCompile::ReqCompile(const string& name)
	:nm(name)
{
}

const std::string& ReqCompile::name() const
{
	return nm;
}

const char* ReqCompile::snippet_prefix = "rust_import_";

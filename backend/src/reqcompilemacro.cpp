#include "reqcompilemacro.h"

using std::unique_ptr;
using std::string;

#include <llvm/IR/Module.h>

using namespace llvm;

#include "defconstint.h"


ReqCompileMacro::ReqCompileMacro(const string& name)
	:ReqCompile(name), snippet_name(snippet_prefix + name)
{
}

void ReqCompileMacro::addCodeTo(string &code) const
{
	code += "auto ";
	code += snippet_name;
	code += " = ";
	code += name();
	code += ";\n";
}

unique_ptr<Def> ReqCompileMacro::makeDef(const Module &module) const
{
	auto gv = module.getNamedGlobal(snippet_name);
	if (!gv) return nullptr;
	if (!gv->hasInitializer()) return nullptr;

	auto init = gv->getInitializer();
	auto type = init->getType();

	if (type->isIntegerTy())
	{
		auto apint = init->getUniqueInteger();
		auto bits = apint.getBitWidth();
		if (bits <= 64)
		{
			auto value = apint.getSExtValue();
			return unique_ptr<Def>(new DefConstInt(value, bits));
		}
	}

	return nullptr;
}

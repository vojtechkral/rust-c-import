#include "request.h"

using std::string;

#include "reqcompilemacro.h"


Request::Request(const std::string& name)
	:nm(name)
{
	// For now, always create ReqCompile
	req_compile.reset(new ReqCompileMacro(nm));
}

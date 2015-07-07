#ifndef C_IMPORT_REQUEST_H
#define C_IMPORT_REQUEST_H

#include <memory>
#include <string>
#include <utility>

#include "def.h"
#include "reqcompile.h"


class Request
{
private:
	std::string nm;

	std::unique_ptr<Def> df;
	std::unique_ptr<ReqCompile> req_compile;
public:
	Request(const std::string& nm);

	const std::string& name() const { return nm; }
	void setDef(std::unique_ptr<Def>& def) { df = std::move(def); }
	const Def* def() const { return df.get(); }
	const ReqCompile* reqCompile() const { return req_compile.get(); }
};


#endif  //C_IMPORT_REQUEST_H

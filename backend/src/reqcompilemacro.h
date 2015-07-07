#ifndef C_IMPORT_REQCOMPILEMACRO_H
#define C_IMPORT_REQCOMPILEMACRO_H

#include "reqcompile.h"


class ReqCompileMacro: public ReqCompile
{
private:
	const std::string snippet_name;
public:
	ReqCompileMacro(const std::string& name);

	virtual void addCodeTo(std::string& code) const;
	virtual std::unique_ptr<Def> makeDef(const llvm::Module& module) const;
};


#endif  //C_IMPORT_REQCOMPILEMACRO_H

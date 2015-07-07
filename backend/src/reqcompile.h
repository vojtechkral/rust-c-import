#ifndef C_IMPORT_REQCOMPILE_H
#define C_IMPORT_REQCOMPILE_H

#include <memory>
#include <string>

#include "def.h"


namespace llvm { class Module; }

class ReqCompile
{
private:
	const std::string& nm;
protected:
	static const char* snippet_prefix;
public:
	ReqCompile(const std::string& name);

	const std::string& name() const;

	virtual void addCodeTo(std::string& code) const = 0;
	virtual std::unique_ptr<Def> makeDef(const llvm::Module& module) const = 0;
};

#endif  //C_IMPORT_REQCOMPILE_H

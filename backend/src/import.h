#ifndef C_IMPORT_IMPORT_H
#define C_IMPORT_IMPORT_H

#include <memory>
#include <vector>

#include "requests.h"


namespace llvm
{
	class Module;
}


class Import
{
private:
	Requests& rqs;

	void make_import_snippet(const char* header, std::string& out);
	bool get_globals(const std::unique_ptr<llvm::Module> &module);

	static const char* ipaths[];
	static const char* input_name;
public:
	Import(Requests &requests);

	int import(const char* header);
};


#endif  //C_IMPORT_IMPORT_H

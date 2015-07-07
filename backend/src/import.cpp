#include "import.h"

#include <vector>
#include <string>
#include <memory>
#include <iostream>

using std::unique_ptr;
using std::string;

#include <clang/Frontend/TextDiagnosticPrinter.h>
#include <clang/Frontend/CompilerInvocation.h>
#include <clang/Frontend/CompilerInstance.h>
#include <clang/CodeGen/CodeGenAction.h>
#include <clang/Driver/Tool.h>
#include <clang/Driver/Driver.h>
#include <clang/Driver/Compilation.h>
#include <llvm/Support/TargetSelect.h>
#include <llvm/Support/Host.h>
#include <llvm/Support/Program.h>
#include <llvm/Support/ErrorHandling.h>
#include <llvm/Support/DataTypes.h>
#include <llvm/Support/MemoryBuffer.h>
#include <llvm/IR/Module.h>

using namespace ::clang;
using namespace ::llvm;

#include "config.h"


Import::Import(Requests &requests)
  :rqs(requests)
{
	InitializeAllTargets();
	InitializeAllTargetMCs();
	InitializeAllAsmPrinters();
	InitializeAllAsmParsers();
}

const char* Import::ipaths[] = INCLUDE_PATHS;
const char* Import::input_name = "import.cpp";

void Import::make_import_snippet(const char *header, std::string &out)
{
	out.clear();
	out += "extern \"C\" {\n#include \"";
	out += header;
	out += "\"\n}\n\n";

	for (auto r: rqs)
	{
		auto req_compile = r->reqCompile();
		if (req_compile) req_compile->addCodeTo(out);
	}
}

bool Import::get_globals(const std::unique_ptr<llvm::Module>& module)
{
	for (auto r: rqs)
	{
		auto req_compile = r->reqCompile();
		if (req_compile)
		{
			auto def = req_compile->makeDef(*module);
			r->setDef(def);
		}
	}

	return true;
}

int Import::import(const char *header)
{
	// Setup diagnostics stuff
	IntrusiveRefCntPtr<DiagnosticIDs> diag_ids(new DiagnosticIDs());
	IntrusiveRefCntPtr<DiagnosticOptions> diag_opts = new DiagnosticOptions();
	auto diag = new TextDiagnosticPrinter(llvm::errs(), &*diag_opts);
	DiagnosticsEngine diag_engine(diag_ids, &*diag_opts, diag);

	// Try to find clang install dir
	auto clang_bin = sys::findProgramByName("clang");
	if (!clang_bin) return false;
	auto clang_path = llvm::sys::path::parent_path(*clang_bin);

	// Init Driver
	Triple triple(sys::getProcessTriple());
	driver::Driver driver(clang_path, triple.str(), diag_engine);
	driver.setCheckInputsExist(false);

	// Init Driver (user-specified) args
	driver::ArgStringList d_args;
	d_args.push_back("clang");
	d_args.push_back("-std=c++11");
	for (auto path = ipaths; *path; path++)
	{
		d_args.push_back("-I");
		d_args.push_back(*path);
	}
	d_args.push_back(input_name);

	// Init Compilation and obtain Command
	std::unique_ptr<driver::Compilation> compilation(driver.BuildCompilation(d_args));
	if (!compilation) return -1;
	const driver::JobList &jobs = compilation->getJobs();
	if (jobs.size() < 1 || !isa<driver::Command>(*jobs.begin())) return -2;
	const driver::Command &command = cast<driver::Command>(*jobs.begin());
		// The driver also creates a linker job, but we're not going to use that

	// Init CompilerInvocation
	auto ci_args = command.getArguments();
	std::unique_ptr<CompilerInvocation> ci(new CompilerInvocation);
	CompilerInvocation::CreateFromArgs(*ci, const_cast<const char**>(ci_args.data()),
		const_cast<const char**>(ci_args.data()) + ci_args.size(), diag_engine);

	// Init CompilerInsatce
	CompilerInstance clang;
	clang.setInvocation(ci.release());

	// Create code snippet to compile
	string snippet;
	make_import_snippet(header, snippet);
	auto membuffer = MemoryBuffer::getMemBuffer(snippet.c_str());
	clang.getPreprocessorOpts().addRemappedFile(input_name, membuffer.release());

	// Conpile the code
	clang.createDiagnostics();
	if (!clang.hasDiagnostics()) return -3;
	std::unique_ptr<CodeGenAction> action(new EmitLLVMOnlyAction());
	if (!clang.ExecuteAction(*action)) return -4;

	// Evaluate results
	auto module = action->takeModule();
	return get_globals(module) ? 0 : -5;
}


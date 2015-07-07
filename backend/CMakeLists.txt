cmake_minimum_required(VERSION 2.6)
project(rust-c-import-backend)

find_package(LLVM REQUIRED CONFIG)

message(STATUS "Found LLVM ${LLVM_PACKAGE_VERSION}")
message(STATUS "Using LLVMConfig.cmake in: ${LLVM_DIR}")


# Figure out include dirs
#   Clang is basically unable to locate its resource dir
#   unless resulting binary is placed in the same dir as clang
#   which unsurprisingly usually isn't the case.
# This is an attempt to ensure all needed include paths are present
execute_process(
	COMMAND clang -xc++ -E -v "${CMAKE_CURRENT_SOURCE_DIR}/src/empty.h"
	OUTPUT_VARIABLE clang_stdout
	ERROR_VARIABLE clang_stderr
)
string(REGEX MATCH "#include <\\.\\.\\.> search starts here:.*End of search list\\." ipaths ${clang_stderr})
string(REGEX REPLACE "#include <\\.\\.\\.> search starts here:\n(.*)\nEnd of search list\\." "\\1" ipaths ${ipaths})
string(REPLACE "\n" ";" ipaths ${ipaths})
set(INCLUDE_PATHS "{")
foreach(path ${ipaths})
	string(STRIP ${path} path)
	message(STATUS "Add clang include path: ${path}")
	set(INCLUDE_PATHS "${INCLUDE_PATHS}\"${path}\",")
endforeach()
set(INCLUDE_PATHS "${INCLUDE_PATHS} 0 }")
configure_file("src/config.h.in" "config.h")


set(backend_sources
	"src/import.cpp"
	"src/request.cpp"
	"src/request.cpp"
	"src/requests.cpp"
	"src/reqcompile.cpp"
	"src/reqcompilemacro.cpp"
	"src/def.cpp"
	"src/defconst.cpp"
	"src/defconstint.cpp"
	"src/main.cpp"
)

set(backend_libs
	"jsoncpp"
)

set(clang_libs
	"clangTooling"
	"clangFrontend"
	"clangDriver"
	"clangParse"
	"clangSema"
	"clangEdit"
	"clangLex"
	"clangCodeGen"
	# ^order

#	"clangARCMigrate"
	"clangAST"
#	"clangASTMatchers"
	"clangAnalysis"
	"clangBasic"
#	"clangDynamicASTMatchers"
#	"clangFormat"
#	"clangFrontendTool"
#	"clangIndex"
#	"clangRewrite"
#	"clangRewriteFrontend"
	"clangSerialization"
#	"clangStaticAnalyzerCheckers"
#	"clangStaticAnalyzerCore"
#	"clangStaticAnalyzerFrontend"
#	"clangToolingCore"
)

include_directories(${LLVM_INCLUDE_DIRS})
add_definitions(${LLVM_DEFINITIONS})

set(llvm_libs
#	"LLVMLTO"
	"LLVMObjCARCOpts"
	"LLVMLinker"
	"LLVMBitWriter"
	"LLVMIRReader"
#	"LLVMAsmParser"
	"LLVMR600CodeGen"
#	"LLVMipo"
#	"LLVMVectorize"
#	"LLVMR600AsmParser"
#	"LLVMR600Desc"
#	"LLVMR600Info"
#	"LLVMR600AsmPrinter"
#	"LLVMSystemZDisassembler"
	"LLVMSystemZCodeGen"
	"LLVMSystemZAsmParser"
#	"LLVMSystemZDesc"
#	"LLVMSystemZInfo"
#	"LLVMSystemZAsmPrinter"
#	"LLVMHexagonDisassembler"
	"LLVMHexagonCodeGen"
#	"LLVMHexagonDesc"
#	"LLVMHexagonInfo"
	"LLVMNVPTXCodeGen"
#	"LLVMNVPTXDesc"
#	"LLVMNVPTXInfo"
#	"LLVMNVPTXAsmPrinter"
	"LLVMCppBackendCodeGen"
#	"LLVMCppBackendInfo"
	"LLVMMSP430CodeGen"
#	"LLVMMSP430Desc"
#	"LLVMMSP430Info"
#	"LLVMMSP430AsmPrinter"
#	"LLVMXCoreDisassembler"
	"LLVMXCoreCodeGen"
#	"LLVMXCoreDesc"
#	"LLVMXCoreInfo"
#	"LLVMXCoreAsmPrinter"
#	"LLVMMipsDisassembler"
	"LLVMMipsCodeGen"
	"LLVMMipsAsmParser"
#	"LLVMMipsDesc"
#	"LLVMMipsInfo"
#	"LLVMMipsAsmPrinter"
#	"LLVMAArch64Disassembler"
	"LLVMAArch64CodeGen"
	"LLVMAArch64AsmParser"
#	"LLVMAArch64Desc"
#	"LLVMAArch64Info"
#	"LLVMAArch64AsmPrinter"
#	"LLVMAArch64Utils"
#	"LLVMARMDisassembler"
	"LLVMARMCodeGen"
	"LLVMARMAsmParser"
#	"LLVMARMDesc"
#	"LLVMARMInfo"
#	"LLVMARMAsmPrinter"
#	"LLVMPowerPCDisassembler"
	"LLVMPowerPCCodeGen"
	"LLVMPowerPCAsmParser"
#	"LLVMPowerPCDesc"
#	"LLVMPowerPCInfo"
#	"LLVMPowerPCAsmPrinter"
#	"LLVMSparcDisassembler"
	"LLVMSparcCodeGen"
	"LLVMSparcAsmParser"
#	"LLVMSparcDesc"
#	"LLVMSparcInfo"
#	"LLVMSparcAsmPrinter"
	"LLVMTableGen"
#	"LLVMDebugInfo"
	"LLVMOption"
#	"LLVMX86Disassembler"
	"LLVMX86AsmParser"
	"LLVMX86CodeGen"
#	"LLVMSelectionDAG"
#	"LLVMAsmPrinter"
#	"LLVMX86Desc"
#	"LLVMMCDisassembler"
#	"LLVMX86Info"
#	"LLVMX86AsmPrinter"
#	"LLVMX86Utils"
#	"LLVMMCJIT"
#	"LLVMLineEditor"
	"LLVMInstrumentation"
#	"LLVMInterpreter"
#	"LLVMExecutionEngine"
#	"LLVMRuntimeDyld"
	"LLVMCodeGen"
#	"LLVMScalarOpts"
#	"LLVMProfileData"
#	"LLVMObject"
#	"LLVMMCParser"
#	"LLVMBitReader"
#	"LLVMInstCombine"
#	"LLVMTransformUtils"
#	"LLVMipa"
#	"LLVMAnalysis"
#	"LLVMTarget"
#	"LLVMMC"
#	"LLVMCore"
#	"LLVMSupport"
)


include_directories(${CMAKE_BINARY_DIR})
set(CMAKE_POSITION_INDEPENDENT_CODE 1)
add_executable(clang-rust-import ${backend_sources})
target_compile_features(clang-rust-import PRIVATE cxx_generalized_initializers)
target_link_libraries(clang-rust-import ${backend_libs} ${clang_libs} ${llvm_libs})  # Order matters

# Libs file
if(LIBS_FILE)
	file(WRITE ${LIBS_FILE} "")
	foreach(lib ${clang_libs} ${llvm_libs})
		file(APPEND ${LIBS_FILE} "${lib}" "\n")
	endforeach()
endif()

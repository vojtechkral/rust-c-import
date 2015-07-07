#ifndef C_IMPORT_DEFCONSTINT_H
#define C_IMPORT_DEFCONSTINT_H

#include <cstdint>

#include "defconst.h"


class DefConstInt: public DefConst
{
private:
	int64_t value;
	unsigned bits;
public:
	DefConstInt(uint64_t value, unsigned bits);

	virtual Json::Value serialize() const;
};


#endif  //C_IMPORT_DEFCONSTINT_H

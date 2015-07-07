#include "defconstint.h"


DefConstInt::DefConstInt(uint64_t value, unsigned bits)
	:value(value), bits(bits)
{
}

Json::Value DefConstInt::serialize() const
{
	Json::Value ret(Json::objectValue);

	Json::Value intobj(Json::objectValue);
	intobj["bits"] = bits;
	intobj["value"] = (Json::Value::Int64)value;

	ret["const"]["int"] = intobj;

	return ret;
}

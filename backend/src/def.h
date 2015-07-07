#ifndef C_IMPORT_DEF_H
#define C_IMPORT_DEF_H

#include <json/json.h>


class Def
{
private:
public:
	Def();

	virtual Json::Value serialize() const = 0;
};


#endif  //C_IMPORT_DEF_H

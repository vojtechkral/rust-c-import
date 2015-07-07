#ifndef C_IMPORT_INTERFACE_H
#define C_IMPORT_INTERFACE_H

#include <vector>
#include <iostream>

#include "request.h"


class Import;

class Requests: public std::vector<Request*>
{
public:
	Requests();
	~Requests();

	void readRequests(std::istream& stream);
	void writeRequests(std::ostream& stream) const;
};

std::istream& operator>>(std::istream& stream, Requests& iface);
std::ostream& operator<<(std::ostream& stream, const Requests& iface);


#endif  //C_IMPORT_INTERFACE_H

#include "requests.h"

#include <json/json.h>

using std::istream;
using std::ostream;


Requests::Requests()
{
}

Requests::~Requests()
{
	for (auto r: *this) delete r;
}

void Requests::readRequests(istream &stream)
{
	Json::Value root;

	stream >> root;
	if (!root.isArray()) return;
	for (unsigned i = 0; i < root.size(); i++)
	{
		auto req_name = root[i].asString();
		push_back(new Request(req_name));
	}
}

void Requests::writeRequests(ostream &stream) const
{
	Json::Value results(Json::arrayValue);

	for (auto req = cbegin(); req != cend(); ++req)
	{
		auto def = (*req)->def();
		if (def) results.append(def->serialize());
	}

	stream << results;
}


std::istream& operator>>(std::istream &stream, Requests &iface)
{
	iface.readRequests(stream);
	return stream;
}


std::ostream& operator<<(std::ostream &stream, const Requests &iface)
{
	iface.writeRequests(stream);
	return stream;
}

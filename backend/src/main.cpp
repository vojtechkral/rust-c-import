#include <iostream>

#include "requests.h"
#include "import.h"

using std::cout;
using std::cin;
using std::endl;


int main(int argc, char *argv[])
{
	if (argc != 2) return 1;

	Requests requests;

	cin >> requests;

	Import import(requests);
	auto ret = import.import(argv[1]);
	if (ret) return ret;

	cout << requests << endl;

	return 0;
}

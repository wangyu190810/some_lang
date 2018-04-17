#include <iostream>

void point_type() {
	int ival = 1014;
	int *pi = &ival;
	int **ppi = &pi;

	std::cout << "The value of ival \n"
		<< "direct value :" << ival << "\n"
		<< "inderect value :" << *pi << "\n"
		<< "doubly indirect  value " << **ppi
		<< std::endl;
}


int main() {
	int sum = 0;
	for (int val = 1; val <= 10; val++) {
		sum += val;

	};
	std::cout << "Sum of 1 to 10 inclusive is "
		<< sum << std::endl;
	point_type();
	return 0;
}


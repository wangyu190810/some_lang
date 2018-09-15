#include <iostream>


void test_one(){
    std::cout << "Hello world\n";
    std::cout << "Enter tow number :" << std::endl;
    int v1 = 0, v2 = 0;
    std::cin >> v1 >> v2;
    std::cout << "The mut of " << v1 << " and " << v2 << " is " << v1 * v2 << std::endl;
    std::cout << "Enter tow number :" << std::endl;   
}

int main(){
    // std::cout << "Enter tow number :" << std::endl;
    // int v1 = 0, v2 = 0;
    // std::cin >> v1 >> v2;
    // std::cout << "The sum of " << v1 << " and " << v2 << " is " << v1 + v2 << std::endl;
    test_one();
    return 0;
}





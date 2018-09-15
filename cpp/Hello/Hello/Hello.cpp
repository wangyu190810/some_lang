// Hello.cpp : 定义控制台应用程序的入口点。
//

#include "stdafx.h"


// -----------------------------
// 编写程序输出所有的水仙花数。所谓水仙花数是指一个三位整数，其各位数的立方和等于该数。
// 例如：153=1*1*1+5*5*5+3*3*3
// -----------------------------

#include "stdafx.h"
#include <iostream>
using namespace std;
int main()
{
	int a, b, c;
	cout << "所有水仙花数为：" << endl;
	for (int i = 100; i <= 999; i++)
	{
		a = i / 100;
		b = i % 100 / 10;
		c = i % 10;
		if (a*a*a + b*b*b + c*c*c == i)
		{
			cout << i << endl;
		}
	}
	//System("pause");
	//ring str;
	char abc[1] = { 0 };
	cin >> a;
	return 0;
}

// Hello.cpp : �������̨Ӧ�ó������ڵ㡣
//

#include "stdafx.h"


// -----------------------------
// ��д����������е�ˮ�ɻ�������νˮ�ɻ�����ָһ����λ���������λ���������͵��ڸ�����
// ���磺153=1*1*1+5*5*5+3*3*3
// -----------------------------

#include "stdafx.h"
#include <iostream>
using namespace std;
int main()
{
	int a, b, c;
	cout << "����ˮ�ɻ���Ϊ��" << endl;
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

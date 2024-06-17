#include<iostream>

int main()
{
	int a, b, c;
	std::cin >> a;
	std::cin >> b;
	std::cin >> c;

	b += c;
	a += b / 60;
	b %= 60;
	if (a>=24)
	{
		a -= 24;
	}

	std::cout << a << " " << b;
}
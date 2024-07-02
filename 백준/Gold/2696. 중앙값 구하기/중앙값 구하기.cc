#include<iostream>
#include<vector>
#include<algorithm>

std::vector<int> v;

int medium(int end)
{
	int medium;
	std::vector<int> w;
	for (int i = 0; i <= end; i++)
	{
		w.push_back(v[i]);
	}
	std::sort(w.begin(), w.end());
	medium = w[end / 2];
	w.clear();
	return medium;
}

int main()
{
	int t, m, temp;
	std::cin >> t;
	for (int i = 0; i < t; i++)
	{
		std::cin >> m;
		for (int j = 0; j < m; j++)
		{
			std::cin >> temp;
			v.push_back(temp);
		}
		std::cout << (m + 1) / 2 << "\n";
		for (int j = 0; j < (m + 1) / 2; j++)
		{
			std::cout << medium(2 * j) << " ";
			if (j % 10 == 9)
				std::cout << "\n";
		}
		std::cout << "\n";
		v.clear();
	}

	return 0;
}
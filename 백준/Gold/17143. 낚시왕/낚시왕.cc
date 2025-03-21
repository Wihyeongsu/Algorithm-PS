#include<iostream>
#include<vector>
#include<algorithm>
#include<cmath>
#include<string.h>

using namespace std;

// 열에서 가장 위에 위치한 상어 탐색
// 상어 이동
// 상어 이동에 cycle 존재
// 위치, 방향 정보를 index로 표현
// ex) 1, 2, 3, 4, 5, 6, 5, 4, 3, 2


typedef struct Shark
{
	int r,c,s,d,z;
}SHARK;



int map[101][101]; // Shark size map
vector<SHARK> sharks;

void moveShark(int& size, int& pos, int& dir, int& speed, int& cycle)
{
	int ipos = size + abs(size - pos) * (dir == 1 || dir == 4) ? -1 : 1; // to index
	ipos = (ipos + speed - 1) % cycle + 1; // move

	pos = size - abs(ipos - size); // to position
	dir = (ipos >= size) ? 1 : 2;
}

int main()
{
	int R, C, M, result = 0;
	int fisher = 0;

	cin >> R >> C >> M;

	// Cycle size
	int cx = 2 * C - 2;
	int cy = 2 * R - 2;

	for (int i = 0; i < M; i++)
	{
		SHARK temp;
		cin >> temp.r >> temp.c >> temp.s >> temp.d >> temp.z;
		sharks.push_back(temp);
		map[temp.r][temp.c] = temp.z;
	}
	
	while (fisher < C)
	{
		// No shark remained
		if (sharks.empty())
		{
			break;
		}
		
		// 1. Move fisher
		fisher++;	
		
		// 2. Hunt shark O(n)
		// cout << "(" << fisher << ") ";
		for (int row = 1; row <= R; row++)
		{
			if (map[row][fisher] > 0)
			{
				// cout << "Catch!: " << map[row][fisher];
				result += map[row][fisher];
				map[row][fisher] = 0;
				break;
			}
		}
		// cout << "\n";

		// 3. Move sharks
		int to_map[101][101];
		memset(to_map, 0, sizeof(to_map)); // Essential!!
		vector<SHARK> to_sharks;
		while (!sharks.empty())
		{
			SHARK shark = sharks.back();
			sharks.pop_back();
			
			// Check presence
			if (map[shark.r][shark.c] != shark.z)
			{
				continue;
			}
			map[shark.r][shark.c] = 0;
			
			// Move y
			if (shark.d <= 2)
			{
				int d = (shark.d == 1) ? 1 : -1;
				int ipos = R + abs(R - shark.r) * d; // to index
				ipos = (ipos + shark.s - 1) % cy+ 1; // move

				shark.r =  R - abs(ipos - R); // to position
				shark.d = (ipos >= R) ? 1 : 2;
			}
			// Move x
			else
			{
				// moveShark(C, shark.c, shark.d, shark.s, cx);
				int d = (shark.d == 4) ? 1 : -1;
				int ipos = C + abs(C - shark.c) * d; // to index
				ipos = (ipos + shark.s - 1) % cx+ 1; // move

				shark.c =  C - abs(ipos - C); // to position
				shark.d = (ipos >= C) ? 4 : 3;
			}
			// cout << "Size " << shark.z << " Move to (" << shark.r << ", " << shark.c << ")\n";
			
			// Check if position is same
			if (to_map[shark.r][shark.c] > shark.z)
			{
				continue;
			}
			
			
			to_map[shark.r][shark.c] = shark.z;
			to_sharks.push_back(shark);

			
		}
		
		sharks = to_sharks;
		for (int i = 1; i <= R; i++)
		{
			for (int j = 1; j <= C; j++)
			{
				map[i][j] = to_map[i][j];
				// cout << map[i][j] << " ";
			}
			// cout << "\n";
		}
		
	}

	cout << result;


	return 0;
}
## Advent of Code 2023
This winter I tried learning rust with advent of code. Although I wasn't regular, it was a fun experience!

### Solutions
#### Day 1
Brute force the answer.
#### Day 2
Nothing fancy in the question, parsing the input was the only issue which could have been done elegantly but I just wrote all the conditions.
#### Day 3
1. For each digit of a number in the grid, check if there are any special characters in the neighbouring cells.
2. Maintain the coordinates of gears adjacent to each number in the grid.
#### Day 4
1. Maintain the winning numbers.
2. While processing all the matches from card $i$, we can push the count of resulting copies we win to future cards, essentially processing all the cards in sequence. 
#### Day 6
1. Brute force for the answer for each race which is $O(t_{i})$ for $i_{}^{th}$ race. 
2. Parse the input and brute force the answer as $4e8$ operations would hardly take 3-4 seconds.
#### Day 7
1. This was a pretty annoying question to implement. I wanted to learn ```struct``` in rust so my resultant implementation was a bit lengthy implementing ```Ord``` properties to compare hands used for custom sorting.
2. The only difference between both the parts was how ```type``` or ```property``` was computed with the introduction of joker. Quiete a lot of casework, could have implemented more elegantly.
#### Day 8
1. Tracing the sequence of instructions, maintaining the count of moves till we reach the destination.
2. In this part, I think the question was ambiguous and did not mention that for each pair of source-destination it is guaranteed that cycle length = distance from source to destination. Fortunately, the input had this condition which meant that the answer was just $lcm(f_{1}, lcm(f_{2}, ..lcm(f_{n - 1}, f_{n})..))$ where $f_{i}$ is length of $i_{}^{th}$ cycle. If the condition was not satisfied, then we probably need to use chinese remainder theorem to find the answer.
#### Day 9
1. Brute force using recursion.
2. Similar to part 1, only return values of recursion changes.
#### Day 10
1. If $k$ is the length of the cycle, then answer is $\left \lceil K / 2 \right \rceil$.
#### Day 11
1. Maintain the indices of empty rows and columns and compute the distance between each pair of galaxies.
2. Similar to part 1, only expansion factor changes.
#### Day 12
1. Use Dynamic Programming to compute the number of possible arrangements. The states are defined as $dp[i][j]$ - number of possible arrangements that can be formed using $spring[i..n]$ with contiguous group of damaged springs $groups[j..m]$.
2. Use the same approach as part 1 but with modified input. 
#### Day 13
1. Each mountain can be solved separately.
2. The difference between part 1 and part 2 is the definition of a mirror. In part 1 we had exact reflection, In part 2 we have exactly 1 smudge or unequal point.
#### Day 14
1. Simulate the tilt.
2. No idea how to solve this part deterministically. Some observations upon tinkering is that there are cycles and grid cycles back to same states. This can be leveraged to find the initial length of cycle and avoid simulating the tilt and roll over and over again.
#### Day 15
1. Follow the ```HASH``` algorithm mentioned in the question.
2. Maintain each of the 256 boxes and update as per instructions.
#### Day 16
1. Do a DFS (depth-first-search) simulating the light tracing through the grid. Maintain a  ```cache[x][y]``` which stores direction vectors ```(dx, dy)``` from which the light had entered the cell ```(x, y)```. This will prevent our recursion from blowing up or lead to exponential time complexity.
2. Simulate the light entering from all four edges and take the one which gives the maximum answer.
#### Day 17
1. Use Dijkstra's algorithm to find the shortest path.
2. Change the conditions when transitioning to a different cell in dijkstra.
#### Day 18
1. Notice the dig plan forms a polygon. Use [Shoelace's formula](https://en.wikipedia.org/wiki/Shoelace_formula) for area under polygon: $\tfrac{|\sum\limits_{i=0}^n (y[i] + y[(i + 1)\mod n]) * (x[i] + x[(i + 1)\mod n])|}{2}$ and [Pick's theorem](https://en.wikipedia.org/wiki/Pick%27s_theorem) to find the count of enclosed grid points.
2. Construct the polygon using the instructions given - converting hexadecimal to decimal for edge length.
#### Day 21
1. Do a DFS from the starting position, maintaining a ```cache[x][y]``` which stores the distance from starting position travelled to reach cell ```(x, y)``` to prevent our recursion from blowing up and make redundant calls.


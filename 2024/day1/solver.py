
from typing import Tuple, List

Data = Tuple[List[int], List[int]]

def parse(file) -> Data:
    a = []
    b = []
    with open(file, 'r') as f:
        line = f.readline().strip()
        while line:
            parts = line.split()
            a.append(int(parts[0]))
            b.append(int(parts[1]))
            line = f.readline().strip()
    return a, b


def solve(data: Data) -> Tuple[int, int]:
    a, b = data
    a.sort(reverse=True)
    b.sort(reverse=True)

    # Part 1
    sum = 0

    for i,j in zip(a, b):
        sum += abs(i-j)

    # Part 2
    freq = {}
    similarity = 0
    for x in b:
        freq[x] = freq.get(x, 0) + 1
    for x in a:
        similarity += x * freq.get(x, 0)

    return sum, similarity
    


if __name__ == "__main__":
    print(solve(parse("./input-simple.txt")))
    print(solve(parse("./input.txt")))

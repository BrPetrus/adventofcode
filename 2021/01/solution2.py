from typing import Deque
from collections import deque


def main() -> int:
    with open("input.txt") as data:
        nums: Deque[int] = deque()
        for _ in range(3):
            value = int(data.readline().strip())
            nums.append(value)

        result = 0
        for line in data:
            value = int(line.strip())
            if nums[0] < value:
                result += 1
            nums.popleft()
            nums.append(value)
    return result


if __name__ == "__main__":
    print(main())

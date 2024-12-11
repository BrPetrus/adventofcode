from functools import reduce
from typing import List
from collections.abc import Generator

Report = List[int]

def parse_input(path: str) -> List[Report]:
    reports = []
    with open(path, 'r') as reports_file:
        line = reports_file.readline().strip()
        while line != "":
            reports.append(list(map(int, line.split(' '))))
            line = reports_file.readline().strip()
    print(f"Read {len(reports)} lines")
    return reports

def increasing_predicate(l: List[int]) -> bool:
    return reduce(
        lambda x, y: x and y[0] < y[1],
        zip(l, l[1:]),
        True
    )

def decreasing_predicate(l: List[int]) -> bool:
    return reduce(
        lambda x, y: x and y[0] > y[1],
        zip(l, l[1:]),
        True
    )

def difference_predicate(l: List[int]) -> bool:
    return reduce(
        lambda x, y: x and (0 < abs(y[0] - y[1]) < 4),
        zip(l, l[1:]),
        True
    )

def is_safe(l: List[int]) -> bool:
    return (increasing_predicate(l) or decreasing_predicate(l)) \
            and difference_predicate(l)


def solve_part1(reports: List[Report]) -> int:

    which_safe = map(is_safe, reports)
    return sum(map(lambda x: 1 if x else 0, which_safe)) 


def solve_part2(reports: List[Report]) -> int:
    def generate(l: List[int]) -> Generator[List[int], None, None]:
        for i in range(len(l)):
            yield l[:i] + l[i+1:]

    result = 0

    for report in reports:
        # Generate all variations
        result += any(map(is_safe, generate(report)))

    return result




if __name__ == "__main__":
    simple = parse_input("input-simple.txt")
    print(f"Solution to simple version {solve_part1(simple)}")
    print(f"Solution to simple version part 2 {solve_part2(simple)}")

    full = parse_input("input-full.txt")
    print(f"Solution to full version {solve_part1(full)}")
    print(f"Solution to full version part 2 {solve_part2(full)}")




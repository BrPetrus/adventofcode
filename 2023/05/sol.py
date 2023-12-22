from io import TextIOBase
from typing import Union

Mapping = dict[int, int]
Configuration = tuple[list[int], list[Mapping]]

def read_input(stream: TextIOBase) -> Configuration:
    def read_mapping():
        mapping = {}
        assert stream.readline() != ''  # Read header

        line = stream.readline()
        while line != '\n':
            print(line.strip().split(' '))
            dest, src, length = line.split(' ')
            length = int(length)
            dest = int(dest)
            src = int(src)
            for i in range(length):
                mapping[src+i] = dest+i
            line = stream.readline()
        return mapping

    # Read seed numbers
    seeds_str = stream.readline().strip()
    label, seed_str = seeds_str.split(':')
    assert label == "seeds"
    seeds = [int(x) for x in seed_str.strip().split(' ')]

    # Start reading the mappings
    mappings = []
    while stream.readline() != "":
        mappings.append(read_mapping())

    return seeds, mappings

def sol(input_stream: TextIOBase) -> int:
    seeds, mappings = read_input(input_stream)
    print(seeds)
    print(len(mappings))

    best_seed: Union[None, tuple[int, int]] = None
    for seed in seeds:
        location = seed
        for mapping in mappings:
            location = mapping.get(location, location)
        assert location != -1
        if best_seed is None or best_seed[1] > location:
            best_seed = (seed, location)
    assert best_seed is not None
    print(f"Best seed={best_seed[0]} at location={best_seed[1]}")
    return best_seed[1]


if __name__ == "__main__":
    with open("input.txt", "r") as stream:
        print(sol(stream))



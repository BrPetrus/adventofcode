import io

INPUT_PATH = "input.txt"

def solve(data: io.TextIOBase, red_limit: int, green_limit: int, blue_limit: int) -> tuple[int, int]:
    cum_ids = 0
    cum_powers = 0
    for line in data:
        print(line)

        # Extract game's ID
        id, content = line.split(":")
        id = id.split(" ")[1]

        # Maximums
        reds, blues, greens = 0, 0, 0

        # Extract games
        content = content.strip()
        turns = content.split(";")
        for turn in turns:
            turn = turn.strip()
            turn = turn.split(",")
            for type in turn:
                type = type.strip()
                number, color = type.split(" ")
                number = int(number)
                if color == "red":
                    reds = max(reds, number)
                elif color == "blue":
                    blues = max(blues, number)
                elif color == "green":
                    greens = max(greens, number)
                else:
                    assert False
        if reds <= red_limit and blues <= blue_limit and greens <= green_limit:
            cum_ids += int(id)
        cum_powers += (reds * blues * greens)
    return cum_ids, cum_powers
            



if __name__ == "__main__":
    with open('02/input.txt', 'r') as input_file:
        sol = solve(input_file, 12, 13, 14)
        print(f"Sol p. 1 = {sol[0]}\n"
              f"Sol p. 2 = {sol[1]}")
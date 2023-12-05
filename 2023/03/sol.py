from io import TextIOBase

def near_symbol(map: list[list[int]], x: int, y: int) -> bool:
    moves = [(0, 1), (1, 0), (1, 1), (-1, 0), (0, -1), (-1, -1), (1, -1), (-1, 1)]
    for dx, dy in moves:
        new_x, new_y = x + dx, y + dy
        if new_x < 0 or new_y < 0:
            continue
        if new_x >= len(map) or new_y >= len(map[0]):
            continue
        symbol = map[new_x][new_y]
        if symbol != '.' and not symbol.isnumeric():
            return True
    return False


def sol(input_data: TextIOBase) -> int:
    buffer = [input_data.readline().strip()] # First line
    running = True
    i = 0
    cum_num = 0
    while running:
        line = input_data.readline().strip()
        if line == '':
            running = False
        else:
            buffer.append(line)

        if len(buffer) == 4:
            buffer.pop(0)
            i -= 1

        found_symbol = False
        j = 0
        num = 0
        while j < len(buffer[i]):
            if not buffer[i][j].isnumeric():
                if found_symbol:
                    cum_num += num
                num = 0
                found_symbol = False
            elif buffer[i][j].isnumeric():
                num = num*10 + int(buffer[i][j])
                if not found_symbol and near_symbol(buffer, i, j):
                    found_symbol = True
            j += 1
        if found_symbol:
            cum_num += num
            num = 0
        i += 1
    return cum_num


if __name__ == "__main__":
    with open('03/input.txt', 'r') as input_file:
        sol = sol(input_file)
        print(f"Sol p. 1 = {sol}")

def main() -> int:
    with open("input.txt") as data:
        previous_value = int(data.readline().strip())
        count = 0
        for line in data:
            value = int(line.strip())
            if value > previous_value:
                count += 1
            previous_value = value
    return count


if __name__ == "__main__":
    print(f"main returned {main()}")

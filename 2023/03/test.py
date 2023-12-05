from sol import sol

def test_small():
    with open('03/input-small.txt', 'r') as input_file:
        answer = sol(input_file)
        assert answer == 4361

def test_large():
    with open('03/input.txt', 'r') as input_file:
        answer = sol(input_file)
        assert answer == 560670
from enum import auto
from typing import Optional, Any, Dict

Automaton = Any

def parse_input(path: str) -> str:
    with open(path, 'r') as f:
        line = f.readline().strip()
    return line

class AutomatonSolver:
    rules = {
        'm': ['u'],
        'u': ['l'],
        'l': ['('],
        '(': ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ','],
        ',': ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ')'],
    }
    end_state = ')'
    start_state = '('


    def parse(self, input: str) -> Optional[Tuple[int, int]]:
        parsed_chars = 0
        state = self.start_state

        for i in range(len(input)):
            symbol = input[i]
            if i != len(input)-1:
                hint = input[i+1]
            else:
                hint = None
            parsed_chars += 1
            
            if symbol == self.end_state:
                return 

            # Invalid char
            if symbol not in self.rules[state]:
                return False, parsed_chars

            # 





if __name__ == "__main__":
    automaton = FiniteAutomataSolution(rules)

    simple = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
    print(f"Solution simple part 1: {automaton.parse(simple)}")
    

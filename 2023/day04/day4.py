def parse_winning_and_having_nums(input: str) -> tuple[list[str], list[str]]:
    # strip Game description
    input_without_desc: str = input.split(':')[1]
    having_and_winning_nums: list[str] = input_without_desc.split('|')

    winning_nums: list[str] = having_and_winning_nums[0].split()
    having_nums: list[str] = having_and_winning_nums[1].split()
    
    return (winning_nums, having_nums)

def value_of_scratchcard(win_and_have: tuple[list[str], list[str]]) -> int:
    (win, have) = win_and_have
    num_matches = len([x for x in have if x in win])

    if num_matches == 0:
        return 0
    else:
        return 2 ** (num_matches - 1)
    
with open('input') as file:
    sum_of_values: int = sum([value_of_scratchcard(parse_winning_and_having_nums(line)) for line in file])

    print(sum_of_values)
def parse_winning_and_having_nums(input: str) -> tuple[list[str], list[str]]:
    # strip Game description
    input_without_desc = input.split(':')[1]
    having_and_winning_nums = input_without_desc.split('|')

    winning_nums = having_and_winning_nums[0].split()
    having_nums = having_and_winning_nums[1].split()
    
    return (winning_nums, having_nums)

def value_of_scratchcard(win_and_have: tuple[list[str], list[str]]) -> int:
    num_matches = num_of_matches(win_and_have)

    if num_matches == 0:
        return 0
    else:
        return 2 ** (num_matches - 1)
    
def num_of_matches(win_and_have: tuple[list[str], list[str]]) -> int:
    (win, have) = win_and_have
    return len([x for x in have if x in win])

def update_num_of_scratchcards(num_scratchcards: list[int], current_win_and_have: tuple[list[str], list[str]], current_idx: int) -> list[int]:
    current_card_num = num_scratchcards[current_idx]
    num_matches = num_of_matches(current_win_and_have)

    for i in range(current_idx + 1, current_idx + num_matches + 1):
        num_scratchcards[i] += current_card_num

    return num_scratchcards


with open('input') as file:
    # part 1
    sum_of_values = sum([value_of_scratchcard(parse_winning_and_having_nums(line)) for line in file])

    print(sum_of_values)

    # part 2
    file.seek(0)

    num_scratchcards = [1 for _ in file]

    file.seek(0)

    for i, line in enumerate(file):
        num_scratchcards = update_num_of_scratchcards(num_scratchcards, parse_winning_and_having_nums(line), i)

    sum_of_numbers = sum(num_scratchcards)

    print(sum_of_numbers)
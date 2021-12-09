def read_input():
    with open('./src/day6/input.txt') as input_file:
        contents = input_file.read().strip()

        return [int(val) for val in contents.split(',')]


def simulate_day(fishes):
    new_fishes = []

    for i in range(len(fishes)):
        fishes[i] = fishes[i] - 1

        if fishes[i] == -1:
            fishes[i] = 6
            new_fishes.append(8)

    return fishes + new_fishes


def get_num_fish_after_num_days(num_days):
    fishes = read_input()

    for _ in range(num_days):
        fishes = simulate_day(fishes)

    return len(fishes)

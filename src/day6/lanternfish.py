def read_input():
    with open('./src/day6/input.txt') as input_file:
        contents = input_file.read().strip()

        fish = [int(val) for val in contents.split(',')]
        return [fish.count(n) for n in range(9)]


def simulate_day(school):
    # get the number of fish about to spawn
    spawns = school[0]

    # move all the fish to the left (aka subtract a day) and add the new spawns to the end
    school = school[1:] + [spawns]

    # move the ones about to spawn to the '6' column
    school[6] += spawns

    return school


def get_num_fish_after_num_days(num_days):
    school = read_input()

    for _ in range(num_days):
        school = simulate_day(school)

    return sum(school)

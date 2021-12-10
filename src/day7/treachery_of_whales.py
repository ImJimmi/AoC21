def read_input():
    with open('./src/day7/input.txt') as input_file:
        contents = input_file.read().strip()
        return [int(pos) for pos in contents.split(',')]


def get_total_fuel_cost_for_position(starting_positions, target_position):
    return sum([abs(target_position - pos) for pos in starting_positions])


def get_fuel_cost_to_align_crabs():
    positions = read_input()
    max_position = max(positions)

    return min([get_total_fuel_cost_for_position(positions, target) for target in range(max_position)])

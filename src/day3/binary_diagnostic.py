def read_input():
    with open('src/day3/input.txt') as input_file:
        content = input_file.read().strip()
        readings = content.split('\n')

        return readings

    return ''


def most_common_bit_in_position(readings, position):
    count = 0

    for reading in readings:
        if reading[position] == '1':
            count += 1

    if count > len(readings) / 2:
        return '1'

    return '0'


def calculate_power_consumption():
    readings = read_input()
    num_bits = len(readings[0])

    gamma = ''.join(
        [most_common_bit_in_position(readings, bit) for bit in range(num_bits)]
    )
    epsilon = ''.join(['0' if bit == '1' else '1' for bit in gamma])

    power = int(gamma, 2) * int(epsilon, 2)
    return power

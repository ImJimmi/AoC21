from os import read


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

    if count >= len(readings) / 2:
        return '1'

    return '0'


def least_common_bit_in_position(readings, position):
    most_common = most_common_bit_in_position(readings, position)
    return '1' if most_common == '0' else '0'


def calculate_power_consumption():
    readings = read_input()
    num_bits = len(readings[0])

    gamma = ''.join(
        [most_common_bit_in_position(readings, bit) for bit in range(num_bits)]
    )
    epsilon = ''.join(['0' if bit == '1' else '1' for bit in gamma])

    power = int(gamma, 2) * int(epsilon, 2)
    return power


def filter_readings(readings, filter):
    num_bits = len(readings[0])
    filtered = list(readings)

    for bit in range(num_bits):
        to_keep = filter(filtered, bit)

        for reading in list(filtered):
            if reading[bit] != to_keep:
                filtered.remove(reading)

            if len(filtered) == 1:
                return int(filtered[0], 2)

    return filtered


def calculate_oxygen_rating(readings):
    return filter_readings(readings, most_common_bit_in_position)


def calculate_co2_rating(readings):
    return filter_readings(readings, least_common_bit_in_position)


def calculate_life_support_rating():
    readings = read_input()

    oxygen_rating = calculate_oxygen_rating(readings)
    co2_rating = calculate_co2_rating(readings)

    return oxygen_rating * co2_rating

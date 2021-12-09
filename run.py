from src.day3 import binary_diagnostic
from src.day4 import giant_squid
from src.day5 import hydrothermal_venture
from src.day6 import lanternfish

if __name__ == '__main__':
    print('3.1:', binary_diagnostic.calculate_power_consumption())
    print('3.2:', binary_diagnostic.calculate_life_support_rating())
    print('')
    print('4.1: ', giant_squid.calculate_final_score_of_winning_board())
    print('4.2: ', giant_squid.calculate_final_score_of_losing_board())
    print('')
    print('5.1: ', hydrothermal_venture.find_overlapping_points(False))
    print('5.2: ', hydrothermal_venture.find_overlapping_points(True))
    print('')
    print('6.1: ', lanternfish.get_num_fish_after_num_days(80))

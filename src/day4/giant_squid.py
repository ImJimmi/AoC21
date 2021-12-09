def read_input():
    with open('./src/day4/input.txt') as input_file:
        contents = input_file.read()
        lines = contents.split('\n')

        rng = [int(value) for value in lines[0].split(',')]
        string_boards = [board.strip()
                         for board in '\n'.join(lines[2:]).split('\n\n')]
        boards = [[[int(value) for value in line.split()]
                   for line in board.split('\n')] for board in string_boards]

        return rng, boards


def line_has_won(line):
    return list(line) == ['X'] * len(line)


def horizontal_line_has_won(board):
    return True in [line_has_won(line) for line in board]


def rotate_board(board):
    return list(zip(*board))


def vertical_line_has_won(board):
    return horizontal_line_has_won(rotate_board(board))


def board_has_won(board):
    return horizontal_line_has_won(board) or vertical_line_has_won(board)


def mark_number(board, number):
    for line in board:
        for i, value in enumerate(line):
            if number == value:
                line[i] = 'X'


def sum_unmarked(board):
    return sum([sum([val for val in line if val != 'X']) for line in board])


class BoardInfo:
    def __init__(self):
        self.board = []
        self.winning_number = None

    def get_score(self):
        return sum_unmarked(self.board) * self.winning_number


def sort_boards_in_order_of_winning(boards, rng):
    result = []

    boards_copy = list(boards)

    for number in rng:
        for board in list(boards_copy):
            mark_number(board, number)

            if board_has_won(board):
                info = BoardInfo()

                info.board = board
                info.winning_number = number

                result.append(info)
                boards_copy.remove(board)

    return result


def calculate_final_score_of_winning_board():
    rng, boards = read_input()

    board_info = sort_boards_in_order_of_winning(boards, rng)

    return board_info[0].get_score()


def calculate_final_score_of_losing_board():
    rng, boards = read_input()

    board_info = sort_boards_in_order_of_winning(boards, rng)

    return board_info[-1].get_score()

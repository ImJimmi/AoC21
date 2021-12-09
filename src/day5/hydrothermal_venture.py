class Line:
    def __init__(self):
        self.start = [0, 0]
        self.stop = [0, 0]

    def from_string(str):
        line = Line()

        point_strings = [point_str.strip() for point_str in str.split('->')]

        line.start = [int(value) for value in point_strings[0].split(',')]
        line.stop = [int(value) for value in point_strings[1].split(',')]

        return line

    def __repr__(self):
        return str(self.start) + ' -> ' + str(self.stop)

    def is_vertical(self):
        return self.start[0] == self.stop[0]

    def is_horizontal(self):
        return self.start[1] == self.stop[1]

    def get_x_range(self):
        return range(min(self.start[0], self.stop[0]),
                     max(self.start[0], self.stop[0]) + 1)

    def get_y_range(self):
        return range(min(self.start[1], self.stop[1]),
                     max(self.start[1], self.stop[1]) + 1)


def read_input():
    with open('./src/day5/input.txt') as input_file:
        contents = input_file.read().strip()
        lines = contents.split('\n')

        return [Line.from_string(line) for line in lines]


def find_overlapping_points():
    lines = [line for line in read_input() if line.is_horizontal()
             or line.is_vertical()]

    width = max([max([line.start[0], line.stop[0]]) for line in lines]) + 1
    height = max([max([line.start[1], line.stop[1]]) for line in lines]) + 1

    grid = [[0 for _ in range(width)] for _ in range(height)]

    for line in lines:
        for y in line.get_y_range():
            for x in line.get_x_range():
                grid[y][x] += 1

    count = 0

    for y in range(height):
        for x in range(width):
            if grid[y][x] > 1:
                count += 1

    return count

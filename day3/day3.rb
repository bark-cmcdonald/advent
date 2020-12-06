require 'pry'

INPUT = File.readlines('day3.txt').map { |x| x.strip }
EXAMPLE_INPUT = %w(
  ..##.......
  #...#...#..
  .#....#..#.
  ..#.#...#.#
  .#...##..#.
  ..#.##.....
  .#.#.#....#
  .#........#
  #.##...#...
  #...##....#
  .#..#...#.#
)
TREE = '#'.freeze

POSITION_SHIFT_PER_ROW = 3

def count(grid)
  grid.count(TREE)
end

def trajectory(grid, slopes)
  slopes.map do |position_shift_per_row, y|
    grid_row_length = grid[0].length
    trees = grid.each_slice(y).map { |x, _| x }.map.with_index do |latitudinal_geology, row|

      horizontal_shift = row * position_shift_per_row
      position = horizontal_shift % grid_row_length
      latitudinal_geology[position]
    end

    trees.count('#')
  end.reduce(:*)
end

# Part 1
puts(trajectory(INPUT, [[3, 1]]))
# Part 2
puts(trajectory(INPUT, [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]]))

# frozen_string_literal: true

require 'pry'
require 'set'

class Edge
  attr_accessor :weight
  attr_accessor :vertex
  def initialize(vertex:, weight:)
    @vertex = vertex
    @weight = weight
  end
end

class Vertex
  attr_accessor :name
  attr_accessor :edges
  def initialize(name:)
    @name = name
    @edges = []
  end
  def to_s
    name
  end
end

file_path = File.expand_path('day7.txt', __dir__)
input     = File.read(file_path)

vertices = Hash.new { |hash, key| hash[key] = Vertex.new(name: key) }

input.split("\n").each do |line|
  parent, contain = line.match(/(\w+ \w+) bags contain (.*)\./)[1..2]

  children = contain.split(',').map { |child| child.match(/(\w+ \w+) bag/)[1] }.map { |x| vertices[x] }.compact
  edge_values = contain.split(',').map { |x| x.match(/\d+/) }.compact.map { |x| x[0].to_i }

  children.each.with_index do |child, index|
    e = Edge.new(vertex: child, weight: edge_values[index] || 0)

    vertices[parent].edges << e
  end
end

# queue = ['shiny gold']

def part1(vertex)
  bag_colors = Set.new
  queue = [vertex]

  while queue.any?
    child = queue.shift

    child.edges.each do |edge|
      bag_colors.add(edge)
      queue << edge.vertex
    end
  end

  bag_colors.size
end


def part2(vertex)
  sum = 0
  vertex.edges.each do |edge|
    vertex_sum = traverse(edge.vertex)
    sum += edge.weight + (vertex_sum * edge.weight)
  end
  sum
end

binding.pry
puts part1(vertices['shiny gold'])
# puts part2(vertices['shiny gold'])


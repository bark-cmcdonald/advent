require 'pry'
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
end



shiny_gold = Vertex.new(name: 'shiny gold')
vibrant_plum = Vertex.new(name: 'vibrant plum')
dark_olive = Vertex.new(name: 'dark olive')
dotted_black = Vertex.new(name: 'dotted black')
faded_blue = Vertex.new(name: 'faded blue')

dark_olive.edges << Edge.new(vertex: dotted_black, weight: 4)
dark_olive.edges << Edge.new(vertex: faded_blue, weight: 3)
vibrant_plum.edges << Edge.new(vertex: faded_blue, weight: 5)
vibrant_plum.edges << Edge.new(vertex: dotted_black, weight: 6)
shiny_gold.edges << Edge.new(vertex: vibrant_plum, weight: 2)
shiny_gold.edges << Edge.new(vertex: dark_olive, weight: 1)


# Edge
#   Vertex
#   Weight
def traverse(vertex)
  sum = 0
  vertex.edges.each do |edge|
    vertex_sum = traverse(edge.vertex)
    sum += edge.weight + (vertex_sum * edge.weight)
  end
  sum
end

puts traverse(shiny_gold)

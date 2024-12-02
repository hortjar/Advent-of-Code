package cz.hortjar.aoc

import cz.hortjar.aoc.base.Base
import cz.hortjar.aoc.day01.Day01
import cz.hortjar.aoc.day02.Day02

fun main() {
  // TODO: get this from cli args
  val day = 2;
  var solver: Base = when(day) {
    1 -> Day01()
    else -> Day02()
  }

  val part1 = solver.solve01()
  val part2 = solver.solve02()

  println("Day $day")
  println("Part 1: $part1")
  println("Part 2: $part2")
}

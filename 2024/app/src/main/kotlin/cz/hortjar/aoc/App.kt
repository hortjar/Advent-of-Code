package cz.hortjar.aoc

import cz.hortjar.aoc.base.Base
import cz.hortjar.aoc.day01.Day01
import cz.hortjar.aoc.day02.Day02
import cz.hortjar.aoc.day03.Day03

fun main() {
  // TODO: get this from cli args
  val day = 3;
  var solver: Base = when (day) {
    1 -> Day01()
    2 -> Day02()
    else -> Day03()
  }

  println("Day $day")

  val part1 = solver.solve01()
  println("Part 1: $part1")

  val part2 = solver.solve02()
  println("Part 2: $part2")
}

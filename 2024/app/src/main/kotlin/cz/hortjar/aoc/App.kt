package cz.hortjar.aoc

import cz.hortjar.aoc.day01.Day01
import java.io.File
import java.nio.file.Paths

fun readInput(day: Int): List<String> {
  var paddedDay = "$day".padStart(2, '0')
  var path = Paths.get("").toAbsolutePath().toString()

  return File("$path/app/src/main/kotlin/cz/hortjar/aoc/day$paddedDay/input").readLines()
}

fun main() {
  var d01 = Day01()
  d01.solve01()
  d01.solve02()
}

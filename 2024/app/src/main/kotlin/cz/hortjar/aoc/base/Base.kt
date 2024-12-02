package cz.hortjar.aoc.base

import java.io.File
import java.nio.file.Paths

abstract class Base(day: Int) {
  var input: List<String> = emptyList()

  init {
    var paddedDay = "$day".padStart(2, '0')
    var path = Paths.get("").toAbsolutePath().toString()

    input = File("$path/app/src/main/kotlin/cz/hortjar/aoc/day$paddedDay/input").readLines()
  }

  abstract fun solve01(): Int
  abstract fun solve02(): Int
}
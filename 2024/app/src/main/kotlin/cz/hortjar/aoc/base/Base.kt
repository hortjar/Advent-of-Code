package cz.hortjar.aoc.base

import java.io.File
import java.nio.file.Paths

abstract class Base(day: Int, debug: Boolean = false) {
  var input: List<String> = emptyList()

  init {
    val paddedDay = "$day".padStart(2, '0')
    val path = Paths.get("").toAbsolutePath().toString()
    val suffix = if (debug) "_test" else ""

    input = File("$path/app/src/main/kotlin/cz/hortjar/aoc/day$paddedDay/input$suffix").readLines()
  }

  abstract fun solve01(): Int
  abstract fun solve02(): Int
}
package cz.hortjar.aoc.day02

import cz.hortjar.aoc.base.Base
import kotlin.math.abs

class Day02 : Base(2) {
  override fun solve01(): Int {
    val safeLines = this.input.fold(0) { acc, line ->
      val splitLine = line.split(" ")
      var lineArray: List<Int> = emptyList()
      for (i in 0..<splitLine.size - 1) {
        val num = splitLine[i].toInt()
        val nextNum = splitLine[i + 1].toInt()

        lineArray += (num - nextNum)
      }

      val hasIllegalDifference = lineArray.count { num -> num > 3 || num < -3 || num == 0 } > 0
      val hasPositive = lineArray.count { num -> num > 0 } > 0
      val hasNegative = lineArray.count { num -> num < 0 } > 0

      val res = !hasIllegalDifference && hasPositive xor hasNegative
      
      if (res) acc + 1 else acc
    }

    return safeLines
  }

  override fun solve02(): Int {
    return 0
  }
}
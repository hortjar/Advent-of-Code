package cz.hortjar.aoc.day01

import cz.hortjar.aoc.base.Base
import kotlin.math.abs

class Day01: Base(1) {
  private fun getIntArraySorted(lines: List<String>): List<List<Int>> {
    val intArray = lines.fold(listOf(listOf<Int>(), listOf<Int>())) { acc, line ->
      val pair = line.replace("   ", " ") .split(' ')
      listOf(acc[0] + (pair[0].toInt()), acc[1] + (pair[1].toInt()))
    }
    return listOf(intArray[0].sorted(), intArray[1].sorted());
  }

  override fun solve01(): Int {
    val intArray = getIntArraySorted(this.input)

    var result: Int = 0
    for (i in 0..<intArray[0].size){
      result += abs(intArray[0][i] - intArray[1][i])
    }

    return result
  }

  override fun solve02(): Int {
    val intArray = getIntArraySorted(this.input)

    var result: Int = 0
    for (leftNum in intArray[0]) {
      val rightCount = intArray[1].count { num -> num == leftNum }

      result += (leftNum * rightCount)
    }

    return result
  }
}
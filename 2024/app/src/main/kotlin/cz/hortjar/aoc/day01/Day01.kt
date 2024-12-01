package cz.hortjar.aoc.day01

import cz.hortjar.aoc.readInput
import java.io.File
import kotlin.math.abs

class Day01 {
  private fun getIntArraySorted(lines: List<String>): List<List<Int>> {
    val intArray = lines.fold(listOf(listOf<Int>(), listOf<Int>())) { acc, line ->
      val pair = line.replace("   ", " ") .split(' ')
      listOf(acc[0] + (pair[0].toInt()), acc[1] + (pair[1].toInt()))
    }
    return listOf(intArray[0].sorted(), intArray[1].sorted());
  }

  fun solve01() {
    val lines = readInput(1)

    val intArray = getIntArraySorted(lines)

    var result: Int = 0
    for (i in 0..<intArray[0].size){
      result += abs(intArray[0][i] - intArray[1][i])
    }

    println(result)
  }

  fun solve02(){
    val lines = readInput(1)

    val intArray = getIntArraySorted(lines)

    var result: Int = 0
    for (leftNum in intArray[0]) {
      val rightCount = intArray[1].count { num -> num == leftNum }

      result += (leftNum * rightCount)
    }

    println(result)
  }
}
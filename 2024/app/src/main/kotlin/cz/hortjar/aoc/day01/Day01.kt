package cz.hortjar.aoc.day01

import cz.hortjar.aoc.readInput
import java.io.File
import kotlin.math.abs

class Day01 {
  private fun getIntArraySorted(lines: List<String>): MutableList<MutableList<Int>> {
    var intArray = lines.fold(mutableListOf<MutableList<Int>>()) { acc, line ->
      if (acc.size == 0) {
        acc.add(mutableListOf())
        acc.add(mutableListOf())
      }
      val pair = line.replace("   ", " ") .split(' ')
      acc[0].add(pair[0].toInt())
      acc[1].add(pair[1].toInt())
      acc
    }

    intArray[0].sort()
    intArray[1].sort()

    return intArray;
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
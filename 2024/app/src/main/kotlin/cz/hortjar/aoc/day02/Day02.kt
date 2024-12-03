package cz.hortjar.aoc.day02

import cz.hortjar.aoc.base.Base

class Day02 : Base(2) {
  private fun getNumbers(): List<List<Int>> {
    return this.input.map { item ->
      item.split(" ").map { number ->
        number.toInt()
      }
    }
  }

  private fun generateMissingCombinations(input: List<Int>): List<List<Int>> {
    return input.indices.map { index ->
      input.filterIndexed { i, _ -> i != index }
    }
  }

  private fun isLineSafe(line: List<Int>): Boolean {
    var lineDiff: List<Int> = emptyList()
    for (i in 0..<line.size - 1) {
      val num = line[i]
      val nextNum = line[i + 1]

      lineDiff += (num - nextNum)
    }

    val hasIllegalDifference = lineDiff.count { num -> num > 3 || num < -3 || num == 0 } > 0
    val hasPositive = lineDiff.count { num -> num > 0 } > 0
    val hasNegative = lineDiff.count { num -> num < 0 } > 0

    return !hasIllegalDifference && hasPositive xor hasNegative
  }

  override fun solve01(): Int {
    val safeLines = getNumbers().fold(0) { acc, line ->
      if (isLineSafe(line)) acc + 1 else acc
    }

    return safeLines
  }

  override fun solve02(): Int {
    val safeLines = getNumbers().fold(0) { acc, line ->
      val combinations = generateMissingCombinations(line)
      val safe = combinations.fold(0) { combinationAcc, combination ->
        if (isLineSafe(combination)) combinationAcc + 1 else combinationAcc
      }
      if (safe > 0) acc + 1 else acc
    }

    return safeLines
  }
}
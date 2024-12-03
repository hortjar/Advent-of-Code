package cz.hortjar.aoc.day03

import cz.hortjar.aoc.base.Base

class Day03 : Base(3) {
  private val mulRegex = """(mul\(\d+,\d+\))""".toRegex()
  private val mulRegexPart2 = """(do\(\)|don't\(\))|(mul\(\d+,\d+\))""".toRegex()
  private val numbersRegex = """(\d+,\d+)""".toRegex()

  override fun solve01(): Int {
    val result =
      mulRegex.findAll(this.input.joinToString()).map { mul ->
        numbersRegex.find(mul.value)?.value
      }.filter { value -> value != null }
        .fold(0) { acc, value ->
          val split = value!!.split(',')
          acc + (split[0].toInt() * split[1].toInt())
        }
    return result;
  }

  override fun solve02(): Int {
    var skipMultiply = false
    val result =
      mulRegexPart2.findAll(this.input.joinToString()).map { mul ->
        mul.value
      }.fold(0) { acc, value ->
        var res = acc;
        if (value == "don't()") {
          skipMultiply = true
        } else if (value == "do()") {
          skipMultiply = false
        } else if (!skipMultiply) {
          val split = numbersRegex.find(value)!!.value.split(',')
          res += (split[0].toInt() * split[1].toInt())
        }
        res
      }
    return result;
  }
}
import kotlin.math.abs

fun readInput(): List<String> {
    var list: MutableList<String> = mutableListOf()
    var currentLine: String
    while (true) {
        currentLine = readlnOrNull() ?: break
        list.add(currentLine)
    }
    return list
}

fun main() {
    val listOfLines = readInput()
    var leftColumn: MutableList<String> = mutableListOf()
    var rightColumn: MutableList<String> = mutableListOf()

    var tokens: List<String> = mutableListOf()
    for (line in listOfLines) {
        tokens = line.split(" ")

        leftColumn.add(tokens.first())
        rightColumn.add(tokens.last())
    }

    leftColumn.sort()
    rightColumn.sort()

    var sumOfDistances: Int = 0;

    for (i in 0..leftColumn.size-1) {
        sumOfDistances += abs(leftColumn[i].toInt() - rightColumn[i].toInt())
    }

    println("sumOfDistances: $sumOfDistances")
}
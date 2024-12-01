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

fun searchForSimilarity(number: String, column: List<String>): Int {
    var appearances = 0;
    for (elem in column) {
        if (number == elem) {appearances += 1}
    }

    return appearances * number.toInt()
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


    var sumOfSimilarities: Int = 0;
    var cacheOfSimilarity: MutableMap<String, Int> = mutableMapOf()

    for (i in 0..leftColumn.size-1) {
        val cachedScore: Int? = cacheOfSimilarity.get(leftColumn[i]);
        if (cachedScore != null) {
            sumOfSimilarities += cachedScore;
        } else {
            val score: Int = searchForSimilarity(leftColumn[i], rightColumn)
            cacheOfSimilarity.put(leftColumn[i], score);
            sumOfSimilarities += score;
        }
    }

    println("sumOfSimilarities: $sumOfSimilarities")
}
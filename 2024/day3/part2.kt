fun readInput(): List<String> {
    var list: MutableList<String> = mutableListOf()
    var currentLine: String
    while (true) {
        currentLine = readlnOrNull() ?: break
        list.add(currentLine.trim())
    }
    return list
}

fun getPartialSum(line: String): Int {
    var partialSumOfMul: Int = 0
    
    val regex = Regex("mul\\([0-9]{1,3},[0-9]{1,3}\\)")
    val matches = regex.findAll(line)
    for (match in matches) { 
        var firstInt = match.value.split(",")[0].replace("mul(", "").toInt();
        var secondInt = match.value.split(",")[1].replace(")", "").toInt();

        partialSumOfMul += firstInt * secondInt
    }

    return partialSumOfMul
}

fun main() {
    val listOfLines = readInput()

    var sumOfMul: Int = 0;

    var startsEnabled = true
    for (line in listOfLines) {
        var myline = if (startsEnabled) "do()" + line else "don't()" + line
        var dontTokens = myline.split("don't()")
        for (dontToken in dontTokens) {
            var doTokens: MutableList<String> = dontToken.split("do()").toMutableList()
            if (doTokens.size == 1) continue
            doTokens.removeAt(0)
            sumOfMul += getPartialSum(doTokens.joinToString())
        }

        startsEnabled = if (dontTokens.last().contains("do()")) true else false
    }

    println("Sum of muls: $sumOfMul")
}
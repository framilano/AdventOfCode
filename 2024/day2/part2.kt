
fun readInput(): List<String> {
    var list: MutableList<String> = mutableListOf()
    var currentLine: String
    while (true) {
        currentLine = readlnOrNull() ?: break
        list.add(currentLine.trim())
    }
    return list
}

fun checkSafety(isIncreasing: Boolean, tokens: List<String>): Boolean {
    var difference = 0
    for (index in 1..tokens.size - 1) {
        difference = tokens[index-1].toInt() - tokens[index].toInt()
        if (!isIncreasing && difference >= 1 && difference <= 3) {continue}
        else if (isIncreasing && difference <= -1 && difference >= -3) {continue}
        else {
            return false
        }
    }
    return true
}

fun checkSlop(tokens: List<String>): Boolean {
    for (index in 1..tokens.size-1) {
        if (tokens[index-1].toInt() < tokens[index].toInt()) return true;
        if (tokens[index-1].toInt() > tokens[index].toInt()) return false;
        if (tokens[index-1].toInt() == tokens[index].toInt()) continue;
    }
    return true
}

fun main() {
    val listOfLines = readInput()

    var numberOfSafeReports: Int = 0
    
    for (line in listOfLines) {
        val originalTokens: MutableList<String> = line.split(" ").toMutableList();
        var isIncreasing = checkSlop(originalTokens)
        
        var isSafe = checkSafety(isIncreasing, originalTokens)
        
        if (isSafe) {
            numberOfSafeReports += 1
        } else {
            for (index in 0..originalTokens.size-1) {
                val newTokens = originalTokens.toMutableList()
                newTokens.removeAt(index)
                isIncreasing = checkSlop(newTokens)
                if (checkSafety(isIncreasing, newTokens)) {
                    numberOfSafeReports += 1
                    break;
                }
            }
        }
    }

    println("Number of safe reports: $numberOfSafeReports")
}
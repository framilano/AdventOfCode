fun readInput(): List<String> {
    var list: MutableList<String> = mutableListOf()
    var currentLine: String
    while (true) {
        currentLine = readlnOrNull() ?: break
        list.add(currentLine.trim())
    }
    return list
}

fun main() {
    val listOfLines = readInput()

    var numberOfSafeReports: Int = 0

    var increasing = false;
    for (line in listOfLines) {
        val tokens = line.split(" ");
        if (tokens[0].toInt() < tokens[1].toInt()) {increasing = true}
        else {increasing = false}
        
        var difference = 0
        var safe = true
        for (index in 1..tokens.size - 1) {
            difference = tokens[index-1].toInt() - tokens[index].toInt()
            if (!increasing && difference >= 1 && difference <= 3) {continue}
            else if (increasing && difference <= -1 && difference >= -3) {continue}
            else safe = false;
        }
        if (safe) numberOfSafeReports += 1
    }

    println("Number of safe reports: $numberOfSafeReports")
}
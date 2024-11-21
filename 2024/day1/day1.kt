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
    for (line in listOfLines) {
        println(line)
    }
}
fun readInput(): List<String> {
    var list: MutableList<String> = mutableListOf()
    var currentLine: String
    while (true) {
        currentLine = readlnOrNull() ?: break
        list.add(currentLine.trim())
    }
    return list
}

fun readMatrixInput(): List<List<Char>> {
    var list: MutableList<MutableList<Char>> = mutableListOf()
    var currentLine: String
    while (true) {
        currentLine = readlnOrNull() ?: break
        var newRow: MutableList<Char> =  mutableListOf()
        for (elem in currentLine) {
            newRow.add(elem)
        }
        list.add(newRow)
    }
    return list
}

fun printMatrix(matrix: List<List<Char>>) {
    for (row in matrix) {
        for (column in row) {
            print(column)
        }
        println()
    }
}

fun main() {
    val listOfLines = readInput()
}
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

fun searchForXmasFromCell(rowIndex: Int, columnIndex: Int, matrixOfInput: List<List<Char>>): Int {
    var partialSumOfXmas: Int = 0

    //Search up
    try {
        if (matrixOfInput[rowIndex-1][columnIndex] == 'M' && 
            matrixOfInput[rowIndex-2][columnIndex] == 'A' && 
            matrixOfInput[rowIndex-3][columnIndex] == 'S') {partialSumOfXmas += 1}
    } catch (e: IndexOutOfBoundsException) {}

    //Search down
    try {
        if (matrixOfInput[rowIndex+1][columnIndex] == 'M' && 
            matrixOfInput[rowIndex+2][columnIndex] == 'A' && 
            matrixOfInput[rowIndex+3][columnIndex] == 'S') {partialSumOfXmas += 1}
    } catch (e: IndexOutOfBoundsException) {}

    //Search left
    try {
        if (matrixOfInput[rowIndex][columnIndex-1] == 'M' && 
            matrixOfInput[rowIndex][columnIndex-2] == 'A' && 
            matrixOfInput[rowIndex][columnIndex-3] == 'S') {partialSumOfXmas += 1}
    } catch (e: IndexOutOfBoundsException) {}

    //Search right
    try {
        if (matrixOfInput[rowIndex][columnIndex+1] == 'M' && 
            matrixOfInput[rowIndex][columnIndex+2] == 'A' && 
            matrixOfInput[rowIndex][columnIndex+3] == 'S') {partialSumOfXmas += 1}
    } catch (e: IndexOutOfBoundsException) {}

    //Search up left
    try {
        if (matrixOfInput[rowIndex-1][columnIndex-1] == 'M' && 
            matrixOfInput[rowIndex-2][columnIndex-2] == 'A' && 
            matrixOfInput[rowIndex-3][columnIndex-3] == 'S') {partialSumOfXmas += 1}
    } catch (e: IndexOutOfBoundsException) {}

    //Search up right
    try {
        if (matrixOfInput[rowIndex-1][columnIndex+1] == 'M' && 
            matrixOfInput[rowIndex-2][columnIndex+2] == 'A' && 
            matrixOfInput[rowIndex-3][columnIndex+3] == 'S') {partialSumOfXmas += 1}
    } catch (e: IndexOutOfBoundsException) {}

    //Search down left
    try {
        if (matrixOfInput[rowIndex+1][columnIndex-1] == 'M' && 
            matrixOfInput[rowIndex+2][columnIndex-2] == 'A' && 
            matrixOfInput[rowIndex+3][columnIndex-3] == 'S') {partialSumOfXmas += 1}
    } catch (e: IndexOutOfBoundsException) {}

    //Search down right
    try {
        if (matrixOfInput[rowIndex+1][columnIndex+1] == 'M' && 
            matrixOfInput[rowIndex+2][columnIndex+2] == 'A' && 
            matrixOfInput[rowIndex+3][columnIndex+3] == 'S') {partialSumOfXmas += 1}
    } catch (e: IndexOutOfBoundsException) {}

    return partialSumOfXmas
}

fun main() {
    val matrixOfInput = readMatrixInput()

    var sumOfXmas: Int = 0

    for ((rowIndex, rowValue) in matrixOfInput.withIndex()) {
        for ((columnIndex, columnValue) in rowValue.withIndex()) {
            if (columnValue == 'X') {sumOfXmas += searchForXmasFromCell(rowIndex, columnIndex, matrixOfInput)}
        }
    }

    println("SumOfXmas = $sumOfXmas")
}
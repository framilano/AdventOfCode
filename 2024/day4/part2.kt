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

fun searchForMasCrossFromCell(rowIndex: Int, columnIndex: Int, matrixOfInput: List<List<Char>>): Int {
    var partialSumOfMasCross: Int = 0

    //Search up
    try {
        if (
            (
                (matrixOfInput[rowIndex-1][columnIndex-1] == 'M' && 
                matrixOfInput[rowIndex+1][columnIndex+1] == 'S') || 
                (matrixOfInput[rowIndex-1][columnIndex-1] == 'S' && 
                matrixOfInput[rowIndex+1][columnIndex+1] == 'M')
            ) &&
            (
                (matrixOfInput[rowIndex-1][columnIndex+1] == 'M' && 
                matrixOfInput[rowIndex+1][columnIndex-1] == 'S') || 
                (matrixOfInput[rowIndex-1][columnIndex+1] == 'S' && 
                matrixOfInput[rowIndex+1][columnIndex-1] == 'M')
            )
        ) {partialSumOfMasCross += 1}
    } catch (e: IndexOutOfBoundsException) {}

    return partialSumOfMasCross
}

fun main() {
    val matrixOfInput = readMatrixInput()

    var sumOfMasCross: Int = 0

    for ((rowIndex, rowValue) in matrixOfInput.withIndex()) {
        for ((columnIndex, columnValue) in rowValue.withIndex()) {
            if (columnValue == 'A') {sumOfMasCross += searchForMasCrossFromCell(rowIndex, columnIndex, matrixOfInput)}
        }
    }

    println("SumOfMasCross = $sumOfMasCross")
}
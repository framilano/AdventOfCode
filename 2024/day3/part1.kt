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

    var sumOfMul: Int = 0;

    val regex = Regex("mul\\([0-9]{1,3},[0-9]{1,3}\\)")
    for (line in listOfLines) {
        val matches = regex.findAll(line)
        for (match in matches) { 
            var firstInt = match.value.split(",")[0].replace("mul(", "").toInt();
            var secondInt = match.value.split(",")[1].replace(")", "").toInt();

            sumOfMul += firstInt * secondInt
        }
    }

    println("Sum of muls: $sumOfMul")
}

fun readInput(): List<String> {
    var list: MutableList<String> = mutableListOf()
    var currentLine: String
    while (true) {
        currentLine = readlnOrNull() ?: break
        list.add(currentLine.trim())
    }
    return list
}

fun getValidMiddles(update: List<Int>, listOfOrderingRules: MutableMap<Int, List<Int>>): Int {
    for (index in 0..update.size-1) {
        var key = update[index]
        if (listOfOrderingRules[key] == null) continue
        for (index2 in 0..index) {
            if (listOfOrderingRules[key]!!.contains(update[index2])) {return 0}
        }
    }

    return update[update.size / 2]
}

fun main() {
    val listOfLines = readInput()
    val listOfOrderingRules: MutableMap<Int, List<Int>> = mutableMapOf()
    val listOfUpdates: MutableList<List<Int>> = mutableListOf()

    var fillOrder = true
    for (line in listOfLines) {
        if (line == "\n" || line == "") {fillOrder = false; continue}

        if (fillOrder) {
            var first = line.split("|")[0].toInt();
            var second = line.split("|")[1].toInt()
            listOfOrderingRules[first] = if (listOfOrderingRules[first] != null) listOfOrderingRules[first]!!.plus(second) else listOf(second)
        } else {
            listOfUpdates.add(line.split(",").map {it.toInt()})
        }
    }

    var sumOfMiddles: Int = 0

    for (update in listOfUpdates) {
        sumOfMiddles += getValidMiddles(update, listOfOrderingRules)
    }

    println("Sum of middles: $sumOfMiddles")
}
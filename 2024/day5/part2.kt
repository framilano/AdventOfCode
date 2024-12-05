fun readInput(): List<String> {
    var list: MutableList<String> = mutableListOf()
    var currentLine: String
    while (true) {
        currentLine = readlnOrNull() ?: break
        list.add(currentLine.trim())
    }
    return list
}

fun getValidMiddles(update: MutableList<Int>, listOfOrderingRules: MutableMap<Int, List<Int>>): Int {
    var temp = 0
    var key = 0
    var changed = false
    for (index in 0..update.size-1) {
        key = update[index]
        if (listOfOrderingRules[key] == null) continue
        for (index2 in 0..index) {
            if (listOfOrderingRules[key]!!.contains(update[index2])) {
                changed = true
                temp = update[index]
                update[index] = update[index2]
                update[index2] = temp
            }
        }
    }

    if (changed) return update[update.size / 2]
    else return 0
}

fun main() {
    val listOfLines = readInput()
    val listOfOrderingRules: MutableMap<Int, List<Int>> = mutableMapOf()
    val listOfUpdates: MutableList<MutableList<Int>> = mutableListOf()

    var fillOrder = true
    for (line in listOfLines) {
        if (line == "\n" || line == "") {fillOrder = false; continue}

        if (fillOrder) {
            var first = line.split("|")[0].toInt();
            var second = line.split("|")[1].toInt()
            listOfOrderingRules[first] = if (listOfOrderingRules[first] != null) listOfOrderingRules[first]!!.plus(second) else listOf(second)
        } else {
            listOfUpdates.add(line.split(",").map {it.toInt()}.toMutableList())
        }
    }

    var sumOfMiddles: Int = 0

    for (update in listOfUpdates) {
        sumOfMiddles += getValidMiddles(update, listOfOrderingRules)
    }

    println("Sum of middles: $sumOfMiddles")
}
package bigsquarekotlin.app

fun main() {
    val four = 1.run {
        plus(3)
        toString()
    }
    println("Four is $four")

    val five = 1.let {
        it.plus(4)
    }
    println("Five is $five")

    val goo = "Woohoo".apply {
        plus("Goo")
        21
    }
    println("Goo is $goo")
    data class Coord(var y : Int = 0, var x : Int = 0)

    val allSoe = Coord().also {
        it.y = 1
        it.x = 2
    }
    println("Allsoe is $allSoe")
}
# Kotlin for AoC 2024 🎅

## Compile
`kotlinc .\hello_world.kt -include-runtime -d hello_world.jar`
## Run
`java -jar hello_world.jar`
## Compile and run current folder file (assuming foldername is the same as .kt filename)
`kotlinc .\%CD%.kt -include-runtime -d %CD%.jar && java -jar %CD%.jar`
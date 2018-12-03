const gettingStarted = require("./gettingStarted")
const anoncredsRevocation = require("./anoncredsRevocation")

run()

async function run() {
    await gettingStarted.run()
    await anoncredsRevocation.run()
}
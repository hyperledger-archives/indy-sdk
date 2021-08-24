const gettingStarted = require("./gettingStarted")
const anoncredsRevocation = require("./anoncredsRevocation")

run()

async function run() {
    console.log("starting....");
    await gettingStarted.run()
    await anoncredsRevocation.run()
}
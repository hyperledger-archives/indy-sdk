const commandLineArgs = require('command-line-args')
const commandLineUsage = require('command-line-usage')

module.exports.runScript = async function runScript (optionDefinitions, usageDefinition, areOptionsValid, runFunction) {
  const usage = commandLineUsage(usageDefinition)

  let options = {}
  try {
    options = commandLineArgs(optionDefinitions)
  } catch (error) {
    console.error('Error parsing arguments')
    console.error(error)
    console.log(usage)
    return
  }
  if (options.help) {
    console.error('Help requested.')
    console.log(usage)
    return
  }
  if (!areOptionsValid(options)) {
    console.error('Invalid options.')
    console.log(usage)
    return
  }
  await runFunction(options)
}

const express = require('express')
const bodyParser = require('body-parser')

const PORT = 7209

const app = express()
app.use(bodyParser.json())

const FgRed = '\x1b[31m'
const FgGreen = '\x1b[32m'
const FgYellow = '\x1b[33m'
const FgBlue = '\x1b[34m'
const FgMagenta = '\x1b[35m'
const FgCyan = '\x1b[36m'
const FgWhite = '\x1b[37m'

const colors = [FgRed, FgGreen, FgYellow, FgBlue, FgMagenta, FgCyan, FgWhite]
let colorIdx = 0

const agentColors = {}

function getAgentColor (agentId) {
  if (!agentColors[agentId]) {
    agentColors[agentId] = colors[colorIdx]
    colorIdx = (colorIdx + 1) % colors.length
  }
  return agentColors[agentId]
}

async function run () {
  const notifications = {}

  app.post('/notifications/:agentId', async function (req, res) {
    const { agentId } = req.params
    console.log(getAgentColor(agentId), `${new Date()}] ${agentId}: ${JSON.stringify(req.body, null, 2)}`)
    if (!notifications[agentId]) {
      notifications[agentId] = []
    }
    notifications[agentId].push(req.body)
    return res.status(200).send()
  })

  app.get('/notifications', async function (req, res) {
    return res.status(200).send(JSON.stringify(notifications))
  })

  app.use(function (req, res, next) {
    console.error(`Request ${req.method} '${req.originalUrl}' was not matched with any handler.\nRequest header:${JSON.stringify(req.headers, null, 2)}\nRequest body: ${JSON.stringify(req.body, null, 2)}`)
    res.status(404).send({ message: `Your request: '${req.originalUrl}' didn't reach any handler.` })
  })

  app.listen(PORT, () => console.log(`Server listening on port ${PORT}!`))
}

run()

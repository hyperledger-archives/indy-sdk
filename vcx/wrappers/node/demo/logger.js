const { createLogger, format, transports } = require('winston')
const { label } = format

const prettyFormatter = format.combine(
  format.printf(
    info => `${info.label} [${info.level}]: ${info.message}`
  )
)

const logger = createLogger({
  level: 'debug',
  format: format.combine(
    label({ label: 'VCX Node demo:' }),
    format.colorize({ all: true }),
    prettyFormatter
  ),
  transports: [
    new transports.Console()
  ]
})

module.exports = logger

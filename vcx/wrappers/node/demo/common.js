const { initRustAPI, initVcxWithConfig, provisionAgent } = require('./../dist/src')
const ffi = require('ffi')

async function loadPostgresPlugin (provisionConfig) {
  const myffi = ffi.Library('/usr/local/lib/libindystrgpostgres.dylib', { postgresstorage_init: ['void', []] })
  await myffi.postgresstorage_init()
}

async function initLibNullPay () {
  const myffi = ffi.Library('/usr/local/lib/libnullpay.dylib', { nullpay_init: ['void', []] })
  await myffi.nullpay_init()
}

async function initRustApiAndLogger (logLevel) {
  const rustApi = initRustAPI()
  await rustApi.vcx_set_default_logger(logLevel)
}

async function provisionAgentInAgency (config) {
  return JSON.parse(await provisionAgent(JSON.stringify(config)))
}

async function initVcxWithProvisionedAgentConfig (config) {
  config.institution_name = 'faber'
  config.institution_logo_url = 'http://robohash.org/234'
  config.genesis_path = `${__dirname}/docker.txn`
  await initVcxWithConfig(JSON.stringify(config))
}

function getRandomInt (min, max) {
  min = Math.ceil(min)
  max = Math.floor(max)
  return Math.floor(Math.random() * (max - min)) + min
}

module.exports.loadPostgresPlugin = loadPostgresPlugin
module.exports.initLibNullPay = initLibNullPay
module.exports.initRustApiAndLogger = initRustApiAndLogger
module.exports.provisionAgentInAgency = provisionAgentInAgency
module.exports.initVcxWithProvisionedAgentConfig = initVcxWithProvisionedAgentConfig
module.exports.getRandomInt = getRandomInt

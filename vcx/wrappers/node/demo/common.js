import {initRustAPI, initVcxWithConfig, provisionAgent} from "./../dist/src";
import * as ffi from "ffi";
import * as os from 'os'
import 'fs';

export async function loadPostgresPlugin (provisionConfig) {
    const platform = os.platform()
    const libPath = platform === "darwin" ? '/usr/local/lib/libindystrgpostgres.dylib' : '/usr/lib/libindystrgpostgres.so'
    const myffi = ffi.Library(libPath, {postgresstorage_init: ['void', []]});
    await myffi.postgresstorage_init()
}

export async function initLibNullPay() {
    const platform = os.platform()
    const libPath = platform === "darwin" ? '/usr/local/lib/libnullpay.dylib' : '/usr/lib/libnullpay.so'
    const myffi = ffi.Library(libPath, {nullpay_init: ['void', []]});
    await myffi.nullpay_init();
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

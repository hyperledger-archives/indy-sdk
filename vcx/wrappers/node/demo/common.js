import {initRustAPI, initVcxWithConfig, provisionAgent} from "./../dist/src";
import * as ffi from "ffi";
import 'fs';

export async function initLibNullPay() {
    const myffi = ffi.Library('/usr/local/lib/libnullpay.dylib', {nullpay_init: ['void', []]});
    await myffi.nullpay_init();
}

export async function initRustApiAndLogger(logLevel) {
    let rustApi = initRustAPI();
    await rustApi.vcx_set_default_logger(logLevel);
}

export async function provisionAgentInAgency(config) {
    return JSON.parse(await provisionAgent(JSON.stringify(config)));
}

export async function initVcxWithProvisionedAgentConfig(config) {
    config['institution_name'] = 'faber';
    config['institution_logo_url'] = 'http://robohash.org/234';
    config['genesis_path'] = `${__dirname}/docker.txn` ;
    await initVcxWithConfig(JSON.stringify(config));
}

export function getRandomInt(min, max) {
    min = Math.ceil(min);
    max = Math.floor(max);
    return Math.floor(Math.random() * (max - min)) + min;
}

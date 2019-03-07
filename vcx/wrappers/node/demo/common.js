import {initRustAPI, initVcxWithConfig, provisionAgent} from "./../dist/src";
import * as ffi from "ffi";

export async function initLibNullPay() {
    const myffi = ffi.Library('/usr/local/lib/libnullpay.dylib', {nullpay_init: ['void', []]});
    await myffi.nullpay_init();
}

export async function initRustApiAndLogger() {
    let rustApi = initRustAPI();
    await rustApi.vcx_set_default_logger("info");
}

export async function provisionAgentInAgency(config) {
    return JSON.parse(await provisionAgent(JSON.stringify(config)));
}

export async function initVcxWithProvisionedAgentConfig(config) {
    // Set some additional configuration options specific to Faber
    config['institution_name'] = 'faber';
    config['institution_logo_url'] = 'http://robohash.org/234';
    config['genesis_path'] = 'docker.txn';
    await initVcxWithConfig(JSON.stringify(config));
}

export async function getRandomInt(min, max) {
    min = Math.ceil(min);
    max = Math.floor(max);
    return Math.floor(Math.random() * (max - min)) + min;
}

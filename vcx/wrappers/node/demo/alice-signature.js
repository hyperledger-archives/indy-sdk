import {DisclosedProof} from "../dist/src/api/disclosed-proof";
import {Connection} from "../dist/src/api/connection";
import {Credential} from "../dist/src/api/credential";
import {StateType} from "../dist/src";
import readlineSync from 'readline-sync'
import sleepPromise from 'sleep-promise'
import * as demoCommon from './common'
import logger from './logger'

const utime = Math.floor(new Date() / 1000);

const provisionConfig = {
    'agency_url': 'http://localhost:8080',
    'agency_did': 'VsKV7grR1BUE29mG2Fm2kX',
    'agency_verkey': 'Hezce2UWMZ3wUhVkh2LfKSs8nDzWwzs2Win7EzNN3YaR',
    'wallet_name': `node_vcx_demo_alice_wallet_${utime}`,
    'wallet_key': '123',
    'payment_method': 'null',
    'enterprise_seed': '000000000000000000000000Trustee1'
};

const logLevel = 'warn';

async function run() {
    await demoCommon.initLibNullPay();

    logger.info("#0 initialize rust API from NodeJS");
    await demoCommon.initRustApiAndLogger(logLevel);

    logger.info("#1 Provision an agent and wallet, get back configuration details");
    let config = await demoCommon.provisionAgentInAgency(provisionConfig);

    logger.info("#2 Initialize libvcx with new configuration");
    await demoCommon.initVcxWithProvisionedAgentConfig(config);

    logger.info("#9 Input faber.py invitation details");
    const details = readlineSync.question('Enter your invite details: ');
    const jdetails = JSON.parse(details);

    logger.info("#10 Convert to valid json and string and create a connection to faber");
    const connection_to_faber = await Connection.createWithInvite({id: 'faber', invite: JSON.stringify(jdetails)});
    await connection_to_faber.connect({data: '{"use_public_did": true}'});
    await connection_to_faber.updateState();
    const dataUtf8 = 'I am Alice, trust me.';

    logger.info("#10.1 Alice signing some data using PW Verkey.");
    var dataBuffer = Buffer.from(dataUtf8, 'utf8');
    let signatureBuffer = undefined;
    try {
        signatureBuffer = await connection_to_faber.signData(dataBuffer);
    } catch (error) {
        logger.error(error);
        logger.error(error.stack);
        throw Error(`Error occured while connection ${id} was signing data '${dataBase64}'.`)
    }
    if (!signatureBuffer) {
        throw Error(`Error occured while connection ${id} was signing data '${dataBase64}' The resulting signature was empty.`)
    }
    const signatureBase64 = signatureBuffer.toString('base64');
    logger.info(`#10.2 Alice produced signature. Paste following to faber verification script::\n${JSON.stringify({signatureBase64, dataUtf8})}\n`);
}


run();
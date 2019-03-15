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

    logger.info("#11 Wait for faber.py to issue a credential offer");
    await sleepPromise(5000);
    const offers = await Credential.getOffers(connection_to_faber);
    logger.info(`Alice found ${offers.length} credential offers.`);
    logger.debug(JSON.stringify(offers));

    // Create a credential object from the credential offer
    const credential = await Credential.create({sourceId: 'credential', offer: JSON.stringify(offers[0])});

    logger.info("#15 After receiving credential offer, send credential request");
    await credential.sendRequest({connection: connection_to_faber, payment : 0});

    logger.info("#16 Poll agency and accept credential offer from faber");
    let credential_state = await credential.getState();
    while (credential_state !== StateType.Accepted) {
        sleepPromise(2000);
        await credential.updateState();
        credential_state = await credential.getState();
    }

    logger.info("#22 Poll agency for a proof request");
    const requests = await DisclosedProof.getRequests(connection_to_faber);

    logger.info("#23 Create a Disclosed proof object from proof request");
    const proof = await DisclosedProof.create({sourceId: 'proof', request: JSON.stringify(requests[0])});

    logger.info("#24 Query for credentials in the wallet that satisfy the proof request");
    const credentials = await proof.getCredentials();

    // Use the first available credentials to satisfy the proof request
    for (let i = 0; i < Object.keys(credentials['attrs']).length; i++) {
        const attr = Object.keys(credentials['attrs'])[i];
        credentials['attrs'][attr] = {
            'credential': credentials['attrs'][attr][0]
        }
    }

    logger.info("#25 Generate the proof");
    await proof.generateProof({selectedCreds: credentials, selfAttestedAttrs: {}});

    logger.info("#26 Send the proof to faber");
    await proof.sendProof(connection_to_faber);
}


run();
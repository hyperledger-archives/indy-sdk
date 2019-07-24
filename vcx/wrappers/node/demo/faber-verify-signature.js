import {CredentialDef} from "../dist/src/api/credential-def";
import {IssuerCredential} from "../dist/src/api/issuer-credential";
import {Proof} from "../dist/src/api/proof";
import {Connection} from "../dist/src/api/connection";
import {Schema} from "./../dist/src/api/schema";
import {StateType, ProofState} from "../dist/src";
import sleepPromise from 'sleep-promise'
import * as demoCommon from "./common";
import {getRandomInt} from "./common";
import logger from './logger'
import readlineSync from "readline-sync";

const utime = Math.floor(new Date() / 1000);

const provisionConfig = {
    'agency_url': 'http://localhost:8080',
    'agency_did': 'VsKV7grR1BUE29mG2Fm2kX',
    'agency_verkey': 'Hezce2UWMZ3wUhVkh2LfKSs8nDzWwzs2Win7EzNN3YaR',
    'wallet_name': `node_vcx_demo_faber_wallet_${utime}`,
    'wallet_key': '123',
    'payment_method': 'null',
    'enterprise_seed': '000000000000000000000000Trustee1'
};

const logLevel = 'error';

async function run() {
    await demoCommon.initLibNullPay();

    logger.info("#0 initialize rust API from NodeJS");
    await demoCommon.initRustApiAndLogger(logLevel);

    logger.info("#1 Provision an agent and wallet, get back configuration details");
    logger.debug(`Config used to provision agent in agency: ${JSON.stringify(provisionConfig, null, 2)}`);
    let config = await demoCommon.provisionAgentInAgency(provisionConfig);

    logger.info("#2 Initialize libvcx with new configuration");
    logger.debug(`${JSON.stringify(config, null, 2)}`);
    await demoCommon.initVcxWithProvisionedAgentConfig(config);

    logger.info("#3 Create a connection to alice and print out the invite details");
    const connectionToAlice = await Connection.create({id: 'alice'});
    await connectionToAlice.connect('{"use_public_did": true}');
    await connectionToAlice.updateState();
    const details = await connectionToAlice.inviteDetails(false);
    logger.info("\n\n**invite details**");
    logger.info("**You'll ge queried to paste this data to alice side of the demo. This is invitation to connect.**");
    logger.info("**It's assumed this is obtained by Alice from Faber by some existing secure channel.**");
    logger.info("**Could be on website via HTTPS, QR code scanned at Faber institution, ...**");
    logger.info("\n******************\n\n");
    logger.info(JSON.stringify(JSON.parse(details)));
    logger.info("\n\n******************\n\n");

    logger.info("#4 Polling agency and waiting for alice to accept the invitation. (start alice.py now)");
    let connection_state = await connectionToAlice.getState();
    while (connection_state !== StateType.Accepted) {
        await sleepPromise(2000);
        await connectionToAlice.updateState();
        connection_state = await connectionToAlice.getState();
    }
    logger.info(`Connection to alice was Accepted!`);


    logger.info("#9 Input faber.py invitation details");
    const signedData = readlineSync.question('Enter data signed by alice:');
    const {signatureBase64, dataUtf8} = JSON.parse(signedData);
    var data = Buffer.from(dataUtf8, 'utf8');
    var signature = Buffer.from(signatureBase64, 'base64');
    const isVerified = await connectionToAlice.verifySignature({signature, data});
    logger.info(`Is the signature ${signatureBase64} for data ${dataUtf8} correct? ${isVerified}`);
}


run();
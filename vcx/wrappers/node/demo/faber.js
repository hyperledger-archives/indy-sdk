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

    logger.info("#3 Create a new schema on the ledger");
    const version = `${getRandomInt(1, 101)}.${getRandomInt(1, 101)}.${getRandomInt(1, 101)}`;
    const schemaData = {
        data: {
            attrNames: ['name', 'date', 'degree'],
            name: `Schema1`,
            version
        },
        paymentHandle: 0,
        sourceId: 'testSchemaSourceId123'
    };

    const schema = await Schema.create(schemaData);
    const schemaId = await schema.getSchemaId();
    logger.info(`Created schema with id ${schemaId}`);

    logger.info("#4 Create a new credential definition on the ledger");
    const data = {
        name: 'DemoCredential123',
        paymentHandle: 0,
        revocation: false,
        revocationDetails: {
            tailsFile: 'tails.txt',
        },
        schemaId: schemaId,
        sourceId: 'testCredentialDefSourceId123'
    };
    const cred_def = await CredentialDef.create(data);
    const cred_def_id = await cred_def.getCredDefId();
    const credDefHandle = cred_def.handle;
    logger.info(`Created credential with id ${cred_def_id} and handle ${credDefHandle}`);

    logger.info("#5 Create a connection to alice and print out the invite details");
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

    logger.info("#6 Polling agency and waiting for alice to accept the invitation. (start alice.py now)");
    let connection_state = await connectionToAlice.getState();
    while (connection_state !== StateType.Accepted) {
        await sleepPromise(2000);
        await connectionToAlice.updateState();
        connection_state = await connectionToAlice.getState();
    }
    logger.info(`Connection to alice was Accepted!`);

    const schema_attrs = {
        'name': 'alice',
        'date': '05-2018',
        'degree': 'maths',
    };

    logger.info("#12 Create an IssuerCredential object using the schema and credential definition")

    const credentialForAlice = await IssuerCredential.create({
        attr: schema_attrs,
        sourceId: 'alice_degree',
        credDefHandle,
        credentialName: 'cred',
        price: '0'
    });

    logger.info("#13 Issue credential offer to alice");
    await credentialForAlice.sendOffer(connectionToAlice);
    await credentialForAlice.updateState();

    logger.info("#14 Poll agency and wait for alice to send a credential request");
    let credential_state = await credentialForAlice.getState();
    while (credential_state !== StateType.RequestReceived) {
        await sleepPromise(2000);
        await credentialForAlice.updateState();
        credential_state = await credentialForAlice.getState();
    }

    logger.info("#17 Issue credential to alice");
    await credentialForAlice.sendCredential(connectionToAlice);


    logger.info("#18 Wait for alice to accept credential");
    await credentialForAlice.updateState();
    credential_state = await credentialForAlice.getState();
    while (credential_state !== StateType.Accepted) {
        sleepPromise(2000);
        await credentialForAlice.updateState();
        credential_state = await credentialForAlice.getState();
    }

    const proofAttributes = [
        {'name': 'name', 'restrictions': [{'issuer_did': config['institution_did']}]},
        {'name': 'date', 'restrictions': [{'issuer_did': config['institution_did']}]},
        {'name': 'degree', 'restrictions': [{'issuer_did': config['institution_did']}]}
    ];

    logger.info("#19 Create a Proof object");
    const proof = await Proof.create({
        sourceId: "213",
        attrs: proofAttributes,
        name: 'proofForAlice',
        revocationInterval: {}
    });

    logger.info("#20 Request proof of degree from alice");
    await proof.requestProof(connectionToAlice);

    logger.info("#21 Poll agency and wait for alice to provide proof");
    let proofState = await proof.getState();
    while (proofState !== StateType.Accepted) {
        sleepPromise(2000);
        await proof.updateState();
        proofState = await proof.getState();
    }

    logger.info("#27 Process the proof provided by alice");
    await proof.getProof(connectionToAlice);

    logger.info("#28 Check if proof is valid");
    if (proof.proofState === ProofState.Verified) {
        logger.info("proof is verified!!")
    } else {
        logger.info("could not verify proof :(")
    }
}


run();
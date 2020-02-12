"use strict";

const indy = require('indy-sdk');
const util = require('./util');
const assert = require('assert');

async function run() {

    console.log("gettingStarted.js -> started");

    let poolName = 'pool1';
    console.log(`Open Pool Ledger: ${poolName}`);
    let poolGenesisTxnPath = await util.getPoolGenesisTxnPath(poolName);
    let poolConfig = {
        "genesis_txn": poolGenesisTxnPath
    };
    try {
        await indy.createPoolLedgerConfig(poolName, poolConfig);
    } catch(e) {
        if(e.message !== "PoolLedgerConfigAlreadyExistsError") {
            throw e;
        }
    }

    await indy.setProtocolVersion(2)

    let poolHandle = await indy.openPoolLedger(poolName);

    console.log("==============================");
    console.log("=== Getting Trust Anchor credentials for Faber, Acme, Thrift and Government  ==");
    console.log("------------------------------");

    console.log("\"Sovrin Steward\" -> Create wallet");
    let stewardWalletConfig = {'id': 'stewardWalletName'}
    let stewardWalletCredentials = {'key': 'steward_key'}
    try {
        await indy.createWallet(stewardWalletConfig, stewardWalletCredentials)
    } catch(e) {
        if(e.message !== "WalletAlreadyExistsError") {
            throw e;
        }
    }

    let stewardWallet = await indy.openWallet(stewardWalletConfig, stewardWalletCredentials);

    console.log("\"Sovrin Steward\" -> Create and store in Wallet DID from seed");
    let stewardDidInfo = {
        'seed': '000000000000000000000000Steward1'
    };

    let [stewardDid,] = await indy.createAndStoreMyDid(stewardWallet, stewardDidInfo);

    console.log("==============================");
    console.log("== Getting Trust Anchor credentials - Government Onboarding  ==");
    console.log("------------------------------");

    let governmentWalletConfig = {'id': 'governmentWallet'}
    let governmentWalletCredentials = {'key': 'government_key'}
    let [governmentWallet, stewardGovernmentKey, governmentStewardDid, governmentStewardKey] = await onboarding(poolHandle, "Sovrin Steward", stewardWallet, stewardDid, "Government", null, governmentWalletConfig, governmentWalletCredentials);

    console.log("==============================");
    console.log("== Getting Trust Anchor credentials - Government getting Verinym  ==");
    console.log("------------------------------");

    let governmentDid = await getVerinym(poolHandle, "Sovrin Steward", stewardWallet, stewardDid,
        stewardGovernmentKey, "Government", governmentWallet, governmentStewardDid,
        governmentStewardKey, 'TRUST_ANCHOR');

    console.log("==============================");
    console.log("== Getting Trust Anchor credentials - Faber Onboarding  ==");
    console.log("------------------------------");

    let faberWalletConfig = {'id': 'faberWallet'}
    let faberWalletCredentials = {'key': 'faber_key'}
    let [faberWallet, stewardFaberKey, faberStewardDid, faberStewardKey] = await onboarding(poolHandle, "Sovrin Steward", stewardWallet, stewardDid, "Faber", null, faberWalletConfig, faberWalletCredentials);

    console.log("==============================");
    console.log("== Getting Trust Anchor credentials - Faber getting Verinym  ==");
    console.log("------------------------------");

    let faberDid = await getVerinym(poolHandle, "Sovrin Steward", stewardWallet, stewardDid, stewardFaberKey,
        "Faber", faberWallet, faberStewardDid, faberStewardKey, 'TRUST_ANCHOR');

    console.log("==============================");
    console.log("== Getting Trust Anchor credentials - Acme Onboarding  ==");
    console.log("------------------------------");

    let acmeWalletConfig = {'id': 'acmeWallet'}
    let acmeWalletCredentials = {'key': 'acme_key'}
    let [acmeWallet, stewardAcmeKey, acmeStewardDid, acmeStewardKey] = await onboarding(poolHandle, "Sovrin Steward", stewardWallet, stewardDid, "Acme", null, acmeWalletConfig, acmeWalletCredentials);

    console.log("==============================");
    console.log("== Getting Trust Anchor credentials - Acme getting Verinym  ==");
    console.log("------------------------------");

    let acmeDid = await getVerinym(poolHandle, "Sovrin Steward", stewardWallet, stewardDid, stewardAcmeKey,
        "Acme", acmeWallet, acmeStewardDid, acmeStewardKey, 'TRUST_ANCHOR');

    console.log("==============================");
    console.log("== Getting Trust Anchor credentials - Thrift Onboarding  ==");
    console.log("------------------------------");

    let thriftWalletConfig = {'id': 'thriftWallet'}
    let thriftWalletCredentials = {'key': 'thrift_key'}
    let [thriftWallet, stewardThriftKey, thriftStewardDid, thriftStewardKey] = await onboarding(poolHandle, "Sovrin Steward", stewardWallet, stewardDid, "Thrift", null, thriftWalletConfig, thriftWalletCredentials);

    console.log("==============================");
    console.log("== Getting Trust Anchor credentials - Thrift getting Verinym  ==");
    console.log("------------------------------");

    let thriftDid = await getVerinym(poolHandle, "Sovrin Steward", stewardWallet, stewardDid, stewardThriftKey,
        "Thrift", thriftWallet, thriftStewardDid, thriftStewardKey, 'TRUST_ANCHOR');

    console.log("==============================");
    console.log("=== Credential Schemas Setup ==");
    console.log("------------------------------");

    console.log("\"Government\" -> Create \"Job-Certificate\" Schema");
    let [jobCertificateSchemaId, jobCertificateSchema] = await indy.issuerCreateSchema(governmentDid, 'Job-Certificate', '0.2',
        ['first_name', 'last_name', 'salary', 'employee_status',
            'experience']);

    console.log("\"Government\" -> Send \"Job-Certificate\" Schema to Ledger");
    await sendSchema(poolHandle, governmentWallet, governmentDid, jobCertificateSchema);

    console.log("\"Government\" -> Create \"Transcript\" Schema");
    let [transcriptSchemaId, transcriptSchema] = await indy.issuerCreateSchema(governmentDid, 'Transcript', '1.2',
        ['first_name', 'last_name', 'degree', 'status',
            'year', 'average', 'ssn']);
    console.log("\"Government\" -> Send \"Transcript\" Schema to Ledger");
    await sendSchema(poolHandle, governmentWallet, governmentDid, transcriptSchema);

    console.log("==============================");
    console.log("=== Faber Credential Definition Setup ==");
    console.log("------------------------------");

    console.log("\"Faber\" -> Get \"Transcript\" Schema from Ledger");
    [, transcriptSchema] = await getSchema(poolHandle, faberDid, transcriptSchemaId);

    console.log("\"Faber\" -> Create and store in Wallet \"Faber Transcript\" Credential Definition");
    let [faberTranscriptCredDefId, faberTranscriptCredDefJson] = await indy.issuerCreateAndStoreCredentialDef(faberWallet, faberDid, transcriptSchema, 'TAG1', 'CL', '{"support_revocation": false}');

    console.log("\"Faber\" -> Send  \"Faber Transcript\" Credential Definition to Ledger");
    await sendCredDef(poolHandle, faberWallet, faberDid, faberTranscriptCredDefJson);

    console.log("==============================");
    console.log("=== Acme Credential Definition Setup ==");
    console.log("------------------------------");

    console.log("\"Acme\" ->  Get from Ledger \"Job-Certificate\" Schema");
    [, jobCertificateSchema] = await getSchema(poolHandle, acmeDid, jobCertificateSchemaId);

    console.log("\"Acme\" -> Create and store in Wallet \"Acme Job-Certificate\" Credential Definition");
    let [acmeJobCertificateCredDefId, acmeJobCertificateCredDefJson] = await indy.issuerCreateAndStoreCredentialDef(acmeWallet, acmeDid, jobCertificateSchema, 'TAG1', 'CL', '{"support_revocation": false}');

    console.log("\"Acme\" -> Send \"Acme Job-Certificate\" Credential Definition to Ledger");
    await sendCredDef(poolHandle, acmeWallet, acmeDid, acmeJobCertificateCredDefJson);

    console.log("==============================");
    console.log("=== Getting Transcript with Faber ==");
    console.log("==============================");
    console.log("== Getting Transcript with Faber - Onboarding ==");
    console.log("------------------------------");

    let aliceWalletConfig = {'id': 'aliceWallet'}
    let aliceWalletCredentials = {'key': 'alice_key'}
    let [aliceWallet, faberAliceKey, aliceFaberDid, aliceFaberKey, faberAliceConnectionResponse] = await onboarding(poolHandle, "Faber", faberWallet, faberDid, "Alice", null, aliceWalletConfig, aliceWalletCredentials);

    console.log("==============================");
    console.log("== Getting Transcript with Faber - Getting Transcript Credential ==");
    console.log("------------------------------");

    console.log("\"Faber\" -> Create \"Transcript\" Credential Offer for Alice");
    let transcriptCredOfferJson = await indy.issuerCreateCredentialOffer(faberWallet, faberTranscriptCredDefId);

    console.log("\"Faber\" -> Get key for Alice did");
    let aliceFaberVerkey = await indy.keyForDid(poolHandle, faberWallet, faberAliceConnectionResponse['did']);

    console.log("\"Faber\" -> Authcrypt \"Transcript\" Credential Offer for Alice");
    let authcryptedTranscriptCredOffer = await indy.cryptoAuthCrypt(faberWallet, faberAliceKey, aliceFaberVerkey, Buffer.from(JSON.stringify(transcriptCredOfferJson),'utf8'));

    console.log("\"Faber\" -> Send authcrypted \"Transcript\" Credential Offer to Alice");

    console.log("\"Alice\" -> Authdecrypted \"Transcript\" Credential Offer from Faber");
    let [faberAliceVerkey, authdecryptedTranscriptCredOfferJson, authdecryptedTranscriptCredOffer] = await authDecrypt(aliceWallet, aliceFaberKey, authcryptedTranscriptCredOffer);

    console.log("\"Alice\" -> Create and store \"Alice\" Master Secret in Wallet");
    let aliceMasterSecretId = await indy.proverCreateMasterSecret(aliceWallet, null);

    console.log("\"Alice\" -> Get \"Faber Transcript\" Credential Definition from Ledger");
    let faberTranscriptCredDef;
    [faberTranscriptCredDefId, faberTranscriptCredDef] = await getCredDef(poolHandle, aliceFaberDid, authdecryptedTranscriptCredOffer['cred_def_id']);

    console.log("\"Alice\" -> Create \"Transcript\" Credential Request for Faber");
    let [transcriptCredRequestJson, transcriptCredRequestMetadataJson] = await indy.proverCreateCredentialReq(aliceWallet, aliceFaberDid, authdecryptedTranscriptCredOfferJson, faberTranscriptCredDef, aliceMasterSecretId);

    console.log("\"Alice\" -> Authcrypt \"Transcript\" Credential Request for Faber");
    let authcryptedTranscriptCredRequest = await indy.cryptoAuthCrypt(aliceWallet, aliceFaberKey, faberAliceVerkey, Buffer.from(JSON.stringify(transcriptCredRequestJson),'utf8'));

    console.log("\"Alice\" -> Send authcrypted \"Transcript\" Credential Request to Faber");

    console.log("\"Faber\" -> Authdecrypt \"Transcript\" Credential Request from Alice");
    let authdecryptedTranscriptCredRequestJson;
    [aliceFaberVerkey, authdecryptedTranscriptCredRequestJson] = await authDecrypt(faberWallet, faberAliceKey, authcryptedTranscriptCredRequest);

    console.log("\"Faber\" -> Create \"Transcript\" Credential for Alice");
    // note that encoding is not standardized by Indy except that 32-bit integers are encoded as themselves. IS-786
    let transcriptCredValues = {
        "first_name": {"raw": "Alice", "encoded": "1139481716457488690172217916278103335"},
        "last_name": {"raw": "Garcia", "encoded": "5321642780241790123587902456789123452"},
        "degree": {"raw": "Bachelor of Science, Marketing", "encoded": "12434523576212321"},
        "status": {"raw": "graduated", "encoded": "2213454313412354"},
        "ssn": {"raw": "123-45-6789", "encoded": "3124141231422543541"},
        "year": {"raw": "2015", "encoded": "2015"},
        "average": {"raw": "5", "encoded": "5"}
    };

    let [transcriptCredJson] = await indy.issuerCreateCredential(faberWallet, transcriptCredOfferJson, authdecryptedTranscriptCredRequestJson, transcriptCredValues, null, -1);

    console.log("\"Faber\" -> Authcrypt \"Transcript\" Credential for Alice");
    let authcryptedTranscriptCredJson = await indy.cryptoAuthCrypt(faberWallet, faberAliceKey, aliceFaberVerkey, Buffer.from(JSON.stringify(transcriptCredJson),'utf8'));

    console.log("\"Faber\" -> Send authcrypted \"Transcript\" Credential to Alice");

    console.log("\"Alice\" -> Authdecrypted \"Transcript\" Credential from Faber");
    let [, authdecryptedTranscriptCredJson] = await authDecrypt(aliceWallet, aliceFaberKey, authcryptedTranscriptCredJson);

    console.log("\"Alice\" -> Store \"Transcript\" Credential from Faber");
    await indy.proverStoreCredential(aliceWallet, null, transcriptCredRequestMetadataJson,
        authdecryptedTranscriptCredJson, faberTranscriptCredDef, null);

    console.log("==============================");
    console.log("=== Apply for the job with Acme ==");
    console.log("==============================");
    console.log("== Apply for the job with Acme - Onboarding ==");
    console.log("------------------------------");
    let acmeAliceKey, aliceAcmeDid, aliceAcmeKey, acmeAliceConnectionResponse;

    [aliceWallet, acmeAliceKey, aliceAcmeDid, aliceAcmeKey, acmeAliceConnectionResponse] = await onboarding(poolHandle, "Acme", acmeWallet, acmeDid, "Alice", aliceWallet, aliceWalletConfig, aliceWalletCredentials);

    console.log("==============================");
    console.log("== Apply for the job with Acme - Transcript proving ==");
    console.log("------------------------------");

    console.log("\"Acme\" -> Create \"Job-Application\" Proof Request");
    let nonce = await indy.generateNonce()
    let jobApplicationProofRequestJson = {
        'nonce': nonce,
        'name': 'Job-Application',
        'version': '0.1',
        'requested_attributes': {
            'attr1_referent': {
                'name': 'first_name'
            },
            'attr2_referent': {
                'name': 'last_name'
            },
            'attr3_referent': {
                'name': 'degree',
                'restrictions': [{'cred_def_id': faberTranscriptCredDefId}]
            },
            'attr4_referent': {
                'name': 'status',
                'restrictions': [{'cred_def_id': faberTranscriptCredDefId}]
            },
            'attr5_referent': {
                'name': 'ssn',
                'restrictions': [{'cred_def_id': faberTranscriptCredDefId}]
            },
            'attr6_referent': {
                'name': 'phone_number'
            }
        },
        'requested_predicates': {
            'predicate1_referent': {
                'name': 'average',
                'p_type': '>=',
                'p_value': 4,
                'restrictions': [{'cred_def_id': faberTranscriptCredDefId}]
            }
        }
    };

    console.log("\"Acme\" -> Get key for Alice did");
    let aliceAcmeVerkey = await indy.keyForDid(poolHandle, acmeWallet, acmeAliceConnectionResponse['did']);

    console.log("\"Acme\" -> Authcrypt \"Job-Application\" Proof Request for Alice");
    let authcryptedJobApplicationProofRequestJson = await indy.cryptoAuthCrypt(acmeWallet, acmeAliceKey, aliceAcmeVerkey,Buffer.from(JSON.stringify(jobApplicationProofRequestJson),'utf8'));

    console.log("\"Acme\" -> Send authcrypted \"Job-Application\" Proof Request to Alice");

    console.log("\"Alice\" -> Authdecrypt \"Job-Application\" Proof Request from Acme");
    let [acmeAliceVerkey, authdecryptedJobApplicationProofRequestJson] = await authDecrypt(aliceWallet, aliceAcmeKey, authcryptedJobApplicationProofRequestJson);

    console.log("\"Alice\" -> Get credentials for \"Job-Application\" Proof Request");
    let searchForJobApplicationProofRequest = await indy.proverSearchCredentialsForProofReq(aliceWallet, authdecryptedJobApplicationProofRequestJson, null)

    let credentials = await indy.proverFetchCredentialsForProofReq(searchForJobApplicationProofRequest, 'attr1_referent', 100)
    let credForAttr1 = credentials[0]['cred_info'];

    await indy.proverFetchCredentialsForProofReq(searchForJobApplicationProofRequest, 'attr2_referent', 100)
    let credForAttr2 = credentials[0]['cred_info'];

    await indy.proverFetchCredentialsForProofReq(searchForJobApplicationProofRequest, 'attr3_referent', 100)
    let credForAttr3 = credentials[0]['cred_info'];

    await indy.proverFetchCredentialsForProofReq(searchForJobApplicationProofRequest, 'attr4_referent', 100)
    let credForAttr4 = credentials[0]['cred_info'];

    await indy.proverFetchCredentialsForProofReq(searchForJobApplicationProofRequest, 'attr5_referent', 100)
    let credForAttr5 = credentials[0]['cred_info'];

    await indy.proverFetchCredentialsForProofReq(searchForJobApplicationProofRequest, 'predicate1_referent', 100)
    let credForPredicate1 = credentials[0]['cred_info'];

    await indy.proverCloseCredentialsSearchForProofReq(searchForJobApplicationProofRequest)

    let credsForJobApplicationProof = {};
    credsForJobApplicationProof[`${credForAttr1['referent']}`] = credForAttr1;
    credsForJobApplicationProof[`${credForAttr2['referent']}`] = credForAttr2;
    credsForJobApplicationProof[`${credForAttr3['referent']}`] = credForAttr3;
    credsForJobApplicationProof[`${credForAttr4['referent']}`] = credForAttr4;
    credsForJobApplicationProof[`${credForAttr5['referent']}`] = credForAttr5;
    credsForJobApplicationProof[`${credForPredicate1['referent']}`] = credForPredicate1;

    let [schemasJson, credDefsJson, revocStatesJson] = await proverGetEntitiesFromLedger(poolHandle, aliceFaberDid, credsForJobApplicationProof, 'Alice');

    console.log("\"Alice\" -> Create \"Job-Application\" Proof");
    let jobApplicationRequestedCredsJson = {
        'self_attested_attributes': {
            'attr1_referent': 'Alice',
            'attr2_referent': 'Garcia',
            'attr6_referent': '123-45-6789'
        },
        'requested_attributes': {
            'attr3_referent': {'cred_id': credForAttr3['referent'], 'revealed': true},
            'attr4_referent': {'cred_id': credForAttr4['referent'], 'revealed': true},
            'attr5_referent': {'cred_id': credForAttr5['referent'], 'revealed': true},
        },
        'requested_predicates': {'predicate1_referent': {'cred_id': credForPredicate1['referent']}}
    };

    let jobApplicationProofJson = await indy.proverCreateProof(aliceWallet, authdecryptedJobApplicationProofRequestJson,
        jobApplicationRequestedCredsJson, aliceMasterSecretId,
        schemasJson, credDefsJson, revocStatesJson);

    console.log("\"Alice\" -> Authcrypt \"Job-Application\" Proof for Acme");
    let authcryptedJobApplicationProofJson = await indy.cryptoAuthCrypt(aliceWallet, aliceAcmeKey, acmeAliceVerkey,Buffer.from(JSON.stringify(jobApplicationProofJson),'utf8'));

    console.log("\"Alice\" -> Send authcrypted \"Job-Application\" Proof to Acme");

    console.log("\"Acme\" -> Authdecrypted \"Job-Application\" Proof from Alice");
    let decryptedJobApplicationProofJson, decryptedJobApplicationProof;
    [, decryptedJobApplicationProofJson, decryptedJobApplicationProof] = await authDecrypt(acmeWallet, acmeAliceKey, authcryptedJobApplicationProofJson);

    let revocRefDefsJson, revocRegsJson;
    [schemasJson, credDefsJson, revocRefDefsJson, revocRegsJson] = await verifierGetEntitiesFromLedger(poolHandle, acmeDid, decryptedJobApplicationProof['identifiers'], 'Acme');

    console.log("\"Acme\" -> Verify \"Job-Application\" Proof from Alice");
    assert('Bachelor of Science, Marketing' === decryptedJobApplicationProof['requested_proof']['revealed_attrs']['attr3_referent']['raw']);
    assert('graduated' === decryptedJobApplicationProof['requested_proof']['revealed_attrs']['attr4_referent']['raw']);
    assert('123-45-6789' === decryptedJobApplicationProof['requested_proof']['revealed_attrs']['attr5_referent']['raw']);

    assert('Alice' === decryptedJobApplicationProof['requested_proof']['self_attested_attrs']['attr1_referent']);
    assert('Garcia' === decryptedJobApplicationProof['requested_proof']['self_attested_attrs']['attr2_referent']);
    assert('123-45-6789' === decryptedJobApplicationProof['requested_proof']['self_attested_attrs']['attr6_referent']);

    assert(await indy.verifierVerifyProof(jobApplicationProofRequestJson, decryptedJobApplicationProofJson, schemasJson, credDefsJson, revocRefDefsJson, revocRegsJson));

    console.log("==============================");
    console.log("== Apply for the job with Acme - Getting Job-Certificate Credential ==");
    console.log("------------------------------");

    console.log("\"Acme\" -> Create \"Job-Certificate\" Credential Offer for Alice");
    let jobCertificateCredOfferJson = await indy.issuerCreateCredentialOffer(acmeWallet, acmeJobCertificateCredDefId);

    console.log("\"Acme\" -> Get key for Alice did");
    aliceAcmeVerkey = await indy.keyForDid(poolHandle, acmeWallet, acmeAliceConnectionResponse['did']);

    console.log("\"Acme\" -> Authcrypt \"Job-Certificate\" Credential Offer for Alice");
    let authcryptedJobCertificateCredOffer = await indy.cryptoAuthCrypt(acmeWallet, acmeAliceKey, aliceAcmeVerkey,Buffer.from(JSON.stringify(jobCertificateCredOfferJson),'utf8'));

    console.log("\"Acme\" -> Send authcrypted \"Job-Certificate\" Credential Offer to Alice");

    console.log("\"Alice\" -> Authdecrypted \"Job-Certificate\" Credential Offer from Acme");
    let authdecryptedJobCertificateCredOfferJson, authdecryptedJobCertificateCredOffer;
    [acmeAliceVerkey, authdecryptedJobCertificateCredOfferJson, authdecryptedJobCertificateCredOffer] = await authDecrypt(aliceWallet, aliceAcmeKey, authcryptedJobCertificateCredOffer);

    console.log("\"Alice\" -> Get \"Acme Job-Certificate\" Credential Definition from Ledger");
    let acmeJobCertificateCredDef;
    [, acmeJobCertificateCredDef] = await getCredDef(poolHandle, aliceAcmeDid, authdecryptedJobCertificateCredOffer['cred_def_id']);

    console.log("\"Alice\" -> Create and store in Wallet \"Job-Certificate\" Credential Request for Acme");
    let [jobCertificateCredRequestJson, jobCertificateCredRequestMetadataJson] = await indy.proverCreateCredentialReq(aliceWallet, aliceAcmeDid, authdecryptedJobCertificateCredOfferJson, acmeJobCertificateCredDef, aliceMasterSecretId);

    console.log("\"Alice\" -> Authcrypt \"Job-Certificate\" Credential Request for Acme");
    let authcryptedJobCertificateCredRequestJson = await indy.cryptoAuthCrypt(aliceWallet, aliceAcmeKey, acmeAliceVerkey,Buffer.from(JSON.stringify(jobCertificateCredRequestJson),'utf8'));

    console.log("\"Alice\" -> Send authcrypted \"Job-Certificate\" Credential Request to Acme");

    console.log("\"Acme\" -> Authdecrypt \"Job-Certificate\" Credential Request from Alice");
    let authdecryptedJobCertificateCredRequestJson;
    [aliceAcmeVerkey, authdecryptedJobCertificateCredRequestJson] = await authDecrypt(acmeWallet, acmeAliceKey, authcryptedJobCertificateCredRequestJson);

    console.log("\"Acme\" -> Create \"Job-Certificate\" Credential for Alice");
    let aliceJobCertificateCredValuesJson = {
        "first_name": {"raw": "Alice", "encoded": "245712572474217942457235975012103335"},
        "last_name": {"raw": "Garcia", "encoded": "312643218496194691632153761283356127"},
        "employee_status": {"raw": "Permanent", "encoded": "2143135425425143112321314321"},
        "salary": {"raw": "2400", "encoded": "2400"},
        "experience": {"raw": "10", "encoded": "10"}
    };

    let [jobCertificateCredJson] = await indy.issuerCreateCredential(acmeWallet, jobCertificateCredOfferJson, authdecryptedJobCertificateCredRequestJson, aliceJobCertificateCredValuesJson, null, -1);

    console.log("\"Acme\" ->  Authcrypt \"Job-Certificate\" Credential for Alice");
    let authcryptedJobCertificateCredJson = await indy.cryptoAuthCrypt(acmeWallet, acmeAliceKey, aliceAcmeVerkey,Buffer.from(JSON.stringify(jobCertificateCredJson),'utf8'));

    console.log("\"Acme\" ->  Send authcrypted \"Job-Certificate\" Credential to Alice");

    console.log("\"Alice\" -> Authdecrypted \"Job-Certificate\" Credential from Acme");
    let authdecryptedJobCertificateCredJson;
    [, authdecryptedJobCertificateCredJson] = await authDecrypt(aliceWallet, aliceAcmeKey, authcryptedJobCertificateCredJson);

    console.log("\"Alice\" -> Store \"Job-Certificate\" Credential");
    await indy.proverStoreCredential(aliceWallet, null, jobCertificateCredRequestMetadataJson, authdecryptedJobCertificateCredJson, acmeJobCertificateCredDefJson, null);

    console.log("==============================");
    console.log("=== Apply for the loan with Thrift ==");
    console.log("==============================");
    console.log("== Apply for the loan with Thrift - Onboarding ==");
    console.log("------------------------------");

    let thriftAliceKey, aliceThriftDid, aliceThriftKey, thriftAliceConnectionResponse;
    [aliceWallet, thriftAliceKey, aliceThriftDid, aliceThriftKey, thriftAliceConnectionResponse] = await onboarding(poolHandle, "Thrift", thriftWallet, thriftDid,
        "Alice", aliceWallet, aliceWalletConfig, aliceWalletCredentials);

    console.log("==============================");
    console.log("== Apply for the loan with Thrift - Job-Certificate proving  ==");
    console.log("------------------------------");

    console.log("\"Thrift\" -> Create \"Loan-Application-Basic\" Proof Request");
    nonce = await indy.generateNonce()
    let applyLoanProofRequestJson = {
        'nonce': nonce,
        'name': 'Loan-Application-Basic',
        'version': '0.1',
        'requested_attributes': {
            'attr1_referent': {
                'name': 'employee_status',
                'restrictions': [{'cred_def_id': acmeJobCertificateCredDefId}]
            }
        },
        'requested_predicates': {
            'predicate1_referent': {
                'name': 'salary',
                'p_type': '>=',
                'p_value': 2000,
                'restrictions': [{'cred_def_id': acmeJobCertificateCredDefId}]
            },
            'predicate2_referent': {
                'name': 'experience',
                'p_type': '>=',
                'p_value': 1,
                'restrictions': [{'cred_def_id': acmeJobCertificateCredDefId}]
            }
        }
    };

    console.log("\"Thrift\" -> Get key for Alice did");
    let aliceThriftVerkey = await indy.keyForDid(poolHandle, thriftWallet, thriftAliceConnectionResponse['did']);

    console.log("\"Thrift\" -> Authcrypt \"Loan-Application-Basic\" Proof Request for Alice");
    let authcryptedApplyLoanProofRequestJson = await indy.cryptoAuthCrypt(thriftWallet, thriftAliceKey, aliceThriftVerkey,Buffer.from(JSON.stringify(applyLoanProofRequestJson),'utf8'));

    console.log("\"Thrift\" -> Send authcrypted \"Loan-Application-Basic\" Proof Request to Alice");

    console.log("\"Alice\" -> Authdecrypt \"Loan-Application-Basic\" Proof Request from Thrift");
    let [thriftAliceVerkey, authdecryptedApplyLoanProofRequestJson] = await authDecrypt(aliceWallet, aliceThriftKey, authcryptedApplyLoanProofRequestJson);

    console.log("\"Alice\" -> Get credentials for \"Loan-Application-Basic\" Proof Request");

    let searchForJApplyLoanProofRequest = await indy.proverSearchCredentialsForProofReq(aliceWallet, authdecryptedApplyLoanProofRequestJson, null)

    credentials = await indy.proverFetchCredentialsForProofReq(searchForJApplyLoanProofRequest, 'attr1_referent', 100)
    credForAttr1 = credentials[0]['cred_info'];

    await indy.proverFetchCredentialsForProofReq(searchForJApplyLoanProofRequest, 'predicate1_referent', 100)
    credForPredicate1 = credentials[0]['cred_info'];

    await indy.proverFetchCredentialsForProofReq(searchForJApplyLoanProofRequest, 'predicate2_referent', 100)
    let credForPredicate2 = credentials[0]['cred_info'];

    await indy.proverCloseCredentialsSearchForProofReq(searchForJApplyLoanProofRequest)

    let credsForApplyLoanProof = {};
    credsForApplyLoanProof[`${credForAttr1['referent']}`] = credForAttr1;
    credsForApplyLoanProof[`${credForPredicate1['referent']}`] = credForPredicate1;
    credsForApplyLoanProof[`${credForPredicate2['referent']}`] = credForPredicate2;

    [schemasJson, credDefsJson, revocStatesJson] = await proverGetEntitiesFromLedger(poolHandle, aliceThriftDid, credsForApplyLoanProof, 'Alice');

    console.log("\"Alice\" -> Create \"Loan-Application-Basic\" Proof");
    let applyLoanRequestedCredsJson = {
        'self_attested_attributes': {},
        'requested_attributes': {
            'attr1_referent': {'cred_id': credForAttr1['referent'], 'revealed': true}
        },
        'requested_predicates': {
            'predicate1_referent': {'cred_id': credForPredicate1['referent']},
            'predicate2_referent': {'cred_id': credForPredicate2['referent']}
        }
    };
    let aliceApplyLoanProofJson = await indy.proverCreateProof(aliceWallet, authdecryptedApplyLoanProofRequestJson,
        applyLoanRequestedCredsJson, aliceMasterSecretId, schemasJson,
        credDefsJson, revocStatesJson);

    console.log("\"Alice\" -> Authcrypt \"Loan-Application-Basic\" Proof for Thrift");
    let authcryptedAliceApplyLoanProofJson = await indy.cryptoAuthCrypt(aliceWallet, aliceThriftKey, thriftAliceVerkey,Buffer.from(JSON.stringify(aliceApplyLoanProofJson),'utf8'));

    console.log("\"Alice\" -> Send authcrypted \"Loan-Application-Basic\" Proof to Thrift");

    console.log("\"Thrift\" -> Authdecrypted \"Loan-Application-Basic\" Proof from Alice");
    let authdecryptedAliceApplyLoanProofJson, authdecryptedAliceApplyLoanProof;
    [, authdecryptedAliceApplyLoanProofJson, authdecryptedAliceApplyLoanProof] = await authDecrypt(thriftWallet, thriftAliceKey, authcryptedAliceApplyLoanProofJson);

    console.log("\"Thrift\" -> Get Schemas, Credential Definitions and Revocation Registries from Ledger required for Proof verifying");

    let revocDefsJson;
    [schemasJson, credDefsJson, revocDefsJson, revocRegsJson] = await verifierGetEntitiesFromLedger(poolHandle, thriftDid,
        authdecryptedAliceApplyLoanProof['identifiers'], 'Thrift');

    console.log("\"Thrift\" -> Verify \"Loan-Application-Basic\" Proof from Alice");
    assert('Permanent' === authdecryptedAliceApplyLoanProof['requested_proof']['revealed_attrs']['attr1_referent']['raw']);

    assert(await indy.verifierVerifyProof(applyLoanProofRequestJson, authdecryptedAliceApplyLoanProofJson, schemasJson, credDefsJson, revocDefsJson, revocRegsJson));

    console.log("==============================");

    console.log("==============================");
    console.log("== Apply for the loan with Thrift - Transcript and Job-Certificate proving  ==");
    console.log("------------------------------");

    console.log("\"Thrift\" -> Create \"Loan-Application-KYC\" Proof Request");
    nonce = await indy.generateNonce()
    let applyLoanKycProofRequestJson = {
        'nonce': nonce,
        'name': 'Loan-Application-KYC',
        'version': '0.1',
        'requested_attributes': {
            'attr1_referent': {'name': 'first_name'},
            'attr2_referent': {'name': 'last_name'},
            'attr3_referent': {'name': 'ssn'}
        },
        'requested_predicates': {}
    };

    console.log("\"Thrift\" -> Get key for Alice did");
    aliceThriftVerkey = await indy.keyForDid(poolHandle, thriftWallet, thriftAliceConnectionResponse['did']);

    console.log("\"Thrift\" -> Authcrypt \"Loan-Application-KYC\" Proof Request for Alice");
    let authcryptedApplyLoanKycProofRequestJson = await indy.cryptoAuthCrypt(thriftWallet, thriftAliceKey, aliceThriftVerkey,Buffer.from(JSON.stringify(applyLoanKycProofRequestJson),'utf8'));

    console.log("\"Thrift\" -> Send authcrypted \"Loan-Application-KYC\" Proof Request to Alice");

    console.log("\"Alice\" -> Authdecrypt \"Loan-Application-KYC\" Proof Request from Thrift");
    let authdecryptedApplyLoanKycProofRequestJson;
    [thriftAliceVerkey, authdecryptedApplyLoanKycProofRequestJson] = await authDecrypt(aliceWallet, aliceThriftKey, authcryptedApplyLoanKycProofRequestJson);

    console.log("\"Alice\" -> Get credentials for \"Loan-Application-KYC\" Proof Request");

    let searchForApplyLoanKycProofRequest = await indy.proverSearchCredentialsForProofReq(aliceWallet, authdecryptedApplyLoanKycProofRequestJson, null)

    credentials = await indy.proverFetchCredentialsForProofReq(searchForApplyLoanKycProofRequest, 'attr1_referent', 100)
    credForAttr1 = credentials[0]['cred_info'];

    credentials = await indy.proverFetchCredentialsForProofReq(searchForApplyLoanKycProofRequest, 'attr2_referent', 100)
    credForAttr2 = credentials[0]['cred_info'];

    credentials = await indy.proverFetchCredentialsForProofReq(searchForApplyLoanKycProofRequest, 'attr3_referent', 100)
    credForAttr3 = credentials[0]['cred_info'];

    await indy.proverCloseCredentialsSearchForProofReq(searchForApplyLoanKycProofRequest)

    let credsForApplyLoanKycProof = {};
    credsForApplyLoanKycProof[`${credForAttr1['referent']}`] = credForAttr1;
    credsForApplyLoanKycProof[`${credForAttr2['referent']}`] = credForAttr2;
    credsForApplyLoanKycProof[`${credForAttr3['referent']}`] = credForAttr3;

    [schemasJson, credDefsJson, revocStatesJson] = await proverGetEntitiesFromLedger(poolHandle, aliceThriftDid, credsForApplyLoanKycProof, 'Alice');

    console.log("\"Alice\" -> Create \"Loan-Application-KYC\" Proof");

    let applyLoanKycRequestedCredsJson = {
        'self_attested_attributes': {},
        'requested_attributes': {
            'attr1_referent': {'cred_id': credForAttr1['referent'], 'revealed': true},
            'attr2_referent': {'cred_id': credForAttr2['referent'], 'revealed': true},
            'attr3_referent': {'cred_id': credForAttr3['referent'], 'revealed': true}
        },
        'requested_predicates': {}
    };

    let aliceApplyLoanKycProofJson = await indy.proverCreateProof(aliceWallet, authdecryptedApplyLoanKycProofRequestJson,
        applyLoanKycRequestedCredsJson, aliceMasterSecretId,
        schemasJson, credDefsJson, revocStatesJson);

    console.log("\"Alice\" -> Authcrypt \"Loan-Application-KYC\" Proof for Thrift");
    let authcryptedAliceApplyLoanKycProofJson = await indy.cryptoAuthCrypt(aliceWallet, aliceThriftKey, thriftAliceVerkey,Buffer.from(JSON.stringify(aliceApplyLoanKycProofJson),'utf8'));

    console.log("\"Alice\" -> Send authcrypted \"Loan-Application-KYC\" Proof to Thrift");

    console.log("\"Thrift\" -> Authdecrypted \"Loan-Application-KYC\" Proof from Alice");
    let authdecryptedAliceApplyLoanKycProof;
    [, authdecryptedAliceApplyLoanKycProof] = await authDecrypt(thriftWallet, thriftAliceKey, authcryptedAliceApplyLoanKycProofJson);

    console.log("\"Thrift\" -> Get Schemas, Credential Definitions and Revocation Registries from Ledger required for Proof verifying");

    [schemasJson, credDefsJson, revocDefsJson, revocRegsJson] = await verifierGetEntitiesFromLedger(poolHandle, thriftDid, authdecryptedAliceApplyLoanKycProof['identifiers'], 'Thrift');

    console.log("\"Thrift\" -> Verify \"Loan-Application-KYC\" Proof from Alice");
    assert('Alice' === authdecryptedAliceApplyLoanKycProof['requested_proof']['revealed_attrs']['attr1_referent']['raw']);
    assert('Garcia' === authdecryptedAliceApplyLoanKycProof['requested_proof']['revealed_attrs']['attr2_referent']['raw']);
    assert('123-45-6789' === authdecryptedAliceApplyLoanKycProof['requested_proof']['revealed_attrs']['attr3_referent']['raw']);

    assert(await indy.verifierVerifyProof(applyLoanKycProofRequestJson, authdecryptedAliceApplyLoanKycProof, schemasJson, credDefsJson, revocDefsJson, revocRegsJson));

    console.log("==============================");

    console.log(" \"Sovrin Steward\" -> Close and Delete wallet");
    await indy.closeWallet(stewardWallet);
    await indy.deleteWallet(stewardWalletConfig, stewardWalletCredentials);

    console.log("\"Government\" -> Close and Delete wallet");
    await indy.closeWallet(governmentWallet);
    await indy.deleteWallet(governmentWalletConfig, governmentWalletCredentials);

    console.log("\"Faber\" -> Close and Delete wallet");
    await indy.closeWallet(faberWallet);
    await indy.deleteWallet(faberWalletConfig, faberWalletCredentials);

    console.log("\"Acme\" -> Close and Delete wallet");
    await indy.closeWallet(acmeWallet);
    await indy.deleteWallet(acmeWalletConfig, acmeWalletCredentials);

    console.log("\"Thrift\" -> Close and Delete wallet");
    await indy.closeWallet(thriftWallet);
    await indy.deleteWallet(thriftWalletConfig, thriftWalletCredentials);

    console.log("\"Alice\" -> Close and Delete wallet");
    await indy.closeWallet(aliceWallet);
    await indy.deleteWallet(aliceWalletConfig, aliceWalletCredentials);

    console.log("Close and Delete pool");
    await indy.closePoolLedger(poolHandle);
    await indy.deletePoolLedgerConfig(poolName);

    console.log("Getting started -> done")
}

async function onboarding(poolHandle, From, fromWallet, fromDid, to, toWallet, toWalletConfig, toWalletCredentials) {
    console.log(`\"${From}\" > Create and store in Wallet \"${From} ${to}\" DID`);
    let [fromToDid, fromToKey] = await indy.createAndStoreMyDid(fromWallet, {});

    console.log(`\"${From}\" > Send Nym to Ledger for \"${From} ${to}\" DID`);
    await sendNym(poolHandle, fromWallet, fromDid, fromToDid, fromToKey, null);

    console.log(`\"${From}\" > Send connection request to ${to} with \"${From} ${to}\" DID and nonce`);
    let connectionRequest = {
        did: fromToDid,
        nonce: 123456789
    };

    if (!toWallet) {
        console.log(`\"${to}\" > Create wallet"`);
        try {
            await indy.createWallet(toWalletConfig, toWalletCredentials);
        } catch(e) {
            if(e.message !== "WalletAlreadyExistsError") {
                throw e;
            }
        }
        toWallet = await indy.openWallet(toWalletConfig, toWalletCredentials);
    }

    console.log(`\"${to}\" > Create and store in Wallet \"${to} ${From}\" DID`);
    let [toFromDid, toFromKey] = await indy.createAndStoreMyDid(toWallet, {});

    console.log(`\"${to}\" > Get key for did from \"${From}\" connection request`);
    let fromToVerkey = await indy.keyForDid(poolHandle, toWallet, connectionRequest.did);

    console.log(`\"${to}\" > Anoncrypt connection response for \"${From}\" with \"${to} ${From}\" DID, verkey and nonce`);
    let connectionResponse = JSON.stringify({
        'did': toFromDid,
        'verkey': toFromKey,
        'nonce': connectionRequest['nonce']
    });
    let anoncryptedConnectionResponse = await indy.cryptoAnonCrypt(fromToVerkey, Buffer.from(connectionResponse, 'utf8'));

    console.log(`\"${to}\" > Send anoncrypted connection response to \"${From}\"`);

    console.log(`\"${From}\" > Anondecrypt connection response from \"${to}\"`);
    let decryptedConnectionResponse = JSON.parse(Buffer.from(await indy.cryptoAnonDecrypt(fromWallet, fromToKey, anoncryptedConnectionResponse)));

    console.log(`\"${From}\" > Authenticates \"${to}\" by comparision of Nonce`);
    if (connectionRequest['nonce'] !== decryptedConnectionResponse['nonce']) {
        throw Error("nonces don't match!");
    }

    console.log(`\"${From}\" > Send Nym to Ledger for \"${to} ${From}\" DID`);
    await sendNym(poolHandle, fromWallet, fromDid, decryptedConnectionResponse['did'], decryptedConnectionResponse['verkey'], null);

    return [toWallet, fromToKey, toFromDid, toFromKey, decryptedConnectionResponse];
}

async function getVerinym(poolHandle, From, fromWallet, fromDid, fromToKey, to, toWallet, toFromDid, toFromKey, role) {
    console.log(`\"${to}\" > Create and store in Wallet \"${to}\" new DID"`);
    let [toDid, toKey] = await indy.createAndStoreMyDid(toWallet, {});

    console.log(`\"${to}\" > Authcrypt \"${to} DID info\" for \"${From}\"`);
    let didInfoJson = JSON.stringify({
        'did': toDid,
        'verkey': toKey
    });
    let authcryptedDidInfo = await indy.cryptoAuthCrypt(toWallet, toFromKey, fromToKey, Buffer.from(didInfoJson, 'utf8'));

    console.log(`\"${to}\" > Send authcrypted \"${to} DID info\" to ${From}`);

    console.log(`\"${From}\" > Authdecrypted \"${to} DID info\" from ${to}`);
    let [senderVerkey, authdecryptedDidInfo] =
        await indy.cryptoAuthDecrypt(fromWallet, fromToKey, Buffer.from(authcryptedDidInfo));

    let authdecryptedDidInfoJson = JSON.parse(Buffer.from(authdecryptedDidInfo));
    console.log(`\"${From}\" > Authenticate ${to} by comparision of Verkeys`);
    let retrievedVerkey = await indy.keyForDid(poolHandle, fromWallet, toFromDid);
    if (senderVerkey !== retrievedVerkey) {
        throw Error("Verkey is not the same");
    }

    console.log(`\"${From}\" > Send Nym to Ledger for \"${to} DID\" with ${role} Role`);
    await sendNym(poolHandle, fromWallet, fromDid, authdecryptedDidInfoJson['did'], authdecryptedDidInfoJson['verkey'], role);

    return toDid;
}

async function sendNym(poolHandle, walletHandle, Did, newDid, newKey, role) {
    let nymRequest = await indy.buildNymRequest(Did, newDid, newKey, null, role);
    await indy.signAndSubmitRequest(poolHandle, walletHandle, Did, nymRequest);
}

async function sendSchema(poolHandle, walletHandle, Did, schema) {
    // schema = JSON.stringify(schema); // FIXME: Check JSON parsing
    let schemaRequest = await indy.buildSchemaRequest(Did, schema);
    await indy.signAndSubmitRequest(poolHandle, walletHandle, Did, schemaRequest)
}

async function sendCredDef(poolHandle, walletHandle, did, credDef) {
    let credDefRequest = await indy.buildCredDefRequest(did, credDef);
    await indy.signAndSubmitRequest(poolHandle, walletHandle, did, credDefRequest);
}

async function getSchema(poolHandle, did, schemaId) {
    let getSchemaRequest = await indy.buildGetSchemaRequest(did, schemaId);
    let getSchemaResponse = await indy.submitRequest(poolHandle, getSchemaRequest);
    return await indy.parseGetSchemaResponse(getSchemaResponse);
}

async function getCredDef(poolHandle, did, schemaId) {
    let getCredDefRequest = await indy.buildGetCredDefRequest(did, schemaId);
    let getCredDefResponse = await indy.submitRequest(poolHandle, getCredDefRequest);
    return await indy.parseGetCredDefResponse(getCredDefResponse);
}

async function proverGetEntitiesFromLedger(poolHandle, did, identifiers, actor) {
    let schemas = {};
    let credDefs = {};
    let revStates = {};

    for(let referent of Object.keys(identifiers)) {
        let item = identifiers[referent];
        console.log(`\"${actor}\" -> Get Schema from Ledger`);
        let [receivedSchemaId, receivedSchema] = await getSchema(poolHandle, did, item['schema_id']);
        schemas[receivedSchemaId] = receivedSchema;

        console.log(`\"${actor}\" -> Get Claim Definition from Ledger`);
        let [receivedCredDefId, receivedCredDef] = await getCredDef(poolHandle, did, item['cred_def_id']);
        credDefs[receivedCredDefId] = receivedCredDef;

        if (item.rev_reg_seq_no) {
            // TODO Create Revocation States
        }
    }

    return [schemas, credDefs, revStates];
}


async function verifierGetEntitiesFromLedger(poolHandle, did, identifiers, actor) {
    let schemas = {};
    let credDefs = {};
    let revRegDefs = {};
    let revRegs = {};

    for(let referent of Object.keys(identifiers)) {
        let item = identifiers[referent];
        console.log(`"${actor}" -> Get Schema from Ledger`);
        let [receivedSchemaId, receivedSchema] = await getSchema(poolHandle, did, item['schema_id']);
        schemas[receivedSchemaId] = receivedSchema;

        console.log(`"${actor}" -> Get Claim Definition from Ledger`);
        let [receivedCredDefId, receivedCredDef] = await getCredDef(poolHandle, did, item['cred_def_id']);
        credDefs[receivedCredDefId] = receivedCredDef;

        if (item.rev_reg_seq_no) {
            // TODO Get Revocation Definitions and Revocation Registries
        }
    }

    return [schemas, credDefs, revRegDefs, revRegs];
}

async function authDecrypt(walletHandle, key, message) {
    let [fromVerkey, decryptedMessageJsonBuffer] = await indy.cryptoAuthDecrypt(walletHandle, key, message);
    let decryptedMessage = JSON.parse(decryptedMessageJsonBuffer);
    let decryptedMessageJson = JSON.stringify(decryptedMessage);
    return [fromVerkey, decryptedMessageJson, decryptedMessage];
}

if (require.main.filename == __filename) {
    run()
}

module.exports = {
    run
}

var capi = require('./indyBinding')
var wrapIndyCallback = require('./wrapIndyCallback')
var IndyError = require('./IndyError')

function toJson (val) {
  if (val === null || val === void 0) {
    return null
  }
  if (typeof val === 'string') {
    return val
  }
  return JSON.stringify(val)
}

function fromJson (val) {
  if (typeof val === 'string' && val !== '') {
    try {
      return JSON.parse(val)
    } catch (e) {
    }
  }
  return val
}

var indy = {}

indy.capi = capi // if you want to skip the json dance, IndyError, and promise support

indy.setRuntimeConfig = function setRuntimeConfig (config) {
  var err = capi.setRuntimeConfig(toJson(config))
  if (err !== 0) {
    throw new IndyError(err)
  }
}

indy.setDefaultLogger = function setDefaultLogger (pattern) {
  var err = capi.setDefaultLogger(pattern)
  if (err !== 0) {
    throw new IndyError(err)
  }
}

indy.setLogger = function setLogger (logFn) {
  var err = capi.setLogger(logFn)
  if (err !== 0) {
    throw new IndyError(err)
  }
}

indy.issuerCreateSchema = function issuerCreateSchema (issuerDid, name, version, attrNames, cb) {
  cb = wrapIndyCallback(cb, function (data) {
    return [data[0], fromJson(data[1])]
  })
  capi.issuerCreateSchema(issuerDid, name, version, toJson(attrNames), cb)
  return cb.promise
}

indy.issuerCreateAndStoreCredentialDef = function issuerCreateAndStoreCredentialDef (wh, issuerDid, schema, tag, signatureType, config, cb) {
  cb = wrapIndyCallback(cb, function (data) {
    return [data[0], fromJson(data[1])]
  })
  capi.issuerCreateAndStoreCredentialDef(wh, issuerDid, toJson(schema), tag, signatureType, toJson(config), cb)
  return cb.promise
}

indy.issuerRotateCredentialDefStart = function issuerRotateCredentialDefStart (wh, credDefId, config, cb) {
  cb = wrapIndyCallback(cb, function (data) {
    return fromJson(data[0])
  })
  capi.issuerRotateCredentialDefStart(wh, credDefId, toJson(config), cb)
  return cb.promise
}

indy.issuerRotateCredentialDefApply = function issuerRotateCredentialDefApply (wh, credDefId, cb) {
  cb = wrapIndyCallback(cb)
  capi.issuerRotateCredentialDefApply(wh, credDefId, cb)
  return cb.promise
}

indy.issuerCreateAndStoreRevocReg = function issuerCreateAndStoreRevocReg (wh, issuerDid, revocDefType, tag, credDefId, config, tailsWriterHandle, cb) {
  cb = wrapIndyCallback(cb, function (data) {
    return [data[0], fromJson(data[1]), fromJson(data[2])]
  })
  capi.issuerCreateAndStoreRevocReg(wh, issuerDid, revocDefType, tag, credDefId, toJson(config), tailsWriterHandle, cb)
  return cb.promise
}

indy.issuerCreateCredentialOffer = function issuerCreateCredentialOffer (wh, credDefId, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.issuerCreateCredentialOffer(wh, credDefId, cb)
  return cb.promise
}

indy.issuerCreateCredential = function issuerCreateCredential (wh, credOffer, credReq, credValues, revRegId, blobStorageReaderHandle, cb) {
  cb = wrapIndyCallback(cb, function (data) {
    return [fromJson(data[0]), data[1], fromJson(data[2])]
  })
  capi.issuerCreateCredential(wh, toJson(credOffer), toJson(credReq), toJson(credValues), revRegId, blobStorageReaderHandle, cb)
  return cb.promise
}

indy.issuerRevokeCredential = function issuerRevokeCredential (wh, blobStorageReaderHandle, revRegId, credRevocId, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.issuerRevokeCredential(wh, blobStorageReaderHandle, revRegId, credRevocId, cb)
  return cb.promise
}

indy.issuerMergeRevocationRegistryDeltas = function issuerMergeRevocationRegistryDeltas (revRegDelta, otherRevRegDelta, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.issuerMergeRevocationRegistryDeltas(toJson(revRegDelta), toJson(otherRevRegDelta), cb)
  return cb.promise
}

indy.proverCreateMasterSecret = function proverCreateMasterSecret (wh, masterSecretId, cb) {
  cb = wrapIndyCallback(cb)
  capi.proverCreateMasterSecret(wh, masterSecretId, cb)
  return cb.promise
}

indy.proverCreateCredentialReq = function proverCreateCredentialReq (wh, proverDid, credOffer, credDef, masterSecretId, cb) {
  cb = wrapIndyCallback(cb, function (data) {
    return [fromJson(data[0]), fromJson(data[1])]
  })
  capi.proverCreateCredentialReq(wh, proverDid, toJson(credOffer), toJson(credDef), masterSecretId, cb)
  return cb.promise
}

indy.proverStoreCredential = function proverStoreCredential (wh, credId, credReqMetadata, cred, credDef, revRegDef, cb) {
  cb = wrapIndyCallback(cb)
  capi.proverStoreCredential(wh, credId, toJson(credReqMetadata), toJson(cred), toJson(credDef), toJson(revRegDef), cb)
  return cb.promise
}

indy.proverGetCredentials = function proverGetCredentials (wh, filter, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.proverGetCredentials(wh, toJson(filter), cb)
  return cb.promise
}

indy.proverGetCredential = function proverGetCredential (wh, credId, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.proverGetCredential(wh, credId, cb)
  return cb.promise
}

indy.proverSearchCredentials = function proverSearchCredentials (wh, query, cb) {
  cb = wrapIndyCallback(cb)
  capi.proverSearchCredentials(wh, toJson(query), cb)
  return cb.promise
}

indy.proverFetchCredentials = function proverFetchCredentials (sh, count, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.proverFetchCredentials(sh, count, cb)
  return cb.promise
}

indy.proverCloseCredentialsSearch = function proverCloseCredentialsSearch (sh, cb) {
  cb = wrapIndyCallback(cb)
  capi.proverCloseCredentialsSearch(sh, cb)
  return cb.promise
}

indy.proverGetCredentialsForProofReq = function proverGetCredentialsForProofReq (wh, proofRequest, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.proverGetCredentialsForProofReq(wh, toJson(proofRequest), cb)
  return cb.promise
}

indy.proverSearchCredentialsForProofReq = function proverSearchCredentialsForProofReq (wh, proofRequest, extraQuery, cb) {
  cb = wrapIndyCallback(cb)
  capi.proverSearchCredentialsForProofReq(wh, toJson(proofRequest), toJson(extraQuery), cb)
  return cb.promise
}

indy.proverFetchCredentialsForProofReq = function proverFetchCredentialsForProofReq (sh, itemReferent, count, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.proverFetchCredentialsForProofReq(sh, itemReferent, count, cb)
  return cb.promise
}

indy.proverCloseCredentialsSearchForProofReq = function proverCloseCredentialsSearchForProofReq (sh, cb) {
  cb = wrapIndyCallback(cb)
  capi.proverCloseCredentialsSearchForProofReq(sh, cb)
  return cb.promise
}

indy.proverCreateProof = function proverCreateProof (wh, proofReq, requestedCredentials, masterSecretName, schemas, credentialDefs, revStates, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.proverCreateProof(wh, toJson(proofReq), toJson(requestedCredentials), masterSecretName, toJson(schemas), toJson(credentialDefs), toJson(revStates), cb)
  return cb.promise
}

indy.verifierVerifyProof = function verifierVerifyProof (proofRequest, proof, schemas, credentialDefsJsons, revRegDefs, revRegs, cb) {
  cb = wrapIndyCallback(cb)
  capi.verifierVerifyProof(toJson(proofRequest), toJson(proof), toJson(schemas), toJson(credentialDefsJsons), toJson(revRegDefs), toJson(revRegs), cb)
  return cb.promise
}

indy.createRevocationState = function createRevocationState (blobStorageReaderHandle, revRegDef, revRegDelta, timestamp, credRevId, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.createRevocationState(blobStorageReaderHandle, toJson(revRegDef), toJson(revRegDelta), timestamp, credRevId, cb)
  return cb.promise
}

indy.updateRevocationState = function updateRevocationState (blobStorageReaderHandle, revState, revRegDef, revRegDelta, timestamp, credRevId, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.updateRevocationState(blobStorageReaderHandle, toJson(revState), toJson(revRegDef), toJson(revRegDelta), timestamp, credRevId, cb)
  return cb.promise
}

indy.generateNonce = function generateNonce (cb) {
  cb = wrapIndyCallback(cb)
  capi.generateNonce(cb)
  return cb.promise
}

indy.openBlobStorageReader = function openBlobStorageReader (type, config, cb) {
  cb = wrapIndyCallback(cb)
  capi.openBlobStorageReader(type, toJson(config), cb)
  return cb.promise
}

indy.openBlobStorageWriter = function openBlobStorageWriter (type, config, cb) {
  cb = wrapIndyCallback(cb)
  capi.openBlobStorageWriter(type, toJson(config), cb)
  return cb.promise
}

indy.createKey = function createKey (wh, key, cb) {
  cb = wrapIndyCallback(cb)
  capi.createKey(wh, toJson(key), cb)
  return cb.promise
}

indy.setKeyMetadata = function setKeyMetadata (wh, verkey, metadata, cb) {
  cb = wrapIndyCallback(cb)
  capi.setKeyMetadata(wh, verkey, metadata, cb)
  return cb.promise
}

indy.getKeyMetadata = function getKeyMetadata (wh, verkey, cb) {
  cb = wrapIndyCallback(cb)
  capi.getKeyMetadata(wh, verkey, cb)
  return cb.promise
}

indy.cryptoSign = function cryptoSign (wh, signerVk, messageRaw, cb) {
  cb = wrapIndyCallback(cb)
  capi.cryptoSign(wh, signerVk, messageRaw, cb)
  return cb.promise
}

indy.cryptoVerify = function cryptoVerify (signerVk, messageRaw, signatureRaw, cb) {
  cb = wrapIndyCallback(cb)
  capi.cryptoVerify(signerVk, messageRaw, signatureRaw, cb)
  return cb.promise
}

indy.cryptoAuthCrypt = function cryptoAuthCrypt (wh, senderVk, recipientVk, messageRaw, cb) {
  cb = wrapIndyCallback(cb)
  capi.cryptoAuthCrypt(wh, senderVk, recipientVk, messageRaw, cb)
  return cb.promise
}

indy.cryptoAuthDecrypt = function cryptoAuthDecrypt (wh, recipientVk, encryptedMsgRaw, cb) {
  cb = wrapIndyCallback(cb)
  capi.cryptoAuthDecrypt(wh, recipientVk, encryptedMsgRaw, cb)
  return cb.promise
}

indy.cryptoAnonCrypt = function cryptoAnonCrypt (recipientVk, messageRaw, cb) {
  cb = wrapIndyCallback(cb)
  capi.cryptoAnonCrypt(recipientVk, messageRaw, cb)
  return cb.promise
}

indy.cryptoAnonDecrypt = function cryptoAnonDecrypt (wh, recipientVk, encryptedMsg, cb) {
  cb = wrapIndyCallback(cb)
  capi.cryptoAnonDecrypt(wh, recipientVk, encryptedMsg, cb)
  return cb.promise
}

indy.packMessage = function packMessage (wh, message, receiverKeys, sender, cb) {
  cb = wrapIndyCallback(cb)
  capi.packMessage(wh, message, toJson(receiverKeys), sender, cb)
  return cb.promise
}

indy.unpackMessage = function unpackMessage (wh, jwe, cb) {
  cb = wrapIndyCallback(cb)
  capi.unpackMessage(wh, jwe, cb)
  return cb.promise
}

indy.createAndStoreMyDid = function createAndStoreMyDid (wh, did, cb) {
  cb = wrapIndyCallback(cb)
  capi.createAndStoreMyDid(wh, toJson(did), cb)
  return cb.promise
}

indy.replaceKeysStart = function replaceKeysStart (wh, did, identity, cb) {
  cb = wrapIndyCallback(cb)
  capi.replaceKeysStart(wh, did, toJson(identity), cb)
  return cb.promise
}

indy.replaceKeysApply = function replaceKeysApply (wh, did, cb) {
  cb = wrapIndyCallback(cb)
  capi.replaceKeysApply(wh, did, cb)
  return cb.promise
}

indy.storeTheirDid = function storeTheirDid (wh, identity, cb) {
  cb = wrapIndyCallback(cb)
  capi.storeTheirDid(wh, toJson(identity), cb)
  return cb.promise
}

indy.keyForDid = function keyForDid (poolHandle, wh, did, cb) {
  cb = wrapIndyCallback(cb)
  capi.keyForDid(poolHandle, wh, did, cb)
  return cb.promise
}

indy.keyForLocalDid = function keyForLocalDid (wh, did, cb) {
  cb = wrapIndyCallback(cb)
  capi.keyForLocalDid(wh, did, cb)
  return cb.promise
}

indy.setEndpointForDid = function setEndpointForDid (wh, did, address, transportKey, cb) {
  cb = wrapIndyCallback(cb)
  capi.setEndpointForDid(wh, did, address, transportKey, cb)
  return cb.promise
}

indy.getEndpointForDid = function getEndpointForDid (wh, poolHandle, did, cb) {
  cb = wrapIndyCallback(cb)
  capi.getEndpointForDid(wh, poolHandle, did, cb)
  return cb.promise
}

indy.setDidMetadata = function setDidMetadata (wh, did, metadata, cb) {
  cb = wrapIndyCallback(cb)
  capi.setDidMetadata(wh, did, metadata, cb)
  return cb.promise
}

indy.getDidMetadata = function getDidMetadata (wh, did, cb) {
  cb = wrapIndyCallback(cb)
  capi.getDidMetadata(wh, did, cb)
  return cb.promise
}

indy.getMyDidWithMeta = function getMyDidWithMeta (wh, myDid, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.getMyDidWithMeta(wh, myDid, cb)
  return cb.promise
}

indy.listMyDidsWithMeta = function listMyDidsWithMeta (wh, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.listMyDidsWithMeta(wh, cb)
  return cb.promise
}

indy.abbreviateVerkey = function abbreviateVerkey (did, fullVerkey, cb) {
  cb = wrapIndyCallback(cb)
  capi.abbreviateVerkey(did, fullVerkey, cb)
  return cb.promise
}

indy.signAndSubmitRequest = function signAndSubmitRequest (poolHandle, wh, submitterDid, request, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.signAndSubmitRequest(poolHandle, wh, submitterDid, toJson(request), cb)
  return cb.promise
}

indy.submitRequest = function submitRequest (poolHandle, request, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.submitRequest(poolHandle, toJson(request), cb)
  return cb.promise
}

indy.submitAction = function submitAction (poolHandle, request, nodes, timeout, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.submitAction(poolHandle, toJson(request), toJson(nodes), timeout == null ? -1 : timeout, cb)
  return cb.promise
}

indy.signRequest = function signRequest (wh, submitterDid, request, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.signRequest(wh, submitterDid, toJson(request), cb)
  return cb.promise
}

indy.multiSignRequest = function multiSignRequest (wh, submitterDid, request, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.multiSignRequest(wh, submitterDid, toJson(request), cb)
  return cb.promise
}

indy.buildGetDdoRequest = function buildGetDdoRequest (submitterDid, targetDid, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.buildGetDdoRequest(submitterDid, targetDid, cb)
  return cb.promise
}

indy.buildNymRequest = function buildNymRequest (submitterDid, targetDid, verkey, alias, role, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.buildNymRequest(submitterDid, targetDid, verkey, alias, role, cb)
  return cb.promise
}

indy.buildAttribRequest = function buildAttribRequest (submitterDid, targetDid, hash, raw, enc, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.buildAttribRequest(submitterDid, targetDid, hash, toJson(raw), enc, cb)
  return cb.promise
}

indy.buildGetAttribRequest = function buildGetAttribRequest (submitterDid, targetDid, hash, raw, enc, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.buildGetAttribRequest(submitterDid, targetDid, hash, raw, enc, cb)
  return cb.promise
}

indy.buildGetNymRequest = function buildGetNymRequest (submitterDid, targetDid, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.buildGetNymRequest(submitterDid, targetDid, cb)
  return cb.promise
}

indy.buildSchemaRequest = function buildSchemaRequest (submitterDid, data, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.buildSchemaRequest(submitterDid, toJson(data), cb)
  return cb.promise
}

indy.buildGetSchemaRequest = function buildGetSchemaRequest (submitterDid, id, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.buildGetSchemaRequest(submitterDid, id, cb)
  return cb.promise
}

indy.parseGetSchemaResponse = function parseGetSchemaResponse (getSchemaResponse, cb) {
  cb = wrapIndyCallback(cb, function (data) {
    return [data[0], fromJson(data[1])]
  })
  capi.parseGetSchemaResponse(toJson(getSchemaResponse), cb)
  return cb.promise
}

indy.buildCredDefRequest = function buildCredDefRequest (submitterDid, data, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.buildCredDefRequest(submitterDid, toJson(data), cb)
  return cb.promise
}

indy.buildGetCredDefRequest = function buildGetCredDefRequest (submitterDid, id, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.buildGetCredDefRequest(submitterDid, id, cb)
  return cb.promise
}

indy.parseGetCredDefResponse = function parseGetCredDefResponse (getCredDefResponse, cb) {
  cb = wrapIndyCallback(cb, function (data) {
    return [data[0], fromJson(data[1])]
  })
  capi.parseGetCredDefResponse(toJson(getCredDefResponse), cb)
  return cb.promise
}

indy.buildNodeRequest = function buildNodeRequest (submitterDid, targetDid, data, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.buildNodeRequest(submitterDid, targetDid, toJson(data), cb)
  return cb.promise
}

indy.buildGetValidatorInfoRequest = function buildGetValidatorInfoRequest (submitterDid, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.buildGetValidatorInfoRequest(submitterDid, cb)
  return cb.promise
}

indy.buildGetTxnRequest = function buildGetTxnRequest (submitterDid, ledgerType, seqNo, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.buildGetTxnRequest(submitterDid, ledgerType, seqNo, cb)
  return cb.promise
}

indy.buildPoolConfigRequest = function buildPoolConfigRequest (submitterDid, writes, force, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.buildPoolConfigRequest(submitterDid, writes, force, cb)
  return cb.promise
}

indy.buildPoolRestartRequest = function buildPoolRestartRequest (submitterDid, action, datetime, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.buildPoolRestartRequest(submitterDid, action, datetime, cb)
  return cb.promise
}

indy.buildPoolUpgradeRequest = function buildPoolUpgradeRequest (submitterDid, name, version, action, sha256, timeout, schedule, justification, reinstall, force, package_, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.buildPoolUpgradeRequest(submitterDid, name, version, action, sha256, timeout, schedule, justification, reinstall, force, package_, cb)
  return cb.promise
}

indy.buildRevocRegDefRequest = function buildRevocRegDefRequest (submitterDid, data, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.buildRevocRegDefRequest(submitterDid, toJson(data), cb)
  return cb.promise
}

indy.buildGetRevocRegDefRequest = function buildGetRevocRegDefRequest (submitterDid, id, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.buildGetRevocRegDefRequest(submitterDid, id, cb)
  return cb.promise
}

indy.parseGetRevocRegDefResponse = function parseGetRevocRegDefResponse (getRevocRefDefResponse, cb) {
  cb = wrapIndyCallback(cb, function (data) {
    return [data[0], fromJson(data[1])]
  })
  capi.parseGetRevocRegDefResponse(toJson(getRevocRefDefResponse), cb)
  return cb.promise
}

indy.buildRevocRegEntryRequest = function buildRevocRegEntryRequest (submitterDid, revocRegDefId, revDefType, value, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.buildRevocRegEntryRequest(submitterDid, revocRegDefId, revDefType, toJson(value), cb)
  return cb.promise
}

indy.buildGetRevocRegRequest = function buildGetRevocRegRequest (submitterDid, revocRegDefId, timestamp, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.buildGetRevocRegRequest(submitterDid, revocRegDefId, timestamp, cb)
  return cb.promise
}

indy.parseGetRevocRegResponse = function parseGetRevocRegResponse (getRevocRegResponse, cb) {
  cb = wrapIndyCallback(cb, function (data) {
    return [data[0], fromJson(data[1]), data[2]]
  })
  capi.parseGetRevocRegResponse(toJson(getRevocRegResponse), cb)
  return cb.promise
}

indy.buildGetRevocRegDeltaRequest = function buildGetRevocRegDeltaRequest (submitterDid, revocRegDefId, from, to, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.buildGetRevocRegDeltaRequest(submitterDid, revocRegDefId, from, to, cb)
  return cb.promise
}

indy.parseGetRevocRegDeltaResponse = function parseGetRevocRegDeltaResponse (getRevocRegDeltaResponse, cb) {
  cb = wrapIndyCallback(cb, function (data) {
    return [data[0], fromJson(data[1]), data[2]]
  })
  capi.parseGetRevocRegDeltaResponse(toJson(getRevocRegDeltaResponse), cb)
  return cb.promise
}

indy.buildAuthRuleRequest = function buildAuthRuleRequest (submitterDid, txnType, action, field, oldValue, newValue, constraint, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.buildAuthRuleRequest(submitterDid, txnType, action, field, oldValue, newValue, toJson(constraint), cb)
  return cb.promise
}

indy.buildAuthRulesRequest = function buildAuthRulesRequest (submitterDid, data, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.buildAuthRulesRequest(submitterDid, toJson(data), cb)
  return cb.promise
}

indy.buildGetAuthRuleRequest = function buildGetAuthRuleRequest (submitterDid, txnType, action, field, oldValue, newValue, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.buildGetAuthRuleRequest(submitterDid, txnType, action, field, oldValue, newValue, cb)
  return cb.promise
}

indy.buildTxnAuthorAgreementRequest = function buildTxnAuthorAgreementRequest (submitterDid, text, version, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.buildTxnAuthorAgreementRequest(submitterDid, text, version, cb)
  return cb.promise
}

indy.buildGetTxnAuthorAgreementRequest = function buildGetTxnAuthorAgreementRequest (submitterDid, data, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.buildGetTxnAuthorAgreementRequest(submitterDid, toJson(data), cb)
  return cb.promise
}

indy.buildAcceptanceMechanismsRequest = function buildAcceptanceMechanismsRequest (submitterDid, aml, version, amlContext, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.buildAcceptanceMechanismsRequest(submitterDid, toJson(aml), version, amlContext, cb)
  return cb.promise
}

indy.buildGetAcceptanceMechanismsRequest = function buildGetAcceptanceMechanismsRequest (submitterDid, timestamp, version, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.buildGetAcceptanceMechanismsRequest(submitterDid, timestamp == null ? -1 : timestamp, version, cb)
  return cb.promise
}

indy.appendTxnAuthorAgreementAcceptanceToRequest = function appendTxnAuthorAgreementAcceptanceToRequest (request, text, version, taaDigest, accMechType, timeOfAcceptance, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.appendTxnAuthorAgreementAcceptanceToRequest(toJson(request), text, version, taaDigest, accMechType, timeOfAcceptance, cb)
  return cb.promise
}

indy.appendRequestEndorser = function appendRequestEndorser (request, endorserDid, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.appendRequestEndorser(toJson(request), endorserDid, cb)
  return cb.promise
}

indy.getResponseMetadata = function getResponseMetadata (response, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.getResponseMetadata(toJson(response), cb)
  return cb.promise
}

indy.addWalletRecord = function addWalletRecord (wh, type, id, value, tags, cb) {
  cb = wrapIndyCallback(cb)
  capi.addWalletRecord(wh, type, id, value, toJson(tags), cb)
  return cb.promise
}

indy.updateWalletRecordValue = function updateWalletRecordValue (wh, type, id, value, cb) {
  cb = wrapIndyCallback(cb)
  capi.updateWalletRecordValue(wh, type, id, value, cb)
  return cb.promise
}

indy.updateWalletRecordTags = function updateWalletRecordTags (wh, type, id, tags, cb) {
  cb = wrapIndyCallback(cb)
  capi.updateWalletRecordTags(wh, type, id, toJson(tags), cb)
  return cb.promise
}

indy.addWalletRecordTags = function addWalletRecordTags (wh, type, id, tags, cb) {
  cb = wrapIndyCallback(cb)
  capi.addWalletRecordTags(wh, type, id, toJson(tags), cb)
  return cb.promise
}

indy.deleteWalletRecordTags = function deleteWalletRecordTags (wh, type, id, tagNames, cb) {
  cb = wrapIndyCallback(cb)
  capi.deleteWalletRecordTags(wh, type, id, toJson(tagNames), cb)
  return cb.promise
}

indy.deleteWalletRecord = function deleteWalletRecord (wh, type, id, cb) {
  cb = wrapIndyCallback(cb)
  capi.deleteWalletRecord(wh, type, id, cb)
  return cb.promise
}

indy.getWalletRecord = function getWalletRecord (wh, type, id, options, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.getWalletRecord(wh, type, id, toJson(options), cb)
  return cb.promise
}

indy.openWalletSearch = function openWalletSearch (wh, type, query, options, cb) {
  cb = wrapIndyCallback(cb)
  capi.openWalletSearch(wh, type, toJson(query), toJson(options), cb)
  return cb.promise
}

indy.fetchWalletSearchNextRecords = function fetchWalletSearchNextRecords (wh, walletSearchHandle, count, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.fetchWalletSearchNextRecords(wh, walletSearchHandle, count, cb)
  return cb.promise
}

indy.closeWalletSearch = function closeWalletSearch (walletSearchHandle, cb) {
  cb = wrapIndyCallback(cb)
  capi.closeWalletSearch(walletSearchHandle, cb)
  return cb.promise
}

indy.getSchema = function getSchema (poolHandle, wh, submitterDid, id, options, cb) {
  cb = wrapIndyCallback(cb, function (data) {
    return fromJson(data)
  })
  capi.getSchema(poolHandle, wh, submitterDid, id, toJson(options), cb)
  return cb.promise
}

indy.getCredDef = function getCredDef (poolHandle, wh, submitterDid, id, options, cb) {
  cb = wrapIndyCallback(cb, function (data) {
    return fromJson(data)
  })
  capi.getCredDef(poolHandle, wh, submitterDid, id, toJson(options), cb)
  return cb.promise
}

indy.purgeSchemaCache = function purgeSchemaCache (wh, options, cb) {
  cb = wrapIndyCallback(cb)
  capi.purgeSchemaCache(wh, toJson(options), cb)
  return cb.promise
}

indy.purgeCredDefCache = function purgeCredDefCache (wh, options, cb) {
  cb = wrapIndyCallback(cb)
  capi.purgeCredDefCache(wh, toJson(options), cb)
  return cb.promise
}

indy.isPairwiseExists = function isPairwiseExists (wh, theirDid, cb) {
  cb = wrapIndyCallback(cb)
  capi.isPairwiseExists(wh, theirDid, cb)
  return cb.promise
}

indy.createPairwise = function createPairwise (wh, theirDid, myDid, metadata, cb) {
  cb = wrapIndyCallback(cb)
  capi.createPairwise(wh, theirDid, myDid, metadata, cb)
  return cb.promise
}

indy.listPairwise = function listPairwise (wh, cb) {
  cb = wrapIndyCallback(cb, function (data) {
    return fromJson(data).map(fromJson)
  })
  capi.listPairwise(wh, cb)
  return cb.promise
}

indy.getPairwise = function getPairwise (wh, theirDid, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.getPairwise(wh, theirDid, cb)
  return cb.promise
}

indy.setPairwiseMetadata = function setPairwiseMetadata (wh, theirDid, metadata, cb) {
  cb = wrapIndyCallback(cb)
  capi.setPairwiseMetadata(wh, theirDid, metadata, cb)
  return cb.promise
}

indy.createPaymentAddress = function createPaymentAddress (wh, paymentMethod, config, cb) {
  cb = wrapIndyCallback(cb)
  capi.createPaymentAddress(wh, paymentMethod, toJson(config), cb)
  return cb.promise
}

indy.listPaymentAddresses = function listPaymentAddresses (wh, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.listPaymentAddresses(wh, cb)
  return cb.promise
}

indy.addRequestFees = function addRequestFees (wh, submitterDid, req, inputs, outputs, extra, cb) {
  cb = wrapIndyCallback(cb, function (data) {
    return [fromJson(data[0]), data[1]]
  })
  capi.addRequestFees(wh, submitterDid, toJson(req), toJson(inputs), toJson(outputs), extra, cb)
  return cb.promise
}

indy.parseResponseWithFees = function parseResponseWithFees (paymentMethod, resp, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.parseResponseWithFees(paymentMethod, toJson(resp), cb)
  return cb.promise
}

indy.buildGetPaymentSourcesRequest = function buildGetPaymentSourcesRequest (wh, submitterDid, paymentAddress, cb) {
  cb = wrapIndyCallback(cb, function (data) {
    return [fromJson(data[0]), data[1]]
  })
  capi.buildGetPaymentSourcesRequest(wh, submitterDid, paymentAddress, cb)
  return cb.promise
}

indy.buildGetPaymentSourcesWithFromRequest = function buildGetPaymentSourcesWithFromRequest (wh, submitterDid, paymentAddress, from, cb) {
  cb = wrapIndyCallback(cb, function (data) {
    return [fromJson(data[0]), data[1]]
  })
  capi.buildGetPaymentSourcesWithFromRequest(wh, submitterDid, paymentAddress, from, cb)
  return cb.promise
}

indy.parseGetPaymentSourcesResponse = function parseGetPaymentSourcesResponse (paymentMethod, resp, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.parseGetPaymentSourcesResponse(paymentMethod, toJson(resp), cb)
  return cb.promise
}

indy.parseGetPaymentSourcesWithFromResponse = function parseGetPaymentSourcesWithFromResponse (paymentMethod, resp, cb) {
  cb = wrapIndyCallback(cb, function (data) {
    return [fromJson(data[0]), data[1]]
  })
  capi.parseGetPaymentSourcesWithFromResponse(paymentMethod, toJson(resp), cb)
  return cb.promise
}

indy.buildPaymentReq = function buildPaymentReq (wh, submitterDid, inputs, outputs, extra, cb) {
  cb = wrapIndyCallback(cb, function (data) {
    return [fromJson(data[0]), data[1]]
  })
  capi.buildPaymentReq(wh, submitterDid, toJson(inputs), toJson(outputs), extra, cb)
  return cb.promise
}

indy.parsePaymentResponse = function parsePaymentResponse (paymentMethod, resp, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.parsePaymentResponse(paymentMethod, toJson(resp), cb)
  return cb.promise
}

indy.preparePaymentExtraWithAcceptanceData = function preparePaymentExtraWithAcceptanceData (extra, text, version, taaDigest, accMechType, timeOfAcceptance, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.preparePaymentExtraWithAcceptanceData(toJson(extra), text, version, taaDigest, accMechType, timeOfAcceptance, cb)
  return cb.promise
}

indy.buildMintReq = function buildMintReq (wh, submitterDid, outputs, extra, cb) {
  cb = wrapIndyCallback(cb, function (data) {
    return [fromJson(data[0]), data[1]]
  })
  capi.buildMintReq(wh, submitterDid, toJson(outputs), extra, cb)
  return cb.promise
}

indy.buildSetTxnFeesReq = function buildSetTxnFeesReq (wh, submitterDid, paymentMethod, fees, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.buildSetTxnFeesReq(wh, submitterDid, paymentMethod, toJson(fees), cb)
  return cb.promise
}

indy.buildGetTxnFeesReq = function buildGetTxnFeesReq (wh, submitterDid, paymentMethod, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.buildGetTxnFeesReq(wh, submitterDid, paymentMethod, cb)
  return cb.promise
}

indy.parseGetTxnFeesResponse = function parseGetTxnFeesResponse (paymentMethod, resp, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.parseGetTxnFeesResponse(paymentMethod, toJson(resp), cb)
  return cb.promise
}

indy.buildVerifyPaymentReq = function buildVerifyPaymentReq (wh, submitterDid, receipt, cb) {
  cb = wrapIndyCallback(cb, function (data) {
    return [fromJson(data[0]), data[1]]
  })
  capi.buildVerifyPaymentReq(wh, submitterDid, receipt, cb)
  return cb.promise
}

indy.parseVerifyPaymentResponse = function parseVerifyPaymentResponse (paymentMethod, resp, cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.parseVerifyPaymentResponse(paymentMethod, toJson(resp), cb)
  return cb.promise
}

indy.getRequestInfo = function getRequestInfo (getAuthRuleResponse, requesterInfo, fees, cb) {
  cb = wrapIndyCallback(cb, function (data) {
    return fromJson(data)
  })
  capi.getRequestInfo(toJson(getAuthRuleResponse), toJson(requesterInfo), toJson(fees), cb)
  return cb.promise
}

indy.signWithAddress = function signWithAddress (wh, address, message, cb) {
  cb = wrapIndyCallback(cb)
  capi.signWithAddress(wh, address, message, cb)
  return cb.promise
}

indy.verifyWithAddress = function verifyWithAddress (address, message, signature, cb) {
  cb = wrapIndyCallback(cb)
  capi.verifyWithAddress(address, message, signature, cb)
  return cb.promise
}

indy.createPoolLedgerConfig = function createPoolLedgerConfig (configName, config, cb) {
  cb = wrapIndyCallback(cb)
  capi.createPoolLedgerConfig(configName, toJson(config), cb)
  return cb.promise
}

indy.openPoolLedger = function openPoolLedger (configName, config, cb) {
  cb = wrapIndyCallback(cb)
  capi.openPoolLedger(configName, toJson(config), cb)
  return cb.promise
}

indy.refreshPoolLedger = function refreshPoolLedger (handle, cb) {
  cb = wrapIndyCallback(cb)
  capi.refreshPoolLedger(handle, cb)
  return cb.promise
}

indy.listPools = function listPools (cb) {
  cb = wrapIndyCallback(cb, fromJson)
  capi.listPools(cb)
  return cb.promise
}

indy.closePoolLedger = function closePoolLedger (handle, cb) {
  cb = wrapIndyCallback(cb)
  capi.closePoolLedger(handle, cb)
  return cb.promise
}

indy.deletePoolLedgerConfig = function deletePoolLedgerConfig (configName, cb) {
  cb = wrapIndyCallback(cb)
  capi.deletePoolLedgerConfig(configName, cb)
  return cb.promise
}

indy.setProtocolVersion = function setProtocolVersion (protocolVersion, cb) {
  cb = wrapIndyCallback(cb)
  capi.setProtocolVersion(protocolVersion, cb)
  return cb.promise
}

indy.createWallet = function createWallet (config, credentials, cb) {
  cb = wrapIndyCallback(cb)
  capi.createWallet(toJson(config), toJson(credentials), cb)
  return cb.promise
}

indy.openWallet = function openWallet (config, credentials, cb) {
  cb = wrapIndyCallback(cb)
  capi.openWallet(toJson(config), toJson(credentials), cb)
  return cb.promise
}

indy.exportWallet = function exportWallet (wh, exportConfig, cb) {
  cb = wrapIndyCallback(cb)
  capi.exportWallet(wh, toJson(exportConfig), cb)
  return cb.promise
}

indy.importWallet = function importWallet (config, credentials, importConfig, cb) {
  cb = wrapIndyCallback(cb)
  capi.importWallet(toJson(config), toJson(credentials), toJson(importConfig), cb)
  return cb.promise
}

indy.closeWallet = function closeWallet (wh, cb) {
  cb = wrapIndyCallback(cb)
  capi.closeWallet(wh, cb)
  return cb.promise
}

indy.deleteWallet = function deleteWallet (config, credentials, cb) {
  cb = wrapIndyCallback(cb)
  capi.deleteWallet(toJson(config), toJson(credentials), cb)
  return cb.promise
}

indy.generateWalletKey = function generateWalletKey (config, cb) {
  cb = wrapIndyCallback(cb)
  capi.generateWalletKey(toJson(config), cb)
  return cb.promise
}

module.exports = indy

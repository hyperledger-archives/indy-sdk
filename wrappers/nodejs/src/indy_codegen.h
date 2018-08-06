void issuerCreateSchema_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0, const char* arg1) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringString(xerr, arg0, arg1);
  }
}
NAN_METHOD(issuerCreateSchema) {
  INDY_ASSERT_NARGS(issuerCreateSchema, 5)
  INDY_ASSERT_STRING(issuerCreateSchema, 0, issuerDid)
  INDY_ASSERT_STRING(issuerCreateSchema, 1, name)
  INDY_ASSERT_STRING(issuerCreateSchema, 2, version)
  INDY_ASSERT_STRING(issuerCreateSchema, 3, attrNames)
  INDY_ASSERT_FUNCTION(issuerCreateSchema, 4)
  const char* arg0 = argToCString(info[0]);
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  const char* arg3 = argToCString(info[3]);
  IndyCallback* icb = argToIndyCb(info[4]);
  indyCalled(icb, indy_issuer_create_schema(icb->handle, arg0, arg1, arg2, arg3, issuerCreateSchema_cb));
  delete arg0;
  delete arg1;
  delete arg2;
  delete arg3;
}

void issuerCreateAndStoreCredentialDef_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0, const char* arg1) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringString(xerr, arg0, arg1);
  }
}
NAN_METHOD(issuerCreateAndStoreCredentialDef) {
  INDY_ASSERT_NARGS(issuerCreateAndStoreCredentialDef, 7)
  INDY_ASSERT_NUMBER(issuerCreateAndStoreCredentialDef, 0, wh)
  INDY_ASSERT_STRING(issuerCreateAndStoreCredentialDef, 1, issuerDid)
  INDY_ASSERT_STRING(issuerCreateAndStoreCredentialDef, 2, schema)
  INDY_ASSERT_STRING(issuerCreateAndStoreCredentialDef, 3, tag)
  INDY_ASSERT_STRING(issuerCreateAndStoreCredentialDef, 4, signatureType)
  INDY_ASSERT_STRING(issuerCreateAndStoreCredentialDef, 5, config)
  INDY_ASSERT_FUNCTION(issuerCreateAndStoreCredentialDef, 6)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  const char* arg3 = argToCString(info[3]);
  const char* arg4 = argToCString(info[4]);
  const char* arg5 = argToCString(info[5]);
  IndyCallback* icb = argToIndyCb(info[6]);
  indyCalled(icb, indy_issuer_create_and_store_credential_def(icb->handle, arg0, arg1, arg2, arg3, arg4, arg5, issuerCreateAndStoreCredentialDef_cb));
  delete arg1;
  delete arg2;
  delete arg3;
  delete arg4;
  delete arg5;
}

void issuerCreateAndStoreRevocReg_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0, const char* arg1, const char* arg2) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringStringString(xerr, arg0, arg1, arg2);
  }
}
NAN_METHOD(issuerCreateAndStoreRevocReg) {
  INDY_ASSERT_NARGS(issuerCreateAndStoreRevocReg, 8)
  INDY_ASSERT_NUMBER(issuerCreateAndStoreRevocReg, 0, wh)
  INDY_ASSERT_STRING(issuerCreateAndStoreRevocReg, 1, issuerDid)
  INDY_ASSERT_STRING(issuerCreateAndStoreRevocReg, 2, revocDefType)
  INDY_ASSERT_STRING(issuerCreateAndStoreRevocReg, 3, tag)
  INDY_ASSERT_STRING(issuerCreateAndStoreRevocReg, 4, credDefId)
  INDY_ASSERT_STRING(issuerCreateAndStoreRevocReg, 5, config)
  INDY_ASSERT_NUMBER(issuerCreateAndStoreRevocReg, 6, tailsWriterHandle)
  INDY_ASSERT_FUNCTION(issuerCreateAndStoreRevocReg, 7)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  const char* arg3 = argToCString(info[3]);
  const char* arg4 = argToCString(info[4]);
  const char* arg5 = argToCString(info[5]);
  indy_handle_t arg6 = info[6]->Int32Value();
  IndyCallback* icb = argToIndyCb(info[7]);
  indyCalled(icb, indy_issuer_create_and_store_revoc_reg(icb->handle, arg0, arg1, arg2, arg3, arg4, arg5, arg6, issuerCreateAndStoreRevocReg_cb));
  delete arg1;
  delete arg2;
  delete arg3;
  delete arg4;
  delete arg5;
}

void issuerCreateCredentialOffer_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(issuerCreateCredentialOffer) {
  INDY_ASSERT_NARGS(issuerCreateCredentialOffer, 3)
  INDY_ASSERT_NUMBER(issuerCreateCredentialOffer, 0, wh)
  INDY_ASSERT_STRING(issuerCreateCredentialOffer, 1, credDefId)
  INDY_ASSERT_FUNCTION(issuerCreateCredentialOffer, 2)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_issuer_create_credential_offer(icb->handle, arg0, arg1, issuerCreateCredentialOffer_cb));
  delete arg1;
}

void issuerCreateCredential_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0, const char* arg1, const char* arg2) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringStringString(xerr, arg0, arg1, arg2);
  }
}
NAN_METHOD(issuerCreateCredential) {
  INDY_ASSERT_NARGS(issuerCreateCredential, 7)
  INDY_ASSERT_NUMBER(issuerCreateCredential, 0, wh)
  INDY_ASSERT_STRING(issuerCreateCredential, 1, credOffer)
  INDY_ASSERT_STRING(issuerCreateCredential, 2, credReq)
  INDY_ASSERT_STRING(issuerCreateCredential, 3, credValues)
  INDY_ASSERT_STRING(issuerCreateCredential, 4, revRegId)
  INDY_ASSERT_NUMBER(issuerCreateCredential, 5, blobStorageReaderHandle)
  INDY_ASSERT_FUNCTION(issuerCreateCredential, 6)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  const char* arg3 = argToCString(info[3]);
  const char* arg4 = argToCString(info[4]);
  indy_i32_t arg5 = info[5]->Int32Value();
  IndyCallback* icb = argToIndyCb(info[6]);
  indyCalled(icb, indy_issuer_create_credential(icb->handle, arg0, arg1, arg2, arg3, arg4, arg5, issuerCreateCredential_cb));
  delete arg1;
  delete arg2;
  delete arg3;
  delete arg4;
}

void issuerRevokeCredential_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(issuerRevokeCredential) {
  INDY_ASSERT_NARGS(issuerRevokeCredential, 5)
  INDY_ASSERT_NUMBER(issuerRevokeCredential, 0, wh)
  INDY_ASSERT_NUMBER(issuerRevokeCredential, 1, blobStorageReaderHandle)
  INDY_ASSERT_STRING(issuerRevokeCredential, 2, revRegId)
  INDY_ASSERT_STRING(issuerRevokeCredential, 3, credRevocId)
  INDY_ASSERT_FUNCTION(issuerRevokeCredential, 4)
  indy_handle_t arg0 = info[0]->Int32Value();
  indy_i32_t arg1 = info[1]->Int32Value();
  const char* arg2 = argToCString(info[2]);
  const char* arg3 = argToCString(info[3]);
  IndyCallback* icb = argToIndyCb(info[4]);
  indyCalled(icb, indy_issuer_revoke_credential(icb->handle, arg0, arg1, arg2, arg3, issuerRevokeCredential_cb));
  delete arg2;
  delete arg3;
}

void issuerMergeRevocationRegistryDeltas_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(issuerMergeRevocationRegistryDeltas) {
  INDY_ASSERT_NARGS(issuerMergeRevocationRegistryDeltas, 3)
  INDY_ASSERT_STRING(issuerMergeRevocationRegistryDeltas, 0, revRegDelta)
  INDY_ASSERT_STRING(issuerMergeRevocationRegistryDeltas, 1, otherRevRegDelta)
  INDY_ASSERT_FUNCTION(issuerMergeRevocationRegistryDeltas, 2)
  const char* arg0 = argToCString(info[0]);
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_issuer_merge_revocation_registry_deltas(icb->handle, arg0, arg1, issuerMergeRevocationRegistryDeltas_cb));
  delete arg0;
  delete arg1;
}

void proverCreateMasterSecret_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(proverCreateMasterSecret) {
  INDY_ASSERT_NARGS(proverCreateMasterSecret, 3)
  INDY_ASSERT_NUMBER(proverCreateMasterSecret, 0, wh)
  INDY_ASSERT_STRING(proverCreateMasterSecret, 1, masterSecretId)
  INDY_ASSERT_FUNCTION(proverCreateMasterSecret, 2)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_prover_create_master_secret(icb->handle, arg0, arg1, proverCreateMasterSecret_cb));
  delete arg1;
}

void proverCreateCredentialReq_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0, const char* arg1) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringString(xerr, arg0, arg1);
  }
}
NAN_METHOD(proverCreateCredentialReq) {
  INDY_ASSERT_NARGS(proverCreateCredentialReq, 6)
  INDY_ASSERT_NUMBER(proverCreateCredentialReq, 0, wh)
  INDY_ASSERT_STRING(proverCreateCredentialReq, 1, proverDid)
  INDY_ASSERT_STRING(proverCreateCredentialReq, 2, credOffer)
  INDY_ASSERT_STRING(proverCreateCredentialReq, 3, credDef)
  INDY_ASSERT_STRING(proverCreateCredentialReq, 4, masterSecretId)
  INDY_ASSERT_FUNCTION(proverCreateCredentialReq, 5)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  const char* arg3 = argToCString(info[3]);
  const char* arg4 = argToCString(info[4]);
  IndyCallback* icb = argToIndyCb(info[5]);
  indyCalled(icb, indy_prover_create_credential_req(icb->handle, arg0, arg1, arg2, arg3, arg4, proverCreateCredentialReq_cb));
  delete arg1;
  delete arg2;
  delete arg3;
  delete arg4;
}

void proverStoreCredential_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(proverStoreCredential) {
  INDY_ASSERT_NARGS(proverStoreCredential, 7)
  INDY_ASSERT_NUMBER(proverStoreCredential, 0, wh)
  INDY_ASSERT_STRING(proverStoreCredential, 1, credId)
  INDY_ASSERT_STRING(proverStoreCredential, 2, credReqMetadata)
  INDY_ASSERT_STRING(proverStoreCredential, 3, cred)
  INDY_ASSERT_STRING(proverStoreCredential, 4, credDef)
  INDY_ASSERT_STRING(proverStoreCredential, 5, revRegDef)
  INDY_ASSERT_FUNCTION(proverStoreCredential, 6)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  const char* arg3 = argToCString(info[3]);
  const char* arg4 = argToCString(info[4]);
  const char* arg5 = argToCString(info[5]);
  IndyCallback* icb = argToIndyCb(info[6]);
  indyCalled(icb, indy_prover_store_credential(icb->handle, arg0, arg1, arg2, arg3, arg4, arg5, proverStoreCredential_cb));
  delete arg1;
  delete arg2;
  delete arg3;
  delete arg4;
  delete arg5;
}

void proverGetCredentials_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(proverGetCredentials) {
  INDY_ASSERT_NARGS(proverGetCredentials, 3)
  INDY_ASSERT_NUMBER(proverGetCredentials, 0, wh)
  INDY_ASSERT_STRING(proverGetCredentials, 1, filter)
  INDY_ASSERT_FUNCTION(proverGetCredentials, 2)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_prover_get_credentials(icb->handle, arg0, arg1, proverGetCredentials_cb));
  delete arg1;
}

void proverGetCredential_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(proverGetCredential) {
  INDY_ASSERT_NARGS(proverGetCredential, 3)
  INDY_ASSERT_NUMBER(proverGetCredential, 0, wh)
  INDY_ASSERT_STRING(proverGetCredential, 1, credId)
  INDY_ASSERT_FUNCTION(proverGetCredential, 2)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_prover_get_credential(icb->handle, arg0, arg1, proverGetCredential_cb));
  delete arg1;
}

void proverSearchCredentials_cb(indy_handle_t handle, indy_error_t xerr, indy_handle_t arg0, indy_u32_t arg1) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbHandleU32(xerr, arg0, arg1);
  }
}
NAN_METHOD(proverSearchCredentials) {
  INDY_ASSERT_NARGS(proverSearchCredentials, 3)
  INDY_ASSERT_NUMBER(proverSearchCredentials, 0, wh)
  INDY_ASSERT_STRING(proverSearchCredentials, 1, query)
  INDY_ASSERT_FUNCTION(proverSearchCredentials, 2)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_prover_search_credentials(icb->handle, arg0, arg1, proverSearchCredentials_cb));
  delete arg1;
}

void proverFetchCredentials_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(proverFetchCredentials) {
  INDY_ASSERT_NARGS(proverFetchCredentials, 3)
  INDY_ASSERT_NUMBER(proverFetchCredentials, 0, sh)
  INDY_ASSERT_NUMBER(proverFetchCredentials, 1, count)
  INDY_ASSERT_FUNCTION(proverFetchCredentials, 2)
  indy_handle_t arg0 = info[0]->Int32Value();
  indy_u32_t arg1 = info[1]->Uint32Value();
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_prover_fetch_credentials(icb->handle, arg0, arg1, proverFetchCredentials_cb));
}

void proverCloseCredentialsSearch_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(proverCloseCredentialsSearch) {
  INDY_ASSERT_NARGS(proverCloseCredentialsSearch, 2)
  INDY_ASSERT_NUMBER(proverCloseCredentialsSearch, 0, sh)
  INDY_ASSERT_FUNCTION(proverCloseCredentialsSearch, 1)
  indy_handle_t arg0 = info[0]->Int32Value();
  IndyCallback* icb = argToIndyCb(info[1]);
  indyCalled(icb, indy_prover_close_credentials_search(icb->handle, arg0, proverCloseCredentialsSearch_cb));
}

void proverGetCredentialsForProofReq_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(proverGetCredentialsForProofReq) {
  INDY_ASSERT_NARGS(proverGetCredentialsForProofReq, 3)
  INDY_ASSERT_NUMBER(proverGetCredentialsForProofReq, 0, wh)
  INDY_ASSERT_STRING(proverGetCredentialsForProofReq, 1, proofRequest)
  INDY_ASSERT_FUNCTION(proverGetCredentialsForProofReq, 2)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_prover_get_credentials_for_proof_req(icb->handle, arg0, arg1, proverGetCredentialsForProofReq_cb));
  delete arg1;
}

void proverSearchCredentialsForProofReq_cb(indy_handle_t handle, indy_error_t xerr, indy_handle_t arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbHandle(xerr, arg0);
  }
}
NAN_METHOD(proverSearchCredentialsForProofReq) {
  INDY_ASSERT_NARGS(proverSearchCredentialsForProofReq, 4)
  INDY_ASSERT_NUMBER(proverSearchCredentialsForProofReq, 0, wh)
  INDY_ASSERT_STRING(proverSearchCredentialsForProofReq, 1, proofRequest)
  INDY_ASSERT_STRING(proverSearchCredentialsForProofReq, 2, extraQuery)
  INDY_ASSERT_FUNCTION(proverSearchCredentialsForProofReq, 3)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  IndyCallback* icb = argToIndyCb(info[3]);
  indyCalled(icb, indy_prover_search_credentials_for_proof_req(icb->handle, arg0, arg1, arg2, proverSearchCredentialsForProofReq_cb));
  delete arg1;
  delete arg2;
}

void proverFetchCredentialsForProofReq_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(proverFetchCredentialsForProofReq) {
  INDY_ASSERT_NARGS(proverFetchCredentialsForProofReq, 4)
  INDY_ASSERT_NUMBER(proverFetchCredentialsForProofReq, 0, sh)
  INDY_ASSERT_STRING(proverFetchCredentialsForProofReq, 1, itemReferent)
  INDY_ASSERT_NUMBER(proverFetchCredentialsForProofReq, 2, count)
  INDY_ASSERT_FUNCTION(proverFetchCredentialsForProofReq, 3)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  indy_u32_t arg2 = info[2]->Uint32Value();
  IndyCallback* icb = argToIndyCb(info[3]);
  indyCalled(icb, indy_prover_fetch_credentials_for_proof_req(icb->handle, arg0, arg1, arg2, proverFetchCredentialsForProofReq_cb));
  delete arg1;
}

void proverCloseCredentialsSearchForProofReq_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(proverCloseCredentialsSearchForProofReq) {
  INDY_ASSERT_NARGS(proverCloseCredentialsSearchForProofReq, 2)
  INDY_ASSERT_NUMBER(proverCloseCredentialsSearchForProofReq, 0, sh)
  INDY_ASSERT_FUNCTION(proverCloseCredentialsSearchForProofReq, 1)
  indy_handle_t arg0 = info[0]->Int32Value();
  IndyCallback* icb = argToIndyCb(info[1]);
  indyCalled(icb, indy_prover_close_credentials_search_for_proof_req(icb->handle, arg0, proverCloseCredentialsSearchForProofReq_cb));
}

void proverCreateProof_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(proverCreateProof) {
  INDY_ASSERT_NARGS(proverCreateProof, 8)
  INDY_ASSERT_NUMBER(proverCreateProof, 0, wh)
  INDY_ASSERT_STRING(proverCreateProof, 1, proofReq)
  INDY_ASSERT_STRING(proverCreateProof, 2, requestedCredentials)
  INDY_ASSERT_STRING(proverCreateProof, 3, masterSecretName)
  INDY_ASSERT_STRING(proverCreateProof, 4, schemas)
  INDY_ASSERT_STRING(proverCreateProof, 5, credentialDefs)
  INDY_ASSERT_STRING(proverCreateProof, 6, revStates)
  INDY_ASSERT_FUNCTION(proverCreateProof, 7)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  const char* arg3 = argToCString(info[3]);
  const char* arg4 = argToCString(info[4]);
  const char* arg5 = argToCString(info[5]);
  const char* arg6 = argToCString(info[6]);
  IndyCallback* icb = argToIndyCb(info[7]);
  indyCalled(icb, indy_prover_create_proof(icb->handle, arg0, arg1, arg2, arg3, arg4, arg5, arg6, proverCreateProof_cb));
  delete arg1;
  delete arg2;
  delete arg3;
  delete arg4;
  delete arg5;
  delete arg6;
}

void verifierVerifyProof_cb(indy_handle_t handle, indy_error_t xerr, indy_bool_t arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbBoolean(xerr, arg0);
  }
}
NAN_METHOD(verifierVerifyProof) {
  INDY_ASSERT_NARGS(verifierVerifyProof, 7)
  INDY_ASSERT_STRING(verifierVerifyProof, 0, proofRequest)
  INDY_ASSERT_STRING(verifierVerifyProof, 1, proof)
  INDY_ASSERT_STRING(verifierVerifyProof, 2, schemas)
  INDY_ASSERT_STRING(verifierVerifyProof, 3, credentialDefsJsons)
  INDY_ASSERT_STRING(verifierVerifyProof, 4, revRegDefs)
  INDY_ASSERT_STRING(verifierVerifyProof, 5, revRegs)
  INDY_ASSERT_FUNCTION(verifierVerifyProof, 6)
  const char* arg0 = argToCString(info[0]);
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  const char* arg3 = argToCString(info[3]);
  const char* arg4 = argToCString(info[4]);
  const char* arg5 = argToCString(info[5]);
  IndyCallback* icb = argToIndyCb(info[6]);
  indyCalled(icb, indy_verifier_verify_proof(icb->handle, arg0, arg1, arg2, arg3, arg4, arg5, verifierVerifyProof_cb));
  delete arg0;
  delete arg1;
  delete arg2;
  delete arg3;
  delete arg4;
  delete arg5;
}

void createRevocationState_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(createRevocationState) {
  INDY_ASSERT_NARGS(createRevocationState, 6)
  INDY_ASSERT_NUMBER(createRevocationState, 0, blobStorageReaderHandle)
  INDY_ASSERT_STRING(createRevocationState, 1, revRegDef)
  INDY_ASSERT_STRING(createRevocationState, 2, revRegDelta)
  INDY_ASSERT_NUMBER(createRevocationState, 3, timestamp)
  INDY_ASSERT_STRING(createRevocationState, 4, credRevId)
  INDY_ASSERT_FUNCTION(createRevocationState, 5)
  indy_i32_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  long long arg3 = info[3]->Uint32Value();
  const char* arg4 = argToCString(info[4]);
  IndyCallback* icb = argToIndyCb(info[5]);
  indyCalled(icb, indy_create_revocation_state(icb->handle, arg0, arg1, arg2, arg3, arg4, createRevocationState_cb));
  delete arg1;
  delete arg2;
  delete arg4;
}

void updateRevocationState_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(updateRevocationState) {
  INDY_ASSERT_NARGS(updateRevocationState, 7)
  INDY_ASSERT_NUMBER(updateRevocationState, 0, blobStorageReaderHandle)
  INDY_ASSERT_STRING(updateRevocationState, 1, revState)
  INDY_ASSERT_STRING(updateRevocationState, 2, revRegDef)
  INDY_ASSERT_STRING(updateRevocationState, 3, revRegDelta)
  INDY_ASSERT_NUMBER(updateRevocationState, 4, timestamp)
  INDY_ASSERT_STRING(updateRevocationState, 5, credRevId)
  INDY_ASSERT_FUNCTION(updateRevocationState, 6)
  indy_i32_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  const char* arg3 = argToCString(info[3]);
  long long arg4 = info[4]->Uint32Value();
  const char* arg5 = argToCString(info[5]);
  IndyCallback* icb = argToIndyCb(info[6]);
  indyCalled(icb, indy_update_revocation_state(icb->handle, arg0, arg1, arg2, arg3, arg4, arg5, updateRevocationState_cb));
  delete arg1;
  delete arg2;
  delete arg3;
  delete arg5;
}

void openBlobStorageReader_cb(indy_handle_t handle, indy_error_t xerr, indy_i32_t arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbI32(xerr, arg0);
  }
}
NAN_METHOD(openBlobStorageReader) {
  INDY_ASSERT_NARGS(openBlobStorageReader, 3)
  INDY_ASSERT_STRING(openBlobStorageReader, 0, type)
  INDY_ASSERT_STRING(openBlobStorageReader, 1, config)
  INDY_ASSERT_FUNCTION(openBlobStorageReader, 2)
  const char* arg0 = argToCString(info[0]);
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_open_blob_storage_reader(icb->handle, arg0, arg1, openBlobStorageReader_cb));
  delete arg0;
  delete arg1;
}

void openBlobStorageWriter_cb(indy_handle_t handle, indy_error_t xerr, indy_i32_t arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbI32(xerr, arg0);
  }
}
NAN_METHOD(openBlobStorageWriter) {
  INDY_ASSERT_NARGS(openBlobStorageWriter, 3)
  INDY_ASSERT_STRING(openBlobStorageWriter, 0, type)
  INDY_ASSERT_STRING(openBlobStorageWriter, 1, config)
  INDY_ASSERT_FUNCTION(openBlobStorageWriter, 2)
  const char* arg0 = argToCString(info[0]);
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_open_blob_storage_writer(icb->handle, arg0, arg1, openBlobStorageWriter_cb));
  delete arg0;
  delete arg1;
}

void createKey_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(createKey) {
  INDY_ASSERT_NARGS(createKey, 3)
  INDY_ASSERT_NUMBER(createKey, 0, wh)
  INDY_ASSERT_STRING(createKey, 1, key)
  INDY_ASSERT_FUNCTION(createKey, 2)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_create_key(icb->handle, arg0, arg1, createKey_cb));
  delete arg1;
}

void setKeyMetadata_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(setKeyMetadata) {
  INDY_ASSERT_NARGS(setKeyMetadata, 4)
  INDY_ASSERT_NUMBER(setKeyMetadata, 0, wh)
  INDY_ASSERT_STRING(setKeyMetadata, 1, verkey)
  INDY_ASSERT_STRING(setKeyMetadata, 2, metadata)
  INDY_ASSERT_FUNCTION(setKeyMetadata, 3)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  IndyCallback* icb = argToIndyCb(info[3]);
  indyCalled(icb, indy_set_key_metadata(icb->handle, arg0, arg1, arg2, setKeyMetadata_cb));
  delete arg1;
  delete arg2;
}

void getKeyMetadata_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(getKeyMetadata) {
  INDY_ASSERT_NARGS(getKeyMetadata, 3)
  INDY_ASSERT_NUMBER(getKeyMetadata, 0, wh)
  INDY_ASSERT_STRING(getKeyMetadata, 1, verkey)
  INDY_ASSERT_FUNCTION(getKeyMetadata, 2)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_get_key_metadata(icb->handle, arg0, arg1, getKeyMetadata_cb));
  delete arg1;
}

void cryptoSign_cb(indy_handle_t handle, indy_error_t xerr, const indy_u8_t* arg0data, indy_u32_t arg0len) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbBuffer(xerr, arg0data, arg0len);
  }
}
NAN_METHOD(cryptoSign) {
  INDY_ASSERT_NARGS(cryptoSign, 4)
  INDY_ASSERT_NUMBER(cryptoSign, 0, wh)
  INDY_ASSERT_STRING(cryptoSign, 1, signerVk)
  INDY_ASSERT_UINT8ARRAY(cryptoSign, 2, messageRaw)
  INDY_ASSERT_FUNCTION(cryptoSign, 3)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const indy_u8_t* arg2data = (indy_u8_t*)node::Buffer::Data(info[2]->ToObject());
  indy_u32_t arg2len = node::Buffer::Length(info[2]);
  IndyCallback* icb = argToIndyCb(info[3]);
  indyCalled(icb, indy_crypto_sign(icb->handle, arg0, arg1, arg2data, arg2len, cryptoSign_cb));
  delete arg1;
}

void cryptoVerify_cb(indy_handle_t handle, indy_error_t xerr, indy_bool_t arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbBoolean(xerr, arg0);
  }
}
NAN_METHOD(cryptoVerify) {
  INDY_ASSERT_NARGS(cryptoVerify, 4)
  INDY_ASSERT_STRING(cryptoVerify, 0, signerVk)
  INDY_ASSERT_UINT8ARRAY(cryptoVerify, 1, messageRaw)
  INDY_ASSERT_UINT8ARRAY(cryptoVerify, 2, signatureRaw)
  INDY_ASSERT_FUNCTION(cryptoVerify, 3)
  const char* arg0 = argToCString(info[0]);
  const indy_u8_t* arg1data = (indy_u8_t*)node::Buffer::Data(info[1]->ToObject());
  indy_u32_t arg1len = node::Buffer::Length(info[1]);
  const indy_u8_t* arg2data = (indy_u8_t*)node::Buffer::Data(info[2]->ToObject());
  indy_u32_t arg2len = node::Buffer::Length(info[2]);
  IndyCallback* icb = argToIndyCb(info[3]);
  indyCalled(icb, indy_crypto_verify(icb->handle, arg0, arg1data, arg1len, arg2data, arg2len, cryptoVerify_cb));
  delete arg0;
}

void cryptoAuthCrypt_cb(indy_handle_t handle, indy_error_t xerr, const indy_u8_t* arg0data, indy_u32_t arg0len) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbBuffer(xerr, arg0data, arg0len);
  }
}
NAN_METHOD(cryptoAuthCrypt) {
  INDY_ASSERT_NARGS(cryptoAuthCrypt, 5)
  INDY_ASSERT_NUMBER(cryptoAuthCrypt, 0, wh)
  INDY_ASSERT_STRING(cryptoAuthCrypt, 1, senderVk)
  INDY_ASSERT_STRING(cryptoAuthCrypt, 2, recipientVk)
  INDY_ASSERT_UINT8ARRAY(cryptoAuthCrypt, 3, messageRaw)
  INDY_ASSERT_FUNCTION(cryptoAuthCrypt, 4)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  const indy_u8_t* arg3data = (indy_u8_t*)node::Buffer::Data(info[3]->ToObject());
  indy_u32_t arg3len = node::Buffer::Length(info[3]);
  IndyCallback* icb = argToIndyCb(info[4]);
  indyCalled(icb, indy_crypto_auth_crypt(icb->handle, arg0, arg1, arg2, arg3data, arg3len, cryptoAuthCrypt_cb));
  delete arg1;
  delete arg2;
}

void cryptoAuthDecrypt_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0, const indy_u8_t* arg1data, indy_u32_t arg1len) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringBuffer(xerr, arg0, arg1data, arg1len);
  }
}
NAN_METHOD(cryptoAuthDecrypt) {
  INDY_ASSERT_NARGS(cryptoAuthDecrypt, 4)
  INDY_ASSERT_NUMBER(cryptoAuthDecrypt, 0, wh)
  INDY_ASSERT_STRING(cryptoAuthDecrypt, 1, recipientVk)
  INDY_ASSERT_UINT8ARRAY(cryptoAuthDecrypt, 2, encryptedMsgRaw)
  INDY_ASSERT_FUNCTION(cryptoAuthDecrypt, 3)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const indy_u8_t* arg2data = (indy_u8_t*)node::Buffer::Data(info[2]->ToObject());
  indy_u32_t arg2len = node::Buffer::Length(info[2]);
  IndyCallback* icb = argToIndyCb(info[3]);
  indyCalled(icb, indy_crypto_auth_decrypt(icb->handle, arg0, arg1, arg2data, arg2len, cryptoAuthDecrypt_cb));
  delete arg1;
}

void cryptoAnonCrypt_cb(indy_handle_t handle, indy_error_t xerr, const indy_u8_t* arg0data, indy_u32_t arg0len) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbBuffer(xerr, arg0data, arg0len);
  }
}
NAN_METHOD(cryptoAnonCrypt) {
  INDY_ASSERT_NARGS(cryptoAnonCrypt, 3)
  INDY_ASSERT_STRING(cryptoAnonCrypt, 0, recipientVk)
  INDY_ASSERT_UINT8ARRAY(cryptoAnonCrypt, 1, messageRaw)
  INDY_ASSERT_FUNCTION(cryptoAnonCrypt, 2)
  const char* arg0 = argToCString(info[0]);
  const indy_u8_t* arg1data = (indy_u8_t*)node::Buffer::Data(info[1]->ToObject());
  indy_u32_t arg1len = node::Buffer::Length(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_crypto_anon_crypt(icb->handle, arg0, arg1data, arg1len, cryptoAnonCrypt_cb));
  delete arg0;
}

void cryptoAnonDecrypt_cb(indy_handle_t handle, indy_error_t xerr, const indy_u8_t* arg0data, indy_u32_t arg0len) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbBuffer(xerr, arg0data, arg0len);
  }
}
NAN_METHOD(cryptoAnonDecrypt) {
  INDY_ASSERT_NARGS(cryptoAnonDecrypt, 4)
  INDY_ASSERT_NUMBER(cryptoAnonDecrypt, 0, wh)
  INDY_ASSERT_STRING(cryptoAnonDecrypt, 1, recipientVk)
  INDY_ASSERT_UINT8ARRAY(cryptoAnonDecrypt, 2, encryptedMsg)
  INDY_ASSERT_FUNCTION(cryptoAnonDecrypt, 3)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const indy_u8_t* arg2data = (indy_u8_t*)node::Buffer::Data(info[2]->ToObject());
  indy_u32_t arg2len = node::Buffer::Length(info[2]);
  IndyCallback* icb = argToIndyCb(info[3]);
  indyCalled(icb, indy_crypto_anon_decrypt(icb->handle, arg0, arg1, arg2data, arg2len, cryptoAnonDecrypt_cb));
  delete arg1;
}

void createAndStoreMyDid_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0, const char *const arg1) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringString(xerr, arg0, arg1);
  }
}
NAN_METHOD(createAndStoreMyDid) {
  INDY_ASSERT_NARGS(createAndStoreMyDid, 3)
  INDY_ASSERT_NUMBER(createAndStoreMyDid, 0, wh)
  INDY_ASSERT_STRING(createAndStoreMyDid, 1, did)
  INDY_ASSERT_FUNCTION(createAndStoreMyDid, 2)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_create_and_store_my_did(icb->handle, arg0, arg1, createAndStoreMyDid_cb));
  delete arg1;
}

void replaceKeysStart_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(replaceKeysStart) {
  INDY_ASSERT_NARGS(replaceKeysStart, 4)
  INDY_ASSERT_NUMBER(replaceKeysStart, 0, wh)
  INDY_ASSERT_STRING(replaceKeysStart, 1, did)
  INDY_ASSERT_STRING(replaceKeysStart, 2, identity)
  INDY_ASSERT_FUNCTION(replaceKeysStart, 3)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  IndyCallback* icb = argToIndyCb(info[3]);
  indyCalled(icb, indy_replace_keys_start(icb->handle, arg0, arg1, arg2, replaceKeysStart_cb));
  delete arg1;
  delete arg2;
}

void replaceKeysApply_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(replaceKeysApply) {
  INDY_ASSERT_NARGS(replaceKeysApply, 3)
  INDY_ASSERT_NUMBER(replaceKeysApply, 0, wh)
  INDY_ASSERT_STRING(replaceKeysApply, 1, did)
  INDY_ASSERT_FUNCTION(replaceKeysApply, 2)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_replace_keys_apply(icb->handle, arg0, arg1, replaceKeysApply_cb));
  delete arg1;
}

void storeTheirDid_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(storeTheirDid) {
  INDY_ASSERT_NARGS(storeTheirDid, 3)
  INDY_ASSERT_NUMBER(storeTheirDid, 0, wh)
  INDY_ASSERT_STRING(storeTheirDid, 1, identity)
  INDY_ASSERT_FUNCTION(storeTheirDid, 2)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_store_their_did(icb->handle, arg0, arg1, storeTheirDid_cb));
  delete arg1;
}

void keyForDid_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(keyForDid) {
  INDY_ASSERT_NARGS(keyForDid, 4)
  INDY_ASSERT_NUMBER(keyForDid, 0, poolHandle)
  INDY_ASSERT_NUMBER(keyForDid, 1, wh)
  INDY_ASSERT_STRING(keyForDid, 2, did)
  INDY_ASSERT_FUNCTION(keyForDid, 3)
  indy_handle_t arg0 = info[0]->Int32Value();
  indy_handle_t arg1 = info[1]->Int32Value();
  const char* arg2 = argToCString(info[2]);
  IndyCallback* icb = argToIndyCb(info[3]);
  indyCalled(icb, indy_key_for_did(icb->handle, arg0, arg1, arg2, keyForDid_cb));
  delete arg2;
}

void keyForLocalDid_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(keyForLocalDid) {
  INDY_ASSERT_NARGS(keyForLocalDid, 3)
  INDY_ASSERT_NUMBER(keyForLocalDid, 0, wh)
  INDY_ASSERT_STRING(keyForLocalDid, 1, did)
  INDY_ASSERT_FUNCTION(keyForLocalDid, 2)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_key_for_local_did(icb->handle, arg0, arg1, keyForLocalDid_cb));
  delete arg1;
}

void setEndpointForDid_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(setEndpointForDid) {
  INDY_ASSERT_NARGS(setEndpointForDid, 5)
  INDY_ASSERT_NUMBER(setEndpointForDid, 0, wh)
  INDY_ASSERT_STRING(setEndpointForDid, 1, did)
  INDY_ASSERT_STRING(setEndpointForDid, 2, address)
  INDY_ASSERT_STRING(setEndpointForDid, 3, transportKey)
  INDY_ASSERT_FUNCTION(setEndpointForDid, 4)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  const char* arg3 = argToCString(info[3]);
  IndyCallback* icb = argToIndyCb(info[4]);
  indyCalled(icb, indy_set_endpoint_for_did(icb->handle, arg0, arg1, arg2, arg3, setEndpointForDid_cb));
  delete arg1;
  delete arg2;
  delete arg3;
}

void getEndpointForDid_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0, const char *const arg1) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringString(xerr, arg0, arg1);
  }
}
NAN_METHOD(getEndpointForDid) {
  INDY_ASSERT_NARGS(getEndpointForDid, 4)
  INDY_ASSERT_NUMBER(getEndpointForDid, 0, wh)
  INDY_ASSERT_NUMBER(getEndpointForDid, 1, poolHandle)
  INDY_ASSERT_STRING(getEndpointForDid, 2, did)
  INDY_ASSERT_FUNCTION(getEndpointForDid, 3)
  indy_handle_t arg0 = info[0]->Int32Value();
  indy_handle_t arg1 = info[1]->Int32Value();
  const char* arg2 = argToCString(info[2]);
  IndyCallback* icb = argToIndyCb(info[3]);
  indyCalled(icb, indy_get_endpoint_for_did(icb->handle, arg0, arg1, arg2, getEndpointForDid_cb));
  delete arg2;
}

void setDidMetadata_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(setDidMetadata) {
  INDY_ASSERT_NARGS(setDidMetadata, 4)
  INDY_ASSERT_NUMBER(setDidMetadata, 0, wh)
  INDY_ASSERT_STRING(setDidMetadata, 1, did)
  INDY_ASSERT_STRING(setDidMetadata, 2, metadata)
  INDY_ASSERT_FUNCTION(setDidMetadata, 3)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  IndyCallback* icb = argToIndyCb(info[3]);
  indyCalled(icb, indy_set_did_metadata(icb->handle, arg0, arg1, arg2, setDidMetadata_cb));
  delete arg1;
  delete arg2;
}

void getDidMetadata_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(getDidMetadata) {
  INDY_ASSERT_NARGS(getDidMetadata, 3)
  INDY_ASSERT_NUMBER(getDidMetadata, 0, wh)
  INDY_ASSERT_STRING(getDidMetadata, 1, did)
  INDY_ASSERT_FUNCTION(getDidMetadata, 2)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_get_did_metadata(icb->handle, arg0, arg1, getDidMetadata_cb));
  delete arg1;
}

void getMyDidWithMeta_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(getMyDidWithMeta) {
  INDY_ASSERT_NARGS(getMyDidWithMeta, 3)
  INDY_ASSERT_NUMBER(getMyDidWithMeta, 0, wh)
  INDY_ASSERT_STRING(getMyDidWithMeta, 1, myDid)
  INDY_ASSERT_FUNCTION(getMyDidWithMeta, 2)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_get_my_did_with_meta(icb->handle, arg0, arg1, getMyDidWithMeta_cb));
  delete arg1;
}

void listMyDidsWithMeta_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(listMyDidsWithMeta) {
  INDY_ASSERT_NARGS(listMyDidsWithMeta, 2)
  INDY_ASSERT_NUMBER(listMyDidsWithMeta, 0, wh)
  INDY_ASSERT_FUNCTION(listMyDidsWithMeta, 1)
  indy_handle_t arg0 = info[0]->Int32Value();
  IndyCallback* icb = argToIndyCb(info[1]);
  indyCalled(icb, indy_list_my_dids_with_meta(icb->handle, arg0, listMyDidsWithMeta_cb));
}

void abbreviateVerkey_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(abbreviateVerkey) {
  INDY_ASSERT_NARGS(abbreviateVerkey, 3)
  INDY_ASSERT_STRING(abbreviateVerkey, 0, did)
  INDY_ASSERT_STRING(abbreviateVerkey, 1, fullVerkey)
  INDY_ASSERT_FUNCTION(abbreviateVerkey, 2)
  const char* arg0 = argToCString(info[0]);
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_abbreviate_verkey(icb->handle, arg0, arg1, abbreviateVerkey_cb));
  delete arg0;
  delete arg1;
}

void signAndSubmitRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(signAndSubmitRequest) {
  INDY_ASSERT_NARGS(signAndSubmitRequest, 5)
  INDY_ASSERT_NUMBER(signAndSubmitRequest, 0, poolHandle)
  INDY_ASSERT_NUMBER(signAndSubmitRequest, 1, wh)
  INDY_ASSERT_STRING(signAndSubmitRequest, 2, submitterDid)
  INDY_ASSERT_STRING(signAndSubmitRequest, 3, request)
  INDY_ASSERT_FUNCTION(signAndSubmitRequest, 4)
  indy_handle_t arg0 = info[0]->Int32Value();
  indy_handle_t arg1 = info[1]->Int32Value();
  const char* arg2 = argToCString(info[2]);
  const char* arg3 = argToCString(info[3]);
  IndyCallback* icb = argToIndyCb(info[4]);
  indyCalled(icb, indy_sign_and_submit_request(icb->handle, arg0, arg1, arg2, arg3, signAndSubmitRequest_cb));
  delete arg2;
  delete arg3;
}

void submitRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(submitRequest) {
  INDY_ASSERT_NARGS(submitRequest, 3)
  INDY_ASSERT_NUMBER(submitRequest, 0, poolHandle)
  INDY_ASSERT_STRING(submitRequest, 1, request)
  INDY_ASSERT_FUNCTION(submitRequest, 2)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_submit_request(icb->handle, arg0, arg1, submitRequest_cb));
  delete arg1;
}

void signRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(signRequest) {
  INDY_ASSERT_NARGS(signRequest, 4)
  INDY_ASSERT_NUMBER(signRequest, 0, wh)
  INDY_ASSERT_STRING(signRequest, 1, submitterDid)
  INDY_ASSERT_STRING(signRequest, 2, request)
  INDY_ASSERT_FUNCTION(signRequest, 3)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  IndyCallback* icb = argToIndyCb(info[3]);
  indyCalled(icb, indy_sign_request(icb->handle, arg0, arg1, arg2, signRequest_cb));
  delete arg1;
  delete arg2;
}

void multiSignRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(multiSignRequest) {
  INDY_ASSERT_NARGS(multiSignRequest, 4)
  INDY_ASSERT_NUMBER(multiSignRequest, 0, wh)
  INDY_ASSERT_STRING(multiSignRequest, 1, submitterDid)
  INDY_ASSERT_STRING(multiSignRequest, 2, request)
  INDY_ASSERT_FUNCTION(multiSignRequest, 3)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  IndyCallback* icb = argToIndyCb(info[3]);
  indyCalled(icb, indy_multi_sign_request(icb->handle, arg0, arg1, arg2, multiSignRequest_cb));
  delete arg1;
  delete arg2;
}

void buildGetDdoRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildGetDdoRequest) {
  INDY_ASSERT_NARGS(buildGetDdoRequest, 3)
  INDY_ASSERT_STRING(buildGetDdoRequest, 0, submitterDid)
  INDY_ASSERT_STRING(buildGetDdoRequest, 1, targetDid)
  INDY_ASSERT_FUNCTION(buildGetDdoRequest, 2)
  const char* arg0 = argToCString(info[0]);
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_build_get_ddo_request(icb->handle, arg0, arg1, buildGetDdoRequest_cb));
  delete arg0;
  delete arg1;
}

void buildNymRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildNymRequest) {
  INDY_ASSERT_NARGS(buildNymRequest, 6)
  INDY_ASSERT_STRING(buildNymRequest, 0, submitterDid)
  INDY_ASSERT_STRING(buildNymRequest, 1, targetDid)
  INDY_ASSERT_STRING(buildNymRequest, 2, verkey)
  INDY_ASSERT_STRING(buildNymRequest, 3, alias)
  INDY_ASSERT_STRING(buildNymRequest, 4, role)
  INDY_ASSERT_FUNCTION(buildNymRequest, 5)
  const char* arg0 = argToCString(info[0]);
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  const char* arg3 = argToCString(info[3]);
  const char* arg4 = argToCString(info[4]);
  IndyCallback* icb = argToIndyCb(info[5]);
  indyCalled(icb, indy_build_nym_request(icb->handle, arg0, arg1, arg2, arg3, arg4, buildNymRequest_cb));
  delete arg0;
  delete arg1;
  delete arg2;
  delete arg3;
  delete arg4;
}

void buildAttribRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildAttribRequest) {
  INDY_ASSERT_NARGS(buildAttribRequest, 6)
  INDY_ASSERT_STRING(buildAttribRequest, 0, submitterDid)
  INDY_ASSERT_STRING(buildAttribRequest, 1, targetDid)
  INDY_ASSERT_STRING(buildAttribRequest, 2, hash)
  INDY_ASSERT_STRING(buildAttribRequest, 3, raw)
  INDY_ASSERT_STRING(buildAttribRequest, 4, enc)
  INDY_ASSERT_FUNCTION(buildAttribRequest, 5)
  const char* arg0 = argToCString(info[0]);
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  const char* arg3 = argToCString(info[3]);
  const char* arg4 = argToCString(info[4]);
  IndyCallback* icb = argToIndyCb(info[5]);
  indyCalled(icb, indy_build_attrib_request(icb->handle, arg0, arg1, arg2, arg3, arg4, buildAttribRequest_cb));
  delete arg0;
  delete arg1;
  delete arg2;
  delete arg3;
  delete arg4;
}

void buildGetAttribRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildGetAttribRequest) {
  INDY_ASSERT_NARGS(buildGetAttribRequest, 6)
  INDY_ASSERT_STRING(buildGetAttribRequest, 0, submitterDid)
  INDY_ASSERT_STRING(buildGetAttribRequest, 1, targetDid)
  INDY_ASSERT_STRING(buildGetAttribRequest, 2, hash)
  INDY_ASSERT_STRING(buildGetAttribRequest, 3, raw)
  INDY_ASSERT_STRING(buildGetAttribRequest, 4, enc)
  INDY_ASSERT_FUNCTION(buildGetAttribRequest, 5)
  const char* arg0 = argToCString(info[0]);
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  const char* arg3 = argToCString(info[3]);
  const char* arg4 = argToCString(info[4]);
  IndyCallback* icb = argToIndyCb(info[5]);
  indyCalled(icb, indy_build_get_attrib_request(icb->handle, arg0, arg1, arg2, arg3, arg4, buildGetAttribRequest_cb));
  delete arg0;
  delete arg1;
  delete arg2;
  delete arg3;
  delete arg4;
}

void buildGetNymRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildGetNymRequest) {
  INDY_ASSERT_NARGS(buildGetNymRequest, 3)
  INDY_ASSERT_STRING(buildGetNymRequest, 0, submitterDid)
  INDY_ASSERT_STRING(buildGetNymRequest, 1, targetDid)
  INDY_ASSERT_FUNCTION(buildGetNymRequest, 2)
  const char* arg0 = argToCString(info[0]);
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_build_get_nym_request(icb->handle, arg0, arg1, buildGetNymRequest_cb));
  delete arg0;
  delete arg1;
}

void buildSchemaRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildSchemaRequest) {
  INDY_ASSERT_NARGS(buildSchemaRequest, 3)
  INDY_ASSERT_STRING(buildSchemaRequest, 0, submitterDid)
  INDY_ASSERT_STRING(buildSchemaRequest, 1, data)
  INDY_ASSERT_FUNCTION(buildSchemaRequest, 2)
  const char* arg0 = argToCString(info[0]);
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_build_schema_request(icb->handle, arg0, arg1, buildSchemaRequest_cb));
  delete arg0;
  delete arg1;
}

void buildGetSchemaRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildGetSchemaRequest) {
  INDY_ASSERT_NARGS(buildGetSchemaRequest, 3)
  INDY_ASSERT_STRING(buildGetSchemaRequest, 0, submitterDid)
  INDY_ASSERT_STRING(buildGetSchemaRequest, 1, id)
  INDY_ASSERT_FUNCTION(buildGetSchemaRequest, 2)
  const char* arg0 = argToCString(info[0]);
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_build_get_schema_request(icb->handle, arg0, arg1, buildGetSchemaRequest_cb));
  delete arg0;
  delete arg1;
}

void parseGetSchemaResponse_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0, const char* arg1) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringString(xerr, arg0, arg1);
  }
}
NAN_METHOD(parseGetSchemaResponse) {
  INDY_ASSERT_NARGS(parseGetSchemaResponse, 2)
  INDY_ASSERT_STRING(parseGetSchemaResponse, 0, getSchemaResponse)
  INDY_ASSERT_FUNCTION(parseGetSchemaResponse, 1)
  const char* arg0 = argToCString(info[0]);
  IndyCallback* icb = argToIndyCb(info[1]);
  indyCalled(icb, indy_parse_get_schema_response(icb->handle, arg0, parseGetSchemaResponse_cb));
  delete arg0;
}

void buildCredDefRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildCredDefRequest) {
  INDY_ASSERT_NARGS(buildCredDefRequest, 3)
  INDY_ASSERT_STRING(buildCredDefRequest, 0, submitterDid)
  INDY_ASSERT_STRING(buildCredDefRequest, 1, data)
  INDY_ASSERT_FUNCTION(buildCredDefRequest, 2)
  const char* arg0 = argToCString(info[0]);
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_build_cred_def_request(icb->handle, arg0, arg1, buildCredDefRequest_cb));
  delete arg0;
  delete arg1;
}

void buildGetCredDefRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildGetCredDefRequest) {
  INDY_ASSERT_NARGS(buildGetCredDefRequest, 3)
  INDY_ASSERT_STRING(buildGetCredDefRequest, 0, submitterDid)
  INDY_ASSERT_STRING(buildGetCredDefRequest, 1, id)
  INDY_ASSERT_FUNCTION(buildGetCredDefRequest, 2)
  const char* arg0 = argToCString(info[0]);
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_build_get_cred_def_request(icb->handle, arg0, arg1, buildGetCredDefRequest_cb));
  delete arg0;
  delete arg1;
}

void parseGetCredDefResponse_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0, const char* arg1) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringString(xerr, arg0, arg1);
  }
}
NAN_METHOD(parseGetCredDefResponse) {
  INDY_ASSERT_NARGS(parseGetCredDefResponse, 2)
  INDY_ASSERT_STRING(parseGetCredDefResponse, 0, getCredDefResponse)
  INDY_ASSERT_FUNCTION(parseGetCredDefResponse, 1)
  const char* arg0 = argToCString(info[0]);
  IndyCallback* icb = argToIndyCb(info[1]);
  indyCalled(icb, indy_parse_get_cred_def_response(icb->handle, arg0, parseGetCredDefResponse_cb));
  delete arg0;
}

void buildNodeRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildNodeRequest) {
  INDY_ASSERT_NARGS(buildNodeRequest, 4)
  INDY_ASSERT_STRING(buildNodeRequest, 0, submitterDid)
  INDY_ASSERT_STRING(buildNodeRequest, 1, targetDid)
  INDY_ASSERT_STRING(buildNodeRequest, 2, data)
  INDY_ASSERT_FUNCTION(buildNodeRequest, 3)
  const char* arg0 = argToCString(info[0]);
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  IndyCallback* icb = argToIndyCb(info[3]);
  indyCalled(icb, indy_build_node_request(icb->handle, arg0, arg1, arg2, buildNodeRequest_cb));
  delete arg0;
  delete arg1;
  delete arg2;
}

void buildGetValidatorInfoRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildGetValidatorInfoRequest) {
  INDY_ASSERT_NARGS(buildGetValidatorInfoRequest, 2)
  INDY_ASSERT_STRING(buildGetValidatorInfoRequest, 0, submitterDid)
  INDY_ASSERT_FUNCTION(buildGetValidatorInfoRequest, 1)
  const char* arg0 = argToCString(info[0]);
  IndyCallback* icb = argToIndyCb(info[1]);
  indyCalled(icb, indy_build_get_validator_info_request(icb->handle, arg0, buildGetValidatorInfoRequest_cb));
  delete arg0;
}

void buildGetTxnRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildGetTxnRequest) {
  INDY_ASSERT_NARGS(buildGetTxnRequest, 4)
  INDY_ASSERT_STRING(buildGetTxnRequest, 0, submitterDid)
  INDY_ASSERT_STRING(buildGetTxnRequest, 1, ledgerType)
  INDY_ASSERT_NUMBER(buildGetTxnRequest, 2, data)
  INDY_ASSERT_FUNCTION(buildGetTxnRequest, 3)
  const char* arg0 = argToCString(info[0]);
  const char* arg1 = argToCString(info[1]);
  indy_i32_t arg2 = info[2]->Int32Value();
  IndyCallback* icb = argToIndyCb(info[3]);
  indyCalled(icb, indy_build_get_txn_request(icb->handle, arg0, arg1, arg2, buildGetTxnRequest_cb));
  delete arg0;
  delete arg1;
}

void buildPoolConfigRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildPoolConfigRequest) {
  INDY_ASSERT_NARGS(buildPoolConfigRequest, 4)
  INDY_ASSERT_STRING(buildPoolConfigRequest, 0, submitterDid)
  INDY_ASSERT_BOOLEAN(buildPoolConfigRequest, 1, writes)
  INDY_ASSERT_BOOLEAN(buildPoolConfigRequest, 2, force)
  INDY_ASSERT_FUNCTION(buildPoolConfigRequest, 3)
  const char* arg0 = argToCString(info[0]);
  indy_bool_t arg1 = info[1]->IsTrue();
  indy_bool_t arg2 = info[2]->IsTrue();
  IndyCallback* icb = argToIndyCb(info[3]);
  indyCalled(icb, indy_build_pool_config_request(icb->handle, arg0, arg1, arg2, buildPoolConfigRequest_cb));
  delete arg0;
}

void buildPoolRestartRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildPoolRestartRequest) {
  INDY_ASSERT_NARGS(buildPoolRestartRequest, 4)
  INDY_ASSERT_STRING(buildPoolRestartRequest, 0, submitterDid)
  INDY_ASSERT_STRING(buildPoolRestartRequest, 1, action)
  INDY_ASSERT_STRING(buildPoolRestartRequest, 2, datetime)
  INDY_ASSERT_FUNCTION(buildPoolRestartRequest, 3)
  const char* arg0 = argToCString(info[0]);
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  IndyCallback* icb = argToIndyCb(info[3]);
  indyCalled(icb, indy_build_pool_restart_request(icb->handle, arg0, arg1, arg2, buildPoolRestartRequest_cb));
  delete arg0;
  delete arg1;
  delete arg2;
}

void buildPoolUpgradeRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildPoolUpgradeRequest) {
  INDY_ASSERT_NARGS(buildPoolUpgradeRequest, 12)
  INDY_ASSERT_STRING(buildPoolUpgradeRequest, 0, submitterDid)
  INDY_ASSERT_STRING(buildPoolUpgradeRequest, 1, name)
  INDY_ASSERT_STRING(buildPoolUpgradeRequest, 2, version)
  INDY_ASSERT_STRING(buildPoolUpgradeRequest, 3, action)
  INDY_ASSERT_STRING(buildPoolUpgradeRequest, 4, sha256)
  INDY_ASSERT_NUMBER(buildPoolUpgradeRequest, 5, timeout)
  INDY_ASSERT_STRING(buildPoolUpgradeRequest, 6, schedule)
  INDY_ASSERT_STRING(buildPoolUpgradeRequest, 7, justification)
  INDY_ASSERT_BOOLEAN(buildPoolUpgradeRequest, 8, reinstall)
  INDY_ASSERT_BOOLEAN(buildPoolUpgradeRequest, 9, force)
  INDY_ASSERT_STRING(buildPoolUpgradeRequest, 10, package_)
  INDY_ASSERT_FUNCTION(buildPoolUpgradeRequest, 11)
  const char* arg0 = argToCString(info[0]);
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  const char* arg3 = argToCString(info[3]);
  const char* arg4 = argToCString(info[4]);
  indy_i32_t arg5 = info[5]->Int32Value();
  const char* arg6 = argToCString(info[6]);
  const char* arg7 = argToCString(info[7]);
  indy_bool_t arg8 = info[8]->IsTrue();
  indy_bool_t arg9 = info[9]->IsTrue();
  const char* arg10 = argToCString(info[10]);
  IndyCallback* icb = argToIndyCb(info[11]);
  indyCalled(icb, indy_build_pool_upgrade_request(icb->handle, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, buildPoolUpgradeRequest_cb));
  delete arg0;
  delete arg1;
  delete arg2;
  delete arg3;
  delete arg4;
  delete arg6;
  delete arg7;
  delete arg10;
}

void buildRevocRegDefRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildRevocRegDefRequest) {
  INDY_ASSERT_NARGS(buildRevocRegDefRequest, 3)
  INDY_ASSERT_STRING(buildRevocRegDefRequest, 0, submitterDid)
  INDY_ASSERT_STRING(buildRevocRegDefRequest, 1, data)
  INDY_ASSERT_FUNCTION(buildRevocRegDefRequest, 2)
  const char* arg0 = argToCString(info[0]);
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_build_revoc_reg_def_request(icb->handle, arg0, arg1, buildRevocRegDefRequest_cb));
  delete arg0;
  delete arg1;
}

void buildGetRevocRegDefRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildGetRevocRegDefRequest) {
  INDY_ASSERT_NARGS(buildGetRevocRegDefRequest, 3)
  INDY_ASSERT_STRING(buildGetRevocRegDefRequest, 0, submitterDid)
  INDY_ASSERT_STRING(buildGetRevocRegDefRequest, 1, id)
  INDY_ASSERT_FUNCTION(buildGetRevocRegDefRequest, 2)
  const char* arg0 = argToCString(info[0]);
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_build_get_revoc_reg_def_request(icb->handle, arg0, arg1, buildGetRevocRegDefRequest_cb));
  delete arg0;
  delete arg1;
}

void parseGetRevocRegDefResponse_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0, const char* arg1) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringString(xerr, arg0, arg1);
  }
}
NAN_METHOD(parseGetRevocRegDefResponse) {
  INDY_ASSERT_NARGS(parseGetRevocRegDefResponse, 2)
  INDY_ASSERT_STRING(parseGetRevocRegDefResponse, 0, getRevocRefDefResponse)
  INDY_ASSERT_FUNCTION(parseGetRevocRegDefResponse, 1)
  const char* arg0 = argToCString(info[0]);
  IndyCallback* icb = argToIndyCb(info[1]);
  indyCalled(icb, indy_parse_get_revoc_reg_def_response(icb->handle, arg0, parseGetRevocRegDefResponse_cb));
  delete arg0;
}

void buildRevocRegEntryRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildRevocRegEntryRequest) {
  INDY_ASSERT_NARGS(buildRevocRegEntryRequest, 5)
  INDY_ASSERT_STRING(buildRevocRegEntryRequest, 0, submitterDid)
  INDY_ASSERT_STRING(buildRevocRegEntryRequest, 1, revocRegDefId)
  INDY_ASSERT_STRING(buildRevocRegEntryRequest, 2, revDefType)
  INDY_ASSERT_STRING(buildRevocRegEntryRequest, 3, value)
  INDY_ASSERT_FUNCTION(buildRevocRegEntryRequest, 4)
  const char* arg0 = argToCString(info[0]);
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  const char* arg3 = argToCString(info[3]);
  IndyCallback* icb = argToIndyCb(info[4]);
  indyCalled(icb, indy_build_revoc_reg_entry_request(icb->handle, arg0, arg1, arg2, arg3, buildRevocRegEntryRequest_cb));
  delete arg0;
  delete arg1;
  delete arg2;
  delete arg3;
}

void buildGetRevocRegRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildGetRevocRegRequest) {
  INDY_ASSERT_NARGS(buildGetRevocRegRequest, 4)
  INDY_ASSERT_STRING(buildGetRevocRegRequest, 0, submitterDid)
  INDY_ASSERT_STRING(buildGetRevocRegRequest, 1, revocRegDefId)
  INDY_ASSERT_NUMBER(buildGetRevocRegRequest, 2, timestamp)
  INDY_ASSERT_FUNCTION(buildGetRevocRegRequest, 3)
  const char* arg0 = argToCString(info[0]);
  const char* arg1 = argToCString(info[1]);
  long long arg2 = info[2]->Uint32Value();
  IndyCallback* icb = argToIndyCb(info[3]);
  indyCalled(icb, indy_build_get_revoc_reg_request(icb->handle, arg0, arg1, arg2, buildGetRevocRegRequest_cb));
  delete arg0;
  delete arg1;
}

void parseGetRevocRegResponse_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0, const char* arg1, unsigned long long arg2) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringStringTimestamp(xerr, arg0, arg1, arg2);
  }
}
NAN_METHOD(parseGetRevocRegResponse) {
  INDY_ASSERT_NARGS(parseGetRevocRegResponse, 2)
  INDY_ASSERT_STRING(parseGetRevocRegResponse, 0, getRevocRegResponse)
  INDY_ASSERT_FUNCTION(parseGetRevocRegResponse, 1)
  const char* arg0 = argToCString(info[0]);
  IndyCallback* icb = argToIndyCb(info[1]);
  indyCalled(icb, indy_parse_get_revoc_reg_response(icb->handle, arg0, parseGetRevocRegResponse_cb));
  delete arg0;
}

void buildGetRevocRegDeltaRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildGetRevocRegDeltaRequest) {
  INDY_ASSERT_NARGS(buildGetRevocRegDeltaRequest, 5)
  INDY_ASSERT_STRING(buildGetRevocRegDeltaRequest, 0, submitterDid)
  INDY_ASSERT_STRING(buildGetRevocRegDeltaRequest, 1, revocRegDefId)
  INDY_ASSERT_NUMBER(buildGetRevocRegDeltaRequest, 2, from)
  INDY_ASSERT_NUMBER(buildGetRevocRegDeltaRequest, 3, to)
  INDY_ASSERT_FUNCTION(buildGetRevocRegDeltaRequest, 4)
  const char* arg0 = argToCString(info[0]);
  const char* arg1 = argToCString(info[1]);
  long long arg2 = info[2]->Uint32Value();
  long long arg3 = info[3]->Uint32Value();
  IndyCallback* icb = argToIndyCb(info[4]);
  indyCalled(icb, indy_build_get_revoc_reg_delta_request(icb->handle, arg0, arg1, arg2, arg3, buildGetRevocRegDeltaRequest_cb));
  delete arg0;
  delete arg1;
}

void parseGetRevocRegDeltaResponse_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0, const char* arg1, unsigned long long arg2) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringStringTimestamp(xerr, arg0, arg1, arg2);
  }
}
NAN_METHOD(parseGetRevocRegDeltaResponse) {
  INDY_ASSERT_NARGS(parseGetRevocRegDeltaResponse, 2)
  INDY_ASSERT_STRING(parseGetRevocRegDeltaResponse, 0, getRevocRegDeltaResponse)
  INDY_ASSERT_FUNCTION(parseGetRevocRegDeltaResponse, 1)
  const char* arg0 = argToCString(info[0]);
  IndyCallback* icb = argToIndyCb(info[1]);
  indyCalled(icb, indy_parse_get_revoc_reg_delta_response(icb->handle, arg0, parseGetRevocRegDeltaResponse_cb));
  delete arg0;
}

void addWalletRecord_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(addWalletRecord) {
  INDY_ASSERT_NARGS(addWalletRecord, 6)
  INDY_ASSERT_NUMBER(addWalletRecord, 0, wh)
  INDY_ASSERT_STRING(addWalletRecord, 1, type)
  INDY_ASSERT_STRING(addWalletRecord, 2, id)
  INDY_ASSERT_STRING(addWalletRecord, 3, value)
  INDY_ASSERT_STRING(addWalletRecord, 4, tags)
  INDY_ASSERT_FUNCTION(addWalletRecord, 5)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  const char* arg3 = argToCString(info[3]);
  const char* arg4 = argToCString(info[4]);
  IndyCallback* icb = argToIndyCb(info[5]);
  indyCalled(icb, indy_add_wallet_record(icb->handle, arg0, arg1, arg2, arg3, arg4, addWalletRecord_cb));
  delete arg1;
  delete arg2;
  delete arg3;
  delete arg4;
}

void updateWalletRecordValue_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(updateWalletRecordValue) {
  INDY_ASSERT_NARGS(updateWalletRecordValue, 5)
  INDY_ASSERT_NUMBER(updateWalletRecordValue, 0, wh)
  INDY_ASSERT_STRING(updateWalletRecordValue, 1, type)
  INDY_ASSERT_STRING(updateWalletRecordValue, 2, id)
  INDY_ASSERT_STRING(updateWalletRecordValue, 3, value)
  INDY_ASSERT_FUNCTION(updateWalletRecordValue, 4)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  const char* arg3 = argToCString(info[3]);
  IndyCallback* icb = argToIndyCb(info[4]);
  indyCalled(icb, indy_update_wallet_record_value(icb->handle, arg0, arg1, arg2, arg3, updateWalletRecordValue_cb));
  delete arg1;
  delete arg2;
  delete arg3;
}

void updateWalletRecordTags_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(updateWalletRecordTags) {
  INDY_ASSERT_NARGS(updateWalletRecordTags, 5)
  INDY_ASSERT_NUMBER(updateWalletRecordTags, 0, wh)
  INDY_ASSERT_STRING(updateWalletRecordTags, 1, type)
  INDY_ASSERT_STRING(updateWalletRecordTags, 2, id)
  INDY_ASSERT_STRING(updateWalletRecordTags, 3, tags)
  INDY_ASSERT_FUNCTION(updateWalletRecordTags, 4)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  const char* arg3 = argToCString(info[3]);
  IndyCallback* icb = argToIndyCb(info[4]);
  indyCalled(icb, indy_update_wallet_record_tags(icb->handle, arg0, arg1, arg2, arg3, updateWalletRecordTags_cb));
  delete arg1;
  delete arg2;
  delete arg3;
}

void addWalletRecordTags_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(addWalletRecordTags) {
  INDY_ASSERT_NARGS(addWalletRecordTags, 5)
  INDY_ASSERT_NUMBER(addWalletRecordTags, 0, wh)
  INDY_ASSERT_STRING(addWalletRecordTags, 1, type)
  INDY_ASSERT_STRING(addWalletRecordTags, 2, id)
  INDY_ASSERT_STRING(addWalletRecordTags, 3, tags)
  INDY_ASSERT_FUNCTION(addWalletRecordTags, 4)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  const char* arg3 = argToCString(info[3]);
  IndyCallback* icb = argToIndyCb(info[4]);
  indyCalled(icb, indy_add_wallet_record_tags(icb->handle, arg0, arg1, arg2, arg3, addWalletRecordTags_cb));
  delete arg1;
  delete arg2;
  delete arg3;
}

void deleteWalletRecordTags_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(deleteWalletRecordTags) {
  INDY_ASSERT_NARGS(deleteWalletRecordTags, 5)
  INDY_ASSERT_NUMBER(deleteWalletRecordTags, 0, wh)
  INDY_ASSERT_STRING(deleteWalletRecordTags, 1, type)
  INDY_ASSERT_STRING(deleteWalletRecordTags, 2, id)
  INDY_ASSERT_STRING(deleteWalletRecordTags, 3, tagNames)
  INDY_ASSERT_FUNCTION(deleteWalletRecordTags, 4)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  const char* arg3 = argToCString(info[3]);
  IndyCallback* icb = argToIndyCb(info[4]);
  indyCalled(icb, indy_delete_wallet_record_tags(icb->handle, arg0, arg1, arg2, arg3, deleteWalletRecordTags_cb));
  delete arg1;
  delete arg2;
  delete arg3;
}

void deleteWalletRecord_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(deleteWalletRecord) {
  INDY_ASSERT_NARGS(deleteWalletRecord, 4)
  INDY_ASSERT_NUMBER(deleteWalletRecord, 0, wh)
  INDY_ASSERT_STRING(deleteWalletRecord, 1, type)
  INDY_ASSERT_STRING(deleteWalletRecord, 2, id)
  INDY_ASSERT_FUNCTION(deleteWalletRecord, 3)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  IndyCallback* icb = argToIndyCb(info[3]);
  indyCalled(icb, indy_delete_wallet_record(icb->handle, arg0, arg1, arg2, deleteWalletRecord_cb));
  delete arg1;
  delete arg2;
}

void getWalletRecord_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(getWalletRecord) {
  INDY_ASSERT_NARGS(getWalletRecord, 5)
  INDY_ASSERT_NUMBER(getWalletRecord, 0, wh)
  INDY_ASSERT_STRING(getWalletRecord, 1, type)
  INDY_ASSERT_STRING(getWalletRecord, 2, id)
  INDY_ASSERT_STRING(getWalletRecord, 3, options)
  INDY_ASSERT_FUNCTION(getWalletRecord, 4)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  const char* arg3 = argToCString(info[3]);
  IndyCallback* icb = argToIndyCb(info[4]);
  indyCalled(icb, indy_get_wallet_record(icb->handle, arg0, arg1, arg2, arg3, getWalletRecord_cb));
  delete arg1;
  delete arg2;
  delete arg3;
}

void openWalletSearch_cb(indy_handle_t handle, indy_error_t xerr, indy_handle_t arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbHandle(xerr, arg0);
  }
}
NAN_METHOD(openWalletSearch) {
  INDY_ASSERT_NARGS(openWalletSearch, 5)
  INDY_ASSERT_NUMBER(openWalletSearch, 0, wh)
  INDY_ASSERT_STRING(openWalletSearch, 1, type)
  INDY_ASSERT_STRING(openWalletSearch, 2, query)
  INDY_ASSERT_STRING(openWalletSearch, 3, options)
  INDY_ASSERT_FUNCTION(openWalletSearch, 4)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  const char* arg3 = argToCString(info[3]);
  IndyCallback* icb = argToIndyCb(info[4]);
  indyCalled(icb, indy_open_wallet_search(icb->handle, arg0, arg1, arg2, arg3, openWalletSearch_cb));
  delete arg1;
  delete arg2;
  delete arg3;
}

void fetchWalletSearchNextRecords_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(fetchWalletSearchNextRecords) {
  INDY_ASSERT_NARGS(fetchWalletSearchNextRecords, 4)
  INDY_ASSERT_NUMBER(fetchWalletSearchNextRecords, 0, wh)
  INDY_ASSERT_NUMBER(fetchWalletSearchNextRecords, 1, walletSearchHandle)
  INDY_ASSERT_NUMBER(fetchWalletSearchNextRecords, 2, count)
  INDY_ASSERT_FUNCTION(fetchWalletSearchNextRecords, 3)
  indy_handle_t arg0 = info[0]->Int32Value();
  indy_handle_t arg1 = info[1]->Int32Value();
  indy_u32_t arg2 = info[2]->Uint32Value();
  IndyCallback* icb = argToIndyCb(info[3]);
  indyCalled(icb, indy_fetch_wallet_search_next_records(icb->handle, arg0, arg1, arg2, fetchWalletSearchNextRecords_cb));
}

void closeWalletSearch_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(closeWalletSearch) {
  INDY_ASSERT_NARGS(closeWalletSearch, 2)
  INDY_ASSERT_NUMBER(closeWalletSearch, 0, walletSearchHandle)
  INDY_ASSERT_FUNCTION(closeWalletSearch, 1)
  indy_handle_t arg0 = info[0]->Int32Value();
  IndyCallback* icb = argToIndyCb(info[1]);
  indyCalled(icb, indy_close_wallet_search(icb->handle, arg0, closeWalletSearch_cb));
}

void isPairwiseExists_cb(indy_handle_t handle, indy_error_t xerr, indy_bool_t arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbBoolean(xerr, arg0);
  }
}
NAN_METHOD(isPairwiseExists) {
  INDY_ASSERT_NARGS(isPairwiseExists, 3)
  INDY_ASSERT_NUMBER(isPairwiseExists, 0, wh)
  INDY_ASSERT_STRING(isPairwiseExists, 1, theirDid)
  INDY_ASSERT_FUNCTION(isPairwiseExists, 2)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_is_pairwise_exists(icb->handle, arg0, arg1, isPairwiseExists_cb));
  delete arg1;
}

void createPairwise_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(createPairwise) {
  INDY_ASSERT_NARGS(createPairwise, 5)
  INDY_ASSERT_NUMBER(createPairwise, 0, wh)
  INDY_ASSERT_STRING(createPairwise, 1, theirDid)
  INDY_ASSERT_STRING(createPairwise, 2, myDid)
  INDY_ASSERT_STRING(createPairwise, 3, metadata)
  INDY_ASSERT_FUNCTION(createPairwise, 4)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  const char* arg3 = argToCString(info[3]);
  IndyCallback* icb = argToIndyCb(info[4]);
  indyCalled(icb, indy_create_pairwise(icb->handle, arg0, arg1, arg2, arg3, createPairwise_cb));
  delete arg1;
  delete arg2;
  delete arg3;
}

void listPairwise_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(listPairwise) {
  INDY_ASSERT_NARGS(listPairwise, 2)
  INDY_ASSERT_NUMBER(listPairwise, 0, wh)
  INDY_ASSERT_FUNCTION(listPairwise, 1)
  indy_handle_t arg0 = info[0]->Int32Value();
  IndyCallback* icb = argToIndyCb(info[1]);
  indyCalled(icb, indy_list_pairwise(icb->handle, arg0, listPairwise_cb));
}

void getPairwise_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(getPairwise) {
  INDY_ASSERT_NARGS(getPairwise, 3)
  INDY_ASSERT_NUMBER(getPairwise, 0, wh)
  INDY_ASSERT_STRING(getPairwise, 1, theirDid)
  INDY_ASSERT_FUNCTION(getPairwise, 2)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_get_pairwise(icb->handle, arg0, arg1, getPairwise_cb));
  delete arg1;
}

void setPairwiseMetadata_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(setPairwiseMetadata) {
  INDY_ASSERT_NARGS(setPairwiseMetadata, 4)
  INDY_ASSERT_NUMBER(setPairwiseMetadata, 0, wh)
  INDY_ASSERT_STRING(setPairwiseMetadata, 1, theirDid)
  INDY_ASSERT_STRING(setPairwiseMetadata, 2, metadata)
  INDY_ASSERT_FUNCTION(setPairwiseMetadata, 3)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  IndyCallback* icb = argToIndyCb(info[3]);
  indyCalled(icb, indy_set_pairwise_metadata(icb->handle, arg0, arg1, arg2, setPairwiseMetadata_cb));
  delete arg1;
  delete arg2;
}

void createPaymentAddress_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(createPaymentAddress) {
  INDY_ASSERT_NARGS(createPaymentAddress, 4)
  INDY_ASSERT_NUMBER(createPaymentAddress, 0, wh)
  INDY_ASSERT_STRING(createPaymentAddress, 1, paymentMethod)
  INDY_ASSERT_STRING(createPaymentAddress, 2, config)
  INDY_ASSERT_FUNCTION(createPaymentAddress, 3)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  IndyCallback* icb = argToIndyCb(info[3]);
  indyCalled(icb, indy_create_payment_address(icb->handle, arg0, arg1, arg2, createPaymentAddress_cb));
  delete arg1;
  delete arg2;
}

void listPaymentAddresses_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(listPaymentAddresses) {
  INDY_ASSERT_NARGS(listPaymentAddresses, 2)
  INDY_ASSERT_NUMBER(listPaymentAddresses, 0, wh)
  INDY_ASSERT_FUNCTION(listPaymentAddresses, 1)
  indy_handle_t arg0 = info[0]->Int32Value();
  IndyCallback* icb = argToIndyCb(info[1]);
  indyCalled(icb, indy_list_payment_addresses(icb->handle, arg0, listPaymentAddresses_cb));
}

void addRequestFees_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0, const char* arg1) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringString(xerr, arg0, arg1);
  }
}
NAN_METHOD(addRequestFees) {
  INDY_ASSERT_NARGS(addRequestFees, 7)
  INDY_ASSERT_NUMBER(addRequestFees, 0, wh)
  INDY_ASSERT_STRING(addRequestFees, 1, submitterDid)
  INDY_ASSERT_STRING(addRequestFees, 2, req)
  INDY_ASSERT_STRING(addRequestFees, 3, inputs)
  INDY_ASSERT_STRING(addRequestFees, 4, outputs)
  INDY_ASSERT_STRING(addRequestFees, 5, extra)
  INDY_ASSERT_FUNCTION(addRequestFees, 6)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  const char* arg3 = argToCString(info[3]);
  const char* arg4 = argToCString(info[4]);
  const char* arg5 = argToCString(info[5]);
  IndyCallback* icb = argToIndyCb(info[6]);
  indyCalled(icb, indy_add_request_fees(icb->handle, arg0, arg1, arg2, arg3, arg4, arg5, addRequestFees_cb));
  delete arg1;
  delete arg2;
  delete arg3;
  delete arg4;
  delete arg5;
}

void parseResponseWithFees_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(parseResponseWithFees) {
  INDY_ASSERT_NARGS(parseResponseWithFees, 3)
  INDY_ASSERT_STRING(parseResponseWithFees, 0, paymentMethod)
  INDY_ASSERT_STRING(parseResponseWithFees, 1, resp)
  INDY_ASSERT_FUNCTION(parseResponseWithFees, 2)
  const char* arg0 = argToCString(info[0]);
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_parse_response_with_fees(icb->handle, arg0, arg1, parseResponseWithFees_cb));
  delete arg0;
  delete arg1;
}

void buildGetPaymentSourcesRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0, const char* arg1) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringString(xerr, arg0, arg1);
  }
}
NAN_METHOD(buildGetPaymentSourcesRequest) {
  INDY_ASSERT_NARGS(buildGetPaymentSourcesRequest, 4)
  INDY_ASSERT_NUMBER(buildGetPaymentSourcesRequest, 0, wh)
  INDY_ASSERT_STRING(buildGetPaymentSourcesRequest, 1, submitterDid)
  INDY_ASSERT_STRING(buildGetPaymentSourcesRequest, 2, paymentAddress)
  INDY_ASSERT_FUNCTION(buildGetPaymentSourcesRequest, 3)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  IndyCallback* icb = argToIndyCb(info[3]);
  indyCalled(icb, indy_build_get_payment_sources_request(icb->handle, arg0, arg1, arg2, buildGetPaymentSourcesRequest_cb));
  delete arg1;
  delete arg2;
}

void parseGetPaymentSourcesResponse_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(parseGetPaymentSourcesResponse) {
  INDY_ASSERT_NARGS(parseGetPaymentSourcesResponse, 3)
  INDY_ASSERT_STRING(parseGetPaymentSourcesResponse, 0, paymentMethod)
  INDY_ASSERT_STRING(parseGetPaymentSourcesResponse, 1, resp)
  INDY_ASSERT_FUNCTION(parseGetPaymentSourcesResponse, 2)
  const char* arg0 = argToCString(info[0]);
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_parse_get_payment_sources_response(icb->handle, arg0, arg1, parseGetPaymentSourcesResponse_cb));
  delete arg0;
  delete arg1;
}

void buildPaymentReq_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0, const char* arg1) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringString(xerr, arg0, arg1);
  }
}
NAN_METHOD(buildPaymentReq) {
  INDY_ASSERT_NARGS(buildPaymentReq, 6)
  INDY_ASSERT_NUMBER(buildPaymentReq, 0, wh)
  INDY_ASSERT_STRING(buildPaymentReq, 1, submitterDid)
  INDY_ASSERT_STRING(buildPaymentReq, 2, inputs)
  INDY_ASSERT_STRING(buildPaymentReq, 3, outputs)
  INDY_ASSERT_STRING(buildPaymentReq, 4, extra)
  INDY_ASSERT_FUNCTION(buildPaymentReq, 5)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  const char* arg3 = argToCString(info[3]);
  const char* arg4 = argToCString(info[4]);
  IndyCallback* icb = argToIndyCb(info[5]);
  indyCalled(icb, indy_build_payment_req(icb->handle, arg0, arg1, arg2, arg3, arg4, buildPaymentReq_cb));
  delete arg1;
  delete arg2;
  delete arg3;
  delete arg4;
}

void parsePaymentResponse_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(parsePaymentResponse) {
  INDY_ASSERT_NARGS(parsePaymentResponse, 3)
  INDY_ASSERT_STRING(parsePaymentResponse, 0, paymentMethod)
  INDY_ASSERT_STRING(parsePaymentResponse, 1, resp)
  INDY_ASSERT_FUNCTION(parsePaymentResponse, 2)
  const char* arg0 = argToCString(info[0]);
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_parse_payment_response(icb->handle, arg0, arg1, parsePaymentResponse_cb));
  delete arg0;
  delete arg1;
}

void buildMintReq_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0, const char* arg1) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringString(xerr, arg0, arg1);
  }
}
NAN_METHOD(buildMintReq) {
  INDY_ASSERT_NARGS(buildMintReq, 5)
  INDY_ASSERT_NUMBER(buildMintReq, 0, wh)
  INDY_ASSERT_STRING(buildMintReq, 1, submitterDid)
  INDY_ASSERT_STRING(buildMintReq, 2, outputs)
  INDY_ASSERT_STRING(buildMintReq, 3, extra)
  INDY_ASSERT_FUNCTION(buildMintReq, 4)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  const char* arg3 = argToCString(info[3]);
  IndyCallback* icb = argToIndyCb(info[4]);
  indyCalled(icb, indy_build_mint_req(icb->handle, arg0, arg1, arg2, arg3, buildMintReq_cb));
  delete arg1;
  delete arg2;
  delete arg3;
}

void buildSetTxnFeesReq_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildSetTxnFeesReq) {
  INDY_ASSERT_NARGS(buildSetTxnFeesReq, 5)
  INDY_ASSERT_NUMBER(buildSetTxnFeesReq, 0, wh)
  INDY_ASSERT_STRING(buildSetTxnFeesReq, 1, submitterDid)
  INDY_ASSERT_STRING(buildSetTxnFeesReq, 2, paymentMethod)
  INDY_ASSERT_STRING(buildSetTxnFeesReq, 3, fees)
  INDY_ASSERT_FUNCTION(buildSetTxnFeesReq, 4)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  const char* arg3 = argToCString(info[3]);
  IndyCallback* icb = argToIndyCb(info[4]);
  indyCalled(icb, indy_build_set_txn_fees_req(icb->handle, arg0, arg1, arg2, arg3, buildSetTxnFeesReq_cb));
  delete arg1;
  delete arg2;
  delete arg3;
}

void buildGetTxnFeesReq_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildGetTxnFeesReq) {
  INDY_ASSERT_NARGS(buildGetTxnFeesReq, 4)
  INDY_ASSERT_NUMBER(buildGetTxnFeesReq, 0, wh)
  INDY_ASSERT_STRING(buildGetTxnFeesReq, 1, submitterDid)
  INDY_ASSERT_STRING(buildGetTxnFeesReq, 2, paymentMethod)
  INDY_ASSERT_FUNCTION(buildGetTxnFeesReq, 3)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  IndyCallback* icb = argToIndyCb(info[3]);
  indyCalled(icb, indy_build_get_txn_fees_req(icb->handle, arg0, arg1, arg2, buildGetTxnFeesReq_cb));
  delete arg1;
  delete arg2;
}

void parseGetTxnFeesResponse_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(parseGetTxnFeesResponse) {
  INDY_ASSERT_NARGS(parseGetTxnFeesResponse, 3)
  INDY_ASSERT_STRING(parseGetTxnFeesResponse, 0, paymentMethod)
  INDY_ASSERT_STRING(parseGetTxnFeesResponse, 1, resp)
  INDY_ASSERT_FUNCTION(parseGetTxnFeesResponse, 2)
  const char* arg0 = argToCString(info[0]);
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_parse_get_txn_fees_response(icb->handle, arg0, arg1, parseGetTxnFeesResponse_cb));
  delete arg0;
  delete arg1;
}

void buildVerifyPaymentReq_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0, const char* arg1) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringString(xerr, arg0, arg1);
  }
}
NAN_METHOD(buildVerifyPaymentReq) {
  INDY_ASSERT_NARGS(buildVerifyPaymentReq, 4)
  INDY_ASSERT_NUMBER(buildVerifyPaymentReq, 0, wh)
  INDY_ASSERT_STRING(buildVerifyPaymentReq, 1, submitterDid)
  INDY_ASSERT_STRING(buildVerifyPaymentReq, 2, receipt)
  INDY_ASSERT_FUNCTION(buildVerifyPaymentReq, 3)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  IndyCallback* icb = argToIndyCb(info[3]);
  indyCalled(icb, indy_build_verify_payment_req(icb->handle, arg0, arg1, arg2, buildVerifyPaymentReq_cb));
  delete arg1;
  delete arg2;
}

void parseVerifyPaymentResponse_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(parseVerifyPaymentResponse) {
  INDY_ASSERT_NARGS(parseVerifyPaymentResponse, 3)
  INDY_ASSERT_STRING(parseVerifyPaymentResponse, 0, paymentMethod)
  INDY_ASSERT_STRING(parseVerifyPaymentResponse, 1, resp)
  INDY_ASSERT_FUNCTION(parseVerifyPaymentResponse, 2)
  const char* arg0 = argToCString(info[0]);
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_parse_verify_payment_response(icb->handle, arg0, arg1, parseVerifyPaymentResponse_cb));
  delete arg0;
  delete arg1;
}

void createPoolLedgerConfig_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(createPoolLedgerConfig) {
  INDY_ASSERT_NARGS(createPoolLedgerConfig, 3)
  INDY_ASSERT_STRING(createPoolLedgerConfig, 0, configName)
  INDY_ASSERT_STRING(createPoolLedgerConfig, 1, config)
  INDY_ASSERT_FUNCTION(createPoolLedgerConfig, 2)
  const char* arg0 = argToCString(info[0]);
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_create_pool_ledger_config(icb->handle, arg0, arg1, createPoolLedgerConfig_cb));
  delete arg0;
  delete arg1;
}

void openPoolLedger_cb(indy_handle_t handle, indy_error_t xerr, indy_handle_t arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbHandle(xerr, arg0);
  }
}
NAN_METHOD(openPoolLedger) {
  INDY_ASSERT_NARGS(openPoolLedger, 3)
  INDY_ASSERT_STRING(openPoolLedger, 0, configName)
  INDY_ASSERT_STRING(openPoolLedger, 1, config)
  INDY_ASSERT_FUNCTION(openPoolLedger, 2)
  const char* arg0 = argToCString(info[0]);
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_open_pool_ledger(icb->handle, arg0, arg1, openPoolLedger_cb));
  delete arg0;
  delete arg1;
}

void refreshPoolLedger_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(refreshPoolLedger) {
  INDY_ASSERT_NARGS(refreshPoolLedger, 2)
  INDY_ASSERT_NUMBER(refreshPoolLedger, 0, handle)
  INDY_ASSERT_FUNCTION(refreshPoolLedger, 1)
  indy_handle_t arg0 = info[0]->Int32Value();
  IndyCallback* icb = argToIndyCb(info[1]);
  indyCalled(icb, indy_refresh_pool_ledger(icb->handle, arg0, refreshPoolLedger_cb));
}

void listPools_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(listPools) {
  INDY_ASSERT_NARGS(listPools, 1)
  INDY_ASSERT_FUNCTION(listPools, 0)
  IndyCallback* icb = argToIndyCb(info[0]);
  indyCalled(icb, indy_list_pools(icb->handle, listPools_cb));
}

void closePoolLedger_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(closePoolLedger) {
  INDY_ASSERT_NARGS(closePoolLedger, 2)
  INDY_ASSERT_NUMBER(closePoolLedger, 0, handle)
  INDY_ASSERT_FUNCTION(closePoolLedger, 1)
  indy_handle_t arg0 = info[0]->Int32Value();
  IndyCallback* icb = argToIndyCb(info[1]);
  indyCalled(icb, indy_close_pool_ledger(icb->handle, arg0, closePoolLedger_cb));
}

void deletePoolLedgerConfig_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(deletePoolLedgerConfig) {
  INDY_ASSERT_NARGS(deletePoolLedgerConfig, 2)
  INDY_ASSERT_STRING(deletePoolLedgerConfig, 0, configName)
  INDY_ASSERT_FUNCTION(deletePoolLedgerConfig, 1)
  const char* arg0 = argToCString(info[0]);
  IndyCallback* icb = argToIndyCb(info[1]);
  indyCalled(icb, indy_delete_pool_ledger_config(icb->handle, arg0, deletePoolLedgerConfig_cb));
  delete arg0;
}

void setProtocolVersion_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(setProtocolVersion) {
  INDY_ASSERT_NARGS(setProtocolVersion, 2)
  INDY_ASSERT_NUMBER(setProtocolVersion, 0, protocolVersion)
  INDY_ASSERT_FUNCTION(setProtocolVersion, 1)
  indy_u64_t arg0 = (indy_u64_t)info[0]->Uint32Value();
  IndyCallback* icb = argToIndyCb(info[1]);
  indyCalled(icb, indy_set_protocol_version(icb->handle, arg0, setProtocolVersion_cb));
}

void createWallet_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(createWallet) {
  INDY_ASSERT_NARGS(createWallet, 3)
  INDY_ASSERT_STRING(createWallet, 0, config)
  INDY_ASSERT_STRING(createWallet, 1, credentials)
  INDY_ASSERT_FUNCTION(createWallet, 2)
  const char* arg0 = argToCString(info[0]);
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_create_wallet(icb->handle, arg0, arg1, createWallet_cb));
  delete arg0;
  delete arg1;
}

void openWallet_cb(indy_handle_t handle, indy_error_t xerr, indy_handle_t arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbHandle(xerr, arg0);
  }
}
NAN_METHOD(openWallet) {
  INDY_ASSERT_NARGS(openWallet, 3)
  INDY_ASSERT_STRING(openWallet, 0, config)
  INDY_ASSERT_STRING(openWallet, 1, credentials)
  INDY_ASSERT_FUNCTION(openWallet, 2)
  const char* arg0 = argToCString(info[0]);
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_open_wallet(icb->handle, arg0, arg1, openWallet_cb));
  delete arg0;
  delete arg1;
}

void exportWallet_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(exportWallet) {
  INDY_ASSERT_NARGS(exportWallet, 3)
  INDY_ASSERT_NUMBER(exportWallet, 0, wh)
  INDY_ASSERT_STRING(exportWallet, 1, exportConfig)
  INDY_ASSERT_FUNCTION(exportWallet, 2)
  indy_handle_t arg0 = info[0]->Int32Value();
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_export_wallet(icb->handle, arg0, arg1, exportWallet_cb));
  delete arg1;
}

void importWallet_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(importWallet) {
  INDY_ASSERT_NARGS(importWallet, 4)
  INDY_ASSERT_STRING(importWallet, 0, config)
  INDY_ASSERT_STRING(importWallet, 1, credentials)
  INDY_ASSERT_STRING(importWallet, 2, importConfig)
  INDY_ASSERT_FUNCTION(importWallet, 3)
  const char* arg0 = argToCString(info[0]);
  const char* arg1 = argToCString(info[1]);
  const char* arg2 = argToCString(info[2]);
  IndyCallback* icb = argToIndyCb(info[3]);
  indyCalled(icb, indy_import_wallet(icb->handle, arg0, arg1, arg2, importWallet_cb));
  delete arg0;
  delete arg1;
  delete arg2;
}

void closeWallet_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(closeWallet) {
  INDY_ASSERT_NARGS(closeWallet, 2)
  INDY_ASSERT_NUMBER(closeWallet, 0, wh)
  INDY_ASSERT_FUNCTION(closeWallet, 1)
  indy_handle_t arg0 = info[0]->Int32Value();
  IndyCallback* icb = argToIndyCb(info[1]);
  indyCalled(icb, indy_close_wallet(icb->handle, arg0, closeWallet_cb));
}

void deleteWallet_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(deleteWallet) {
  INDY_ASSERT_NARGS(deleteWallet, 3)
  INDY_ASSERT_STRING(deleteWallet, 0, config)
  INDY_ASSERT_STRING(deleteWallet, 1, credentials)
  INDY_ASSERT_FUNCTION(deleteWallet, 2)
  const char* arg0 = argToCString(info[0]);
  const char* arg1 = argToCString(info[1]);
  IndyCallback* icb = argToIndyCb(info[2]);
  indyCalled(icb, indy_delete_wallet(icb->handle, arg0, arg1, deleteWallet_cb));
  delete arg0;
  delete arg1;
}

NAN_MODULE_INIT(InitAll) {
  Nan::Export(target, "issuerCreateSchema", issuerCreateSchema);
  Nan::Export(target, "issuerCreateAndStoreCredentialDef", issuerCreateAndStoreCredentialDef);
  Nan::Export(target, "issuerCreateAndStoreRevocReg", issuerCreateAndStoreRevocReg);
  Nan::Export(target, "issuerCreateCredentialOffer", issuerCreateCredentialOffer);
  Nan::Export(target, "issuerCreateCredential", issuerCreateCredential);
  Nan::Export(target, "issuerRevokeCredential", issuerRevokeCredential);
  Nan::Export(target, "issuerMergeRevocationRegistryDeltas", issuerMergeRevocationRegistryDeltas);
  Nan::Export(target, "proverCreateMasterSecret", proverCreateMasterSecret);
  Nan::Export(target, "proverCreateCredentialReq", proverCreateCredentialReq);
  Nan::Export(target, "proverStoreCredential", proverStoreCredential);
  Nan::Export(target, "proverGetCredentials", proverGetCredentials);
  Nan::Export(target, "proverGetCredential", proverGetCredential);
  Nan::Export(target, "proverSearchCredentials", proverSearchCredentials);
  Nan::Export(target, "proverFetchCredentials", proverFetchCredentials);
  Nan::Export(target, "proverCloseCredentialsSearch", proverCloseCredentialsSearch);
  Nan::Export(target, "proverGetCredentialsForProofReq", proverGetCredentialsForProofReq);
  Nan::Export(target, "proverSearchCredentialsForProofReq", proverSearchCredentialsForProofReq);
  Nan::Export(target, "proverFetchCredentialsForProofReq", proverFetchCredentialsForProofReq);
  Nan::Export(target, "proverCloseCredentialsSearchForProofReq", proverCloseCredentialsSearchForProofReq);
  Nan::Export(target, "proverCreateProof", proverCreateProof);
  Nan::Export(target, "verifierVerifyProof", verifierVerifyProof);
  Nan::Export(target, "createRevocationState", createRevocationState);
  Nan::Export(target, "updateRevocationState", updateRevocationState);
  Nan::Export(target, "openBlobStorageReader", openBlobStorageReader);
  Nan::Export(target, "openBlobStorageWriter", openBlobStorageWriter);
  Nan::Export(target, "createKey", createKey);
  Nan::Export(target, "setKeyMetadata", setKeyMetadata);
  Nan::Export(target, "getKeyMetadata", getKeyMetadata);
  Nan::Export(target, "cryptoSign", cryptoSign);
  Nan::Export(target, "cryptoVerify", cryptoVerify);
  Nan::Export(target, "cryptoAuthCrypt", cryptoAuthCrypt);
  Nan::Export(target, "cryptoAuthDecrypt", cryptoAuthDecrypt);
  Nan::Export(target, "cryptoAnonCrypt", cryptoAnonCrypt);
  Nan::Export(target, "cryptoAnonDecrypt", cryptoAnonDecrypt);
  Nan::Export(target, "createAndStoreMyDid", createAndStoreMyDid);
  Nan::Export(target, "replaceKeysStart", replaceKeysStart);
  Nan::Export(target, "replaceKeysApply", replaceKeysApply);
  Nan::Export(target, "storeTheirDid", storeTheirDid);
  Nan::Export(target, "keyForDid", keyForDid);
  Nan::Export(target, "keyForLocalDid", keyForLocalDid);
  Nan::Export(target, "setEndpointForDid", setEndpointForDid);
  Nan::Export(target, "getEndpointForDid", getEndpointForDid);
  Nan::Export(target, "setDidMetadata", setDidMetadata);
  Nan::Export(target, "getDidMetadata", getDidMetadata);
  Nan::Export(target, "getMyDidWithMeta", getMyDidWithMeta);
  Nan::Export(target, "listMyDidsWithMeta", listMyDidsWithMeta);
  Nan::Export(target, "abbreviateVerkey", abbreviateVerkey);
  Nan::Export(target, "signAndSubmitRequest", signAndSubmitRequest);
  Nan::Export(target, "submitRequest", submitRequest);
  Nan::Export(target, "signRequest", signRequest);
  Nan::Export(target, "multiSignRequest", multiSignRequest);
  Nan::Export(target, "buildGetDdoRequest", buildGetDdoRequest);
  Nan::Export(target, "buildNymRequest", buildNymRequest);
  Nan::Export(target, "buildAttribRequest", buildAttribRequest);
  Nan::Export(target, "buildGetAttribRequest", buildGetAttribRequest);
  Nan::Export(target, "buildGetNymRequest", buildGetNymRequest);
  Nan::Export(target, "buildSchemaRequest", buildSchemaRequest);
  Nan::Export(target, "buildGetSchemaRequest", buildGetSchemaRequest);
  Nan::Export(target, "parseGetSchemaResponse", parseGetSchemaResponse);
  Nan::Export(target, "buildCredDefRequest", buildCredDefRequest);
  Nan::Export(target, "buildGetCredDefRequest", buildGetCredDefRequest);
  Nan::Export(target, "parseGetCredDefResponse", parseGetCredDefResponse);
  Nan::Export(target, "buildNodeRequest", buildNodeRequest);
  Nan::Export(target, "buildGetValidatorInfoRequest", buildGetValidatorInfoRequest);
  Nan::Export(target, "buildGetTxnRequest", buildGetTxnRequest);
  Nan::Export(target, "buildPoolConfigRequest", buildPoolConfigRequest);
  Nan::Export(target, "buildPoolRestartRequest", buildPoolRestartRequest);
  Nan::Export(target, "buildPoolUpgradeRequest", buildPoolUpgradeRequest);
  Nan::Export(target, "buildRevocRegDefRequest", buildRevocRegDefRequest);
  Nan::Export(target, "buildGetRevocRegDefRequest", buildGetRevocRegDefRequest);
  Nan::Export(target, "parseGetRevocRegDefResponse", parseGetRevocRegDefResponse);
  Nan::Export(target, "buildRevocRegEntryRequest", buildRevocRegEntryRequest);
  Nan::Export(target, "buildGetRevocRegRequest", buildGetRevocRegRequest);
  Nan::Export(target, "parseGetRevocRegResponse", parseGetRevocRegResponse);
  Nan::Export(target, "buildGetRevocRegDeltaRequest", buildGetRevocRegDeltaRequest);
  Nan::Export(target, "parseGetRevocRegDeltaResponse", parseGetRevocRegDeltaResponse);
  Nan::Export(target, "addWalletRecord", addWalletRecord);
  Nan::Export(target, "updateWalletRecordValue", updateWalletRecordValue);
  Nan::Export(target, "updateWalletRecordTags", updateWalletRecordTags);
  Nan::Export(target, "addWalletRecordTags", addWalletRecordTags);
  Nan::Export(target, "deleteWalletRecordTags", deleteWalletRecordTags);
  Nan::Export(target, "deleteWalletRecord", deleteWalletRecord);
  Nan::Export(target, "getWalletRecord", getWalletRecord);
  Nan::Export(target, "openWalletSearch", openWalletSearch);
  Nan::Export(target, "fetchWalletSearchNextRecords", fetchWalletSearchNextRecords);
  Nan::Export(target, "closeWalletSearch", closeWalletSearch);
  Nan::Export(target, "isPairwiseExists", isPairwiseExists);
  Nan::Export(target, "createPairwise", createPairwise);
  Nan::Export(target, "listPairwise", listPairwise);
  Nan::Export(target, "getPairwise", getPairwise);
  Nan::Export(target, "setPairwiseMetadata", setPairwiseMetadata);
  Nan::Export(target, "createPaymentAddress", createPaymentAddress);
  Nan::Export(target, "listPaymentAddresses", listPaymentAddresses);
  Nan::Export(target, "addRequestFees", addRequestFees);
  Nan::Export(target, "parseResponseWithFees", parseResponseWithFees);
  Nan::Export(target, "buildGetPaymentSourcesRequest", buildGetPaymentSourcesRequest);
  Nan::Export(target, "parseGetPaymentSourcesResponse", parseGetPaymentSourcesResponse);
  Nan::Export(target, "buildPaymentReq", buildPaymentReq);
  Nan::Export(target, "parsePaymentResponse", parsePaymentResponse);
  Nan::Export(target, "buildMintReq", buildMintReq);
  Nan::Export(target, "buildSetTxnFeesReq", buildSetTxnFeesReq);
  Nan::Export(target, "buildGetTxnFeesReq", buildGetTxnFeesReq);
  Nan::Export(target, "parseGetTxnFeesResponse", parseGetTxnFeesResponse);
  Nan::Export(target, "buildVerifyPaymentReq", buildVerifyPaymentReq);
  Nan::Export(target, "parseVerifyPaymentResponse", parseVerifyPaymentResponse);
  Nan::Export(target, "createPoolLedgerConfig", createPoolLedgerConfig);
  Nan::Export(target, "openPoolLedger", openPoolLedger);
  Nan::Export(target, "refreshPoolLedger", refreshPoolLedger);
  Nan::Export(target, "listPools", listPools);
  Nan::Export(target, "closePoolLedger", closePoolLedger);
  Nan::Export(target, "deletePoolLedgerConfig", deletePoolLedgerConfig);
  Nan::Export(target, "setProtocolVersion", setProtocolVersion);
  Nan::Export(target, "createWallet", createWallet);
  Nan::Export(target, "openWallet", openWallet);
  Nan::Export(target, "exportWallet", exportWallet);
  Nan::Export(target, "importWallet", importWallet);
  Nan::Export(target, "closeWallet", closeWallet);
  Nan::Export(target, "deleteWallet", deleteWallet);
}
NODE_MODULE(indynodejs, InitAll)

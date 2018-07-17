void issuerCreateSchema_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0, const char* arg1) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringString(xerr, arg0, arg1);
  }
}
NAN_METHOD(issuerCreateSchema) {
  if(info.Length() != 5){
    return Nan::ThrowError(Nan::New("Expected 5 arguments: issuerCreateSchema(issuer_did, name, version, attr_names, cb(err, [ id, schema ]))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for issuer_did: issuerCreateSchema(issuer_did, name, version, attr_names, cb(err, [ id, schema ]))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for name: issuerCreateSchema(issuer_did, name, version, attr_names, cb(err, [ id, schema ]))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for version: issuerCreateSchema(issuer_did, name, version, attr_names, cb(err, [ id, schema ]))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for attr_names: issuerCreateSchema(issuer_did, name, version, attr_names, cb(err, [ id, schema ]))").ToLocalChecked());
  }
  if(!info[4]->IsFunction()) {
    return Nan::ThrowError(Nan::New("issuerCreateSchema arg 4 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[4]).ToLocalChecked());
  indyCalled(icb, indy_issuer_create_schema(icb->handle, arg0, arg1, arg2, arg3, issuerCreateSchema_cb));
  delete arg0UTF;
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
}

void issuerCreateAndStoreCredentialDef_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0, const char* arg1) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringString(xerr, arg0, arg1);
  }
}
NAN_METHOD(issuerCreateAndStoreCredentialDef) {
  if(info.Length() != 7){
    return Nan::ThrowError(Nan::New("Expected 7 arguments: issuerCreateAndStoreCredentialDef(wallet_handle, issuer_did, schema_json, tag, signature_type, config_json, cb(err, [ credDefId, credDef ]))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: issuerCreateAndStoreCredentialDef(wallet_handle, issuer_did, schema_json, tag, signature_type, config_json, cb(err, [ credDefId, credDef ]))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for issuer_did: issuerCreateAndStoreCredentialDef(wallet_handle, issuer_did, schema_json, tag, signature_type, config_json, cb(err, [ credDefId, credDef ]))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for schema_json: issuerCreateAndStoreCredentialDef(wallet_handle, issuer_did, schema_json, tag, signature_type, config_json, cb(err, [ credDefId, credDef ]))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for tag: issuerCreateAndStoreCredentialDef(wallet_handle, issuer_did, schema_json, tag, signature_type, config_json, cb(err, [ credDefId, credDef ]))").ToLocalChecked());
  }
  Nan::Utf8String* arg4UTF = nullptr;
  const char* arg4 = nullptr;
  if(info[4]->IsString()){
    arg4UTF = new Nan::Utf8String(info[4]);
    arg4 = (const char*)(**arg4UTF);
  } else if(!info[4]->IsNull() && !info[4]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for signature_type: issuerCreateAndStoreCredentialDef(wallet_handle, issuer_did, schema_json, tag, signature_type, config_json, cb(err, [ credDefId, credDef ]))").ToLocalChecked());
  }
  Nan::Utf8String* arg5UTF = nullptr;
  const char* arg5 = nullptr;
  if(info[5]->IsString()){
    arg5UTF = new Nan::Utf8String(info[5]);
    arg5 = (const char*)(**arg5UTF);
  } else if(!info[5]->IsNull() && !info[5]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for config_json: issuerCreateAndStoreCredentialDef(wallet_handle, issuer_did, schema_json, tag, signature_type, config_json, cb(err, [ credDefId, credDef ]))").ToLocalChecked());
  }
  if(!info[6]->IsFunction()) {
    return Nan::ThrowError(Nan::New("issuerCreateAndStoreCredentialDef arg 6 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[6]).ToLocalChecked());
  indyCalled(icb, indy_issuer_create_and_store_credential_def(icb->handle, arg0, arg1, arg2, arg3, arg4, arg5, issuerCreateAndStoreCredentialDef_cb));
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
  delete arg4UTF;
  delete arg5UTF;
}

void issuerCreateAndStoreRevocReg_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0, const char* arg1, const char* arg2) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringStringString(xerr, arg0, arg1, arg2);
  }
}
NAN_METHOD(issuerCreateAndStoreRevocReg) {
  if(info.Length() != 8){
    return Nan::ThrowError(Nan::New("Expected 8 arguments: issuerCreateAndStoreRevocReg(wallet_handle, issuer_did, revoc_def_type, tag, cred_def_id, config_json, tails_writer_handle, cb(err, [ revocRegId, revocRegDef, revocRegEntry ]))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: issuerCreateAndStoreRevocReg(wallet_handle, issuer_did, revoc_def_type, tag, cred_def_id, config_json, tails_writer_handle, cb(err, [ revocRegId, revocRegDef, revocRegEntry ]))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for issuer_did: issuerCreateAndStoreRevocReg(wallet_handle, issuer_did, revoc_def_type, tag, cred_def_id, config_json, tails_writer_handle, cb(err, [ revocRegId, revocRegDef, revocRegEntry ]))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for revoc_def_type: issuerCreateAndStoreRevocReg(wallet_handle, issuer_did, revoc_def_type, tag, cred_def_id, config_json, tails_writer_handle, cb(err, [ revocRegId, revocRegDef, revocRegEntry ]))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for tag: issuerCreateAndStoreRevocReg(wallet_handle, issuer_did, revoc_def_type, tag, cred_def_id, config_json, tails_writer_handle, cb(err, [ revocRegId, revocRegDef, revocRegEntry ]))").ToLocalChecked());
  }
  Nan::Utf8String* arg4UTF = nullptr;
  const char* arg4 = nullptr;
  if(info[4]->IsString()){
    arg4UTF = new Nan::Utf8String(info[4]);
    arg4 = (const char*)(**arg4UTF);
  } else if(!info[4]->IsNull() && !info[4]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for cred_def_id: issuerCreateAndStoreRevocReg(wallet_handle, issuer_did, revoc_def_type, tag, cred_def_id, config_json, tails_writer_handle, cb(err, [ revocRegId, revocRegDef, revocRegEntry ]))").ToLocalChecked());
  }
  Nan::Utf8String* arg5UTF = nullptr;
  const char* arg5 = nullptr;
  if(info[5]->IsString()){
    arg5UTF = new Nan::Utf8String(info[5]);
    arg5 = (const char*)(**arg5UTF);
  } else if(!info[5]->IsNull() && !info[5]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for config_json: issuerCreateAndStoreRevocReg(wallet_handle, issuer_did, revoc_def_type, tag, cred_def_id, config_json, tails_writer_handle, cb(err, [ revocRegId, revocRegDef, revocRegEntry ]))").ToLocalChecked());
  }
  if(!info[6]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for tails_writer_handle: issuerCreateAndStoreRevocReg(wallet_handle, issuer_did, revoc_def_type, tag, cred_def_id, config_json, tails_writer_handle, cb(err, [ revocRegId, revocRegDef, revocRegEntry ]))").ToLocalChecked());
  }
  indy_handle_t arg6 = info[6]->Int32Value();
  if(!info[7]->IsFunction()) {
    return Nan::ThrowError(Nan::New("issuerCreateAndStoreRevocReg arg 7 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[7]).ToLocalChecked());
  indyCalled(icb, indy_issuer_create_and_store_revoc_reg(icb->handle, arg0, arg1, arg2, arg3, arg4, arg5, arg6, issuerCreateAndStoreRevocReg_cb));
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
  delete arg4UTF;
  delete arg5UTF;
}

void issuerCreateCredentialOffer_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(issuerCreateCredentialOffer) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: issuerCreateCredentialOffer(wallet_handle, cred_def_id, cb(err, credOffer))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: issuerCreateCredentialOffer(wallet_handle, cred_def_id, cb(err, credOffer))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for cred_def_id: issuerCreateCredentialOffer(wallet_handle, cred_def_id, cb(err, credOffer))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("issuerCreateCredentialOffer arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_issuer_create_credential_offer(icb->handle, arg0, arg1, issuerCreateCredentialOffer_cb));
  delete arg1UTF;
}

void issuerCreateCredential_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0, const char* arg1, const char* arg2) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringStringString(xerr, arg0, arg1, arg2);
  }
}
NAN_METHOD(issuerCreateCredential) {
  if(info.Length() != 7){
    return Nan::ThrowError(Nan::New("Expected 7 arguments: issuerCreateCredential(wallet_handle, cred_offer_json, cred_req_json, cred_values_json, rev_reg_id, blob_storage_reader_handle, cb(err, [ cred, credRevocId, revocRegDelta ]))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: issuerCreateCredential(wallet_handle, cred_offer_json, cred_req_json, cred_values_json, rev_reg_id, blob_storage_reader_handle, cb(err, [ cred, credRevocId, revocRegDelta ]))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for cred_offer_json: issuerCreateCredential(wallet_handle, cred_offer_json, cred_req_json, cred_values_json, rev_reg_id, blob_storage_reader_handle, cb(err, [ cred, credRevocId, revocRegDelta ]))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for cred_req_json: issuerCreateCredential(wallet_handle, cred_offer_json, cred_req_json, cred_values_json, rev_reg_id, blob_storage_reader_handle, cb(err, [ cred, credRevocId, revocRegDelta ]))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for cred_values_json: issuerCreateCredential(wallet_handle, cred_offer_json, cred_req_json, cred_values_json, rev_reg_id, blob_storage_reader_handle, cb(err, [ cred, credRevocId, revocRegDelta ]))").ToLocalChecked());
  }
  Nan::Utf8String* arg4UTF = nullptr;
  const char* arg4 = nullptr;
  if(info[4]->IsString()){
    arg4UTF = new Nan::Utf8String(info[4]);
    arg4 = (const char*)(**arg4UTF);
  } else if(!info[4]->IsNull() && !info[4]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for rev_reg_id: issuerCreateCredential(wallet_handle, cred_offer_json, cred_req_json, cred_values_json, rev_reg_id, blob_storage_reader_handle, cb(err, [ cred, credRevocId, revocRegDelta ]))").ToLocalChecked());
  }
  if(!info[5]->IsInt32()){
    return Nan::ThrowError(Nan::New("Expected indy_i32_t for blob_storage_reader_handle: issuerCreateCredential(wallet_handle, cred_offer_json, cred_req_json, cred_values_json, rev_reg_id, blob_storage_reader_handle, cb(err, [ cred, credRevocId, revocRegDelta ]))").ToLocalChecked());
  }
  indy_i32_t arg5 = info[5]->Int32Value();
  if(!info[6]->IsFunction()) {
    return Nan::ThrowError(Nan::New("issuerCreateCredential arg 6 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[6]).ToLocalChecked());
  indyCalled(icb, indy_issuer_create_credential(icb->handle, arg0, arg1, arg2, arg3, arg4, arg5, issuerCreateCredential_cb));
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
  delete arg4UTF;
}

void issuerRevokeCredential_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(issuerRevokeCredential) {
  if(info.Length() != 5){
    return Nan::ThrowError(Nan::New("Expected 5 arguments: issuerRevokeCredential(wallet_handle, blob_storage_reader_handle, rev_reg_id, cred_revoc_id, cb(err, revocRegDelta))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: issuerRevokeCredential(wallet_handle, blob_storage_reader_handle, rev_reg_id, cred_revoc_id, cb(err, revocRegDelta))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  if(!info[1]->IsInt32()){
    return Nan::ThrowError(Nan::New("Expected indy_i32_t for blob_storage_reader_handle: issuerRevokeCredential(wallet_handle, blob_storage_reader_handle, rev_reg_id, cred_revoc_id, cb(err, revocRegDelta))").ToLocalChecked());
  }
  indy_i32_t arg1 = info[1]->Int32Value();
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for rev_reg_id: issuerRevokeCredential(wallet_handle, blob_storage_reader_handle, rev_reg_id, cred_revoc_id, cb(err, revocRegDelta))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for cred_revoc_id: issuerRevokeCredential(wallet_handle, blob_storage_reader_handle, rev_reg_id, cred_revoc_id, cb(err, revocRegDelta))").ToLocalChecked());
  }
  if(!info[4]->IsFunction()) {
    return Nan::ThrowError(Nan::New("issuerRevokeCredential arg 4 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[4]).ToLocalChecked());
  indyCalled(icb, indy_issuer_revoke_credential(icb->handle, arg0, arg1, arg2, arg3, issuerRevokeCredential_cb));
  delete arg2UTF;
  delete arg3UTF;
}

void issuerMergeRevocationRegistryDeltas_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(issuerMergeRevocationRegistryDeltas) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: issuerMergeRevocationRegistryDeltas(rev_reg_delta_json, other_rev_reg_delta_json, cb(err, mergedRevRegDelta))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for rev_reg_delta_json: issuerMergeRevocationRegistryDeltas(rev_reg_delta_json, other_rev_reg_delta_json, cb(err, mergedRevRegDelta))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for other_rev_reg_delta_json: issuerMergeRevocationRegistryDeltas(rev_reg_delta_json, other_rev_reg_delta_json, cb(err, mergedRevRegDelta))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("issuerMergeRevocationRegistryDeltas arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_issuer_merge_revocation_registry_deltas(icb->handle, arg0, arg1, issuerMergeRevocationRegistryDeltas_cb));
  delete arg0UTF;
  delete arg1UTF;
}

void proverCreateMasterSecret_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(proverCreateMasterSecret) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: proverCreateMasterSecret(wallet_handle, master_secret_id, cb(err, outMasterSecretId))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: proverCreateMasterSecret(wallet_handle, master_secret_id, cb(err, outMasterSecretId))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for master_secret_id: proverCreateMasterSecret(wallet_handle, master_secret_id, cb(err, outMasterSecretId))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("proverCreateMasterSecret arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_prover_create_master_secret(icb->handle, arg0, arg1, proverCreateMasterSecret_cb));
  delete arg1UTF;
}

void proverCreateCredentialReq_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0, const char* arg1) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringString(xerr, arg0, arg1);
  }
}
NAN_METHOD(proverCreateCredentialReq) {
  if(info.Length() != 6){
    return Nan::ThrowError(Nan::New("Expected 6 arguments: proverCreateCredentialReq(wallet_handle, prover_did, cred_offer_json, cred_def_json, master_secret_id, cb(err, [ credReq, credReqMetadata ]))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: proverCreateCredentialReq(wallet_handle, prover_did, cred_offer_json, cred_def_json, master_secret_id, cb(err, [ credReq, credReqMetadata ]))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for prover_did: proverCreateCredentialReq(wallet_handle, prover_did, cred_offer_json, cred_def_json, master_secret_id, cb(err, [ credReq, credReqMetadata ]))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for cred_offer_json: proverCreateCredentialReq(wallet_handle, prover_did, cred_offer_json, cred_def_json, master_secret_id, cb(err, [ credReq, credReqMetadata ]))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for cred_def_json: proverCreateCredentialReq(wallet_handle, prover_did, cred_offer_json, cred_def_json, master_secret_id, cb(err, [ credReq, credReqMetadata ]))").ToLocalChecked());
  }
  Nan::Utf8String* arg4UTF = nullptr;
  const char* arg4 = nullptr;
  if(info[4]->IsString()){
    arg4UTF = new Nan::Utf8String(info[4]);
    arg4 = (const char*)(**arg4UTF);
  } else if(!info[4]->IsNull() && !info[4]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for master_secret_id: proverCreateCredentialReq(wallet_handle, prover_did, cred_offer_json, cred_def_json, master_secret_id, cb(err, [ credReq, credReqMetadata ]))").ToLocalChecked());
  }
  if(!info[5]->IsFunction()) {
    return Nan::ThrowError(Nan::New("proverCreateCredentialReq arg 5 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[5]).ToLocalChecked());
  indyCalled(icb, indy_prover_create_credential_req(icb->handle, arg0, arg1, arg2, arg3, arg4, proverCreateCredentialReq_cb));
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
  delete arg4UTF;
}

void proverStoreCredential_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(proverStoreCredential) {
  if(info.Length() != 7){
    return Nan::ThrowError(Nan::New("Expected 7 arguments: proverStoreCredential(wallet_handle, cred_id, cred_req_metadata_json, cred_json, cred_def_json, rev_reg_def_json, cb(err, outCredId))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: proverStoreCredential(wallet_handle, cred_id, cred_req_metadata_json, cred_json, cred_def_json, rev_reg_def_json, cb(err, outCredId))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for cred_id: proverStoreCredential(wallet_handle, cred_id, cred_req_metadata_json, cred_json, cred_def_json, rev_reg_def_json, cb(err, outCredId))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for cred_req_metadata_json: proverStoreCredential(wallet_handle, cred_id, cred_req_metadata_json, cred_json, cred_def_json, rev_reg_def_json, cb(err, outCredId))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for cred_json: proverStoreCredential(wallet_handle, cred_id, cred_req_metadata_json, cred_json, cred_def_json, rev_reg_def_json, cb(err, outCredId))").ToLocalChecked());
  }
  Nan::Utf8String* arg4UTF = nullptr;
  const char* arg4 = nullptr;
  if(info[4]->IsString()){
    arg4UTF = new Nan::Utf8String(info[4]);
    arg4 = (const char*)(**arg4UTF);
  } else if(!info[4]->IsNull() && !info[4]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for cred_def_json: proverStoreCredential(wallet_handle, cred_id, cred_req_metadata_json, cred_json, cred_def_json, rev_reg_def_json, cb(err, outCredId))").ToLocalChecked());
  }
  Nan::Utf8String* arg5UTF = nullptr;
  const char* arg5 = nullptr;
  if(info[5]->IsString()){
    arg5UTF = new Nan::Utf8String(info[5]);
    arg5 = (const char*)(**arg5UTF);
  } else if(!info[5]->IsNull() && !info[5]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for rev_reg_def_json: proverStoreCredential(wallet_handle, cred_id, cred_req_metadata_json, cred_json, cred_def_json, rev_reg_def_json, cb(err, outCredId))").ToLocalChecked());
  }
  if(!info[6]->IsFunction()) {
    return Nan::ThrowError(Nan::New("proverStoreCredential arg 6 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[6]).ToLocalChecked());
  indyCalled(icb, indy_prover_store_credential(icb->handle, arg0, arg1, arg2, arg3, arg4, arg5, proverStoreCredential_cb));
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
  delete arg4UTF;
  delete arg5UTF;
}

void proverGetCredentials_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(proverGetCredentials) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: proverGetCredentials(wallet_handle, filter_json, cb(err, credentials))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: proverGetCredentials(wallet_handle, filter_json, cb(err, credentials))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for filter_json: proverGetCredentials(wallet_handle, filter_json, cb(err, credentials))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("proverGetCredentials arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_prover_get_credentials(icb->handle, arg0, arg1, proverGetCredentials_cb));
  delete arg1UTF;
}

void proverGetCredential_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(proverGetCredential) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: proverGetCredential(wallet_handle, cred_id, cb(err, credentials))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: proverGetCredential(wallet_handle, cred_id, cb(err, credentials))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for cred_id: proverGetCredential(wallet_handle, cred_id, cb(err, credentials))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("proverGetCredential arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_prover_get_credential(icb->handle, arg0, arg1, proverGetCredential_cb));
  delete arg1UTF;
}

void proverSearchCredentials_cb(indy_handle_t handle, indy_error_t xerr, indy_handle_t arg0, indy_u32_t arg1) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbI32Usize(xerr, arg0, arg1);
  }
}
NAN_METHOD(proverSearchCredentials) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: proverSearchCredentials(wallet_handle, filter_json, cb(err, search_handle, total_count))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: proverSearchCredentials(wallet_handle, filter_json, cb(err, search_handle, total_count))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for filter_json: proverSearchCredentials(wallet_handle, filter_json, cb(err, search_handle, total_count))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("proverSearchCredentials arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_prover_search_credentials(icb->handle, arg0, arg1, proverSearchCredentials_cb));
  delete arg1UTF;
}

void proverFetchCredentials_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(proverFetchCredentials) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: proverFetchCredentials(search_handle, count, cb(err, credentials))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for search_handle: proverFetchCredentials(search_handle, count, cb(err, credentials))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  if(!info[1]->IsUint32()){
    return Nan::ThrowError(Nan::New("Expected indy_u32_t for count: proverFetchCredentials(search_handle, count, cb(err, credentials))").ToLocalChecked());
  }
  indy_u32_t arg1 = info[1]->Uint32Value();
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("fetchWalletSearchNextRecords arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_prover_fetch_credentials(icb->handle, arg0, arg1, proverFetchCredentials_cb));
}

void proverCloseCredentialsSearch_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(proverCloseCredentialsSearch) {
  if(info.Length() != 2){
    return Nan::ThrowError(Nan::New("Expected 2 arguments: proverCloseCredentialsSearch(search_handle, cb(err))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_search_handle: proverCloseCredentialsSearch(search_handle, cb(err))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  if(!info[1]->IsFunction()) {
    return Nan::ThrowError(Nan::New("proverCloseCredentialsSearch arg 1 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[1]).ToLocalChecked());
  indyCalled(icb, indy_prover_close_credentials_search(icb->handle, arg0, proverCloseCredentialsSearch_cb));
}

void proverGetCredentialsForProofReq_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(proverGetCredentialsForProofReq) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: proverGetCredentialsForProofReq(wallet_handle, proof_request_json, cb(err, credentials))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: proverGetCredentialsForProofReq(wallet_handle, proof_request_json, cb(err, credentials))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for proof_request_json: proverGetCredentialsForProofReq(wallet_handle, proof_request_json, cb(err, credentials))").ToLocalChecked());
  }

  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("proverGetCredentialsForProofReq arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_prover_get_credentials_for_proof_req(icb->handle, arg0, arg1, proverGetCredentialsForProofReq_cb));
  delete arg1UTF;
}

void proverSearchCredentialsForProofReq_cb(indy_handle_t handle, indy_error_t xerr, indy_handle_t arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbI32(xerr, arg0);
  }
}
NAN_METHOD(proverSearchCredentialsForProofReq) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: proverSearchCredentialsForProofReq(wallet_handle, proof_request_json, extra_query_json, cb(err, search_handle))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: proverSearchCredentialsForProofReq(wallet_handle, proof_request_json, extra_query_json, cb(err, credentials))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for proof_request_json: proverSearchCredentialsForProofReq(wallet_handle, proof_request_json, extra_query_json, cb(err, credentials))").ToLocalChecked());
  }

  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for proof_request_json: proverSearchCredentialsForProofReq(wallet_handle, proof_request_json, extra_query_json, cb(err, credentials))").ToLocalChecked());
  }

  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("proverSearchCredentialsForProofReq arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_prover_search_credentials_for_proof_req(icb->handle, arg0, arg1, arg2, proverSearchCredentialsForProofReq_cb));
  delete arg1UTF;
  delete arg2UTF;
}

void proverFetchCredentialsForProofReq_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(proverFetchCredentialsForProofReq) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: proverFetchCredentialsForProofReq(search_handle, item_referent, count, cb(err, credentials))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for search_handle: proverFetchCredentialsForProofReq(search_handle, item_referent, count, cb(err, credentials))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();

  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for search_handle: proverFetchCredentialsForProofReq(search_handle, item_referent, count, cb(err, credentials))").ToLocalChecked());
  }

  if(!info[2]->IsUint32()){
    return Nan::ThrowError(Nan::New("Expected indy_u32_t for count: proverFetchCredentialsForProofReq(search_handle, item_referent, count, cb(err, credentials))").ToLocalChecked());
  }
  indy_u32_t arg2 = info[2]->Uint32Value();
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("proverFetchCredentialsForProofReq arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_prover_fetch_credentials_for_proof_req(icb->handle, arg0, arg1, arg2, proverFetchCredentialsForProofReq_cb));
  delete arg1UTF;
}

void proverCloseCredentialsSearchForProofReq_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(proverCloseCredentialsSearchForProofReq) {
  if(info.Length() != 2){
    return Nan::ThrowError(Nan::New("Expected 2 arguments: proverCloseCredentialsSearchForProofReq(search_handle, cb(err))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_search_handle: proverCloseCredentialsSearchForProofReq(search_handle, cb(err))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  if(!info[1]->IsFunction()) {
    return Nan::ThrowError(Nan::New("proverCloseCredentialsSearchForProofReq arg 1 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[1]).ToLocalChecked());
  indyCalled(icb, indy_prover_close_credentials_search_for_proof_req(icb->handle, arg0, proverCloseCredentialsSearchForProofReq_cb));
}

void proverCreateProof_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(proverCreateProof) {
  if(info.Length() != 8){
    return Nan::ThrowError(Nan::New("Expected 8 arguments: proverCreateProof(wallet_handle, proof_req_json, requested_credentials_json, master_secret_name, schemas_json, credential_defs_json, rev_states_json, cb(err, proof))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: proverCreateProof(wallet_handle, proof_req_json, requested_credentials_json, master_secret_name, schemas_json, credential_defs_json, rev_states_json, cb(err, proof))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for proof_req_json: proverCreateProof(wallet_handle, proof_req_json, requested_credentials_json, master_secret_name, schemas_json, credential_defs_json, rev_states_json, cb(err, proof))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for requested_credentials_json: proverCreateProof(wallet_handle, proof_req_json, requested_credentials_json, master_secret_name, schemas_json, credential_defs_json, rev_states_json, cb(err, proof))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for master_secret_name: proverCreateProof(wallet_handle, proof_req_json, requested_credentials_json, master_secret_name, schemas_json, credential_defs_json, rev_states_json, cb(err, proof))").ToLocalChecked());
  }
  Nan::Utf8String* arg4UTF = nullptr;
  const char* arg4 = nullptr;
  if(info[4]->IsString()){
    arg4UTF = new Nan::Utf8String(info[4]);
    arg4 = (const char*)(**arg4UTF);
  } else if(!info[4]->IsNull() && !info[4]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for schemas_json: proverCreateProof(wallet_handle, proof_req_json, requested_credentials_json, master_secret_name, schemas_json, credential_defs_json, rev_states_json, cb(err, proof))").ToLocalChecked());
  }
  Nan::Utf8String* arg5UTF = nullptr;
  const char* arg5 = nullptr;
  if(info[5]->IsString()){
    arg5UTF = new Nan::Utf8String(info[5]);
    arg5 = (const char*)(**arg5UTF);
  } else if(!info[5]->IsNull() && !info[5]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for credential_defs_json: proverCreateProof(wallet_handle, proof_req_json, requested_credentials_json, master_secret_name, schemas_json, credential_defs_json, rev_states_json, cb(err, proof))").ToLocalChecked());
  }
  Nan::Utf8String* arg6UTF = nullptr;
  const char* arg6 = nullptr;
  if(info[6]->IsString()){
    arg6UTF = new Nan::Utf8String(info[6]);
    arg6 = (const char*)(**arg6UTF);
  } else if(!info[6]->IsNull() && !info[6]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for rev_states_json: proverCreateProof(wallet_handle, proof_req_json, requested_credentials_json, master_secret_name, schemas_json, credential_defs_json, rev_states_json, cb(err, proof))").ToLocalChecked());
  }
  if(!info[7]->IsFunction()) {
    return Nan::ThrowError(Nan::New("proverCreateProof arg 7 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[7]).ToLocalChecked());
  indyCalled(icb, indy_prover_create_proof(icb->handle, arg0, arg1, arg2, arg3, arg4, arg5, arg6, proverCreateProof_cb));
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
  delete arg4UTF;
  delete arg5UTF;
  delete arg6UTF;
}

void verifierVerifyProof_cb(indy_handle_t handle, indy_error_t xerr, indy_bool_t arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbBoolean(xerr, arg0);
  }
}
NAN_METHOD(verifierVerifyProof) {
  if(info.Length() != 7){
    return Nan::ThrowError(Nan::New("Expected 7 arguments: verifierVerifyProof(proof_request_json, proof_json, schemas_json, credential_defs_jsons, rev_reg_defs_json, rev_regs_json, cb(err, valid))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for proof_request_json: verifierVerifyProof(proof_request_json, proof_json, schemas_json, credential_defs_jsons, rev_reg_defs_json, rev_regs_json, cb(err, valid))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for proof_json: verifierVerifyProof(proof_request_json, proof_json, schemas_json, credential_defs_jsons, rev_reg_defs_json, rev_regs_json, cb(err, valid))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for schemas_json: verifierVerifyProof(proof_request_json, proof_json, schemas_json, credential_defs_jsons, rev_reg_defs_json, rev_regs_json, cb(err, valid))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for credential_defs_jsons: verifierVerifyProof(proof_request_json, proof_json, schemas_json, credential_defs_jsons, rev_reg_defs_json, rev_regs_json, cb(err, valid))").ToLocalChecked());
  }
  Nan::Utf8String* arg4UTF = nullptr;
  const char* arg4 = nullptr;
  if(info[4]->IsString()){
    arg4UTF = new Nan::Utf8String(info[4]);
    arg4 = (const char*)(**arg4UTF);
  } else if(!info[4]->IsNull() && !info[4]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for rev_reg_defs_json: verifierVerifyProof(proof_request_json, proof_json, schemas_json, credential_defs_jsons, rev_reg_defs_json, rev_regs_json, cb(err, valid))").ToLocalChecked());
  }
  Nan::Utf8String* arg5UTF = nullptr;
  const char* arg5 = nullptr;
  if(info[5]->IsString()){
    arg5UTF = new Nan::Utf8String(info[5]);
    arg5 = (const char*)(**arg5UTF);
  } else if(!info[5]->IsNull() && !info[5]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for rev_regs_json: verifierVerifyProof(proof_request_json, proof_json, schemas_json, credential_defs_jsons, rev_reg_defs_json, rev_regs_json, cb(err, valid))").ToLocalChecked());
  }
  if(!info[6]->IsFunction()) {
    return Nan::ThrowError(Nan::New("verifierVerifyProof arg 6 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[6]).ToLocalChecked());
  indyCalled(icb, indy_verifier_verify_proof(icb->handle, arg0, arg1, arg2, arg3, arg4, arg5, verifierVerifyProof_cb));
  delete arg0UTF;
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
  delete arg4UTF;
  delete arg5UTF;
}

void createRevocationState_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(createRevocationState) {
  if(info.Length() != 6){
    return Nan::ThrowError(Nan::New("Expected 6 arguments: createRevocationState(blob_storage_reader_handle, rev_reg_def_json, rev_reg_delta_json, timestamp, cred_rev_id, cb(err, revState))").ToLocalChecked());
  }
  if(!info[0]->IsInt32()){
    return Nan::ThrowError(Nan::New("Expected indy_i32_t for blob_storage_reader_handle: createRevocationState(blob_storage_reader_handle, rev_reg_def_json, rev_reg_delta_json, timestamp, cred_rev_id, cb(err, revState))").ToLocalChecked());
  }
  indy_i32_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for rev_reg_def_json: createRevocationState(blob_storage_reader_handle, rev_reg_def_json, rev_reg_delta_json, timestamp, cred_rev_id, cb(err, revState))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for rev_reg_delta_json: createRevocationState(blob_storage_reader_handle, rev_reg_def_json, rev_reg_delta_json, timestamp, cred_rev_id, cb(err, revState))").ToLocalChecked());
  }
  if(!info[3]->IsUint32()){
    return Nan::ThrowError(Nan::New("Expected Timestamp for timestamp: createRevocationState(blob_storage_reader_handle, rev_reg_def_json, rev_reg_delta_json, timestamp, cred_rev_id, cb(err, revState))").ToLocalChecked());
  }
  long long arg3 = info[3]->Uint32Value();
  Nan::Utf8String* arg4UTF = nullptr;
  const char* arg4 = nullptr;
  if(info[4]->IsString()){
    arg4UTF = new Nan::Utf8String(info[4]);
    arg4 = (const char*)(**arg4UTF);
  } else if(!info[4]->IsNull() && !info[4]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for cred_rev_id: createRevocationState(blob_storage_reader_handle, rev_reg_def_json, rev_reg_delta_json, timestamp, cred_rev_id, cb(err, revState))").ToLocalChecked());
  }
  if(!info[5]->IsFunction()) {
    return Nan::ThrowError(Nan::New("createRevocationState arg 5 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[5]).ToLocalChecked());
  indyCalled(icb, indy_create_revocation_state(icb->handle, arg0, arg1, arg2, arg3, arg4, createRevocationState_cb));
  delete arg1UTF;
  delete arg2UTF;
  delete arg4UTF;
}

void updateRevocationState_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(updateRevocationState) {
  if(info.Length() != 7){
    return Nan::ThrowError(Nan::New("Expected 7 arguments: updateRevocationState(blob_storage_reader_handle, rev_state_json, rev_reg_def_json, rev_reg_delta_json, timestamp, cred_rev_id, cb(err, updatedRevState))").ToLocalChecked());
  }
  if(!info[0]->IsInt32()){
    return Nan::ThrowError(Nan::New("Expected indy_i32_t for blob_storage_reader_handle: updateRevocationState(blob_storage_reader_handle, rev_state_json, rev_reg_def_json, rev_reg_delta_json, timestamp, cred_rev_id, cb(err, updatedRevState))").ToLocalChecked());
  }
  indy_i32_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for rev_state_json: updateRevocationState(blob_storage_reader_handle, rev_state_json, rev_reg_def_json, rev_reg_delta_json, timestamp, cred_rev_id, cb(err, updatedRevState))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for rev_reg_def_json: updateRevocationState(blob_storage_reader_handle, rev_state_json, rev_reg_def_json, rev_reg_delta_json, timestamp, cred_rev_id, cb(err, updatedRevState))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for rev_reg_delta_json: updateRevocationState(blob_storage_reader_handle, rev_state_json, rev_reg_def_json, rev_reg_delta_json, timestamp, cred_rev_id, cb(err, updatedRevState))").ToLocalChecked());
  }
  if(!info[4]->IsUint32()){
    return Nan::ThrowError(Nan::New("Expected Timestamp for timestamp: updateRevocationState(blob_storage_reader_handle, rev_state_json, rev_reg_def_json, rev_reg_delta_json, timestamp, cred_rev_id, cb(err, updatedRevState))").ToLocalChecked());
  }
  long long arg4 = info[4]->Uint32Value();
  Nan::Utf8String* arg5UTF = nullptr;
  const char* arg5 = nullptr;
  if(info[5]->IsString()){
    arg5UTF = new Nan::Utf8String(info[5]);
    arg5 = (const char*)(**arg5UTF);
  } else if(!info[5]->IsNull() && !info[5]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for cred_rev_id: updateRevocationState(blob_storage_reader_handle, rev_state_json, rev_reg_def_json, rev_reg_delta_json, timestamp, cred_rev_id, cb(err, updatedRevState))").ToLocalChecked());
  }
  if(!info[6]->IsFunction()) {
    return Nan::ThrowError(Nan::New("updateRevocationState arg 6 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[6]).ToLocalChecked());
  indyCalled(icb, indy_update_revocation_state(icb->handle, arg0, arg1, arg2, arg3, arg4, arg5, updateRevocationState_cb));
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
  delete arg5UTF;
}

void openBlobStorageReader_cb(indy_handle_t handle, indy_error_t xerr, indy_i32_t arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbI32(xerr, arg0);
  }
}
NAN_METHOD(openBlobStorageReader) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: openBlobStorageReader(type_, config_json, cb(err, handle))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for type_: openBlobStorageReader(type_, config_json, cb(err, handle))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for config_json: openBlobStorageReader(type_, config_json, cb(err, handle))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("openBlobStorageReader arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_open_blob_storage_reader(icb->handle, arg0, arg1, openBlobStorageReader_cb));
  delete arg0UTF;
  delete arg1UTF;
}

void openBlobStorageWriter_cb(indy_handle_t handle, indy_error_t xerr, indy_i32_t arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbI32(xerr, arg0);
  }
}
NAN_METHOD(openBlobStorageWriter) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: openBlobStorageWriter(type_, config_json, cb(err, handle))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for type_: openBlobStorageWriter(type_, config_json, cb(err, handle))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for config_json: openBlobStorageWriter(type_, config_json, cb(err, handle))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("openBlobStorageWriter arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_open_blob_storage_writer(icb->handle, arg0, arg1, openBlobStorageWriter_cb));
  delete arg0UTF;
  delete arg1UTF;
}

void createKey_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(createKey) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: createKey(wallet_handle, key_json, cb(err, vk))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: createKey(wallet_handle, key_json, cb(err, vk))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for key_json: createKey(wallet_handle, key_json, cb(err, vk))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("createKey arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_create_key(icb->handle, arg0, arg1, createKey_cb));
  delete arg1UTF;
}

void setKeyMetadata_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(setKeyMetadata) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: setKeyMetadata(wallet_handle, verkey, metadata, cb(err))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: setKeyMetadata(wallet_handle, verkey, metadata, cb(err))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for verkey: setKeyMetadata(wallet_handle, verkey, metadata, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for metadata: setKeyMetadata(wallet_handle, verkey, metadata, cb(err))").ToLocalChecked());
  }
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("setKeyMetadata arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_set_key_metadata(icb->handle, arg0, arg1, arg2, setKeyMetadata_cb));
  delete arg1UTF;
  delete arg2UTF;
}

void getKeyMetadata_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(getKeyMetadata) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: getKeyMetadata(wallet_handle, verkey, cb(err, metadata))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: getKeyMetadata(wallet_handle, verkey, cb(err, metadata))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for verkey: getKeyMetadata(wallet_handle, verkey, cb(err, metadata))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("getKeyMetadata arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_get_key_metadata(icb->handle, arg0, arg1, getKeyMetadata_cb));
  delete arg1UTF;
}

void cryptoSign_cb(indy_handle_t handle, indy_error_t xerr, const indy_u8_t* arg0data, indy_u32_t arg0len) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbBuffer(xerr, arg0data, arg0len);
  }
}
NAN_METHOD(cryptoSign) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: cryptoSign(wallet_handle, signer_vk, message_raw, cb(err, signatureRaw))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: cryptoSign(wallet_handle, signer_vk, message_raw, cb(err, signatureRaw))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for signer_vk: cryptoSign(wallet_handle, signer_vk, message_raw, cb(err, signatureRaw))").ToLocalChecked());
  }
  if(!info[2]->IsUint8Array()){
    return Nan::ThrowError(Nan::New("Expected Buffer for message_raw: cryptoSign(wallet_handle, signer_vk, message_raw, cb(err, signatureRaw))").ToLocalChecked());
  }
  const indy_u8_t* arg2data = (indy_u8_t*)node::Buffer::Data(info[2]->ToObject());
  indy_u32_t arg2len = node::Buffer::Length(info[2]);
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("cryptoSign arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_crypto_sign(icb->handle, arg0, arg1, arg2data, arg2len, cryptoSign_cb));
  delete arg1UTF;
}

void cryptoVerify_cb(indy_handle_t handle, indy_error_t xerr, indy_bool_t arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbBoolean(xerr, arg0);
  }
}
NAN_METHOD(cryptoVerify) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: cryptoVerify(signer_vk, message_raw, signature_raw, cb(err, valid))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for signer_vk: cryptoVerify(signer_vk, message_raw, signature_raw, cb(err, valid))").ToLocalChecked());
  }
  if(!info[1]->IsUint8Array()){
    return Nan::ThrowError(Nan::New("Expected Buffer for message_raw: cryptoVerify(signer_vk, message_raw, signature_raw, cb(err, valid))").ToLocalChecked());
  }
  const indy_u8_t* arg1data = (indy_u8_t*)node::Buffer::Data(info[1]->ToObject());
  indy_u32_t arg1len = node::Buffer::Length(info[1]);
  if(!info[2]->IsUint8Array()){
    return Nan::ThrowError(Nan::New("Expected Buffer for signature_raw: cryptoVerify(signer_vk, message_raw, signature_raw, cb(err, valid))").ToLocalChecked());
  }
  const indy_u8_t* arg2data = (indy_u8_t*)node::Buffer::Data(info[2]->ToObject());
  indy_u32_t arg2len = node::Buffer::Length(info[2]);
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("cryptoVerify arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_crypto_verify(icb->handle, arg0, arg1data, arg1len, arg2data, arg2len, cryptoVerify_cb));
  delete arg0UTF;
}

void cryptoAuthCrypt_cb(indy_handle_t handle, indy_error_t xerr, const indy_u8_t* arg0data, indy_u32_t arg0len) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbBuffer(xerr, arg0data, arg0len);
  }
}
NAN_METHOD(cryptoAuthCrypt) {
  if(info.Length() != 5){
    return Nan::ThrowError(Nan::New("Expected 5 arguments: cryptoAuthCrypt(wallet_handle, sender_vk, recipient_vk, message_raw, cb(err, encryptedMsgRaw))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: cryptoAuthCrypt(wallet_handle, sender_vk, recipient_vk, message_raw, cb(err, encryptedMsgRaw))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for sender_vk: cryptoAuthCrypt(wallet_handle, sender_vk, recipient_vk, message_raw, cb(err, encryptedMsgRaw))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for recipient_vk: cryptoAuthCrypt(wallet_handle, sender_vk, recipient_vk, message_raw, cb(err, encryptedMsgRaw))").ToLocalChecked());
  }
  if(!info[3]->IsUint8Array()){
    return Nan::ThrowError(Nan::New("Expected Buffer for message_raw: cryptoAuthCrypt(wallet_handle, sender_vk, recipient_vk, message_raw, cb(err, encryptedMsgRaw))").ToLocalChecked());
  }
  const indy_u8_t* arg3data = (indy_u8_t*)node::Buffer::Data(info[3]->ToObject());
  indy_u32_t arg3len = node::Buffer::Length(info[3]);
  if(!info[4]->IsFunction()) {
    return Nan::ThrowError(Nan::New("cryptoAuthCrypt arg 4 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[4]).ToLocalChecked());
  indyCalled(icb, indy_crypto_auth_crypt(icb->handle, arg0, arg1, arg2, arg3data, arg3len, cryptoAuthCrypt_cb));
  delete arg1UTF;
  delete arg2UTF;
}

void cryptoAuthDecrypt_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0, const indy_u8_t* arg1data, indy_u32_t arg1len) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringBuffer(xerr, arg0, arg1data, arg1len);
  }
}
NAN_METHOD(cryptoAuthDecrypt) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: cryptoAuthDecrypt(wallet_handle, recipient_vk, encrypted_msg_raw, cb(err, [ senderVk, decryptedMsgRaw ]))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: cryptoAuthDecrypt(wallet_handle, recipient_vk, encrypted_msg_raw, cb(err, [ senderVk, decryptedMsgRaw ]))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for recipient_vk: cryptoAuthDecrypt(wallet_handle, recipient_vk, encrypted_msg_raw, cb(err, [ senderVk, decryptedMsgRaw ]))").ToLocalChecked());
  }
  if(!info[2]->IsUint8Array()){
    return Nan::ThrowError(Nan::New("Expected Buffer for encrypted_msg_raw: cryptoAuthDecrypt(wallet_handle, recipient_vk, encrypted_msg_raw, cb(err, [ senderVk, decryptedMsgRaw ]))").ToLocalChecked());
  }
  const indy_u8_t* arg2data = (indy_u8_t*)node::Buffer::Data(info[2]->ToObject());
  indy_u32_t arg2len = node::Buffer::Length(info[2]);
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("cryptoAuthDecrypt arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_crypto_auth_decrypt(icb->handle, arg0, arg1, arg2data, arg2len, cryptoAuthDecrypt_cb));
  delete arg1UTF;
}

void cryptoAnonCrypt_cb(indy_handle_t handle, indy_error_t xerr, const indy_u8_t* arg0data, indy_u32_t arg0len) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbBuffer(xerr, arg0data, arg0len);
  }
}
NAN_METHOD(cryptoAnonCrypt) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: cryptoAnonCrypt(recipient_vk, message_raw, cb(err, encryptedMsgRaw))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for recipient_vk: cryptoAnonCrypt(recipient_vk, message_raw, cb(err, encryptedMsgRaw))").ToLocalChecked());
  }
  if(!info[1]->IsUint8Array()){
    return Nan::ThrowError(Nan::New("Expected Buffer for message_raw: cryptoAnonCrypt(recipient_vk, message_raw, cb(err, encryptedMsgRaw))").ToLocalChecked());
  }
  const indy_u8_t* arg1data = (indy_u8_t*)node::Buffer::Data(info[1]->ToObject());
  indy_u32_t arg1len = node::Buffer::Length(info[1]);
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("cryptoAnonCrypt arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_crypto_anon_crypt(icb->handle, arg0, arg1data, arg1len, cryptoAnonCrypt_cb));
  delete arg0UTF;
}

void cryptoAnonDecrypt_cb(indy_handle_t handle, indy_error_t xerr, const indy_u8_t* arg0data, indy_u32_t arg0len) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbBuffer(xerr, arg0data, arg0len);
  }
}
NAN_METHOD(cryptoAnonDecrypt) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: cryptoAnonDecrypt(wallet_handle, recipient_vk, encrypted_msg, cb(err, decryptedMsgRaw))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: cryptoAnonDecrypt(wallet_handle, recipient_vk, encrypted_msg, cb(err, decryptedMsgRaw))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for recipient_vk: cryptoAnonDecrypt(wallet_handle, recipient_vk, encrypted_msg, cb(err, decryptedMsgRaw))").ToLocalChecked());
  }
  if(!info[2]->IsUint8Array()){
    return Nan::ThrowError(Nan::New("Expected Buffer for encrypted_msg: cryptoAnonDecrypt(wallet_handle, recipient_vk, encrypted_msg, cb(err, decryptedMsgRaw))").ToLocalChecked());
  }
  const indy_u8_t* arg2data = (indy_u8_t*)node::Buffer::Data(info[2]->ToObject());
  indy_u32_t arg2len = node::Buffer::Length(info[2]);
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("cryptoAnonDecrypt arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_crypto_anon_decrypt(icb->handle, arg0, arg1, arg2data, arg2len, cryptoAnonDecrypt_cb));
  delete arg1UTF;
}

void createAndStoreMyDid_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0, const char *const arg1) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringString(xerr, arg0, arg1);
  }
}
NAN_METHOD(createAndStoreMyDid) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: createAndStoreMyDid(wallet_handle, did_json, cb(err, [ did, verkey ]))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: createAndStoreMyDid(wallet_handle, did_json, cb(err, [ did, verkey ]))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for did_json: createAndStoreMyDid(wallet_handle, did_json, cb(err, [ did, verkey ]))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("createAndStoreMyDid arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_create_and_store_my_did(icb->handle, arg0, arg1, createAndStoreMyDid_cb));
  delete arg1UTF;
}

void replaceKeysStart_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(replaceKeysStart) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: replaceKeysStart(wallet_handle, did, identity_json, cb(err, verkey))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: replaceKeysStart(wallet_handle, did, identity_json, cb(err, verkey))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for did: replaceKeysStart(wallet_handle, did, identity_json, cb(err, verkey))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for identity_json: replaceKeysStart(wallet_handle, did, identity_json, cb(err, verkey))").ToLocalChecked());
  }
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("replaceKeysStart arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_replace_keys_start(icb->handle, arg0, arg1, arg2, replaceKeysStart_cb));
  delete arg1UTF;
  delete arg2UTF;
}

void replaceKeysApply_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(replaceKeysApply) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: replaceKeysApply(wallet_handle, did, cb(err))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: replaceKeysApply(wallet_handle, did, cb(err))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for did: replaceKeysApply(wallet_handle, did, cb(err))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("replaceKeysApply arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_replace_keys_apply(icb->handle, arg0, arg1, replaceKeysApply_cb));
  delete arg1UTF;
}

void storeTheirDid_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(storeTheirDid) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: storeTheirDid(wallet_handle, identity_json, cb(err))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: storeTheirDid(wallet_handle, identity_json, cb(err))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for identity_json: storeTheirDid(wallet_handle, identity_json, cb(err))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("storeTheirDid arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_store_their_did(icb->handle, arg0, arg1, storeTheirDid_cb));
  delete arg1UTF;
}

void keyForDid_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(keyForDid) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: keyForDid(pool_handle, wallet_handle, did, cb(err, key))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for pool_handle: keyForDid(pool_handle, wallet_handle, did, cb(err, key))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  if(!info[1]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: keyForDid(pool_handle, wallet_handle, did, cb(err, key))").ToLocalChecked());
  }
  indy_handle_t arg1 = info[1]->Int32Value();
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for did: keyForDid(pool_handle, wallet_handle, did, cb(err, key))").ToLocalChecked());
  }
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("keyForDid arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_key_for_did(icb->handle, arg0, arg1, arg2, keyForDid_cb));
  delete arg2UTF;
}

void keyForLocalDid_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(keyForLocalDid) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: keyForLocalDid(wallet_handle, did, cb(err, key))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: keyForLocalDid(wallet_handle, did, cb(err, key))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for did: keyForLocalDid(wallet_handle, did, cb(err, key))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("keyForLocalDid arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_key_for_local_did(icb->handle, arg0, arg1, keyForLocalDid_cb));
  delete arg1UTF;
}

void setEndpointForDid_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(setEndpointForDid) {
  if(info.Length() != 5){
    return Nan::ThrowError(Nan::New("Expected 5 arguments: setEndpointForDid(wallet_handle, did, address, transport_key, cb(err))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: setEndpointForDid(wallet_handle, did, address, transport_key, cb(err))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for did: setEndpointForDid(wallet_handle, did, address, transport_key, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for address: setEndpointForDid(wallet_handle, did, address, transport_key, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for transport_key: setEndpointForDid(wallet_handle, did, address, transport_key, cb(err))").ToLocalChecked());
  }
  if(!info[4]->IsFunction()) {
    return Nan::ThrowError(Nan::New("setEndpointForDid arg 4 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[4]).ToLocalChecked());
  indyCalled(icb, indy_set_endpoint_for_did(icb->handle, arg0, arg1, arg2, arg3, setEndpointForDid_cb));
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
}

void getEndpointForDid_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0, const char *const arg1) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringString(xerr, arg0, arg1);
  }
}
NAN_METHOD(getEndpointForDid) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: getEndpointForDid(wallet_handle, pool_handle, did, cb(err, [ address, transportVk ]))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: getEndpointForDid(wallet_handle, pool_handle, did, cb(err, [ address, transportVk ]))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  if(!info[1]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for pool_handle: getEndpointForDid(wallet_handle, pool_handle, did, cb(err, [ address, transportVk ]))").ToLocalChecked());
  }
  indy_handle_t arg1 = info[1]->Int32Value();
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for did: getEndpointForDid(wallet_handle, pool_handle, did, cb(err, [ address, transportVk ]))").ToLocalChecked());
  }
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("getEndpointForDid arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_get_endpoint_for_did(icb->handle, arg0, arg1, arg2, getEndpointForDid_cb));
  delete arg2UTF;
}

void setDidMetadata_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(setDidMetadata) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: setDidMetadata(wallet_handle, did, metadata, cb(err))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: setDidMetadata(wallet_handle, did, metadata, cb(err))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for did: setDidMetadata(wallet_handle, did, metadata, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for metadata: setDidMetadata(wallet_handle, did, metadata, cb(err))").ToLocalChecked());
  }
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("setDidMetadata arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_set_did_metadata(icb->handle, arg0, arg1, arg2, setDidMetadata_cb));
  delete arg1UTF;
  delete arg2UTF;
}

void getDidMetadata_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(getDidMetadata) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: getDidMetadata(wallet_handle, did, cb(err, metadata))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: getDidMetadata(wallet_handle, did, cb(err, metadata))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for did: getDidMetadata(wallet_handle, did, cb(err, metadata))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("getDidMetadata arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_get_did_metadata(icb->handle, arg0, arg1, getDidMetadata_cb));
  delete arg1UTF;
}

void getMyDidWithMeta_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(getMyDidWithMeta) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: getMyDidWithMeta(wallet_handle, my_did, cb(err, didWithMeta))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: getMyDidWithMeta(wallet_handle, my_did, cb(err, didWithMeta))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for my_did: getMyDidWithMeta(wallet_handle, my_did, cb(err, didWithMeta))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("getMyDidWithMeta arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_get_my_did_with_meta(icb->handle, arg0, arg1, getMyDidWithMeta_cb));
  delete arg1UTF;
}

void listMyDidsWithMeta_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(listMyDidsWithMeta) {
  if(info.Length() != 2){
    return Nan::ThrowError(Nan::New("Expected 2 arguments: listMyDidsWithMeta(wallet_handle, cb(err, dids))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: listMyDidsWithMeta(wallet_handle, cb(err, dids))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  if(!info[1]->IsFunction()) {
    return Nan::ThrowError(Nan::New("listMyDidsWithMeta arg 1 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[1]).ToLocalChecked());
  indyCalled(icb, indy_list_my_dids_with_meta(icb->handle, arg0, listMyDidsWithMeta_cb));
}

void abbreviateVerkey_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(abbreviateVerkey) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: abbreviateVerkey(did, full_verkey, cb(err, verkey))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for did: abbreviateVerkey(did, full_verkey, cb(err, verkey))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for full_verkey: abbreviateVerkey(did, full_verkey, cb(err, verkey))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("abbreviateVerkey arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_abbreviate_verkey(icb->handle, arg0, arg1, abbreviateVerkey_cb));
  delete arg0UTF;
  delete arg1UTF;
}

void signAndSubmitRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(signAndSubmitRequest) {
  if(info.Length() != 5){
    return Nan::ThrowError(Nan::New("Expected 5 arguments: signAndSubmitRequest(pool_handle, wallet_handle, submitter_did, request_json, cb(err, requestResult))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for pool_handle: signAndSubmitRequest(pool_handle, wallet_handle, submitter_did, request_json, cb(err, requestResult))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  if(!info[1]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: signAndSubmitRequest(pool_handle, wallet_handle, submitter_did, request_json, cb(err, requestResult))").ToLocalChecked());
  }
  indy_handle_t arg1 = info[1]->Int32Value();
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: signAndSubmitRequest(pool_handle, wallet_handle, submitter_did, request_json, cb(err, requestResult))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for request_json: signAndSubmitRequest(pool_handle, wallet_handle, submitter_did, request_json, cb(err, requestResult))").ToLocalChecked());
  }
  if(!info[4]->IsFunction()) {
    return Nan::ThrowError(Nan::New("signAndSubmitRequest arg 4 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[4]).ToLocalChecked());
  indyCalled(icb, indy_sign_and_submit_request(icb->handle, arg0, arg1, arg2, arg3, signAndSubmitRequest_cb));
  delete arg2UTF;
  delete arg3UTF;
}

void submitRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(submitRequest) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: submitRequest(pool_handle, request_json, cb(err, requestResult))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for pool_handle: submitRequest(pool_handle, request_json, cb(err, requestResult))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for request_json: submitRequest(pool_handle, request_json, cb(err, requestResult))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("submitRequest arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_submit_request(icb->handle, arg0, arg1, submitRequest_cb));
  delete arg1UTF;
}

void signRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(signRequest) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: signRequest(wallet_handle, submitter_did, request_json, cb(err, signedRequest))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: signRequest(wallet_handle, submitter_did, request_json, cb(err, signedRequest))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: signRequest(wallet_handle, submitter_did, request_json, cb(err, signedRequest))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for request_json: signRequest(wallet_handle, submitter_did, request_json, cb(err, signedRequest))").ToLocalChecked());
  }
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("signRequest arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_sign_request(icb->handle, arg0, arg1, arg2, signRequest_cb));
  delete arg1UTF;
  delete arg2UTF;
}

void multiSignRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(multiSignRequest) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: multiSignRequest(wallet_handle, submitter_did, request_json, cb(err, signedRequest))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: multiSignRequest(wallet_handle, submitter_did, request_json, cb(err, signedRequest))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: multiSignRequest(wallet_handle, submitter_did, request_json, cb(err, signedRequest))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for request_json: multiSignRequest(wallet_handle, submitter_did, request_json, cb(err, signedRequest))").ToLocalChecked());
  }
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("multiSignRequest arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_multi_sign_request(icb->handle, arg0, arg1, arg2, multiSignRequest_cb));
  delete arg1UTF;
  delete arg2UTF;
}

void buildGetDdoRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildGetDdoRequest) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: buildGetDdoRequest(submitter_did, target_did, cb(err, requestResult))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: buildGetDdoRequest(submitter_did, target_did, cb(err, requestResult))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for target_did: buildGetDdoRequest(submitter_did, target_did, cb(err, requestResult))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("buildGetDdoRequest arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_build_get_ddo_request(icb->handle, arg0, arg1, buildGetDdoRequest_cb));
  delete arg0UTF;
  delete arg1UTF;
}

void buildNymRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildNymRequest) {
  if(info.Length() != 6){
    return Nan::ThrowError(Nan::New("Expected 6 arguments: buildNymRequest(submitter_did, target_did, verkey, alias, role, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: buildNymRequest(submitter_did, target_did, verkey, alias, role, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for target_did: buildNymRequest(submitter_did, target_did, verkey, alias, role, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for verkey: buildNymRequest(submitter_did, target_did, verkey, alias, role, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for alias: buildNymRequest(submitter_did, target_did, verkey, alias, role, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg4UTF = nullptr;
  const char* arg4 = nullptr;
  if(info[4]->IsString()){
    arg4UTF = new Nan::Utf8String(info[4]);
    arg4 = (const char*)(**arg4UTF);
  } else if(!info[4]->IsNull() && !info[4]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for role: buildNymRequest(submitter_did, target_did, verkey, alias, role, cb(err, request))").ToLocalChecked());
  }
  if(!info[5]->IsFunction()) {
    return Nan::ThrowError(Nan::New("buildNymRequest arg 5 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[5]).ToLocalChecked());
  indyCalled(icb, indy_build_nym_request(icb->handle, arg0, arg1, arg2, arg3, arg4, buildNymRequest_cb));
  delete arg0UTF;
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
  delete arg4UTF;
}

void buildAttribRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildAttribRequest) {
  if(info.Length() != 6){
    return Nan::ThrowError(Nan::New("Expected 6 arguments: buildAttribRequest(submitter_did, target_did, hash, raw, enc, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: buildAttribRequest(submitter_did, target_did, hash, raw, enc, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for target_did: buildAttribRequest(submitter_did, target_did, hash, raw, enc, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for hash: buildAttribRequest(submitter_did, target_did, hash, raw, enc, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for raw: buildAttribRequest(submitter_did, target_did, hash, raw, enc, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg4UTF = nullptr;
  const char* arg4 = nullptr;
  if(info[4]->IsString()){
    arg4UTF = new Nan::Utf8String(info[4]);
    arg4 = (const char*)(**arg4UTF);
  } else if(!info[4]->IsNull() && !info[4]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for enc: buildAttribRequest(submitter_did, target_did, hash, raw, enc, cb(err, request))").ToLocalChecked());
  }
  if(!info[5]->IsFunction()) {
    return Nan::ThrowError(Nan::New("buildAttribRequest arg 5 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[5]).ToLocalChecked());
  indyCalled(icb, indy_build_attrib_request(icb->handle, arg0, arg1, arg2, arg3, arg4, buildAttribRequest_cb));
  delete arg0UTF;
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
  delete arg4UTF;
}

void buildGetAttribRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildGetAttribRequest) {
  if(info.Length() != 6){
    return Nan::ThrowError(Nan::New("Expected 6 arguments: buildGetAttribRequest(submitter_did, target_did, hash, raw, enc, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: buildGetAttribRequest(submitter_did, target_did, hash, raw, enc, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for target_did: buildGetAttribRequest(submitter_did, target_did, hash, raw, enc, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for hash: buildGetAttribRequest(submitter_did, target_did, hash, raw, enc, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for raw: buildGetAttribRequest(submitter_did, target_did, hash, raw, enc, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg4UTF = nullptr;
  const char* arg4 = nullptr;
  if(info[4]->IsString()){
    arg4UTF = new Nan::Utf8String(info[4]);
    arg4 = (const char*)(**arg4UTF);
  } else if(!info[4]->IsNull() && !info[4]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for enc: buildGetAttribRequest(submitter_did, target_did, hash, raw, enc, cb(err, request))").ToLocalChecked());
  }
  if(!info[5]->IsFunction()) {
    return Nan::ThrowError(Nan::New("buildGetAttribRequest arg 5 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[5]).ToLocalChecked());
  indyCalled(icb, indy_build_get_attrib_request(icb->handle, arg0, arg1, arg2, arg3, arg4, buildGetAttribRequest_cb));
  delete arg0UTF;
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
  delete arg4UTF;
}

void buildGetNymRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildGetNymRequest) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: buildGetNymRequest(submitter_did, target_did, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: buildGetNymRequest(submitter_did, target_did, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for target_did: buildGetNymRequest(submitter_did, target_did, cb(err, request))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("buildGetNymRequest arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_build_get_nym_request(icb->handle, arg0, arg1, buildGetNymRequest_cb));
  delete arg0UTF;
  delete arg1UTF;
}

void buildSchemaRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildSchemaRequest) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: buildSchemaRequest(submitter_did, data, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: buildSchemaRequest(submitter_did, data, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for data: buildSchemaRequest(submitter_did, data, cb(err, request))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("buildSchemaRequest arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_build_schema_request(icb->handle, arg0, arg1, buildSchemaRequest_cb));
  delete arg0UTF;
  delete arg1UTF;
}

void buildGetSchemaRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildGetSchemaRequest) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: buildGetSchemaRequest(submitter_did, id, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: buildGetSchemaRequest(submitter_did, id, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for id: buildGetSchemaRequest(submitter_did, id, cb(err, request))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("buildGetSchemaRequest arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_build_get_schema_request(icb->handle, arg0, arg1, buildGetSchemaRequest_cb));
  delete arg0UTF;
  delete arg1UTF;
}

void parseGetSchemaResponse_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0, const char* arg1) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringString(xerr, arg0, arg1);
  }
}
NAN_METHOD(parseGetSchemaResponse) {
  if(info.Length() != 2){
    return Nan::ThrowError(Nan::New("Expected 2 arguments: parseGetSchemaResponse(get_schema_response, cb(err, [ schemaId, schema ]))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for get_schema_response: parseGetSchemaResponse(get_schema_response, cb(err, [ schemaId, schema ]))").ToLocalChecked());
  }
  if(!info[1]->IsFunction()) {
    return Nan::ThrowError(Nan::New("parseGetSchemaResponse arg 1 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[1]).ToLocalChecked());
  indyCalled(icb, indy_parse_get_schema_response(icb->handle, arg0, parseGetSchemaResponse_cb));
  delete arg0UTF;
}

void buildCredDefRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildCredDefRequest) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: buildCredDefRequest(submitter_did, data, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: buildCredDefRequest(submitter_did, data, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for data: buildCredDefRequest(submitter_did, data, cb(err, request))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("buildCredDefRequest arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_build_cred_def_request(icb->handle, arg0, arg1, buildCredDefRequest_cb));
  delete arg0UTF;
  delete arg1UTF;
}

void buildGetCredDefRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildGetCredDefRequest) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: buildGetCredDefRequest(submitter_did, id, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: buildGetCredDefRequest(submitter_did, id, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for id: buildGetCredDefRequest(submitter_did, id, cb(err, request))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("buildGetCredDefRequest arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_build_get_cred_def_request(icb->handle, arg0, arg1, buildGetCredDefRequest_cb));
  delete arg0UTF;
  delete arg1UTF;
}

void parseGetCredDefResponse_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0, const char* arg1) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringString(xerr, arg0, arg1);
  }
}
NAN_METHOD(parseGetCredDefResponse) {
  if(info.Length() != 2){
    return Nan::ThrowError(Nan::New("Expected 2 arguments: parseGetCredDefResponse(get_cred_def_response, cb(err, [ credDefId, credDef ]))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for get_cred_def_response: parseGetCredDefResponse(get_cred_def_response, cb(err, [ credDefId, credDef ]))").ToLocalChecked());
  }
  if(!info[1]->IsFunction()) {
    return Nan::ThrowError(Nan::New("parseGetCredDefResponse arg 1 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[1]).ToLocalChecked());
  indyCalled(icb, indy_parse_get_cred_def_response(icb->handle, arg0, parseGetCredDefResponse_cb));
  delete arg0UTF;
}

void buildNodeRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildNodeRequest) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: buildNodeRequest(submitter_did, target_did, data, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: buildNodeRequest(submitter_did, target_did, data, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for target_did: buildNodeRequest(submitter_did, target_did, data, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for data: buildNodeRequest(submitter_did, target_did, data, cb(err, request))").ToLocalChecked());
  }
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("buildNodeRequest arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_build_node_request(icb->handle, arg0, arg1, arg2, buildNodeRequest_cb));
  delete arg0UTF;
  delete arg1UTF;
  delete arg2UTF;
}

void buildGetValidatorInfoRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildGetValidatorInfoRequest) {
  if(info.Length() != 2){
    return Nan::ThrowError(Nan::New("Expected 2 arguments: buildGetValidatorInfoRequest(submitter_did, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: buildGetValidatorInfoRequest(submitter_did, cb(err, request))").ToLocalChecked());
  }
  if(!info[1]->IsFunction()) {
    return Nan::ThrowError(Nan::New("buildGetValidatorInfoRequest arg 1 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[1]).ToLocalChecked());
  indyCalled(icb, indy_build_get_validator_info_request(icb->handle, arg0, buildGetValidatorInfoRequest_cb));
  delete arg0UTF;
}

void buildGetTxnRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildGetTxnRequest) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: buildGetTxnRequest(submitter_did, ledger_type, data, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: buildGetTxnRequest(submitter_did, ledger_type, data, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for ledger_type: buildGetTxnRequest(submitter_did, ledger_type, data, cb(err, request))").ToLocalChecked());
  }
  if(!info[2]->IsInt32()){
    return Nan::ThrowError(Nan::New("Expected indy_i32_t for data: buildGetTxnRequest(submitter_did, ledger_type, data, cb(err, request))").ToLocalChecked());
  }
  indy_i32_t arg2 = info[2]->Int32Value();
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("buildGetTxnRequest arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_build_get_txn_request(icb->handle, arg0, arg1, arg2, buildGetTxnRequest_cb));
  delete arg0UTF;
  delete arg1UTF;
}

void buildPoolConfigRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildPoolConfigRequest) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: buildPoolConfigRequest(submitter_did, writes, force, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: buildPoolConfigRequest(submitter_did, writes, force, cb(err, request))").ToLocalChecked());
  }
  if(!info[1]->IsBoolean()){
    return Nan::ThrowError(Nan::New("Expected Boolean for writes: buildPoolConfigRequest(submitter_did, writes, force, cb(err, request))").ToLocalChecked());
  }
  indy_bool_t arg1 = info[1]->IsTrue();
  if(!info[2]->IsBoolean()){
    return Nan::ThrowError(Nan::New("Expected Boolean for force: buildPoolConfigRequest(submitter_did, writes, force, cb(err, request))").ToLocalChecked());
  }
  indy_bool_t arg2 = info[2]->IsTrue();
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("buildPoolConfigRequest arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_build_pool_config_request(icb->handle, arg0, arg1, arg2, buildPoolConfigRequest_cb));
  delete arg0UTF;
}

void buildPoolRestartRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildPoolRestartRequest) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: buildPoolRestartRequest(submitter_did, action, datetime, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: buildPoolRestartRequest(submitter_did, action, datetime, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for action: buildPoolRestartRequest(submitter_did, action, datetime, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for datetime: buildPoolRestartRequest(submitter_did, action, datetime, cb(err, request))").ToLocalChecked());
  }
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("buildPoolRestartRequest arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_build_pool_restart_request(icb->handle, arg0, arg1, arg2, buildPoolRestartRequest_cb));
  delete arg0UTF;
  delete arg1UTF;
  delete arg2UTF;
}

void buildPoolUpgradeRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildPoolUpgradeRequest) {
  if(info.Length() != 11){
    return Nan::ThrowError(Nan::New("Expected 11 arguments: buildPoolUpgradeRequest(submitter_did, name, version, action, sha256, timeout, schedule, justification, reinstall, force, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: buildPoolUpgradeRequest(submitter_did, name, version, action, sha256, timeout, schedule, justification, reinstall, force, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for name: buildPoolUpgradeRequest(submitter_did, name, version, action, sha256, timeout, schedule, justification, reinstall, force, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for version: buildPoolUpgradeRequest(submitter_did, name, version, action, sha256, timeout, schedule, justification, reinstall, force, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for action: buildPoolUpgradeRequest(submitter_did, name, version, action, sha256, timeout, schedule, justification, reinstall, force, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg4UTF = nullptr;
  const char* arg4 = nullptr;
  if(info[4]->IsString()){
    arg4UTF = new Nan::Utf8String(info[4]);
    arg4 = (const char*)(**arg4UTF);
  } else if(!info[4]->IsNull() && !info[4]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for sha256: buildPoolUpgradeRequest(submitter_did, name, version, action, sha256, timeout, schedule, justification, reinstall, force, cb(err, request))").ToLocalChecked());
  }
  if(!info[5]->IsInt32()){
    return Nan::ThrowError(Nan::New("Expected indy_i32_t for timeout: buildPoolUpgradeRequest(submitter_did, name, version, action, sha256, timeout, schedule, justification, reinstall, force, cb(err, request))").ToLocalChecked());
  }
  indy_i32_t arg5 = info[5]->Int32Value();
  Nan::Utf8String* arg6UTF = nullptr;
  const char* arg6 = nullptr;
  if(info[6]->IsString()){
    arg6UTF = new Nan::Utf8String(info[6]);
    arg6 = (const char*)(**arg6UTF);
  } else if(!info[6]->IsNull() && !info[6]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for schedule: buildPoolUpgradeRequest(submitter_did, name, version, action, sha256, timeout, schedule, justification, reinstall, force, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg7UTF = nullptr;
  const char* arg7 = nullptr;
  if(info[7]->IsString()){
    arg7UTF = new Nan::Utf8String(info[7]);
    arg7 = (const char*)(**arg7UTF);
  } else if(!info[7]->IsNull() && !info[7]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for justification: buildPoolUpgradeRequest(submitter_did, name, version, action, sha256, timeout, schedule, justification, reinstall, force, cb(err, request))").ToLocalChecked());
  }
  if(!info[8]->IsBoolean()){
    return Nan::ThrowError(Nan::New("Expected Boolean for reinstall: buildPoolUpgradeRequest(submitter_did, name, version, action, sha256, timeout, schedule, justification, reinstall, force, cb(err, request))").ToLocalChecked());
  }
  indy_bool_t arg8 = info[8]->IsTrue();
  if(!info[9]->IsBoolean()){
    return Nan::ThrowError(Nan::New("Expected Boolean for force: buildPoolUpgradeRequest(submitter_did, name, version, action, sha256, timeout, schedule, justification, reinstall, force, cb(err, request))").ToLocalChecked());
  }
  indy_bool_t arg9 = info[9]->IsTrue();
  if(!info[10]->IsFunction()) {
    return Nan::ThrowError(Nan::New("buildPoolUpgradeRequest arg 10 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[10]).ToLocalChecked());
  indyCalled(icb, indy_build_pool_upgrade_request(icb->handle, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, buildPoolUpgradeRequest_cb));
  delete arg0UTF;
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
  delete arg4UTF;
  delete arg6UTF;
  delete arg7UTF;
}

void buildRevocRegDefRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildRevocRegDefRequest) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: buildRevocRegDefRequest(submitter_did, data, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: buildRevocRegDefRequest(submitter_did, data, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for data: buildRevocRegDefRequest(submitter_did, data, cb(err, request))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("buildRevocRegDefRequest arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_build_revoc_reg_def_request(icb->handle, arg0, arg1, buildRevocRegDefRequest_cb));
  delete arg0UTF;
  delete arg1UTF;
}

void buildGetRevocRegDefRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildGetRevocRegDefRequest) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: buildGetRevocRegDefRequest(submitter_did, id, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: buildGetRevocRegDefRequest(submitter_did, id, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for id: buildGetRevocRegDefRequest(submitter_did, id, cb(err, request))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("buildGetRevocRegDefRequest arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_build_get_revoc_reg_def_request(icb->handle, arg0, arg1, buildGetRevocRegDefRequest_cb));
  delete arg0UTF;
  delete arg1UTF;
}

void parseGetRevocRegDefResponse_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0, const char* arg1) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringString(xerr, arg0, arg1);
  }
}
NAN_METHOD(parseGetRevocRegDefResponse) {
  if(info.Length() != 2){
    return Nan::ThrowError(Nan::New("Expected 2 arguments: parseGetRevocRegDefResponse(get_revoc_ref_def_response, cb(err, [ revocRegDefId, revocRegDef ]))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for get_revoc_ref_def_response: parseGetRevocRegDefResponse(get_revoc_ref_def_response, cb(err, [ revocRegDefId, revocRegDef ]))").ToLocalChecked());
  }
  if(!info[1]->IsFunction()) {
    return Nan::ThrowError(Nan::New("parseGetRevocRegDefResponse arg 1 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[1]).ToLocalChecked());
  indyCalled(icb, indy_parse_get_revoc_reg_def_response(icb->handle, arg0, parseGetRevocRegDefResponse_cb));
  delete arg0UTF;
}

void buildRevocRegEntryRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildRevocRegEntryRequest) {
  if(info.Length() != 5){
    return Nan::ThrowError(Nan::New("Expected 5 arguments: buildRevocRegEntryRequest(submitter_did, revoc_reg_def_id, rev_def_type, value, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: buildRevocRegEntryRequest(submitter_did, revoc_reg_def_id, rev_def_type, value, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for revoc_reg_def_id: buildRevocRegEntryRequest(submitter_did, revoc_reg_def_id, rev_def_type, value, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for rev_def_type: buildRevocRegEntryRequest(submitter_did, revoc_reg_def_id, rev_def_type, value, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for value: buildRevocRegEntryRequest(submitter_did, revoc_reg_def_id, rev_def_type, value, cb(err, request))").ToLocalChecked());
  }
  if(!info[4]->IsFunction()) {
    return Nan::ThrowError(Nan::New("buildRevocRegEntryRequest arg 4 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[4]).ToLocalChecked());
  indyCalled(icb, indy_build_revoc_reg_entry_request(icb->handle, arg0, arg1, arg2, arg3, buildRevocRegEntryRequest_cb));
  delete arg0UTF;
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
}

void buildGetRevocRegRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildGetRevocRegRequest) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: buildGetRevocRegRequest(submitter_did, revoc_reg_def_id, timestamp, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: buildGetRevocRegRequest(submitter_did, revoc_reg_def_id, timestamp, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for revoc_reg_def_id: buildGetRevocRegRequest(submitter_did, revoc_reg_def_id, timestamp, cb(err, request))").ToLocalChecked());
  }
  if(!info[2]->IsUint32()){
    return Nan::ThrowError(Nan::New("Expected Timestamp for timestamp: buildGetRevocRegRequest(submitter_did, revoc_reg_def_id, timestamp, cb(err, request))").ToLocalChecked());
  }
  long long arg2 = info[2]->Uint32Value();
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("buildGetRevocRegRequest arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_build_get_revoc_reg_request(icb->handle, arg0, arg1, arg2, buildGetRevocRegRequest_cb));
  delete arg0UTF;
  delete arg1UTF;
}

void parseGetRevocRegResponse_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0, const char* arg1, unsigned long long arg2) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringStringTimestamp(xerr, arg0, arg1, arg2);
  }
}
NAN_METHOD(parseGetRevocRegResponse) {
  if(info.Length() != 2){
    return Nan::ThrowError(Nan::New("Expected 2 arguments: parseGetRevocRegResponse(get_revoc_reg_response, cb(err, [ revocRegDefId, revocReg, timestamp ]))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for get_revoc_reg_response: parseGetRevocRegResponse(get_revoc_reg_response, cb(err, [ revocRegDefId, revocReg, timestamp ]))").ToLocalChecked());
  }
  if(!info[1]->IsFunction()) {
    return Nan::ThrowError(Nan::New("parseGetRevocRegResponse arg 1 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[1]).ToLocalChecked());
  indyCalled(icb, indy_parse_get_revoc_reg_response(icb->handle, arg0, parseGetRevocRegResponse_cb));
  delete arg0UTF;
}

void buildGetRevocRegDeltaRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildGetRevocRegDeltaRequest) {
  if(info.Length() != 5){
    return Nan::ThrowError(Nan::New("Expected 5 arguments: buildGetRevocRegDeltaRequest(submitter_did, revoc_reg_def_id, from, to, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: buildGetRevocRegDeltaRequest(submitter_did, revoc_reg_def_id, from, to, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for revoc_reg_def_id: buildGetRevocRegDeltaRequest(submitter_did, revoc_reg_def_id, from, to, cb(err, request))").ToLocalChecked());
  }
  if(!info[2]->IsUint32()){
    return Nan::ThrowError(Nan::New("Expected Timestamp for from: buildGetRevocRegDeltaRequest(submitter_did, revoc_reg_def_id, from, to, cb(err, request))").ToLocalChecked());
  }
  long long arg2 = info[2]->Uint32Value();
  if(!info[3]->IsUint32()){
    return Nan::ThrowError(Nan::New("Expected Timestamp for to: buildGetRevocRegDeltaRequest(submitter_did, revoc_reg_def_id, from, to, cb(err, request))").ToLocalChecked());
  }
  long long arg3 = info[3]->Uint32Value();
  if(!info[4]->IsFunction()) {
    return Nan::ThrowError(Nan::New("buildGetRevocRegDeltaRequest arg 4 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[4]).ToLocalChecked());
  indyCalled(icb, indy_build_get_revoc_reg_delta_request(icb->handle, arg0, arg1, arg2, arg3, buildGetRevocRegDeltaRequest_cb));
  delete arg0UTF;
  delete arg1UTF;
}

void parseGetRevocRegDeltaResponse_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0, const char* arg1, unsigned long long arg2) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringStringTimestamp(xerr, arg0, arg1, arg2);
  }
}
NAN_METHOD(parseGetRevocRegDeltaResponse) {
  if(info.Length() != 2){
    return Nan::ThrowError(Nan::New("Expected 2 arguments: parseGetRevocRegDeltaResponse(get_revoc_reg_delta_response, cb(err, [ revocRegDefId, revocRegDelta, timestamp ]))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for get_revoc_reg_delta_response: parseGetRevocRegDeltaResponse(get_revoc_reg_delta_response, cb(err, [ revocRegDefId, revocRegDelta, timestamp ]))").ToLocalChecked());
  }
  if(!info[1]->IsFunction()) {
    return Nan::ThrowError(Nan::New("parseGetRevocRegDeltaResponse arg 1 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[1]).ToLocalChecked());
  indyCalled(icb, indy_parse_get_revoc_reg_delta_response(icb->handle, arg0, parseGetRevocRegDeltaResponse_cb));
  delete arg0UTF;
}

void addWalletRecord_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(addWalletRecord) {
  if(info.Length() != 6){
    return Nan::ThrowError(Nan::New("Expected 6 arguments: addWalletRecord(wallet_handle, type_, id, value, tags_json, cb(err))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: addWalletRecord(wallet_handle, type_, id, value, tags_json, cb(err))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for type_: addWalletRecord(wallet_handle, type_, id, value, tags_json, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for id: addWalletRecord(wallet_handle, type_, id, value, tags_json, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for value: addWalletRecord(wallet_handle, type_, id, value, tags_json, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg4UTF = nullptr;
  const char* arg4 = nullptr;
  if(info[4]->IsString()){
    arg4UTF = new Nan::Utf8String(info[4]);
    arg4 = (const char*)(**arg4UTF);
  } else if(!info[4]->IsNull() && !info[4]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for tags_json: addWalletRecord(wallet_handle, type_, id, value, tags_json, cb(err))").ToLocalChecked());
  }
  if(!info[5]->IsFunction()) {
    return Nan::ThrowError(Nan::New("addWalletRecord arg 5 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[5]).ToLocalChecked());
  indyCalled(icb, indy_add_wallet_record(icb->handle, arg0, arg1, arg2, arg3, arg4, addWalletRecord_cb));
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
  delete arg4UTF;
}

void updateWalletRecordValue_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(updateWalletRecordValue) {
  if(info.Length() != 5){
    return Nan::ThrowError(Nan::New("Expected 5 arguments: updateWalletRecordValue(wallet_handle, type_, id, value, cb(err))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: updateWalletRecordValue(wallet_handle, type_, id, value, cb(err))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for type_: updateWalletRecordValue(wallet_handle, type_, id, value, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for id: updateWalletRecordValue(wallet_handle, type_, id, value, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for value: updateWalletRecordValue(wallet_handle, type_, id, value, cb(err))").ToLocalChecked());
  }
  if(!info[4]->IsFunction()) {
    return Nan::ThrowError(Nan::New("updateWalletRecordValue arg 4 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[4]).ToLocalChecked());
  indyCalled(icb, indy_update_wallet_record_value(icb->handle, arg0, arg1, arg2, arg3, updateWalletRecordValue_cb));
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
}

void updateWalletRecordTags_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(updateWalletRecordTags) {
  if(info.Length() != 5){
    return Nan::ThrowError(Nan::New("Expected 5 arguments: updateWalletRecordTags(wallet_handle, type_, id, tags_json, cb(err))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: updateWalletRecordTags(wallet_handle, type_, id, tags_json, cb(err))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for type_: updateWalletRecordTags(wallet_handle, type_, id, tags_json, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for id: updateWalletRecordTags(wallet_handle, type_, id, tags_json, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for tags_json: updateWalletRecordTags(wallet_handle, type_, id, tags_json, cb(err))").ToLocalChecked());
  }
  if(!info[4]->IsFunction()) {
    return Nan::ThrowError(Nan::New("updateWalletRecordTags arg 4 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[4]).ToLocalChecked());
  indyCalled(icb, indy_update_wallet_record_tags(icb->handle, arg0, arg1, arg2, arg3, updateWalletRecordTags_cb));
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
}

void addWalletRecordTags_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(addWalletRecordTags) {
  if(info.Length() != 5){
    return Nan::ThrowError(Nan::New("Expected 5 arguments: addWalletRecordTags(wallet_handle, type_, id, tags_json, cb(err))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: addWalletRecordTags(wallet_handle, type_, id, tags_json, cb(err))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for type_: addWalletRecordTags(wallet_handle, type_, id, tags_json, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for id: addWalletRecordTags(wallet_handle, type_, id, tags_json, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for tags_json: addWalletRecordTags(wallet_handle, type_, id, tags_json, cb(err))").ToLocalChecked());
  }
  if(!info[4]->IsFunction()) {
    return Nan::ThrowError(Nan::New("addWalletRecordTags arg 4 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[4]).ToLocalChecked());
  indyCalled(icb, indy_add_wallet_record_tags(icb->handle, arg0, arg1, arg2, arg3, addWalletRecordTags_cb));
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
}

void deleteWalletRecordTags_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(deleteWalletRecordTags) {
  if(info.Length() != 5){
    return Nan::ThrowError(Nan::New("Expected 5 arguments: deleteWalletRecordTags(wallet_handle, type_, id, tag_names_json, cb(err))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: deleteWalletRecordTags(wallet_handle, type_, id, tag_names_json, cb(err))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for type_: deleteWalletRecordTags(wallet_handle, type_, id, tag_names_json, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for id: deleteWalletRecordTags(wallet_handle, type_, id, tag_names_json, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for tag_names_json: deleteWalletRecordTags(wallet_handle, type_, id, tag_names_json, cb(err))").ToLocalChecked());
  }
  if(!info[4]->IsFunction()) {
    return Nan::ThrowError(Nan::New("deleteWalletRecordTags arg 4 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[4]).ToLocalChecked());
  indyCalled(icb, indy_delete_wallet_record_tags(icb->handle, arg0, arg1, arg2, arg3, deleteWalletRecordTags_cb));
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
}

void deleteWalletRecord_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(deleteWalletRecord) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: deleteWalletRecord(wallet_handle, type_, id, cb(err))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: deleteWalletRecord(wallet_handle, type_, id, cb(err))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for type_: deleteWalletRecord(wallet_handle, type_, id, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for id: deleteWalletRecord(wallet_handle, type_, id, cb(err))").ToLocalChecked());
  }
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("deleteWalletRecord arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_delete_wallet_record(icb->handle, arg0, arg1, arg2, deleteWalletRecord_cb));
  delete arg1UTF;
  delete arg2UTF;
}

void getWalletRecord_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(getWalletRecord) {
  if(info.Length() != 5){
    return Nan::ThrowError(Nan::New("Expected 5 arguments: getWalletRecord(wallet_handle, type_, id, options_json, cb(err, record))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: getWalletRecord(wallet_handle, type_, id, options_json, cb(err, record))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for type_: getWalletRecord(wallet_handle, type_, id, options_json, cb(err, record))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for id: getWalletRecord(wallet_handle, type_, id, options_json, cb(err, record))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for options_json: getWalletRecord(wallet_handle, type_, id, options_json, cb(err, record))").ToLocalChecked());
  }
  if(!info[4]->IsFunction()) {
    return Nan::ThrowError(Nan::New("getWalletRecord arg 4 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[4]).ToLocalChecked());
  indyCalled(icb, indy_get_wallet_record(icb->handle, arg0, arg1, arg2, arg3, getWalletRecord_cb));
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
}

void openWalletSearch_cb(indy_handle_t handle, indy_error_t xerr, indy_handle_t arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbHandle(xerr, arg0);
  }
}
NAN_METHOD(openWalletSearch) {
  if(info.Length() != 5){
    return Nan::ThrowError(Nan::New("Expected 5 arguments: openWalletSearch(wallet_handle, type_, query_json, options_json, cb(err, searchHandle))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: openWalletSearch(wallet_handle, type_, query_json, options_json, cb(err, searchHandle))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for type_: openWalletSearch(wallet_handle, type_, query_json, options_json, cb(err, searchHandle))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for query_json: openWalletSearch(wallet_handle, type_, query_json, options_json, cb(err, searchHandle))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for options_json: openWalletSearch(wallet_handle, type_, query_json, options_json, cb(err, searchHandle))").ToLocalChecked());
  }
  if(!info[4]->IsFunction()) {
    return Nan::ThrowError(Nan::New("openWalletSearch arg 4 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[4]).ToLocalChecked());
  indyCalled(icb, indy_open_wallet_search(icb->handle, arg0, arg1, arg2, arg3, openWalletSearch_cb));
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
}

void fetchWalletSearchNextRecords_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(fetchWalletSearchNextRecords) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: fetchWalletSearchNextRecords(wallet_handle, wallet_search_handle, count, cb(err, records))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: fetchWalletSearchNextRecords(wallet_handle, wallet_search_handle, count, cb(err, records))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  if(!info[1]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_search_handle: fetchWalletSearchNextRecords(wallet_handle, wallet_search_handle, count, cb(err, records))").ToLocalChecked());
  }
  indy_handle_t arg1 = info[1]->Int32Value();
  if(!info[2]->IsUint32()){
    return Nan::ThrowError(Nan::New("Expected indy_u32_t for count: fetchWalletSearchNextRecords(wallet_handle, wallet_search_handle, count, cb(err, records))").ToLocalChecked());
  }
  indy_u32_t arg2 = info[2]->Uint32Value();
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("fetchWalletSearchNextRecords arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_fetch_wallet_search_next_records(icb->handle, arg0, arg1, arg2, fetchWalletSearchNextRecords_cb));
}

void closeWalletSearch_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(closeWalletSearch) {
  if(info.Length() != 2){
    return Nan::ThrowError(Nan::New("Expected 2 arguments: closeWalletSearch(wallet_search_handle, cb(err))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_search_handle: closeWalletSearch(wallet_search_handle, cb(err))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  if(!info[1]->IsFunction()) {
    return Nan::ThrowError(Nan::New("closeWalletSearch arg 1 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[1]).ToLocalChecked());
  indyCalled(icb, indy_close_wallet_search(icb->handle, arg0, closeWalletSearch_cb));
}

void isPairwiseExists_cb(indy_handle_t handle, indy_error_t xerr, indy_bool_t arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbBoolean(xerr, arg0);
  }
}
NAN_METHOD(isPairwiseExists) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: isPairwiseExists(wallet_handle, their_did, cb(err, exists))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: isPairwiseExists(wallet_handle, their_did, cb(err, exists))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for their_did: isPairwiseExists(wallet_handle, their_did, cb(err, exists))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("isPairwiseExists arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_is_pairwise_exists(icb->handle, arg0, arg1, isPairwiseExists_cb));
  delete arg1UTF;
}

void createPairwise_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(createPairwise) {
  if(info.Length() != 5){
    return Nan::ThrowError(Nan::New("Expected 5 arguments: createPairwise(wallet_handle, their_did, my_did, metadata, cb(err))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: createPairwise(wallet_handle, their_did, my_did, metadata, cb(err))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for their_did: createPairwise(wallet_handle, their_did, my_did, metadata, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for my_did: createPairwise(wallet_handle, their_did, my_did, metadata, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for metadata: createPairwise(wallet_handle, their_did, my_did, metadata, cb(err))").ToLocalChecked());
  }
  if(!info[4]->IsFunction()) {
    return Nan::ThrowError(Nan::New("createPairwise arg 4 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[4]).ToLocalChecked());
  indyCalled(icb, indy_create_pairwise(icb->handle, arg0, arg1, arg2, arg3, createPairwise_cb));
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
}

void listPairwise_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(listPairwise) {
  if(info.Length() != 2){
    return Nan::ThrowError(Nan::New("Expected 2 arguments: listPairwise(wallet_handle, cb(err, listPairwise))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: listPairwise(wallet_handle, cb(err, listPairwise))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  if(!info[1]->IsFunction()) {
    return Nan::ThrowError(Nan::New("listPairwise arg 1 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[1]).ToLocalChecked());
  indyCalled(icb, indy_list_pairwise(icb->handle, arg0, listPairwise_cb));
}

void getPairwise_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(getPairwise) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: getPairwise(wallet_handle, their_did, cb(err, pairwiseInfo))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: getPairwise(wallet_handle, their_did, cb(err, pairwiseInfo))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for their_did: getPairwise(wallet_handle, their_did, cb(err, pairwiseInfo))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("getPairwise arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_get_pairwise(icb->handle, arg0, arg1, getPairwise_cb));
  delete arg1UTF;
}

void setPairwiseMetadata_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(setPairwiseMetadata) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: setPairwiseMetadata(wallet_handle, their_did, metadata, cb(err))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: setPairwiseMetadata(wallet_handle, their_did, metadata, cb(err))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for their_did: setPairwiseMetadata(wallet_handle, their_did, metadata, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for metadata: setPairwiseMetadata(wallet_handle, their_did, metadata, cb(err))").ToLocalChecked());
  }
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("setPairwiseMetadata arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_set_pairwise_metadata(icb->handle, arg0, arg1, arg2, setPairwiseMetadata_cb));
  delete arg1UTF;
  delete arg2UTF;
}

void createPaymentAddress_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(createPaymentAddress) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: createPaymentAddress(wallet_handle, payment_method, config, cb(err, paymentAddress))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: createPaymentAddress(wallet_handle, payment_method, config, cb(err, paymentAddress))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for payment_method: createPaymentAddress(wallet_handle, payment_method, config, cb(err, paymentAddress))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for config: createPaymentAddress(wallet_handle, payment_method, config, cb(err, paymentAddress))").ToLocalChecked());
  }
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("createPaymentAddress arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_create_payment_address(icb->handle, arg0, arg1, arg2, createPaymentAddress_cb));
  delete arg1UTF;
  delete arg2UTF;
}

void listPaymentAddresses_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(listPaymentAddresses) {
  if(info.Length() != 2){
    return Nan::ThrowError(Nan::New("Expected 2 arguments: listPaymentAddresses(wallet_handle, cb(err, paymentAddresses))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: listPaymentAddresses(wallet_handle, cb(err, paymentAddresses))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  if(!info[1]->IsFunction()) {
    return Nan::ThrowError(Nan::New("listPaymentAddresses arg 1 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[1]).ToLocalChecked());
  indyCalled(icb, indy_list_payment_addresses(icb->handle, arg0, listPaymentAddresses_cb));
}

void addRequestFees_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0, const char* arg1) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringString(xerr, arg0, arg1);
  }
}
NAN_METHOD(addRequestFees) {
  if(info.Length() != 6){
    return Nan::ThrowError(Nan::New("Expected 6 arguments: addRequestFees(wallet_handle, submitter_did, req_json, inputs_json, outputs_json, cb(err, [ reqWithFees, paymentMethod ]))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: addRequestFees(wallet_handle, submitter_did, req_json, inputs_json, outputs_json, cb(err, [ reqWithFees, paymentMethod ]))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: addRequestFees(wallet_handle, submitter_did, req_json, inputs_json, outputs_json, cb(err, [ reqWithFees, paymentMethod ]))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for req_json: addRequestFees(wallet_handle, submitter_did, req_json, inputs_json, outputs_json, cb(err, [ reqWithFees, paymentMethod ]))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for inputs_json: addRequestFees(wallet_handle, submitter_did, req_json, inputs_json, outputs_json, cb(err, [ reqWithFees, paymentMethod ]))").ToLocalChecked());
  }
  Nan::Utf8String* arg4UTF = nullptr;
  const char* arg4 = nullptr;
  if(info[4]->IsString()){
    arg4UTF = new Nan::Utf8String(info[4]);
    arg4 = (const char*)(**arg4UTF);
  } else if(!info[4]->IsNull() && !info[4]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for outputs_json: addRequestFees(wallet_handle, submitter_did, req_json, inputs_json, outputs_json, cb(err, [ reqWithFees, paymentMethod ]))").ToLocalChecked());
  }
  if(!info[5]->IsFunction()) {
    return Nan::ThrowError(Nan::New("addRequestFees arg 5 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[5]).ToLocalChecked());
  indyCalled(icb, indy_add_request_fees(icb->handle, arg0, arg1, arg2, arg3, arg4, addRequestFees_cb));
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
  delete arg4UTF;
}

void parseResponseWithFees_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(parseResponseWithFees) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: parseResponseWithFees(payment_method, resp_json, cb(err, utxo))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for payment_method: parseResponseWithFees(payment_method, resp_json, cb(err, utxo))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for resp_json: parseResponseWithFees(payment_method, resp_json, cb(err, utxo))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("parseResponseWithFees arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_parse_response_with_fees(icb->handle, arg0, arg1, parseResponseWithFees_cb));
  delete arg0UTF;
  delete arg1UTF;
}

void buildGetUtxoRequest_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0, const char* arg1) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringString(xerr, arg0, arg1);
  }
}
NAN_METHOD(buildGetUtxoRequest) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: buildGetUtxoRequest(wallet_handle, submitter_did, payment_address, cb(err, [ getUtxoTxn, paymentMethod ]))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: buildGetUtxoRequest(wallet_handle, submitter_did, payment_address, cb(err, [ getUtxoTxn, paymentMethod ]))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: buildGetUtxoRequest(wallet_handle, submitter_did, payment_address, cb(err, [ getUtxoTxn, paymentMethod ]))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for payment_address: buildGetUtxoRequest(wallet_handle, submitter_did, payment_address, cb(err, [ getUtxoTxn, paymentMethod ]))").ToLocalChecked());
  }
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("buildGetUtxoRequest arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_build_get_utxo_request(icb->handle, arg0, arg1, arg2, buildGetUtxoRequest_cb));
  delete arg1UTF;
  delete arg2UTF;
}

void parseGetUtxoResponse_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(parseGetUtxoResponse) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: parseGetUtxoResponse(payment_method, resp_json, cb(err, utxo))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for payment_method: parseGetUtxoResponse(payment_method, resp_json, cb(err, utxo))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for resp_json: parseGetUtxoResponse(payment_method, resp_json, cb(err, utxo))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("parseGetUtxoResponse arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_parse_get_utxo_response(icb->handle, arg0, arg1, parseGetUtxoResponse_cb));
  delete arg0UTF;
  delete arg1UTF;
}

void buildPaymentReq_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0, const char* arg1) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringString(xerr, arg0, arg1);
  }
}
NAN_METHOD(buildPaymentReq) {
  if(info.Length() != 5){
    return Nan::ThrowError(Nan::New("Expected 5 arguments: buildPaymentReq(wallet_handle, submitter_did, inputs_json, outputs_json, cb(err, [ paymentReq, paymentMethod ]))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: buildPaymentReq(wallet_handle, submitter_did, inputs_json, outputs_json, cb(err, [ paymentReq, paymentMethod ]))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: buildPaymentReq(wallet_handle, submitter_did, inputs_json, outputs_json, cb(err, [ paymentReq, paymentMethod ]))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for inputs_json: buildPaymentReq(wallet_handle, submitter_did, inputs_json, outputs_json, cb(err, [ paymentReq, paymentMethod ]))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for outputs_json: buildPaymentReq(wallet_handle, submitter_did, inputs_json, outputs_json, cb(err, [ paymentReq, paymentMethod ]))").ToLocalChecked());
  }
  if(!info[4]->IsFunction()) {
    return Nan::ThrowError(Nan::New("buildPaymentReq arg 4 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[4]).ToLocalChecked());
  indyCalled(icb, indy_build_payment_req(icb->handle, arg0, arg1, arg2, arg3, buildPaymentReq_cb));
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
}

void parsePaymentResponse_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(parsePaymentResponse) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: parsePaymentResponse(payment_method, resp_json, cb(err, utxo))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for payment_method: parsePaymentResponse(payment_method, resp_json, cb(err, utxo))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for resp_json: parsePaymentResponse(payment_method, resp_json, cb(err, utxo))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("parsePaymentResponse arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_parse_payment_response(icb->handle, arg0, arg1, parsePaymentResponse_cb));
  delete arg0UTF;
  delete arg1UTF;
}

void buildMintReq_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0, const char* arg1) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringString(xerr, arg0, arg1);
  }
}
NAN_METHOD(buildMintReq) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: buildMintReq(wallet_handle, submitter_did, outputs_json, cb(err, [ mintReq, paymentMethod ]))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: buildMintReq(wallet_handle, submitter_did, outputs_json, cb(err, [ mintReq, paymentMethod ]))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: buildMintReq(wallet_handle, submitter_did, outputs_json, cb(err, [ mintReq, paymentMethod ]))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for outputs_json: buildMintReq(wallet_handle, submitter_did, outputs_json, cb(err, [ mintReq, paymentMethod ]))").ToLocalChecked());
  }
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("buildMintReq arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_build_mint_req(icb->handle, arg0, arg1, arg2, buildMintReq_cb));
  delete arg1UTF;
  delete arg2UTF;
}

void buildSetTxnFeesReq_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildSetTxnFeesReq) {
  if(info.Length() != 5){
    return Nan::ThrowError(Nan::New("Expected 5 arguments: buildSetTxnFeesReq(wallet_handle, submitter_did, payment_method, fees_json, cb(err, setTxnFees))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: buildSetTxnFeesReq(wallet_handle, submitter_did, payment_method, fees_json, cb(err, setTxnFees))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: buildSetTxnFeesReq(wallet_handle, submitter_did, payment_method, fees_json, cb(err, setTxnFees))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for payment_method: buildSetTxnFeesReq(wallet_handle, submitter_did, payment_method, fees_json, cb(err, setTxnFees))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for fees_json: buildSetTxnFeesReq(wallet_handle, submitter_did, payment_method, fees_json, cb(err, setTxnFees))").ToLocalChecked());
  }
  if(!info[4]->IsFunction()) {
    return Nan::ThrowError(Nan::New("buildSetTxnFeesReq arg 4 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[4]).ToLocalChecked());
  indyCalled(icb, indy_build_set_txn_fees_req(icb->handle, arg0, arg1, arg2, arg3, buildSetTxnFeesReq_cb));
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
}

void buildGetTxnFeesReq_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(buildGetTxnFeesReq) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: buildGetTxnFeesReq(wallet_handle, submitter_did, payment_method, cb(err, getTxnFees))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: buildGetTxnFeesReq(wallet_handle, submitter_did, payment_method, cb(err, getTxnFees))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: buildGetTxnFeesReq(wallet_handle, submitter_did, payment_method, cb(err, getTxnFees))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for payment_method: buildGetTxnFeesReq(wallet_handle, submitter_did, payment_method, cb(err, getTxnFees))").ToLocalChecked());
  }
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("buildGetTxnFeesReq arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_build_get_txn_fees_req(icb->handle, arg0, arg1, arg2, buildGetTxnFeesReq_cb));
  delete arg1UTF;
  delete arg2UTF;
}

void parseGetTxnFeesResponse_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(parseGetTxnFeesResponse) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: parseGetTxnFeesResponse(payment_method, resp_json, cb(err, fees))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for payment_method: parseGetTxnFeesResponse(payment_method, resp_json, cb(err, fees))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for resp_json: parseGetTxnFeesResponse(payment_method, resp_json, cb(err, fees))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("parseGetTxnFeesResponse arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_parse_get_txn_fees_response(icb->handle, arg0, arg1, parseGetTxnFeesResponse_cb));
  delete arg0UTF;
  delete arg1UTF;
}

void createPoolLedgerConfig_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(createPoolLedgerConfig) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: createPoolLedgerConfig(config_name, config, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for config_name: createPoolLedgerConfig(config_name, config, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for config: createPoolLedgerConfig(config_name, config, cb(err))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("createPoolLedgerConfig arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_create_pool_ledger_config(icb->handle, arg0, arg1, createPoolLedgerConfig_cb));
  delete arg0UTF;
  delete arg1UTF;
}

void openPoolLedger_cb(indy_handle_t handle, indy_error_t xerr, indy_handle_t arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbHandle(xerr, arg0);
  }
}
NAN_METHOD(openPoolLedger) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: openPoolLedger(config_name, config, cb(err, poolHandle))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for config_name: openPoolLedger(config_name, config, cb(err, poolHandle))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for config: openPoolLedger(config_name, config, cb(err, poolHandle))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("openPoolLedger arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_open_pool_ledger(icb->handle, arg0, arg1, openPoolLedger_cb));
  delete arg0UTF;
  delete arg1UTF;
}

void refreshPoolLedger_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(refreshPoolLedger) {
  if(info.Length() != 2){
    return Nan::ThrowError(Nan::New("Expected 2 arguments: refreshPoolLedger(handle, cb(err))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for handle: refreshPoolLedger(handle, cb(err))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  if(!info[1]->IsFunction()) {
    return Nan::ThrowError(Nan::New("refreshPoolLedger arg 1 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[1]).ToLocalChecked());
  indyCalled(icb, indy_refresh_pool_ledger(icb->handle, arg0, refreshPoolLedger_cb));
}

void listPools_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(listPools) {
  if(info.Length() != 1){
    return Nan::ThrowError(Nan::New("Expected 1 arguments: listPools(cb(err, pools))").ToLocalChecked());
  }
  if(!info[0]->IsFunction()) {
    return Nan::ThrowError(Nan::New("listPools arg 0 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[0]).ToLocalChecked());
  indyCalled(icb, indy_list_pools(icb->handle, listPools_cb));
}

void closePoolLedger_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(closePoolLedger) {
  if(info.Length() != 2){
    return Nan::ThrowError(Nan::New("Expected 2 arguments: closePoolLedger(handle, cb(err))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for handle: closePoolLedger(handle, cb(err))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  if(!info[1]->IsFunction()) {
    return Nan::ThrowError(Nan::New("closePoolLedger arg 1 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[1]).ToLocalChecked());
  indyCalled(icb, indy_close_pool_ledger(icb->handle, arg0, closePoolLedger_cb));
}

void deletePoolLedgerConfig_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(deletePoolLedgerConfig) {
  if(info.Length() != 2){
    return Nan::ThrowError(Nan::New("Expected 2 arguments: deletePoolLedgerConfig(config_name, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for config_name: deletePoolLedgerConfig(config_name, cb(err))").ToLocalChecked());
  }
  if(!info[1]->IsFunction()) {
    return Nan::ThrowError(Nan::New("deletePoolLedgerConfig arg 1 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[1]).ToLocalChecked());
  indyCalled(icb, indy_delete_pool_ledger_config(icb->handle, arg0, deletePoolLedgerConfig_cb));
  delete arg0UTF;
}

void setProtocolVersion_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(setProtocolVersion) {
  if(info.Length() != 2){
    return Nan::ThrowError(Nan::New("Expected 2 arguments: setProtocolVersion(protocol_version, cb(err))").ToLocalChecked());
  }
  if(!info[0]->IsUint32()){
    return Nan::ThrowError(Nan::New("Expected indy_u64_t for protocol_version: setProtocolVersion(protocol_version, cb(err))").ToLocalChecked());
  }
  indy_u64_t arg0 = (indy_u64_t)info[0]->Uint32Value();
  if(!info[1]->IsFunction()) {
    return Nan::ThrowError(Nan::New("setProtocolVersion arg 1 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[1]).ToLocalChecked());
  indyCalled(icb, indy_set_protocol_version(icb->handle, arg0, setProtocolVersion_cb));
}

void createWallet_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(createWallet) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: createWallet(config, credentials, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for config: createWallet(config, credentials, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for credentials: createWallet(config, credentials, cb(err))").ToLocalChecked());
  }

  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("createWallet arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_create_wallet(icb->handle, arg0, arg1, createWallet_cb));
  delete arg0UTF;
  delete arg1UTF;
}

void openWallet_cb(indy_handle_t handle, indy_error_t xerr, indy_handle_t arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbHandle(xerr, arg0);
  }
}
NAN_METHOD(openWallet) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: openWallet(config, credentials, cb(err, handle))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for config: openWallet(config, credentials, cb(err, handle))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for credentials: openWallet(config, credentials, cb(err, handle))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("openWallet arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_open_wallet(icb->handle, arg0, arg1, openWallet_cb));
  delete arg0UTF;
  delete arg1UTF;
}

void closeWallet_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(closeWallet) {
  if(info.Length() != 2){
    return Nan::ThrowError(Nan::New("Expected 2 arguments: closeWallet(handle, cb(err))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for handle: closeWallet(handle, cb(err))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  if(!info[1]->IsFunction()) {
    return Nan::ThrowError(Nan::New("closeWallet arg 1 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[1]).ToLocalChecked());
  indyCalled(icb, indy_close_wallet(icb->handle, arg0, closeWallet_cb));
}

void deleteWallet_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(deleteWallet) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 2 arguments: deleteWallet(config, credentials, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for config: deleteWallet(config, credentials, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for credentials: deleteWallet(config, credentials, cb(err))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("deleteWallet arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_delete_wallet(icb->handle, arg0, arg1, deleteWallet_cb));
  delete arg0UTF;
  delete arg1UTF;
}

void exportWallet_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(exportWallet) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: exportWallet(wallet_handle, export_config_json, cb(err))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: exportWallet(wallet_handle, export_config_json, cb(err))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for export_config_json: exportWallet(wallet_handle, export_config_json, cb(err))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("exportWallet arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_export_wallet(icb->handle, arg0, arg1, exportWallet_cb));
  delete arg1UTF;
}

void importWallet_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(importWallet) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: importWallet(config, credentials, import_config_json, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for config: importWallet(config, credentials, import_config_json, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for credentials: importWallet(config, credentials, import_config_json, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for import_config_json: importWallet(config, credentials, import_config_json, cb(err))").ToLocalChecked());
  }
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("importWallet arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_import_wallet(icb->handle, arg0, arg1, arg2, importWallet_cb));
  delete arg0UTF;
  delete arg1UTF;
  delete arg2UTF;
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
  Nan::Export(target, "buildGetUtxoRequest", buildGetUtxoRequest);
  Nan::Export(target, "parseGetUtxoResponse", parseGetUtxoResponse);
  Nan::Export(target, "buildPaymentReq", buildPaymentReq);
  Nan::Export(target, "parsePaymentResponse", parsePaymentResponse);
  Nan::Export(target, "buildMintReq", buildMintReq);
  Nan::Export(target, "buildSetTxnFeesReq", buildSetTxnFeesReq);
  Nan::Export(target, "buildGetTxnFeesReq", buildGetTxnFeesReq);
  Nan::Export(target, "parseGetTxnFeesResponse", parseGetTxnFeesResponse);
  Nan::Export(target, "createPoolLedgerConfig", createPoolLedgerConfig);
  Nan::Export(target, "openPoolLedger", openPoolLedger);
  Nan::Export(target, "refreshPoolLedger", refreshPoolLedger);
  Nan::Export(target, "listPools", listPools);
  Nan::Export(target, "closePoolLedger", closePoolLedger);
  Nan::Export(target, "deletePoolLedgerConfig", deletePoolLedgerConfig);
  Nan::Export(target, "setProtocolVersion", setProtocolVersion);
  Nan::Export(target, "createWallet", createWallet);
  Nan::Export(target, "openWallet", openWallet);
  Nan::Export(target, "closeWallet", closeWallet);
  Nan::Export(target, "deleteWallet", deleteWallet);
  Nan::Export(target, "exportWallet", exportWallet);
  Nan::Export(target, "importWallet", importWallet);
  Nan::Export(target, "closeWallet", closeWallet);
  Nan::Export(target, "deleteWallet", deleteWallet);
}
NODE_MODULE(indynodejs, InitAll)

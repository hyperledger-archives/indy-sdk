void issuer_create_and_store_claim_def_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(issuer_create_and_store_claim_def) {
  if(info.Length() != 6){
    return Nan::ThrowError(Nan::New("Expected 6 arguments: issuer_create_and_store_claim_def(wallet_handle, issuer_did, schema_json, signature_type, create_non_revoc, cb(err, claimDef))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: issuer_create_and_store_claim_def(wallet_handle, issuer_did, schema_json, signature_type, create_non_revoc, cb(err, claimDef))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for issuer_did: issuer_create_and_store_claim_def(wallet_handle, issuer_did, schema_json, signature_type, create_non_revoc, cb(err, claimDef))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for schema_json: issuer_create_and_store_claim_def(wallet_handle, issuer_did, schema_json, signature_type, create_non_revoc, cb(err, claimDef))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for signature_type: issuer_create_and_store_claim_def(wallet_handle, issuer_did, schema_json, signature_type, create_non_revoc, cb(err, claimDef))").ToLocalChecked());
  }
  if(!info[4]->IsBoolean()){
    return Nan::ThrowError(Nan::New("Expected Boolean for create_non_revoc: issuer_create_and_store_claim_def(wallet_handle, issuer_did, schema_json, signature_type, create_non_revoc, cb(err, claimDef))").ToLocalChecked());
  }
  indy_bool_t arg4 = info[4]->IsTrue();
  if(!info[5]->IsFunction()) {
    return Nan::ThrowError(Nan::New("issuer_create_and_store_claim_def arg 5 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[5]).ToLocalChecked());
  indyCalled(icb, indy_issuer_create_and_store_claim_def(icb->handle, arg0, arg1, arg2, arg3, arg4, issuer_create_and_store_claim_def_cb));
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
}

void issuer_create_and_store_revoc_reg_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(issuer_create_and_store_revoc_reg) {
  if(info.Length() != 5){
    return Nan::ThrowError(Nan::New("Expected 5 arguments: issuer_create_and_store_revoc_reg(wallet_handle, issuer_did, schema_json, max_claim_num, cb(err, revocReg))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: issuer_create_and_store_revoc_reg(wallet_handle, issuer_did, schema_json, max_claim_num, cb(err, revocReg))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for issuer_did: issuer_create_and_store_revoc_reg(wallet_handle, issuer_did, schema_json, max_claim_num, cb(err, revocReg))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for schema_json: issuer_create_and_store_revoc_reg(wallet_handle, issuer_did, schema_json, max_claim_num, cb(err, revocReg))").ToLocalChecked());
  }
  if(!info[3]->IsUint32()){
    return Nan::ThrowError(Nan::New("Expected indy_u32_t for max_claim_num: issuer_create_and_store_revoc_reg(wallet_handle, issuer_did, schema_json, max_claim_num, cb(err, revocReg))").ToLocalChecked());
  }
  indy_u32_t arg3 = info[3]->Uint32Value();
  if(!info[4]->IsFunction()) {
    return Nan::ThrowError(Nan::New("issuer_create_and_store_revoc_reg arg 4 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[4]).ToLocalChecked());
  indyCalled(icb, indy_issuer_create_and_store_revoc_reg(icb->handle, arg0, arg1, arg2, arg3, issuer_create_and_store_revoc_reg_cb));
  delete arg1UTF;
  delete arg2UTF;
}

void issuer_create_claim_offer_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(issuer_create_claim_offer) {
  if(info.Length() != 5){
    return Nan::ThrowError(Nan::New("Expected 5 arguments: issuer_create_claim_offer(wallet_handle, schema_json, issuer_did, prover_did, cb(err, claimOffer))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: issuer_create_claim_offer(wallet_handle, schema_json, issuer_did, prover_did, cb(err, claimOffer))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for schema_json: issuer_create_claim_offer(wallet_handle, schema_json, issuer_did, prover_did, cb(err, claimOffer))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for issuer_did: issuer_create_claim_offer(wallet_handle, schema_json, issuer_did, prover_did, cb(err, claimOffer))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for prover_did: issuer_create_claim_offer(wallet_handle, schema_json, issuer_did, prover_did, cb(err, claimOffer))").ToLocalChecked());
  }
  if(!info[4]->IsFunction()) {
    return Nan::ThrowError(Nan::New("issuer_create_claim_offer arg 4 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[4]).ToLocalChecked());
  indyCalled(icb, indy_issuer_create_claim_offer(icb->handle, arg0, arg1, arg2, arg3, issuer_create_claim_offer_cb));
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
}

void issuer_create_claim_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0, const char* arg1) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringString(xerr, arg0, arg1);
  }
}
NAN_METHOD(issuer_create_claim) {
  if(info.Length() != 5){
    return Nan::ThrowError(Nan::New("Expected 5 arguments: issuer_create_claim(wallet_handle, claim_req_json, claim_json, user_revoc_index, cb(err, [ revocRegUpdate, xclaim ]))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: issuer_create_claim(wallet_handle, claim_req_json, claim_json, user_revoc_index, cb(err, [ revocRegUpdate, xclaim ]))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for claim_req_json: issuer_create_claim(wallet_handle, claim_req_json, claim_json, user_revoc_index, cb(err, [ revocRegUpdate, xclaim ]))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for claim_json: issuer_create_claim(wallet_handle, claim_req_json, claim_json, user_revoc_index, cb(err, [ revocRegUpdate, xclaim ]))").ToLocalChecked());
  }
  if(!info[3]->IsInt32()){
    return Nan::ThrowError(Nan::New("Expected indy_i32_t for user_revoc_index: issuer_create_claim(wallet_handle, claim_req_json, claim_json, user_revoc_index, cb(err, [ revocRegUpdate, xclaim ]))").ToLocalChecked());
  }
  indy_i32_t arg3 = info[3]->Int32Value();
  if(!info[4]->IsFunction()) {
    return Nan::ThrowError(Nan::New("issuer_create_claim arg 4 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[4]).ToLocalChecked());
  indyCalled(icb, indy_issuer_create_claim(icb->handle, arg0, arg1, arg2, arg3, issuer_create_claim_cb));
  delete arg1UTF;
  delete arg2UTF;
}

void issuer_revoke_claim_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(issuer_revoke_claim) {
  if(info.Length() != 5){
    return Nan::ThrowError(Nan::New("Expected 5 arguments: issuer_revoke_claim(wallet_handle, issuer_did, schema_json, user_revoc_index, cb(err, revocRegUpdate))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: issuer_revoke_claim(wallet_handle, issuer_did, schema_json, user_revoc_index, cb(err, revocRegUpdate))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for issuer_did: issuer_revoke_claim(wallet_handle, issuer_did, schema_json, user_revoc_index, cb(err, revocRegUpdate))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for schema_json: issuer_revoke_claim(wallet_handle, issuer_did, schema_json, user_revoc_index, cb(err, revocRegUpdate))").ToLocalChecked());
  }
  if(!info[3]->IsUint32()){
    return Nan::ThrowError(Nan::New("Expected indy_u32_t for user_revoc_index: issuer_revoke_claim(wallet_handle, issuer_did, schema_json, user_revoc_index, cb(err, revocRegUpdate))").ToLocalChecked());
  }
  indy_u32_t arg3 = info[3]->Uint32Value();
  if(!info[4]->IsFunction()) {
    return Nan::ThrowError(Nan::New("issuer_revoke_claim arg 4 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[4]).ToLocalChecked());
  indyCalled(icb, indy_issuer_revoke_claim(icb->handle, arg0, arg1, arg2, arg3, issuer_revoke_claim_cb));
  delete arg1UTF;
  delete arg2UTF;
}

void prover_store_claim_offer_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(prover_store_claim_offer) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: prover_store_claim_offer(wallet_handle, claim_offer_json, cb(err))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: prover_store_claim_offer(wallet_handle, claim_offer_json, cb(err))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for claim_offer_json: prover_store_claim_offer(wallet_handle, claim_offer_json, cb(err))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("prover_store_claim_offer arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_prover_store_claim_offer(icb->handle, arg0, arg1, prover_store_claim_offer_cb));
  delete arg1UTF;
}

void prover_get_claim_offers_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(prover_get_claim_offers) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: prover_get_claim_offers(wallet_handle, filter_json, cb(err, claimOffers))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: prover_get_claim_offers(wallet_handle, filter_json, cb(err, claimOffers))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for filter_json: prover_get_claim_offers(wallet_handle, filter_json, cb(err, claimOffers))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("prover_get_claim_offers arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_prover_get_claim_offers(icb->handle, arg0, arg1, prover_get_claim_offers_cb));
  delete arg1UTF;
}

void prover_create_master_secret_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(prover_create_master_secret) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: prover_create_master_secret(wallet_handle, master_secret_name, cb(err))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: prover_create_master_secret(wallet_handle, master_secret_name, cb(err))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for master_secret_name: prover_create_master_secret(wallet_handle, master_secret_name, cb(err))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("prover_create_master_secret arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_prover_create_master_secret(icb->handle, arg0, arg1, prover_create_master_secret_cb));
  delete arg1UTF;
}

void prover_create_and_store_claim_req_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(prover_create_and_store_claim_req) {
  if(info.Length() != 6){
    return Nan::ThrowError(Nan::New("Expected 6 arguments: prover_create_and_store_claim_req(wallet_handle, prover_did, claim_offer_json, claim_def_json, master_secret_name, cb(err, claimReq))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: prover_create_and_store_claim_req(wallet_handle, prover_did, claim_offer_json, claim_def_json, master_secret_name, cb(err, claimReq))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for prover_did: prover_create_and_store_claim_req(wallet_handle, prover_did, claim_offer_json, claim_def_json, master_secret_name, cb(err, claimReq))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for claim_offer_json: prover_create_and_store_claim_req(wallet_handle, prover_did, claim_offer_json, claim_def_json, master_secret_name, cb(err, claimReq))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for claim_def_json: prover_create_and_store_claim_req(wallet_handle, prover_did, claim_offer_json, claim_def_json, master_secret_name, cb(err, claimReq))").ToLocalChecked());
  }
  Nan::Utf8String* arg4UTF = nullptr;
  const char* arg4 = nullptr;
  if(info[4]->IsString()){
    arg4UTF = new Nan::Utf8String(info[4]);
    arg4 = (const char*)(**arg4UTF);
  } else if(!info[4]->IsNull() && !info[4]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for master_secret_name: prover_create_and_store_claim_req(wallet_handle, prover_did, claim_offer_json, claim_def_json, master_secret_name, cb(err, claimReq))").ToLocalChecked());
  }
  if(!info[5]->IsFunction()) {
    return Nan::ThrowError(Nan::New("prover_create_and_store_claim_req arg 5 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[5]).ToLocalChecked());
  indyCalled(icb, indy_prover_create_and_store_claim_req(icb->handle, arg0, arg1, arg2, arg3, arg4, prover_create_and_store_claim_req_cb));
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
  delete arg4UTF;
}

void prover_store_claim_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(prover_store_claim) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: prover_store_claim(wallet_handle, claims_json, rev_reg_json, cb(err))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: prover_store_claim(wallet_handle, claims_json, rev_reg_json, cb(err))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for claims_json: prover_store_claim(wallet_handle, claims_json, rev_reg_json, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for rev_reg_json: prover_store_claim(wallet_handle, claims_json, rev_reg_json, cb(err))").ToLocalChecked());
  }
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("prover_store_claim arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_prover_store_claim(icb->handle, arg0, arg1, arg2, prover_store_claim_cb));
  delete arg1UTF;
  delete arg2UTF;
}

void prover_get_claims_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(prover_get_claims) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: prover_get_claims(wallet_handle, filter_json, cb(err, claims))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: prover_get_claims(wallet_handle, filter_json, cb(err, claims))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for filter_json: prover_get_claims(wallet_handle, filter_json, cb(err, claims))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("prover_get_claims arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_prover_get_claims(icb->handle, arg0, arg1, prover_get_claims_cb));
  delete arg1UTF;
}

void prover_get_claims_for_proof_req_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(prover_get_claims_for_proof_req) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: prover_get_claims_for_proof_req(wallet_handle, proof_request_json, cb(err, claims))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: prover_get_claims_for_proof_req(wallet_handle, proof_request_json, cb(err, claims))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for proof_request_json: prover_get_claims_for_proof_req(wallet_handle, proof_request_json, cb(err, claims))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("prover_get_claims_for_proof_req arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_prover_get_claims_for_proof_req(icb->handle, arg0, arg1, prover_get_claims_for_proof_req_cb));
  delete arg1UTF;
}

void prover_create_proof_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(prover_create_proof) {
  if(info.Length() != 8){
    return Nan::ThrowError(Nan::New("Expected 8 arguments: prover_create_proof(wallet_handle, proof_req_json, requested_claims_json, schemas_json, master_secret_name, claim_defs_json, revoc_regs_json, cb(err, proof))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: prover_create_proof(wallet_handle, proof_req_json, requested_claims_json, schemas_json, master_secret_name, claim_defs_json, revoc_regs_json, cb(err, proof))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for proof_req_json: prover_create_proof(wallet_handle, proof_req_json, requested_claims_json, schemas_json, master_secret_name, claim_defs_json, revoc_regs_json, cb(err, proof))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for requested_claims_json: prover_create_proof(wallet_handle, proof_req_json, requested_claims_json, schemas_json, master_secret_name, claim_defs_json, revoc_regs_json, cb(err, proof))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for schemas_json: prover_create_proof(wallet_handle, proof_req_json, requested_claims_json, schemas_json, master_secret_name, claim_defs_json, revoc_regs_json, cb(err, proof))").ToLocalChecked());
  }
  Nan::Utf8String* arg4UTF = nullptr;
  const char* arg4 = nullptr;
  if(info[4]->IsString()){
    arg4UTF = new Nan::Utf8String(info[4]);
    arg4 = (const char*)(**arg4UTF);
  } else if(!info[4]->IsNull() && !info[4]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for master_secret_name: prover_create_proof(wallet_handle, proof_req_json, requested_claims_json, schemas_json, master_secret_name, claim_defs_json, revoc_regs_json, cb(err, proof))").ToLocalChecked());
  }
  Nan::Utf8String* arg5UTF = nullptr;
  const char* arg5 = nullptr;
  if(info[5]->IsString()){
    arg5UTF = new Nan::Utf8String(info[5]);
    arg5 = (const char*)(**arg5UTF);
  } else if(!info[5]->IsNull() && !info[5]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for claim_defs_json: prover_create_proof(wallet_handle, proof_req_json, requested_claims_json, schemas_json, master_secret_name, claim_defs_json, revoc_regs_json, cb(err, proof))").ToLocalChecked());
  }
  Nan::Utf8String* arg6UTF = nullptr;
  const char* arg6 = nullptr;
  if(info[6]->IsString()){
    arg6UTF = new Nan::Utf8String(info[6]);
    arg6 = (const char*)(**arg6UTF);
  } else if(!info[6]->IsNull() && !info[6]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for revoc_regs_json: prover_create_proof(wallet_handle, proof_req_json, requested_claims_json, schemas_json, master_secret_name, claim_defs_json, revoc_regs_json, cb(err, proof))").ToLocalChecked());
  }
  if(!info[7]->IsFunction()) {
    return Nan::ThrowError(Nan::New("prover_create_proof arg 7 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[7]).ToLocalChecked());
  indyCalled(icb, indy_prover_create_proof(icb->handle, arg0, arg1, arg2, arg3, arg4, arg5, arg6, prover_create_proof_cb));
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
  delete arg4UTF;
  delete arg5UTF;
  delete arg6UTF;
}

void verifier_verify_proof_cb(indy_handle_t handle, indy_error_t xerr, indy_bool_t arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbBoolean(xerr, arg0);
  }
}
NAN_METHOD(verifier_verify_proof) {
  if(info.Length() != 6){
    return Nan::ThrowError(Nan::New("Expected 6 arguments: verifier_verify_proof(proof_request_json, proof_json, schemas_json, claim_defs_jsons, revoc_regs_json, cb(err, valid))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for proof_request_json: verifier_verify_proof(proof_request_json, proof_json, schemas_json, claim_defs_jsons, revoc_regs_json, cb(err, valid))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for proof_json: verifier_verify_proof(proof_request_json, proof_json, schemas_json, claim_defs_jsons, revoc_regs_json, cb(err, valid))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for schemas_json: verifier_verify_proof(proof_request_json, proof_json, schemas_json, claim_defs_jsons, revoc_regs_json, cb(err, valid))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for claim_defs_jsons: verifier_verify_proof(proof_request_json, proof_json, schemas_json, claim_defs_jsons, revoc_regs_json, cb(err, valid))").ToLocalChecked());
  }
  Nan::Utf8String* arg4UTF = nullptr;
  const char* arg4 = nullptr;
  if(info[4]->IsString()){
    arg4UTF = new Nan::Utf8String(info[4]);
    arg4 = (const char*)(**arg4UTF);
  } else if(!info[4]->IsNull() && !info[4]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for revoc_regs_json: verifier_verify_proof(proof_request_json, proof_json, schemas_json, claim_defs_jsons, revoc_regs_json, cb(err, valid))").ToLocalChecked());
  }
  if(!info[5]->IsFunction()) {
    return Nan::ThrowError(Nan::New("verifier_verify_proof arg 5 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[5]).ToLocalChecked());
  indyCalled(icb, indy_verifier_verify_proof(icb->handle, arg0, arg1, arg2, arg3, arg4, verifier_verify_proof_cb));
  delete arg0UTF;
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
  delete arg4UTF;
}

void create_key_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(create_key) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: create_key(wallet_handle, key_json, cb(err, vk))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: create_key(wallet_handle, key_json, cb(err, vk))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for key_json: create_key(wallet_handle, key_json, cb(err, vk))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("create_key arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_create_key(icb->handle, arg0, arg1, create_key_cb));
  delete arg1UTF;
}

void set_key_metadata_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(set_key_metadata) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: set_key_metadata(wallet_handle, verkey, metadata, cb(err))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: set_key_metadata(wallet_handle, verkey, metadata, cb(err))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for verkey: set_key_metadata(wallet_handle, verkey, metadata, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for metadata: set_key_metadata(wallet_handle, verkey, metadata, cb(err))").ToLocalChecked());
  }
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("set_key_metadata arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_set_key_metadata(icb->handle, arg0, arg1, arg2, set_key_metadata_cb));
  delete arg1UTF;
  delete arg2UTF;
}

void get_key_metadata_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(get_key_metadata) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: get_key_metadata(wallet_handle, verkey, cb(err, metadata))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: get_key_metadata(wallet_handle, verkey, cb(err, metadata))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for verkey: get_key_metadata(wallet_handle, verkey, cb(err, metadata))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("get_key_metadata arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_get_key_metadata(icb->handle, arg0, arg1, get_key_metadata_cb));
  delete arg1UTF;
}

void crypto_sign_cb(indy_handle_t handle, indy_error_t xerr, const indy_u8_t* arg0data, indy_u32_t arg0len) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbBuffer(xerr, arg0data, arg0len);
  }
}
NAN_METHOD(crypto_sign) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: crypto_sign(wallet_handle, my_vk, message_raw, cb(err, signatureRaw))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: crypto_sign(wallet_handle, my_vk, message_raw, cb(err, signatureRaw))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for my_vk: crypto_sign(wallet_handle, my_vk, message_raw, cb(err, signatureRaw))").ToLocalChecked());
  }
  if(!info[2]->IsUint8Array()){
    return Nan::ThrowError(Nan::New("Expected Buffer for message_raw: crypto_sign(wallet_handle, my_vk, message_raw, cb(err, signatureRaw))").ToLocalChecked());
  }
  const indy_u8_t* arg2data = (indy_u8_t*)node::Buffer::Data(info[2]->ToObject());
  indy_u32_t arg2len = node::Buffer::Length(info[2]);
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("crypto_sign arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_crypto_sign(icb->handle, arg0, arg1, arg2data, arg2len, crypto_sign_cb));
  delete arg1UTF;
}

void crypto_verify_cb(indy_handle_t handle, indy_error_t xerr, indy_bool_t arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbBoolean(xerr, arg0);
  }
}
NAN_METHOD(crypto_verify) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: crypto_verify(their_vk, message_raw, signature_raw, cb(err, valid))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for their_vk: crypto_verify(their_vk, message_raw, signature_raw, cb(err, valid))").ToLocalChecked());
  }
  if(!info[1]->IsUint8Array()){
    return Nan::ThrowError(Nan::New("Expected Buffer for message_raw: crypto_verify(their_vk, message_raw, signature_raw, cb(err, valid))").ToLocalChecked());
  }
  const indy_u8_t* arg1data = (indy_u8_t*)node::Buffer::Data(info[1]->ToObject());
  indy_u32_t arg1len = node::Buffer::Length(info[1]);
  if(!info[2]->IsUint8Array()){
    return Nan::ThrowError(Nan::New("Expected Buffer for signature_raw: crypto_verify(their_vk, message_raw, signature_raw, cb(err, valid))").ToLocalChecked());
  }
  const indy_u8_t* arg2data = (indy_u8_t*)node::Buffer::Data(info[2]->ToObject());
  indy_u32_t arg2len = node::Buffer::Length(info[2]);
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("crypto_verify arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_crypto_verify(icb->handle, arg0, arg1data, arg1len, arg2data, arg2len, crypto_verify_cb));
  delete arg0UTF;
}

void crypto_auth_crypt_cb(indy_handle_t handle, indy_error_t xerr, const indy_u8_t* arg0data, indy_u32_t arg0len) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbBuffer(xerr, arg0data, arg0len);
  }
}
NAN_METHOD(crypto_auth_crypt) {
  if(info.Length() != 5){
    return Nan::ThrowError(Nan::New("Expected 5 arguments: crypto_auth_crypt(wallet_handle, my_vk, their_vk, message_raw, cb(err, encryptedMsgRaw))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: crypto_auth_crypt(wallet_handle, my_vk, their_vk, message_raw, cb(err, encryptedMsgRaw))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for my_vk: crypto_auth_crypt(wallet_handle, my_vk, their_vk, message_raw, cb(err, encryptedMsgRaw))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for their_vk: crypto_auth_crypt(wallet_handle, my_vk, their_vk, message_raw, cb(err, encryptedMsgRaw))").ToLocalChecked());
  }
  if(!info[3]->IsUint8Array()){
    return Nan::ThrowError(Nan::New("Expected Buffer for message_raw: crypto_auth_crypt(wallet_handle, my_vk, their_vk, message_raw, cb(err, encryptedMsgRaw))").ToLocalChecked());
  }
  const indy_u8_t* arg3data = (indy_u8_t*)node::Buffer::Data(info[3]->ToObject());
  indy_u32_t arg3len = node::Buffer::Length(info[3]);
  if(!info[4]->IsFunction()) {
    return Nan::ThrowError(Nan::New("crypto_auth_crypt arg 4 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[4]).ToLocalChecked());
  indyCalled(icb, indy_crypto_auth_crypt(icb->handle, arg0, arg1, arg2, arg3data, arg3len, crypto_auth_crypt_cb));
  delete arg1UTF;
  delete arg2UTF;
}

void crypto_auth_decrypt_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0, const indy_u8_t* arg1data, indy_u32_t arg1len) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringBuffer(xerr, arg0, arg1data, arg1len);
  }
}
NAN_METHOD(crypto_auth_decrypt) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: crypto_auth_decrypt(wallet_handle, my_vk, encrypted_msg_raw, cb(err, [ theirVk, decryptedMsgRaw ]))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: crypto_auth_decrypt(wallet_handle, my_vk, encrypted_msg_raw, cb(err, [ theirVk, decryptedMsgRaw ]))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for my_vk: crypto_auth_decrypt(wallet_handle, my_vk, encrypted_msg_raw, cb(err, [ theirVk, decryptedMsgRaw ]))").ToLocalChecked());
  }
  if(!info[2]->IsUint8Array()){
    return Nan::ThrowError(Nan::New("Expected Buffer for encrypted_msg_raw: crypto_auth_decrypt(wallet_handle, my_vk, encrypted_msg_raw, cb(err, [ theirVk, decryptedMsgRaw ]))").ToLocalChecked());
  }
  const indy_u8_t* arg2data = (indy_u8_t*)node::Buffer::Data(info[2]->ToObject());
  indy_u32_t arg2len = node::Buffer::Length(info[2]);
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("crypto_auth_decrypt arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_crypto_auth_decrypt(icb->handle, arg0, arg1, arg2data, arg2len, crypto_auth_decrypt_cb));
  delete arg1UTF;
}

void crypto_anon_crypt_cb(indy_handle_t handle, indy_error_t xerr, const indy_u8_t* arg0data, indy_u32_t arg0len) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbBuffer(xerr, arg0data, arg0len);
  }
}
NAN_METHOD(crypto_anon_crypt) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: crypto_anon_crypt(their_vk, message_raw, cb(err, encryptedMsgRaw))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for their_vk: crypto_anon_crypt(their_vk, message_raw, cb(err, encryptedMsgRaw))").ToLocalChecked());
  }
  if(!info[1]->IsUint8Array()){
    return Nan::ThrowError(Nan::New("Expected Buffer for message_raw: crypto_anon_crypt(their_vk, message_raw, cb(err, encryptedMsgRaw))").ToLocalChecked());
  }
  const indy_u8_t* arg1data = (indy_u8_t*)node::Buffer::Data(info[1]->ToObject());
  indy_u32_t arg1len = node::Buffer::Length(info[1]);
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("crypto_anon_crypt arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_crypto_anon_crypt(icb->handle, arg0, arg1data, arg1len, crypto_anon_crypt_cb));
  delete arg0UTF;
}

void crypto_anon_decrypt_cb(indy_handle_t handle, indy_error_t xerr, const indy_u8_t* arg0data, indy_u32_t arg0len) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbBuffer(xerr, arg0data, arg0len);
  }
}
NAN_METHOD(crypto_anon_decrypt) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: crypto_anon_decrypt(wallet_handle, my_vk, encrypted_msg, cb(err, decryptedMsgRaw))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: crypto_anon_decrypt(wallet_handle, my_vk, encrypted_msg, cb(err, decryptedMsgRaw))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for my_vk: crypto_anon_decrypt(wallet_handle, my_vk, encrypted_msg, cb(err, decryptedMsgRaw))").ToLocalChecked());
  }
  if(!info[2]->IsUint8Array()){
    return Nan::ThrowError(Nan::New("Expected Buffer for encrypted_msg: crypto_anon_decrypt(wallet_handle, my_vk, encrypted_msg, cb(err, decryptedMsgRaw))").ToLocalChecked());
  }
  const indy_u8_t* arg2data = (indy_u8_t*)node::Buffer::Data(info[2]->ToObject());
  indy_u32_t arg2len = node::Buffer::Length(info[2]);
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("crypto_anon_decrypt arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_crypto_anon_decrypt(icb->handle, arg0, arg1, arg2data, arg2len, crypto_anon_decrypt_cb));
  delete arg1UTF;
}

void create_and_store_my_did_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0, const char *const arg1) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringString(xerr, arg0, arg1);
  }
}
NAN_METHOD(create_and_store_my_did) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: create_and_store_my_did(wallet_handle, did_json, cb(err, [ did, verkey ]))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: create_and_store_my_did(wallet_handle, did_json, cb(err, [ did, verkey ]))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for did_json: create_and_store_my_did(wallet_handle, did_json, cb(err, [ did, verkey ]))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("create_and_store_my_did arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_create_and_store_my_did(icb->handle, arg0, arg1, create_and_store_my_did_cb));
  delete arg1UTF;
}

void replace_keys_start_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(replace_keys_start) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: replace_keys_start(wallet_handle, did, identity_json, cb(err, verkey))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: replace_keys_start(wallet_handle, did, identity_json, cb(err, verkey))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for did: replace_keys_start(wallet_handle, did, identity_json, cb(err, verkey))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for identity_json: replace_keys_start(wallet_handle, did, identity_json, cb(err, verkey))").ToLocalChecked());
  }
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("replace_keys_start arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_replace_keys_start(icb->handle, arg0, arg1, arg2, replace_keys_start_cb));
  delete arg1UTF;
  delete arg2UTF;
}

void replace_keys_apply_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(replace_keys_apply) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: replace_keys_apply(wallet_handle, did, cb(err))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: replace_keys_apply(wallet_handle, did, cb(err))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for did: replace_keys_apply(wallet_handle, did, cb(err))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("replace_keys_apply arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_replace_keys_apply(icb->handle, arg0, arg1, replace_keys_apply_cb));
  delete arg1UTF;
}

void store_their_did_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(store_their_did) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: store_their_did(wallet_handle, identity_json, cb(err))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: store_their_did(wallet_handle, identity_json, cb(err))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for identity_json: store_their_did(wallet_handle, identity_json, cb(err))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("store_their_did arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_store_their_did(icb->handle, arg0, arg1, store_their_did_cb));
  delete arg1UTF;
}

void key_for_did_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(key_for_did) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: key_for_did(pool_handle, wallet_handle, did, cb(err, key))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for pool_handle: key_for_did(pool_handle, wallet_handle, did, cb(err, key))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  if(!info[1]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: key_for_did(pool_handle, wallet_handle, did, cb(err, key))").ToLocalChecked());
  }
  indy_handle_t arg1 = info[1]->Int32Value();
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for did: key_for_did(pool_handle, wallet_handle, did, cb(err, key))").ToLocalChecked());
  }
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("key_for_did arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_key_for_did(icb->handle, arg0, arg1, arg2, key_for_did_cb));
  delete arg2UTF;
}

void key_for_local_did_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(key_for_local_did) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: key_for_local_did(wallet_handle, did, cb(err, key))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: key_for_local_did(wallet_handle, did, cb(err, key))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for did: key_for_local_did(wallet_handle, did, cb(err, key))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("key_for_local_did arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_key_for_local_did(icb->handle, arg0, arg1, key_for_local_did_cb));
  delete arg1UTF;
}

void set_endpoint_for_did_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(set_endpoint_for_did) {
  if(info.Length() != 5){
    return Nan::ThrowError(Nan::New("Expected 5 arguments: set_endpoint_for_did(wallet_handle, did, address, transport_key, cb(err))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: set_endpoint_for_did(wallet_handle, did, address, transport_key, cb(err))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for did: set_endpoint_for_did(wallet_handle, did, address, transport_key, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for address: set_endpoint_for_did(wallet_handle, did, address, transport_key, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for transport_key: set_endpoint_for_did(wallet_handle, did, address, transport_key, cb(err))").ToLocalChecked());
  }
  if(!info[4]->IsFunction()) {
    return Nan::ThrowError(Nan::New("set_endpoint_for_did arg 4 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[4]).ToLocalChecked());
  indyCalled(icb, indy_set_endpoint_for_did(icb->handle, arg0, arg1, arg2, arg3, set_endpoint_for_did_cb));
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
}

void get_endpoint_for_did_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0, const char *const arg1) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbStringString(xerr, arg0, arg1);
  }
}
NAN_METHOD(get_endpoint_for_did) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: get_endpoint_for_did(wallet_handle, pool_handle, did, cb(err, [ address, transportVk ]))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: get_endpoint_for_did(wallet_handle, pool_handle, did, cb(err, [ address, transportVk ]))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  if(!info[1]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for pool_handle: get_endpoint_for_did(wallet_handle, pool_handle, did, cb(err, [ address, transportVk ]))").ToLocalChecked());
  }
  indy_handle_t arg1 = info[1]->Int32Value();
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for did: get_endpoint_for_did(wallet_handle, pool_handle, did, cb(err, [ address, transportVk ]))").ToLocalChecked());
  }
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("get_endpoint_for_did arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_get_endpoint_for_did(icb->handle, arg0, arg1, arg2, get_endpoint_for_did_cb));
  delete arg2UTF;
}

void set_did_metadata_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(set_did_metadata) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: set_did_metadata(wallet_handle, did, metadata, cb(err))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: set_did_metadata(wallet_handle, did, metadata, cb(err))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for did: set_did_metadata(wallet_handle, did, metadata, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for metadata: set_did_metadata(wallet_handle, did, metadata, cb(err))").ToLocalChecked());
  }
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("set_did_metadata arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_set_did_metadata(icb->handle, arg0, arg1, arg2, set_did_metadata_cb));
  delete arg1UTF;
  delete arg2UTF;
}

void get_did_metadata_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(get_did_metadata) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: get_did_metadata(wallet_handle, did, cb(err, metadata))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: get_did_metadata(wallet_handle, did, cb(err, metadata))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for did: get_did_metadata(wallet_handle, did, cb(err, metadata))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("get_did_metadata arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_get_did_metadata(icb->handle, arg0, arg1, get_did_metadata_cb));
  delete arg1UTF;
}

void get_my_did_with_meta_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(get_my_did_with_meta) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: get_my_did_with_meta(wallet_handle, my_did, cb(err, didWithMeta))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: get_my_did_with_meta(wallet_handle, my_did, cb(err, didWithMeta))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for my_did: get_my_did_with_meta(wallet_handle, my_did, cb(err, didWithMeta))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("get_my_did_with_meta arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_get_my_did_with_meta(icb->handle, arg0, arg1, get_my_did_with_meta_cb));
  delete arg1UTF;
}

void list_my_dids_with_meta_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(list_my_dids_with_meta) {
  if(info.Length() != 2){
    return Nan::ThrowError(Nan::New("Expected 2 arguments: list_my_dids_with_meta(wallet_handle, cb(err, dids))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: list_my_dids_with_meta(wallet_handle, cb(err, dids))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  if(!info[1]->IsFunction()) {
    return Nan::ThrowError(Nan::New("list_my_dids_with_meta arg 1 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[1]).ToLocalChecked());
  indyCalled(icb, indy_list_my_dids_with_meta(icb->handle, arg0, list_my_dids_with_meta_cb));
}

void abbreviate_verkey_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(abbreviate_verkey) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: abbreviate_verkey(did, full_verkey, cb(err, verkey))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for did: abbreviate_verkey(did, full_verkey, cb(err, verkey))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for full_verkey: abbreviate_verkey(did, full_verkey, cb(err, verkey))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("abbreviate_verkey arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_abbreviate_verkey(icb->handle, arg0, arg1, abbreviate_verkey_cb));
  delete arg0UTF;
  delete arg1UTF;
}

void sign_and_submit_request_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(sign_and_submit_request) {
  if(info.Length() != 5){
    return Nan::ThrowError(Nan::New("Expected 5 arguments: sign_and_submit_request(pool_handle, wallet_handle, submitter_did, request_json, cb(err, requestResult))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for pool_handle: sign_and_submit_request(pool_handle, wallet_handle, submitter_did, request_json, cb(err, requestResult))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  if(!info[1]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: sign_and_submit_request(pool_handle, wallet_handle, submitter_did, request_json, cb(err, requestResult))").ToLocalChecked());
  }
  indy_handle_t arg1 = info[1]->Int32Value();
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: sign_and_submit_request(pool_handle, wallet_handle, submitter_did, request_json, cb(err, requestResult))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for request_json: sign_and_submit_request(pool_handle, wallet_handle, submitter_did, request_json, cb(err, requestResult))").ToLocalChecked());
  }
  if(!info[4]->IsFunction()) {
    return Nan::ThrowError(Nan::New("sign_and_submit_request arg 4 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[4]).ToLocalChecked());
  indyCalled(icb, indy_sign_and_submit_request(icb->handle, arg0, arg1, arg2, arg3, sign_and_submit_request_cb));
  delete arg2UTF;
  delete arg3UTF;
}

void submit_request_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(submit_request) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: submit_request(pool_handle, request_json, cb(err, requestResult))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for pool_handle: submit_request(pool_handle, request_json, cb(err, requestResult))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for request_json: submit_request(pool_handle, request_json, cb(err, requestResult))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("submit_request arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_submit_request(icb->handle, arg0, arg1, submit_request_cb));
  delete arg1UTF;
}

void sign_request_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(sign_request) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: sign_request(wallet_handle, submitter_did, request_json, cb(err, signedRequest))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: sign_request(wallet_handle, submitter_did, request_json, cb(err, signedRequest))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: sign_request(wallet_handle, submitter_did, request_json, cb(err, signedRequest))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for request_json: sign_request(wallet_handle, submitter_did, request_json, cb(err, signedRequest))").ToLocalChecked());
  }
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("sign_request arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_sign_request(icb->handle, arg0, arg1, arg2, sign_request_cb));
  delete arg1UTF;
  delete arg2UTF;
}

void build_get_ddo_request_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(build_get_ddo_request) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: build_get_ddo_request(submitter_did, target_did, cb(err, requestResult))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: build_get_ddo_request(submitter_did, target_did, cb(err, requestResult))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for target_did: build_get_ddo_request(submitter_did, target_did, cb(err, requestResult))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("build_get_ddo_request arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_build_get_ddo_request(icb->handle, arg0, arg1, build_get_ddo_request_cb));
  delete arg0UTF;
  delete arg1UTF;
}

void build_nym_request_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(build_nym_request) {
  if(info.Length() != 6){
    return Nan::ThrowError(Nan::New("Expected 6 arguments: build_nym_request(submitter_did, target_did, verkey, alias, role, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: build_nym_request(submitter_did, target_did, verkey, alias, role, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for target_did: build_nym_request(submitter_did, target_did, verkey, alias, role, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for verkey: build_nym_request(submitter_did, target_did, verkey, alias, role, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for alias: build_nym_request(submitter_did, target_did, verkey, alias, role, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg4UTF = nullptr;
  const char* arg4 = nullptr;
  if(info[4]->IsString()){
    arg4UTF = new Nan::Utf8String(info[4]);
    arg4 = (const char*)(**arg4UTF);
  } else if(!info[4]->IsNull() && !info[4]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for role: build_nym_request(submitter_did, target_did, verkey, alias, role, cb(err, request))").ToLocalChecked());
  }
  if(!info[5]->IsFunction()) {
    return Nan::ThrowError(Nan::New("build_nym_request arg 5 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[5]).ToLocalChecked());
  indyCalled(icb, indy_build_nym_request(icb->handle, arg0, arg1, arg2, arg3, arg4, build_nym_request_cb));
  delete arg0UTF;
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
  delete arg4UTF;
}

void build_attrib_request_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(build_attrib_request) {
  if(info.Length() != 6){
    return Nan::ThrowError(Nan::New("Expected 6 arguments: build_attrib_request(submitter_did, target_did, hash, raw, enc, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: build_attrib_request(submitter_did, target_did, hash, raw, enc, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for target_did: build_attrib_request(submitter_did, target_did, hash, raw, enc, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for hash: build_attrib_request(submitter_did, target_did, hash, raw, enc, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for raw: build_attrib_request(submitter_did, target_did, hash, raw, enc, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg4UTF = nullptr;
  const char* arg4 = nullptr;
  if(info[4]->IsString()){
    arg4UTF = new Nan::Utf8String(info[4]);
    arg4 = (const char*)(**arg4UTF);
  } else if(!info[4]->IsNull() && !info[4]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for enc: build_attrib_request(submitter_did, target_did, hash, raw, enc, cb(err, request))").ToLocalChecked());
  }
  if(!info[5]->IsFunction()) {
    return Nan::ThrowError(Nan::New("build_attrib_request arg 5 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[5]).ToLocalChecked());
  indyCalled(icb, indy_build_attrib_request(icb->handle, arg0, arg1, arg2, arg3, arg4, build_attrib_request_cb));
  delete arg0UTF;
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
  delete arg4UTF;
}

void build_get_attrib_request_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(build_get_attrib_request) {
  if(info.Length() != 6){
    return Nan::ThrowError(Nan::New("Expected 6 arguments: build_get_attrib_request(submitter_did, target_did, hash, raw, enc, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: build_get_attrib_request(submitter_did, target_did, hash, raw, enc, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for target_did: build_get_attrib_request(submitter_did, target_did, hash, raw, enc, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for hash: build_get_attrib_request(submitter_did, target_did, hash, raw, enc, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for raw: build_get_attrib_request(submitter_did, target_did, hash, raw, enc, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg4UTF = nullptr;
  const char* arg4 = nullptr;
  if(info[4]->IsString()){
    arg4UTF = new Nan::Utf8String(info[4]);
    arg4 = (const char*)(**arg4UTF);
  } else if(!info[4]->IsNull() && !info[4]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for enc: build_get_attrib_request(submitter_did, target_did, hash, raw, enc, cb(err, request))").ToLocalChecked());
  }
  if(!info[5]->IsFunction()) {
    return Nan::ThrowError(Nan::New("build_get_attrib_request arg 5 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[5]).ToLocalChecked());
  indyCalled(icb, indy_build_get_attrib_request(icb->handle, arg0, arg1, arg2, arg3, arg4, build_get_attrib_request_cb));
  delete arg0UTF;
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
  delete arg4UTF;
}

void build_get_nym_request_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(build_get_nym_request) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: build_get_nym_request(submitter_did, target_did, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: build_get_nym_request(submitter_did, target_did, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for target_did: build_get_nym_request(submitter_did, target_did, cb(err, request))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("build_get_nym_request arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_build_get_nym_request(icb->handle, arg0, arg1, build_get_nym_request_cb));
  delete arg0UTF;
  delete arg1UTF;
}

void build_schema_request_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(build_schema_request) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: build_schema_request(submitter_did, data, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: build_schema_request(submitter_did, data, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for data: build_schema_request(submitter_did, data, cb(err, request))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("build_schema_request arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_build_schema_request(icb->handle, arg0, arg1, build_schema_request_cb));
  delete arg0UTF;
  delete arg1UTF;
}

void build_get_schema_request_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(build_get_schema_request) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: build_get_schema_request(submitter_did, dest, data, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: build_get_schema_request(submitter_did, dest, data, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for dest: build_get_schema_request(submitter_did, dest, data, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for data: build_get_schema_request(submitter_did, dest, data, cb(err, request))").ToLocalChecked());
  }
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("build_get_schema_request arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_build_get_schema_request(icb->handle, arg0, arg1, arg2, build_get_schema_request_cb));
  delete arg0UTF;
  delete arg1UTF;
  delete arg2UTF;
}

void build_claim_def_txn_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(build_claim_def_txn) {
  if(info.Length() != 5){
    return Nan::ThrowError(Nan::New("Expected 5 arguments: build_claim_def_txn(submitter_did, xref, signature_type, data, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: build_claim_def_txn(submitter_did, xref, signature_type, data, cb(err, request))").ToLocalChecked());
  }
  if(!info[1]->IsInt32()){
    return Nan::ThrowError(Nan::New("Expected indy_i32_t for xref: build_claim_def_txn(submitter_did, xref, signature_type, data, cb(err, request))").ToLocalChecked());
  }
  indy_i32_t arg1 = info[1]->Int32Value();
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for signature_type: build_claim_def_txn(submitter_did, xref, signature_type, data, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for data: build_claim_def_txn(submitter_did, xref, signature_type, data, cb(err, request))").ToLocalChecked());
  }
  if(!info[4]->IsFunction()) {
    return Nan::ThrowError(Nan::New("build_claim_def_txn arg 4 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[4]).ToLocalChecked());
  indyCalled(icb, indy_build_claim_def_txn(icb->handle, arg0, arg1, arg2, arg3, build_claim_def_txn_cb));
  delete arg0UTF;
  delete arg2UTF;
  delete arg3UTF;
}

void build_get_claim_def_txn_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(build_get_claim_def_txn) {
  if(info.Length() != 5){
    return Nan::ThrowError(Nan::New("Expected 5 arguments: build_get_claim_def_txn(submitter_did, xref, signature_type, origin, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: build_get_claim_def_txn(submitter_did, xref, signature_type, origin, cb(err, request))").ToLocalChecked());
  }
  if(!info[1]->IsInt32()){
    return Nan::ThrowError(Nan::New("Expected indy_i32_t for xref: build_get_claim_def_txn(submitter_did, xref, signature_type, origin, cb(err, request))").ToLocalChecked());
  }
  indy_i32_t arg1 = info[1]->Int32Value();
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for signature_type: build_get_claim_def_txn(submitter_did, xref, signature_type, origin, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for origin: build_get_claim_def_txn(submitter_did, xref, signature_type, origin, cb(err, request))").ToLocalChecked());
  }
  if(!info[4]->IsFunction()) {
    return Nan::ThrowError(Nan::New("build_get_claim_def_txn arg 4 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[4]).ToLocalChecked());
  indyCalled(icb, indy_build_get_claim_def_txn(icb->handle, arg0, arg1, arg2, arg3, build_get_claim_def_txn_cb));
  delete arg0UTF;
  delete arg2UTF;
  delete arg3UTF;
}

void build_node_request_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(build_node_request) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: build_node_request(submitter_did, target_did, data, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: build_node_request(submitter_did, target_did, data, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for target_did: build_node_request(submitter_did, target_did, data, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for data: build_node_request(submitter_did, target_did, data, cb(err, request))").ToLocalChecked());
  }
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("build_node_request arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_build_node_request(icb->handle, arg0, arg1, arg2, build_node_request_cb));
  delete arg0UTF;
  delete arg1UTF;
  delete arg2UTF;
}

void build_get_txn_request_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(build_get_txn_request) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: build_get_txn_request(submitter_did, data, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: build_get_txn_request(submitter_did, data, cb(err, request))").ToLocalChecked());
  }
  if(!info[1]->IsInt32()){
    return Nan::ThrowError(Nan::New("Expected indy_i32_t for data: build_get_txn_request(submitter_did, data, cb(err, request))").ToLocalChecked());
  }
  indy_i32_t arg1 = info[1]->Int32Value();
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("build_get_txn_request arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_build_get_txn_request(icb->handle, arg0, arg1, build_get_txn_request_cb));
  delete arg0UTF;
}

void build_pool_config_request_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(build_pool_config_request) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: build_pool_config_request(submitter_did, writes, force, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: build_pool_config_request(submitter_did, writes, force, cb(err, request))").ToLocalChecked());
  }
  if(!info[1]->IsBoolean()){
    return Nan::ThrowError(Nan::New("Expected Boolean for writes: build_pool_config_request(submitter_did, writes, force, cb(err, request))").ToLocalChecked());
  }
  indy_bool_t arg1 = info[1]->IsTrue();
  if(!info[2]->IsBoolean()){
    return Nan::ThrowError(Nan::New("Expected Boolean for force: build_pool_config_request(submitter_did, writes, force, cb(err, request))").ToLocalChecked());
  }
  indy_bool_t arg2 = info[2]->IsTrue();
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("build_pool_config_request arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_build_pool_config_request(icb->handle, arg0, arg1, arg2, build_pool_config_request_cb));
  delete arg0UTF;
}

void build_pool_upgrade_request_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(build_pool_upgrade_request) {
  if(info.Length() != 11){
    return Nan::ThrowError(Nan::New("Expected 11 arguments: build_pool_upgrade_request(submitter_did, name, version, action, sha256, timeout, schedule, justification, reinstall, force, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for submitter_did: build_pool_upgrade_request(submitter_did, name, version, action, sha256, timeout, schedule, justification, reinstall, force, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for name: build_pool_upgrade_request(submitter_did, name, version, action, sha256, timeout, schedule, justification, reinstall, force, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for version: build_pool_upgrade_request(submitter_did, name, version, action, sha256, timeout, schedule, justification, reinstall, force, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for action: build_pool_upgrade_request(submitter_did, name, version, action, sha256, timeout, schedule, justification, reinstall, force, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg4UTF = nullptr;
  const char* arg4 = nullptr;
  if(info[4]->IsString()){
    arg4UTF = new Nan::Utf8String(info[4]);
    arg4 = (const char*)(**arg4UTF);
  } else if(!info[4]->IsNull() && !info[4]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for sha256: build_pool_upgrade_request(submitter_did, name, version, action, sha256, timeout, schedule, justification, reinstall, force, cb(err, request))").ToLocalChecked());
  }
  if(!info[5]->IsInt32()){
    return Nan::ThrowError(Nan::New("Expected indy_i32_t for timeout: build_pool_upgrade_request(submitter_did, name, version, action, sha256, timeout, schedule, justification, reinstall, force, cb(err, request))").ToLocalChecked());
  }
  indy_i32_t arg5 = info[5]->Int32Value();
  Nan::Utf8String* arg6UTF = nullptr;
  const char* arg6 = nullptr;
  if(info[6]->IsString()){
    arg6UTF = new Nan::Utf8String(info[6]);
    arg6 = (const char*)(**arg6UTF);
  } else if(!info[6]->IsNull() && !info[6]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for schedule: build_pool_upgrade_request(submitter_did, name, version, action, sha256, timeout, schedule, justification, reinstall, force, cb(err, request))").ToLocalChecked());
  }
  Nan::Utf8String* arg7UTF = nullptr;
  const char* arg7 = nullptr;
  if(info[7]->IsString()){
    arg7UTF = new Nan::Utf8String(info[7]);
    arg7 = (const char*)(**arg7UTF);
  } else if(!info[7]->IsNull() && !info[7]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for justification: build_pool_upgrade_request(submitter_did, name, version, action, sha256, timeout, schedule, justification, reinstall, force, cb(err, request))").ToLocalChecked());
  }
  if(!info[8]->IsBoolean()){
    return Nan::ThrowError(Nan::New("Expected Boolean for reinstall: build_pool_upgrade_request(submitter_did, name, version, action, sha256, timeout, schedule, justification, reinstall, force, cb(err, request))").ToLocalChecked());
  }
  indy_bool_t arg8 = info[8]->IsTrue();
  if(!info[9]->IsBoolean()){
    return Nan::ThrowError(Nan::New("Expected Boolean for force: build_pool_upgrade_request(submitter_did, name, version, action, sha256, timeout, schedule, justification, reinstall, force, cb(err, request))").ToLocalChecked());
  }
  indy_bool_t arg9 = info[9]->IsTrue();
  if(!info[10]->IsFunction()) {
    return Nan::ThrowError(Nan::New("build_pool_upgrade_request arg 10 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[10]).ToLocalChecked());
  indyCalled(icb, indy_build_pool_upgrade_request(icb->handle, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, build_pool_upgrade_request_cb));
  delete arg0UTF;
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
  delete arg4UTF;
  delete arg6UTF;
  delete arg7UTF;
}

void is_pairwise_exists_cb(indy_handle_t handle, indy_error_t xerr, indy_bool_t arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbBoolean(xerr, arg0);
  }
}
NAN_METHOD(is_pairwise_exists) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: is_pairwise_exists(wallet_handle, their_did, cb(err, exists))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: is_pairwise_exists(wallet_handle, their_did, cb(err, exists))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for their_did: is_pairwise_exists(wallet_handle, their_did, cb(err, exists))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("is_pairwise_exists arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_is_pairwise_exists(icb->handle, arg0, arg1, is_pairwise_exists_cb));
  delete arg1UTF;
}

void create_pairwise_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(create_pairwise) {
  if(info.Length() != 5){
    return Nan::ThrowError(Nan::New("Expected 5 arguments: create_pairwise(wallet_handle, their_did, my_did, metadata, cb(err))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: create_pairwise(wallet_handle, their_did, my_did, metadata, cb(err))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for their_did: create_pairwise(wallet_handle, their_did, my_did, metadata, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for my_did: create_pairwise(wallet_handle, their_did, my_did, metadata, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for metadata: create_pairwise(wallet_handle, their_did, my_did, metadata, cb(err))").ToLocalChecked());
  }
  if(!info[4]->IsFunction()) {
    return Nan::ThrowError(Nan::New("create_pairwise arg 4 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[4]).ToLocalChecked());
  indyCalled(icb, indy_create_pairwise(icb->handle, arg0, arg1, arg2, arg3, create_pairwise_cb));
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
}

void list_pairwise_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(list_pairwise) {
  if(info.Length() != 2){
    return Nan::ThrowError(Nan::New("Expected 2 arguments: list_pairwise(wallet_handle, cb(err, listPairwise))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: list_pairwise(wallet_handle, cb(err, listPairwise))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  if(!info[1]->IsFunction()) {
    return Nan::ThrowError(Nan::New("list_pairwise arg 1 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[1]).ToLocalChecked());
  indyCalled(icb, indy_list_pairwise(icb->handle, arg0, list_pairwise_cb));
}

void get_pairwise_cb(indy_handle_t handle, indy_error_t xerr, const char* arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(get_pairwise) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: get_pairwise(wallet_handle, their_did, cb(err, pairwiseInfo))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: get_pairwise(wallet_handle, their_did, cb(err, pairwiseInfo))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for their_did: get_pairwise(wallet_handle, their_did, cb(err, pairwiseInfo))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("get_pairwise arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_get_pairwise(icb->handle, arg0, arg1, get_pairwise_cb));
  delete arg1UTF;
}

void set_pairwise_metadata_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(set_pairwise_metadata) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: set_pairwise_metadata(wallet_handle, their_did, metadata, cb(err))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for wallet_handle: set_pairwise_metadata(wallet_handle, their_did, metadata, cb(err))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for their_did: set_pairwise_metadata(wallet_handle, their_did, metadata, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for metadata: set_pairwise_metadata(wallet_handle, their_did, metadata, cb(err))").ToLocalChecked());
  }
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("set_pairwise_metadata arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_set_pairwise_metadata(icb->handle, arg0, arg1, arg2, set_pairwise_metadata_cb));
  delete arg1UTF;
  delete arg2UTF;
}

void create_pool_ledger_config_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(create_pool_ledger_config) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: create_pool_ledger_config(config_name, config, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for config_name: create_pool_ledger_config(config_name, config, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for config: create_pool_ledger_config(config_name, config, cb(err))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("create_pool_ledger_config arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_create_pool_ledger_config(icb->handle, arg0, arg1, create_pool_ledger_config_cb));
  delete arg0UTF;
  delete arg1UTF;
}

void open_pool_ledger_cb(indy_handle_t handle, indy_error_t xerr, indy_handle_t arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbHandle(xerr, arg0);
  }
}
NAN_METHOD(open_pool_ledger) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: open_pool_ledger(config_name, config, cb(err, poolHandle))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for config_name: open_pool_ledger(config_name, config, cb(err, poolHandle))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for config: open_pool_ledger(config_name, config, cb(err, poolHandle))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("open_pool_ledger arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_open_pool_ledger(icb->handle, arg0, arg1, open_pool_ledger_cb));
  delete arg0UTF;
  delete arg1UTF;
}

void refresh_pool_ledger_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(refresh_pool_ledger) {
  if(info.Length() != 2){
    return Nan::ThrowError(Nan::New("Expected 2 arguments: refresh_pool_ledger(handle, cb(err))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for handle: refresh_pool_ledger(handle, cb(err))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  if(!info[1]->IsFunction()) {
    return Nan::ThrowError(Nan::New("refresh_pool_ledger arg 1 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[1]).ToLocalChecked());
  indyCalled(icb, indy_refresh_pool_ledger(icb->handle, arg0, refresh_pool_ledger_cb));
}

void list_pools_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(list_pools) {
  if(info.Length() != 1){
    return Nan::ThrowError(Nan::New("Expected 1 arguments: list_pools(cb(err, pools))").ToLocalChecked());
  }
  if(!info[0]->IsFunction()) {
    return Nan::ThrowError(Nan::New("list_pools arg 0 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[0]).ToLocalChecked());
  indyCalled(icb, indy_list_pools(icb->handle, list_pools_cb));
}

void close_pool_ledger_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(close_pool_ledger) {
  if(info.Length() != 2){
    return Nan::ThrowError(Nan::New("Expected 2 arguments: close_pool_ledger(handle, cb(err))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for handle: close_pool_ledger(handle, cb(err))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  if(!info[1]->IsFunction()) {
    return Nan::ThrowError(Nan::New("close_pool_ledger arg 1 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[1]).ToLocalChecked());
  indyCalled(icb, indy_close_pool_ledger(icb->handle, arg0, close_pool_ledger_cb));
}

void delete_pool_ledger_config_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(delete_pool_ledger_config) {
  if(info.Length() != 2){
    return Nan::ThrowError(Nan::New("Expected 2 arguments: delete_pool_ledger_config(config_name, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for config_name: delete_pool_ledger_config(config_name, cb(err))").ToLocalChecked());
  }
  if(!info[1]->IsFunction()) {
    return Nan::ThrowError(Nan::New("delete_pool_ledger_config arg 1 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[1]).ToLocalChecked());
  indyCalled(icb, indy_delete_pool_ledger_config(icb->handle, arg0, delete_pool_ledger_config_cb));
  delete arg0UTF;
}

void create_wallet_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(create_wallet) {
  if(info.Length() != 6){
    return Nan::ThrowError(Nan::New("Expected 6 arguments: create_wallet(pool_name, name, xtype, config, credentials, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for pool_name: create_wallet(pool_name, name, xtype, config, credentials, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for name: create_wallet(pool_name, name, xtype, config, credentials, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for xtype: create_wallet(pool_name, name, xtype, config, credentials, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg3UTF = nullptr;
  const char* arg3 = nullptr;
  if(info[3]->IsString()){
    arg3UTF = new Nan::Utf8String(info[3]);
    arg3 = (const char*)(**arg3UTF);
  } else if(!info[3]->IsNull() && !info[3]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for config: create_wallet(pool_name, name, xtype, config, credentials, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg4UTF = nullptr;
  const char* arg4 = nullptr;
  if(info[4]->IsString()){
    arg4UTF = new Nan::Utf8String(info[4]);
    arg4 = (const char*)(**arg4UTF);
  } else if(!info[4]->IsNull() && !info[4]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for credentials: create_wallet(pool_name, name, xtype, config, credentials, cb(err))").ToLocalChecked());
  }
  if(!info[5]->IsFunction()) {
    return Nan::ThrowError(Nan::New("create_wallet arg 5 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[5]).ToLocalChecked());
  indyCalled(icb, indy_create_wallet(icb->handle, arg0, arg1, arg2, arg3, arg4, create_wallet_cb));
  delete arg0UTF;
  delete arg1UTF;
  delete arg2UTF;
  delete arg3UTF;
  delete arg4UTF;
}

void open_wallet_cb(indy_handle_t handle, indy_error_t xerr, indy_handle_t arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbHandle(xerr, arg0);
  }
}
NAN_METHOD(open_wallet) {
  if(info.Length() != 4){
    return Nan::ThrowError(Nan::New("Expected 4 arguments: open_wallet(name, runtime_config, credentials, cb(err, handle))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for name: open_wallet(name, runtime_config, credentials, cb(err, handle))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for runtime_config: open_wallet(name, runtime_config, credentials, cb(err, handle))").ToLocalChecked());
  }
  Nan::Utf8String* arg2UTF = nullptr;
  const char* arg2 = nullptr;
  if(info[2]->IsString()){
    arg2UTF = new Nan::Utf8String(info[2]);
    arg2 = (const char*)(**arg2UTF);
  } else if(!info[2]->IsNull() && !info[2]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for credentials: open_wallet(name, runtime_config, credentials, cb(err, handle))").ToLocalChecked());
  }
  if(!info[3]->IsFunction()) {
    return Nan::ThrowError(Nan::New("open_wallet arg 3 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[3]).ToLocalChecked());
  indyCalled(icb, indy_open_wallet(icb->handle, arg0, arg1, arg2, open_wallet_cb));
  delete arg0UTF;
  delete arg1UTF;
  delete arg2UTF;
}

void list_wallets_cb(indy_handle_t handle, indy_error_t xerr, const char *const arg0) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbString(xerr, arg0);
  }
}
NAN_METHOD(list_wallets) {
  if(info.Length() != 1){
    return Nan::ThrowError(Nan::New("Expected 1 arguments: list_wallets(cb(err, wallets))").ToLocalChecked());
  }
  if(!info[0]->IsFunction()) {
    return Nan::ThrowError(Nan::New("list_wallets arg 0 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[0]).ToLocalChecked());
  indyCalled(icb, indy_list_wallets(icb->handle, list_wallets_cb));
}

void close_wallet_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(close_wallet) {
  if(info.Length() != 2){
    return Nan::ThrowError(Nan::New("Expected 2 arguments: close_wallet(handle, cb(err))").ToLocalChecked());
  }
  if(!info[0]->IsNumber()){
    return Nan::ThrowError(Nan::New("Expected IndyHandle for handle: close_wallet(handle, cb(err))").ToLocalChecked());
  }
  indy_handle_t arg0 = info[0]->Int32Value();
  if(!info[1]->IsFunction()) {
    return Nan::ThrowError(Nan::New("close_wallet arg 1 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[1]).ToLocalChecked());
  indyCalled(icb, indy_close_wallet(icb->handle, arg0, close_wallet_cb));
}

void delete_wallet_cb(indy_handle_t handle, indy_error_t xerr) {
  IndyCallback* icb = IndyCallback::getCallback(handle);
  if(icb != nullptr){
    icb->cbNone(xerr);
  }
}
NAN_METHOD(delete_wallet) {
  if(info.Length() != 3){
    return Nan::ThrowError(Nan::New("Expected 3 arguments: delete_wallet(name, credentials, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg0UTF = nullptr;
  const char* arg0 = nullptr;
  if(info[0]->IsString()){
    arg0UTF = new Nan::Utf8String(info[0]);
    arg0 = (const char*)(**arg0UTF);
  } else if(!info[0]->IsNull() && !info[0]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for name: delete_wallet(name, credentials, cb(err))").ToLocalChecked());
  }
  Nan::Utf8String* arg1UTF = nullptr;
  const char* arg1 = nullptr;
  if(info[1]->IsString()){
    arg1UTF = new Nan::Utf8String(info[1]);
    arg1 = (const char*)(**arg1UTF);
  } else if(!info[1]->IsNull() && !info[1]->IsUndefined()){
    return Nan::ThrowError(Nan::New("Expected String or null for credentials: delete_wallet(name, credentials, cb(err))").ToLocalChecked());
  }
  if(!info[2]->IsFunction()) {
    return Nan::ThrowError(Nan::New("delete_wallet arg 2 expected callback Function").ToLocalChecked());
  }
  IndyCallback* icb = new IndyCallback(Nan::To<v8::Function>(info[2]).ToLocalChecked());
  indyCalled(icb, indy_delete_wallet(icb->handle, arg0, arg1, delete_wallet_cb));
  delete arg0UTF;
  delete arg1UTF;
}

NAN_MODULE_INIT(InitAll) {
  Nan::Export(target, "issuer_create_and_store_claim_def", issuer_create_and_store_claim_def);
  Nan::Export(target, "issuer_create_and_store_revoc_reg", issuer_create_and_store_revoc_reg);
  Nan::Export(target, "issuer_create_claim_offer", issuer_create_claim_offer);
  Nan::Export(target, "issuer_create_claim", issuer_create_claim);
  Nan::Export(target, "issuer_revoke_claim", issuer_revoke_claim);
  Nan::Export(target, "prover_store_claim_offer", prover_store_claim_offer);
  Nan::Export(target, "prover_get_claim_offers", prover_get_claim_offers);
  Nan::Export(target, "prover_create_master_secret", prover_create_master_secret);
  Nan::Export(target, "prover_create_and_store_claim_req", prover_create_and_store_claim_req);
  Nan::Export(target, "prover_store_claim", prover_store_claim);
  Nan::Export(target, "prover_get_claims", prover_get_claims);
  Nan::Export(target, "prover_get_claims_for_proof_req", prover_get_claims_for_proof_req);
  Nan::Export(target, "prover_create_proof", prover_create_proof);
  Nan::Export(target, "verifier_verify_proof", verifier_verify_proof);
  Nan::Export(target, "create_key", create_key);
  Nan::Export(target, "set_key_metadata", set_key_metadata);
  Nan::Export(target, "get_key_metadata", get_key_metadata);
  Nan::Export(target, "crypto_sign", crypto_sign);
  Nan::Export(target, "crypto_verify", crypto_verify);
  Nan::Export(target, "crypto_auth_crypt", crypto_auth_crypt);
  Nan::Export(target, "crypto_auth_decrypt", crypto_auth_decrypt);
  Nan::Export(target, "crypto_anon_crypt", crypto_anon_crypt);
  Nan::Export(target, "crypto_anon_decrypt", crypto_anon_decrypt);
  Nan::Export(target, "create_and_store_my_did", create_and_store_my_did);
  Nan::Export(target, "replace_keys_start", replace_keys_start);
  Nan::Export(target, "replace_keys_apply", replace_keys_apply);
  Nan::Export(target, "store_their_did", store_their_did);
  Nan::Export(target, "key_for_did", key_for_did);
  Nan::Export(target, "key_for_local_did", key_for_local_did);
  Nan::Export(target, "set_endpoint_for_did", set_endpoint_for_did);
  Nan::Export(target, "get_endpoint_for_did", get_endpoint_for_did);
  Nan::Export(target, "set_did_metadata", set_did_metadata);
  Nan::Export(target, "get_did_metadata", get_did_metadata);
  Nan::Export(target, "get_my_did_with_meta", get_my_did_with_meta);
  Nan::Export(target, "list_my_dids_with_meta", list_my_dids_with_meta);
  Nan::Export(target, "abbreviate_verkey", abbreviate_verkey);
  Nan::Export(target, "sign_and_submit_request", sign_and_submit_request);
  Nan::Export(target, "submit_request", submit_request);
  Nan::Export(target, "sign_request", sign_request);
  Nan::Export(target, "build_get_ddo_request", build_get_ddo_request);
  Nan::Export(target, "build_nym_request", build_nym_request);
  Nan::Export(target, "build_attrib_request", build_attrib_request);
  Nan::Export(target, "build_get_attrib_request", build_get_attrib_request);
  Nan::Export(target, "build_get_nym_request", build_get_nym_request);
  Nan::Export(target, "build_schema_request", build_schema_request);
  Nan::Export(target, "build_get_schema_request", build_get_schema_request);
  Nan::Export(target, "build_claim_def_txn", build_claim_def_txn);
  Nan::Export(target, "build_get_claim_def_txn", build_get_claim_def_txn);
  Nan::Export(target, "build_node_request", build_node_request);
  Nan::Export(target, "build_get_txn_request", build_get_txn_request);
  Nan::Export(target, "build_pool_config_request", build_pool_config_request);
  Nan::Export(target, "build_pool_upgrade_request", build_pool_upgrade_request);
  Nan::Export(target, "is_pairwise_exists", is_pairwise_exists);
  Nan::Export(target, "create_pairwise", create_pairwise);
  Nan::Export(target, "list_pairwise", list_pairwise);
  Nan::Export(target, "get_pairwise", get_pairwise);
  Nan::Export(target, "set_pairwise_metadata", set_pairwise_metadata);
  Nan::Export(target, "create_pool_ledger_config", create_pool_ledger_config);
  Nan::Export(target, "open_pool_ledger", open_pool_ledger);
  Nan::Export(target, "refresh_pool_ledger", refresh_pool_ledger);
  Nan::Export(target, "list_pools", list_pools);
  Nan::Export(target, "close_pool_ledger", close_pool_ledger);
  Nan::Export(target, "delete_pool_ledger_config", delete_pool_ledger_config);
  Nan::Export(target, "create_wallet", create_wallet);
  Nan::Export(target, "open_wallet", open_wallet);
  Nan::Export(target, "list_wallets", list_wallets);
  Nan::Export(target, "close_wallet", close_wallet);
  Nan::Export(target, "delete_wallet", delete_wallet);
}
NODE_MODULE(indynodejs, InitAll)

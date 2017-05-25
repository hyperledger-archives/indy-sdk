//
//  AnoncredsUtils.m
//  libsovrin-demo
//
//  Created by Kirill Neznamov on 24/05/2017.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import "AnoncredsUtils.h"
#import <libsovrin/libsovrin.h>
#import "TestUtils.h"
#import "WalletUtils.h"

@implementation AnoncredsUtils

+ (AnoncredsUtils *)sharedInstance
{
    static AnoncredsUtils *instance = nil;
    static dispatch_once_t dispatch_once_block;
    
    dispatch_once(&dispatch_once_block, ^{
        instance = [AnoncredsUtils new];
    });
    
    return instance;
}

-(NSString*) getGvtSchemaJson:(NSNumber*) seqNo
{
    return [NSString stringWithFormat:@"{"\
                                       "\"name\":\"gvt\"," \
                                       "\"version\":\"1.0\"," \
                                       "\"attribute_names\":[\"age\",\"sex\",\"height\",\"name\"]," \
                                       "\"seq_no\":%d" \
                                       "}", [seqNo integerValue]
    ];
}

-(NSString*) getClaimOfferJson:(NSString*) issuerDid seqNo:(NSNumber*) claimDefSeqNo
{
    return [NSString stringWithFormat:@"{"\
            "\"issuer_did\":\"%@\"," \
            "\"claim_def_seq_no\":%d" \
            "}", issuerDid, [claimDefSeqNo integerValue]
            ];
}

-(NSString*) getGvtClaimJson
{
    return [NSString stringWithFormat:@"{"\
                                       "\"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],"\
                                       "\"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],"\
                                       "\"height\":[\"175\",\"175\"],"\
                                       "\"age\":[\"28\",\"28\"]"\
                                       "}"];
}

-(NSError*) issuerCreateClaimDefinition:(SovrinHandle) walletHandle
                                 schema:(NSString*) schema
                        outClaimDefJson:(NSString**) outClaimDefJson
                        outClaimDefUUID:(NSString**) outClaimDefUUID
{
    __block NSError *err = nil;
    XCTestExpectation* completionExpectation = nil;

    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];

    NSError *ret = [SovrinAnoncreds  issuerCreateAndStoreClaimDef:walletHandle
                                                       schemaJSON:schema
                                                    signatureType:nil
                                                   createNonRevoc:NO
                                                       completion:^(NSError *error, NSString *claimDefJSON, NSString *claimDefUUID)
    {
        err = error;
        if(claimDefJSON && outClaimDefJson)
        {
            *outClaimDefJson = [ NSString stringWithString:claimDefJSON];
        }
        if(claimDefUUID && outClaimDefUUID)
        {
            *outClaimDefUUID = [ NSString stringWithString:claimDefUUID];
        }
        [completionExpectation fulfill];
    }];
    
    if( ret.code != Success)
    {
        return ret;
    }

    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    return err;
}

-(NSError*) createClaimDefinitionAndSetLink:(SovrinHandle) walletHandle
                                     schema:(NSString*) schema
                                      seqNo:(NSNumber*) claimDefSeqNo
                                    outJson:(NSString**) outJson
{
    NSString* uuid = nil;
    
    NSError *ret = [ self issuerCreateClaimDefinition:walletHandle
                                               schema:schema
                                      outClaimDefJson:outJson
                                      outClaimDefUUID:&uuid ];
    if( ret.code != Success )
    {
        return ret;
    }
    
    ret = [[WalletUtils sharedInstance] walletSetSeqNoForValue:walletHandle
                                                  claimDefUUID:uuid
                                                 claimDefSeqNo:claimDefSeqNo];
    return ret;
}

-(NSError*) proverCreateMasterSecret:(SovrinHandle) walletHandle
                    masterSecretName:(NSString*) name
{
    __block NSError *err = nil;
    XCTestExpectation* completionExpectation = nil;
    
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    NSError *ret = [SovrinAnoncreds proverCreateMasterSecret:  walletHandle
                                            masterSecretName:  name
                                                  completion: ^(NSError *error)
    {
        err = error;
        [completionExpectation fulfill];
    }];

    if( ret.code != Success)
    {
        return ret;
    }
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    return err;
}

-(NSError*) proverStoreClaimOffer:(SovrinHandle) walletHandle
                   claimOfferJson:(NSString*) str
{
    __block NSError *err = nil;
    XCTestExpectation* completionExpectation = nil;
    
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    NSError *ret = [SovrinAnoncreds proverStoreClaimOffer:  walletHandle
                                           claimOfferJSON:  str
                                               completion: ^(NSError *error)
    {
        err = error;
        [completionExpectation fulfill];
    }];
    
    if( ret.code != Success)
    {
        return ret;
    }
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    return err;
 
}

-(NSError*) proverGetClaimOffers:(SovrinHandle) walletHandle
                      filterJson:(NSString*) filterJson
              outClaimOffersJSON:(NSString**) outJson
{
    __block NSError *err = nil;
    XCTestExpectation* completionExpectation = nil;
    
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    NSError *ret = [ SovrinAnoncreds proverGetClaimOffers: walletHandle
                                               filterJSON: filterJson
                                               completion:^(NSError *error, NSString *claimOffersJSON)
    {
        err = error;
        if(outJson)
        {
            *outJson = [ NSString stringWithString: claimOffersJSON];
        }
        [completionExpectation fulfill];
    }];

    if( ret.code != Success)
    {
        return ret;
    }
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    return err;
}

-(NSError*) proverCreateAndStoreClaimReq:(SovrinHandle) walletHandle
                               proverDid:(NSString*) pd
                          claimOfferJson:(NSString*) coj
                            claimDefJson:(NSString*) cdj
                        masterSecretName:(NSString*) name
                         outClaimReqJson:(NSString**) outJson
{
    __block NSError *err = nil;
    XCTestExpectation* completionExpectation = nil;
    
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    NSError *ret = [ SovrinAnoncreds proverCreateAndStoreClaimReq: walletHandle
                                                        proverDid: pd
                                                   claimOfferJSON: coj
                                                 masterSecretName: name
                                                     claimDefJSON: cdj
                                                       completion:^(NSError* error, NSString* claimReqJSON)
    {
        err = error;
        if(outJson)
        {
            *outJson = [ NSString stringWithString: claimReqJSON];
        }
        [completionExpectation fulfill];
    }];
    
    if( ret.code != Success)
    {
        return ret;
    }
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    return err;
}

-(NSError*) issuerCreateClaim:(SovrinHandle) walletHandle
                 claimReqJson:(NSString*) claimReqJson
                    claimJson:(NSString*) claimJson
        outRevocRegUpdateJSON:(NSString**) outRevocRegUpdateJson
                 outClaimJson:(NSString**) outClaimJson
{
    __block NSError *err = nil;
    XCTestExpectation* completionExpectation = nil;
    
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    NSError *ret = [SovrinAnoncreds issuerCreateClaim:walletHandle
                                         claimReqJSON:claimReqJson
                                            claimJSON:claimJson
                                        revocRegSeqNo:nil
                                       userRevocIndex:nil completion:^(NSError *error, NSString *revocRegUpdateJSON, NSString *claimJSON)
    {
        err = error;
        if(outRevocRegUpdateJson)
        {
            *outRevocRegUpdateJson = [NSString stringWithString:revocRegUpdateJSON];
        }
        if(claimJson)
        {
            *outClaimJson = [NSString stringWithString:claimJson];
        }
        [completionExpectation fulfill];
    }];
    
    if( ret.code != Success)
    {
        return ret;
    }
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    return err;
}

-(NSError*) proverStoreClaim:(SovrinHandle) walletHandle
                  claimsJson:(NSString*) str
{
    __block NSError *err = nil;
    XCTestExpectation* completionExpectation = nil;
    
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    NSError *ret = [SovrinAnoncreds proverStoreClaim: walletHandle claimsJSON:str completion:^(NSError *error)
    {
        err = error;
        [completionExpectation fulfill];
    }];

    if( ret.code != Success)
    {
        return ret;
    }
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    return err;
}

-(NSError*) proverGetClaimsForProofReq:(SovrinHandle) walletHandle
                      proofRequestJson:(NSString*) str
                         outClaimsJson:(NSString**) outClaimsJson
{
    __block NSError *err = nil;
    XCTestExpectation* completionExpectation = nil;
    
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];

    NSError *ret = [SovrinAnoncreds proverGetClaimsForProofReq: walletHandle
                                                  proofReqJSON: str
                                                    completion:^(NSError *error, NSString *claimsJSON)
    {
        err = error;
        if(outClaimsJson)
        {
            *outClaimsJson = [NSString stringWithString:claimsJSON];
        }
        [completionExpectation fulfill];
    }];
    
    if( ret.code != Success)
    {
        return ret;
    }
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    return err;
}

-(NSError*) proverCreateProof:(SovrinHandle) walletHandle
                 proofReqJson:(NSString*) proofReqJson
          requestedClaimsJson:(NSString*) requestedClaimsJson
                  schemasJson:(NSString*) schemasJson
             masterSecretName:(NSString*) masterSecreteName
                claimDefsJson:(NSString*) claimDefsJson
                revocRegsJson:(NSString*) revocRegsJson
                 outProofJson:(NSString**) outProofJson
{
    __block NSError *err = nil;
    XCTestExpectation* completionExpectation = nil;
    
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
   
    NSError *ret = [SovrinAnoncreds proverCreateProof:walletHandle
                                         proofReqJSON:proofReqJson
                                  requestedClaimsJSON:requestedClaimsJson
                                          schemasJSON:schemasJson
                                     masterSecretName:masterSecreteName
                                        claimDefsJSON:claimDefsJson
                                        revocRegsJSON:revocRegsJson
                                           completion:^(NSError *error, NSString *proofJSON)
    {
        err = error;
        if (outProofJson)
        {
            *outProofJson = [NSString stringWithString:proofJSON];
        }
        [completionExpectation fulfill];
    }];

    if( ret.code != Success)
    {
        return ret;
    }

    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    return err;
}
-(NSError*) verifierVerifyProof:(NSString*) proofRequestJson
                      proofJson:(NSString*) proofJson
                    schemasJson:(NSString*) schemasJson
                  claimDefsJson:(NSString*) claimDefsJson
                  revocRegsJson:(NSString*) revocRegsJson
{
    __block NSError *err = nil;
    XCTestExpectation* completionExpectation = nil;
    
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    ret = [SovrinAnoncreds verifierVerifyProof:<#(SovrinHandle)#> proofReqJSON:<#(NSString *)#> proofJSON:<#(NSString *)#> schemasJSON:<#(NSString *)#> claimDefsJSON:<#(NSString *)#> revocRegsJSON:<#(NSString *)#> completion:<#^(NSError *error, BOOL valid)handler#>]
    if( ret.code != Success)
    {
        return ret;
    }
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    return err;
    
}

pub fn verifier_verify_proof(proof_request_json: &str, proof_json: &str,
                             schemas_json: &str, claim_defs_json: &str, revoc_regs_json: &str) -> Result<bool, ErrorCode> {
    let (sender, receiver) = channel();
    
    let cb = Box::new(move |err, valid| {
        sender.send((err, valid)).unwrap();
    });
    
    let (command_handle, cb) = CallbackUtils::closure_to_verifier_verify_proof_cb(cb);
    
    let proof_request_json = CString::new(proof_request_json).unwrap();
    let proof_json = CString::new(proof_json).unwrap();
    let schemas_json = CString::new(schemas_json).unwrap();
    let claim_defs_json = CString::new(claim_defs_json).unwrap();
    let revoc_regs_json = CString::new(revoc_regs_json).unwrap();
    
    let err = sovrin_verifier_verify_proof(command_handle,
                                           proof_request_json.as_ptr(),
                                           proof_json.as_ptr(),
                                           schemas_json.as_ptr(),
                                           claim_defs_json.as_ptr(),
                                           revoc_regs_json.as_ptr(),
                                           cb);
    
    if err != ErrorCode::Success {
        return Err(err);
    }
    
    let (err, valid) = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();
    
    if err != ErrorCode::Success {
        return Err(err);
    }
    
    Ok(valid)
}
@end

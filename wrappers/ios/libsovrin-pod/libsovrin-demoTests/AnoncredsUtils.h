//
//  AnoncredsUtils.h
//  libsovrin-demo
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <libsovrin/libsovrin.h>

@interface AnoncredsUtils : XCTestCase

+ (AnoncredsUtils *)sharedInstance;

-(NSString*) getGvtSchemaJson:(NSNumber*) seqNo;

-(NSString*) getClaimOfferJson:(NSString*) issuerDid
                         seqNo:(NSNumber*) claimDefSeqNo;

-(NSString*) getGvtClaimJson;

-(NSError*) createClaimDefinitionAndSetLink:(SovrinHandle) walletHandle
                                     schema:(NSString*) schema
                                      seqNo:(NSNumber*) claimDefSeqNo
                                    outJson:(NSString**) outJson;

-(NSError*) proverCreateMasterSecret:(SovrinHandle) walletHandle
                    masterSecretName:(NSString*) name;

-(NSError*) proverStoreClaimOffer:(SovrinHandle) walletHandle
                   claimOfferJson:(NSString*) str;

-(NSError*) proverGetClaimOffers:(SovrinHandle) walletHandle
                      filterJson:(NSString*) filterJson
              outClaimOffersJSON:(NSString**) outJson;

-(NSError*) proverCreateAndStoreClaimReq:(SovrinHandle) walletHandle
                               proverDid:(NSString*) pd
                          claimOfferJson:(NSString*) coj
                            claimDefJson:(NSString*) cdj
                        masterSecretName:(NSString*) name
                         outClaimReqJson:(NSString**) outJson;

-(NSError*) issuerCreateClaim:(SovrinHandle) walletHandle
                 claimReqJson:(NSString*) claimReqJson
                    claimJson:(NSString*) claimJson
        outRevocRegUpdateJSON:(NSString**) outRevocRegUpdateJson
                 outClaimJson:(NSString**) outClaimJson;

-(NSError*) proverStoreClaim:(SovrinHandle) walletHandle
                  claimsJson:(NSString*) str;

-(NSError*) proverGetClaimsForProofReq:(SovrinHandle) walletHandle
                      proofRequestJson:(NSString*) str
                         outClaimsJson:(NSString**) outClaimsJson;
@end

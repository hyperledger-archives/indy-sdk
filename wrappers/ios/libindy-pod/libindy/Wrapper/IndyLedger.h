//
//  IndyLedger.h
//  libindy
//


#import <Foundation/Foundation.h>
#import "IndyTypes.h"

@interface IndyLedger : NSObject

+ (NSError *)signAndSubmitRequestWithWalletHandle:(IndyHandle)walletHandle
                                       poolHandle:(IndyHandle)poolHandle
                                     submitterDID:(NSString *)submitterDid
                                      requestJSON:(NSString *)request
                                       completion:(void (^)(NSError *error, NSString *requestResultJSON)) handler;

+ (NSError *)signRequestWithWalletHandle:(IndyHandle)walletHandle
                            submitterDid:(NSString *)submitterDid
                             requestJson:(NSString *)requestJson
                              completion:(void (^)(NSError *error, NSString *requestResultJSON)) handler;

+ (NSError *)submitRequestWithPoolHandle:(IndyHandle)poolHandle
                             requestJSON:(NSString *)request
                              completion:(void (^)(NSError *error, NSString *requestResultJSON)) handler;

// MARK: - Nym request

+ (NSError *)buildNymRequestWithSubmitterDid:(NSString *)submitterDid
                                   targetDID:(NSString *)targetDid
                                      verkey:(NSString *)key
                                       alias:(NSString *)alias
                                        role:(NSString *)role
                                  completion:(void (^)(NSError *error, NSString *requestJSON)) handler;

+ (NSError *)buildGetNymRequestWithSubmitterDid:(NSString *)submitterDid
                                      targetDID:(NSString *)targetDid
                                     completion:(void (^)(NSError *error, NSString *requestJSON)) handler;

// MARK: - Attribute request

+ (NSError *)buildAttribRequestWithSubmitterDid:(NSString *)submitterDid
                                      targetDID:(NSString *)targetDid
                                           hash:(NSString *)hash
                                            raw:(NSString *)raw
                                            enc:(NSString *)enc
                                     completion:(void (^)(NSError *error, NSString *requestJSON)) handler;

+ (NSError *)buildGetAttribRequestWithSubmitterDid:(NSString *)submitterDid
                                         targetDID:(NSString *)targetDid
                                              data:(NSString *)data
                                        completion:(void (^)(NSError *error, NSString *requestJSON)) handler;

// MARK: - Schema request

+ (NSError *)buildSchemaRequestWithSubmitterDid:(NSString *)submitterDid
                                           data:(NSString *)data
                                     completion:(void (^)(NSError *error, NSString *requestJSON)) handler;

+ (NSError *)buildGetSchemaRequestWithSubmitterDid:(NSString *)submitterDid
                                              dest:(NSString *)dest
                                              data:(NSString *)data
                                        completion:(void (^)(NSError *error, NSString *requestJSON)) handler;

// MARK: - ClaimDefTxn request

+ (NSError *)buildClaimDefTxnWithSubmitterDid:(NSString *)submitterDid
                                         xref:(NSString *)xref
                                signatureType:(NSString *)signatureType
                                         data:(NSString *)data
                                   completion:(void (^)(NSError *error, NSString *requestJSON)) handler;

+ (NSError *)buildGetClaimDefTxnWithSubmitterDid:(NSString *)submitterDid
                                            xref:(NSString *)xref
                                   signatureType:(NSString *)signatureType
                                          origin:(NSString *)origin
                                      completion:(void (^)(NSError *error, NSString *requestJSON)) handler;

// MARK: - Ddo request

+ (NSError *)buildGetDdoRequestWithSubmitterDid:(NSString *)submitterDid
                                      targetDID:(NSString *)targetDid
                                     completion:(void (^)(NSError *error, NSString *requestResultJSON)) handler;

// MARK: - Node request

+ (NSError *)buildNodeRequestWithSubmitterDid:(NSString *)submitterDid
                                    targetDid:(NSString *)targetDid
                                         data:(NSString *)data
                                   completion:(void (^)(NSError *error, NSString *requestJSON)) handler;

// MARK: - Txn request

+ (NSError *)buildGetTxnRequestWithSubmitterDid:(NSString *)submitterDid
                                           data:(NSNumber *)data
                                     completion:(void (^)(NSError *error, NSString *requestJSON)) handler;

@end

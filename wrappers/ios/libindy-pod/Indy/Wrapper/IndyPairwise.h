//
//  IndyPairwise.h
//  Indy
//

#import <Foundation/Foundation.h>
#import "IndyTypes.h"

@interface IndyPairwise : NSObject

/**
 
 Check if a pairwise did is stored for DID.
 
 @param theirDid Remote Did
 @param walletHandle Handle of a wallet, where pairwise DID might be found.
 @completion Completion block, returns error and exists flag.
 */
+ (NSError *)isPairwiseExistsForDid:(NSString *)theirDid
                       walletHandle:(IndyHandle)walletHandle
                         completion:(void (^)(NSError *error, BOOL exists ))completion;

/**
 Will map theirDid to myDid and store in wallet.
 
 @param theirDid Their DID
 @param myDid Pairwise did.
 @param metadata Additional information about theirDid.
 @param walletHandle Handle of wallet, where pairwise will be stored.
 @param completion Completion block, returns error.
 */
+ (NSError *)createPairwiseForTheirDid:(NSString *)theirDid
                                 myDid:(NSString *)myDid
                              metadata:(NSString *)metadata
                          walletHandle:(IndyHandle)walletHandle
                            completion:(void (^)(NSError *error))completion;

/**
 Gets list of pairwise pairs stored in wallet with provided handle.
 
 @param walletHandle Handle of wallet, where pairwise is stored.
 @param completion Completion block, returns error and a list of pairwise pairs.
 */
+ (NSError *)listPairwiseFromWalletHandle:(IndyHandle)walletHandle
                               completion:(void (^)(NSError *error, NSString * listPairwise))completion;

/**
 Get pairwise information for theirDid from wallet.

 @code
 pairwiseInfoJson format:
 {
    "my_did": my DID
    "metadata": metadata
 }
 
 @endcode

 @param theirDid Their DID.
 @param walletHandle Handle of wallet, where pairwise is stored.
 @param completion Completion block, returns error and did info, associated with theirDid.
 */
+ (NSError *)getPairwiseForTheirDid:(NSString *)theirDid
                       walletHandle:(IndyHandle)walletHandle
                         completion:(void (^)(NSError *error, NSString * pairwiseInfoJson))completion;

/**
 Store pairwise metadata for theirDid in wallet.
 
 @param metadata Meradata for theirDid.
 @param walletHandle Handle of wallet, where metadata will be stored.
 @param completion Completion block, returns error.
 */
+ (NSError *)setPairwiseMetadata:(NSString *)metadata
                     forTheirDid:(NSString *)theirDid
                    walletHandle:(IndyHandle)walletHandle
                      completion:(void (^)(NSError *error))completion;
@end

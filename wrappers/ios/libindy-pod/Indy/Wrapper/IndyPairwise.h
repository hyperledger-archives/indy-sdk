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
+ (void)isPairwiseExistsForDid:(NSString *)theirDid
                  walletHandle:(IndyHandle)walletHandle
                    completion:(void (^)(NSError *error, BOOL exists ))completion;

/**
 Will map theirDid to myDid and store in wallet.
 
 @param theirDid Their DID
 @param myDid Pairwise did. Create it by IndyDid:createAndStoreMyDid.
 @param metadata Additional information about theirDid.
 @param walletHandle Handle of wallet, where pairwise will be stored.
 @param completion Completion block, returns error.
 */
+ (void)createPairwiseForTheirDid:(NSString *)theirDid
                            myDid:(NSString *)myDid
                         metadata:(NSString *)metadata
                     walletHandle:(IndyHandle)walletHandle
                       completion:(void (^)(NSError *error))completion;

/**
 Gets list of pairwise pairs stored in wallet with provided handle.
 
 @param walletHandle Handle of wallet, where pairwise is stored.
 @param completion Completion block, returns error and a list of pairwise pairs.
 */
+ (void)listPairwiseFromWalletHandle:(IndyHandle)walletHandle
                          completion:(void (^)(NSError *error, NSString *listPairwise))completion;

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
+ (void)getPairwiseForTheirDid:(NSString *)theirDid
                  walletHandle:(IndyHandle)walletHandle
                    completion:(void (^)(NSError *error, NSString *pairwiseInfoJson))completion;

/**
 Store pairwise metadata for theirDid in wallet.
 
 @param metadata Metadata for theirDid.
 @param walletHandle Handle of wallet, where metadata will be stored.
 @param completion Completion block, returns error.
 */
+ (void)setPairwiseMetadata:(NSString *)metadata
                forTheirDid:(NSString *)theirDid
               walletHandle:(IndyHandle)walletHandle
                 completion:(void (^)(NSError *error))completion;
@end

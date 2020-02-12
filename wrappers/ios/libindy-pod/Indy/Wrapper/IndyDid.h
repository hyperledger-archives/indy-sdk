//
//  IndyDid.h
//  libindy
//


#import <Foundation/Foundation.h>
#import "IndyTypes.h"

@interface IndyDid : NSObject

/**
 Creates keys (signing and encryption keys) for a new
 DID (owned by the caller of the library).

 Identity's DID must be either explicitly provided, or taken as the first 16 bit of verkey.
 Saves the Identity DID with keys in a secured Wallet, so that it can be used to sign
 and encrypt transactions.
 
 @code
 Example didJson:
  {
      "did": string, (optional;
              if not provided and cid param is false then the first 16 bit of the verkey will be used as a new DID;
              if not provided and cid is true then the full verkey will be used as a new DID;
              if provided, then keys will be replaced - key rotation use case)
      "seed": string, (optional) Seed that allows deterministic key creation (if not set random one will be created).
                                 Can be UTF-8, base64 or hex string.
      "crypto_type": string, (optional; if not set then ed25519 curve is used;
                currently only 'ed25519' value is supported for this field)
      "cid": bool, (optional; if not set then false is used;)
      "method_name": string, (optional) method name to create fully qualified did.
  }
 @endcode
 
 @param didJson Identity information as json. See example above.
 @param walletHandle Wallet handler (created by IndyWallet::OpenWalletWithName).
 @param completion Callback that takes command result as parameter. Returns DID, verkey.
 */
+ (void)createAndStoreMyDid:(NSString *)didJson
               walletHandle:(IndyHandle)walletHandle
                 completion:(void (^)(NSError *error,
                         NSString *did,
                         NSString *verkey))completion;

/**
 Generated temporary keys (signing and encryption keys) for an existing
 DID (owned by the caller of the library). 

 @code
 IdentityJson. Identity information as json.
 Example:
  {
      "seed": string, (optional) Seed that allows deterministic key creation (if not set random one will be created).
                                 Can be UTF-8, base64 or hex string.
      "crypto_type": string, (optional; if not set then ed25519 curve is used;
                currently only 'ed25519' value is supported for this field)
  }
 @endcode
 
 @param did Existing DID (owned by the caller of the library).
 @param identityJson Identity information as json. See example above.
 @param walletHandle Wallet handle.
 @param completion Completion block, returns error, verkey.
 */
+ (void)replaceKeysStartForDid:(NSString *)did
                  identityJson:(NSString *)identityJson
                  walletHandle:(IndyHandle)walletHandle
                    completion:(void (^)(NSError *error,
                            NSString *verkey))completion;

/**
 Apply temporary keys as main for an existing DID (owned by the caller of the library).

 @param did Existing DID (owned by the caller of the library).
 @param walletHandle Wallet handle.
 @param completion Completion block, returns error.
 */
+ (void)replaceKeysApplyForDid:(NSString *)did
                  walletHandle:(IndyHandle)walletHandle
                    completion:(void (^)(NSError *error))completion;

/**
 Saves their DID for a pairwise connection in a secured Wallet, so that it can be used to verify transaction.
 Updates DID associated verkey in case DID already exists in the Wallet.

 @code
 {
         "did": string, (required)
         "verkey": string (optional, if only pk is provided),
         "crypto_type": string, (optional; if not set then ed25519 curve is used;
                currently only 'ed25519' value is supported for this field)
 }
 @endcode
 
 @param identityJSON Identity information as json. See example above.
    {
       "did": string, (required)
       "verkey": string
                   - optional is case of adding a new DID, and DID is cryptonym: did == verkey,
                   - mandatory in case of updating an existing DID
    }
 @param walletHandle Wallet handler (created by IndyWallet::OpenWalletWithName).
 @param completion Callback that takes command result as parameter.Returns error code.
 */
+ (void)storeTheirDid:(NSString *)identityJSON
         walletHandle:(IndyHandle)walletHandle
           completion:(void (^)(NSError *error))completion;

/**
  Returns ver key (key id) for the given DID.

  "keyForDid" call follow the idea that we resolve information about their DID from
  the ledger with cache in the local wallet. The "openWallet" call has freshness parameter
  that is used for checking the freshness of cached pool value.

  Note if you don't want to resolve their DID info from the ledger you can use
  "keyForLocalDid" call instead that will look only to local wallet and skip
  freshness checking.

  Note that "createAndStoreMyDid" makes similar wallet record as "createKey".
  As result we can use returned ver key in all generic crypto and messaging functions.

  @param poolHandle   The pool handle.
  @param walletHandle The wallet handle.
  @param did          The DID to resolve key.
  Returns verkey
 */
+ (void)keyForDid:(NSString *)did
       poolHandle:(IndyHandle)poolHandle
     walletHandle:(IndyHandle)walletHandle
       completion:(void (^)(NSError *error, NSString *key))completion;

/**
 Returns ver key (key id) for the given DID.

 "keyForLocalDid" call looks data stored in the local wallet only and skips freshness checking.

 Note if you want to get fresh data from the ledger you can use "keyForDid" call
 instead.

 Note that "createAndStoreMyDid" makes similar wallet record as "createKey".
 As result we can use returned ver key in all generic crypto and messaging functions.

 @param walletHandle Wallet handle (created by openWallet).
 @param did    The DID to resolve key.
 Returns the DIDs ver key (key id).
 */
+ (void)keyForLocalDid:(NSString *)did
          walletHandle:(IndyHandle)walletHandle
            completion:(void (^)(NSError *error, NSString *key))completion;


/**
  Set/replaces endpoint information for the given DID.

  @param walletHandle       The wallet.
  @param did          The DID to resolve endpoint.
  @param address      The DIDs endpoint address.
  @param transportKey The DIDs transport key (ver key, key id).
 Returns no value.
 */
+ (void)setEndpointAddress:(NSString *)address
              transportKey:(NSString *)transportKey
                    forDid:(NSString *)did
              walletHandle:(IndyHandle)walletHandle
                completion:(void (^)(NSError *error))completion;

/**
  Returns endpoint information for the given DID.

  @param walletHandle The wallet.
  @param poolHandle The pool.
  @param did  The DID to resolve endpoint.
 Returns endpoint and transportVk
 */
+ (void)getEndpointForDid:(NSString *)did
             walletHandle:(IndyHandle)walletHandle
               poolHandle:(IndyHandle)poolHandle
               completion:(void (^)(NSError *error, NSString *address, NSString *transportKey))completion;

/**
 Saves/replaces the meta information for the giving DID in the wallet.

 @param walletHandle   The wallet.
 @param did      The DID to store metadata.
 @param metadata The meta information that will be store with the DID.
 Returns no value.
 */
+ (void)setMetadata:(NSString *)metadata
             forDid:(NSString *)did
       walletHandle:(IndyHandle)walletHandle
         completion:(void (^)(NSError *error))completion;

/**
 Retrieves the meta information for the giving DID in the wallet.

 @param walletHandle The wallet.
 @param did    The DID to retrieve metadata.
 Returns metadata
 */
+ (void)getMetadataForDid:(NSString *)did
             walletHandle:(IndyHandle)walletHandle
               completion:(void (^)(NSError *error, NSString *metadata))completion;

/**
 Retrieves abbreviated verkey if it is possible otherwise return full verkey.

 @param did   DID.
 @param fullVerkey    The DIDs verification key,
 Returns the DIDs verification key in either abbreviated or full form
 */
+ (void)abbreviateVerkey:(NSString *)did
              fullVerkey:(NSString *)fullVerkey
              completion:(void (^)(NSError *error, NSString *verkey))completion;


/**
 * Retrieves the information about all DIDs stored in the wallet.
 *
 * @param walletHandle The wallet.
 * Completion returns metadata resolving to a list of dids: [{
 *     "did": string - DID stored in the wallet,
 *     "verkey": string - The DIDs transport key (ver key, key id).,
 *     "metadata": string - The meta information stored with the DID
 *   }]
 */
+ (void)listMyDidsWithMeta:(IndyHandle)walletHandle
                completion:(void (^)(NSError *error, NSString *metadata))completion;
                

/**
 Update DID stored in the wallet to make fully qualified, or to do other DID maintenance.
     - If the DID has no prefix, a prefix will be appended (prepend did:peer to a legacy did)
     - If the DID has a prefix, a prefix will be updated (migrate did:peer to did:peer-new)

 Update DID related entities stored in the wallet.

 @param  walletHandle: Wallet handle (created by open_wallet).
 @param  did: target DID stored in the wallet.
 @param  method: method to apply to the DID.

 @param completion Callback that takes command result as parameter.
 Returns did: fully qualified did
 */
+ (void)qualifyDid:(NSString *)did
            method:(NSString *)method
      walletHandle:(IndyHandle)walletHandle
        completion:(void (^)(NSError *error, NSString *fullQualifiedDid))completion;
       
@end

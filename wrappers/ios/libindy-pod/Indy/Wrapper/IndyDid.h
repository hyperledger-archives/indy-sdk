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
      "seed": string, (optional; if not provide then a random one will be created)
      "crypto_type": string, (optional; if not set then ed25519 curve is used;
                currently only 'ed25519' value is supported for this field)
      "cid": bool, (optional; if not set then false is used;)
  }
 @endcode
 
 @param didJson Identity information as json. See example above.
 @param walletHandle Wallet handler (created by IndyWallet::OpenWalletWithName).
 @param completion Callback that takes command result as parameter. Returns DID, verkey (for verification of signature) and public_key (for decryption).
 */
+ (void)createAndStoreMyDid:(NSString *)didJson
               walletHandle:(IndyHandle)walletHandle
                 completion:(void (^)(NSError *error,
                                      NSString *did,
                                      NSString *verkey)) completion;

/**
 Generates temporary keys (signing and encryption keys) for an existing DID (owned by the caller of the library).
 
 @code
 IdentityJson. Identity information as json.
 Example:
  {
      "seed": string, (optional; if not provide then a random one will be created)
      "crypto_type": string, (optional; if not set then ed25519 curve is used;
                currently only 'ed25519' value is supported for this field)
  }
 @endcode
 
 @param did Existing DID (owned by the caller of the library).
 @param identityJson Identity information as json. See example above.
 @param walletHandle Wallet handle.
 @param completion Completion block, returns error, verkey (for verification of signature) and public_key (for decryption).
 */
+ (void)replaceKeysStartForDid:(NSString *)did
                  identityJson:(NSString *)identityJson
                  walletHandle:(IndyHandle)walletHandle
                    completion:(void (^)(NSError *error,
                                         NSString *verkey)) completion;

/**
 Apply temporary keys as main for an existing DID (owned by the caller of the library).
 
 @param did Existing DID (owned by the caller of the library).
 @param walletHandle Wallet handle.
 @param completion Completion block, returns error.
 */
+ (void)replaceKeysApplyForDid:(NSString *)did
                  walletHandle:(IndyHandle)walletHandle
                    completion:(void (^)(NSError *error)) completion;

/**
 Saves their DID for a pairwise connection in a secured Wallet, so that it can be used to verify transaction.

 @code
 {
         "did": string, (required)
         "verkey": string (optional, if only pk is provided),
         "crypto_type": string, (optional; if not set then ed25519 curve is used;
                currently only 'ed25519' value is supported for this field)
 }
 @endcode
 
 @param identityJSON Identity information as json. See example above.
 @param walletHandle Wallet handler (created by IndyWallet::OpenWalletWithName).
 @param completion Callback that takes command result as parameter.Returns error code.
 */
+ (void)storeTheirDid:(NSString *)identityJSON
         walletHandle:(IndyHandle)walletHandle
           completion:(void (^)(NSError *error)) completion;

+ (void)keyForDid:(NSString *)did
       poolHandle:(IndyHandle)poolHandle
     walletHandle:(IndyHandle)walletHandle
       completion:(void (^)(NSError *error, NSString *key))completion;

+ (void)keyForLocalDid:(NSString *)did
     walletHandle:(IndyHandle)walletHandle
       completion:(void (^)(NSError *error, NSString *key))completion;

+ (void)setEndpointAddress:(NSString *)address transportKey:(NSString *)transportKey forDid:(NSString *)did walletHandle:(IndyHandle)walletHandle completion:(void (^)(NSError *error))completion;

+ (void)getEndpointForDid:(NSString *)did
             walletHandle:(IndyHandle)walletHandle
               completion:(void (^)(NSError *error, NSString *address, NSString *transportKey))completion;

+ (void)setMetadata:(NSString *)metadata
             forDid:(NSString *)did
       walletHandle:(IndyHandle)walletHandle
         completion:(void (^)(NSError *error))completion;

+ (void)getMetadataForDid:(NSString *)did
             walletHandle:(IndyHandle)walletHandle
               completion:(void (^)(NSError *error, NSString *metadata))completion;


+ (void)AbbreviateVerkey:(NSString *)did
           fullVerkey:(NSString *)fullVerkey
               completion:(void (^)(NSError *error, NSString *verkey))completion;

@end

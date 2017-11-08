//
//  IndySignus.h
//  libindy
//


#import <Foundation/Foundation.h>
#import "IndyTypes.h"

@interface IndySignus : NSObject

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

/**
 Signs a message by a signing key associated with my DID.  
 
 The DID with a signing key
 must be already created and stored in a secured wallet (see create_and_store_my_identity)
 
 @param message Message to be signed as NSData
 @param did Signing DID
 @param walletHandle Wallet handler (created by IndyWallet::OpenWalletWithName).
 @param completion Callback that takes command result as parameter. Returns a signature string.
 */
+ (void)signMessage:(NSData*)message
                did:(NSString *)did
       walletHandle:(IndyHandle)walletHandle
         completion:(void (^)(NSError *error,
                              NSData *signature)) completion;

/**
 Verify a signature created by a key associated with a DID.  
 
 If a secure wallet doesn't contain a verkey associated with the given DID,
 then verkey is read from the Ledger.
 Otherwise either an existing verkey from wallet is used (see wallet_store_their_identity),
 or it checks the Ledger (according to freshness settings set during initialization)
 whether verkey is still the same and updates verkey for the DID if needed.
 
 @param signature Signature to be verified.
 @param did DID that signed the message.
 @param message Message that was signed by did.
 @param walletHandle Wallet handler (created by IndyWallet::OpenWalletWithName).
 @param poolHandle Pool handle.
 @param completion Callback that takes command result as parameter. Returns flag valid: true - if signature is valid, false - otherwise
 */
+ (void)verifySignature:(NSData *)signature
             forMessage:(NSData *)message
                    did:(NSString *)did
           walletHandle:(IndyHandle)walletHandle
             poolHandle:(IndyHandle)poolHandle
             completion:(void (^)(NSError *error,
                                  BOOL valid)) completion;

/**
 Encrypts a message by a public key associated with a DID.
 If a secure wallet doesn't contain a public key associated with the given DID,
 then the public key is read from the Ledger.  
 
 Otherwise either an existing public key from wallet is used (see wallet_store_their_identity),
 or it checks the Ledger (according to freshness settings set during initialization)
 whether public key is still the same and updates public key for the DID if needed.

 @param walletHandle Wallet handler (created by IndyWallet::OpenWalletWithName).
 @param poolHandle Pool handle.
 @param myDid Encrypting DID
 @param did Encrypting DID
 @param message Message that is to be encrypted.
 @param completion Callback that takes command result as parameter. Returns an encrypted message and nonce.
 */
+ (void)encryptMessage:(NSData *)message
                 myDid:(NSString *)myDid
                   did:(NSString *)did
          walletHandle:(IndyHandle)walletHandle
                  pool:(IndyHandle)poolHandle
            completion:(void (^)(NSError *error,
                                 NSData *encryptedMsg,
                                 NSData *nonce)) completion;

/**
 Decrypts a message encrypted by a public key associated with my DID.
 The DID with a secret key must be already created and
 stored in a secured wallet (see wallet_create_and_store_my_identity)

 
 @param encryptedMessage Message that is to be decrypted.
 @param myDid DID
 @param did DID that signed the message
 @param nonce Nonce
 @param walletHandle Wallet handler (created by IndyWallet::OpenWalletWithName).
 @param completion Callback that takes command result as parameter. Returns decrypted message.
*/
+ (void)decryptMessage:(NSData *)encryptedMessage
                 myDid:(NSString *)myDid
                   did:(NSString *)did
                 nonce:(NSData *)nonce
          walletHandle:(IndyHandle)walletHandle
            poolHandle:(IndyHandle)poolHandle
            completion:(void (^)(NSError *error,
                                 NSData *decryptedMessage)) completion;

+ (void)encryptMessageSealed:(NSData *)message
                         did:(NSString *)did
                walletHandle:(IndyHandle)walletHandle
                        pool:(IndyHandle)poolHandle
                  completion:(void (^)(NSError *error, NSData *encryptedMsg))completion;

+ (void)decryptMessageSealed:(NSData *)encryptedMessage
                         did:(NSString *)did
                walletHandle:(IndyHandle)walletHandle
                  completion:(void (^)(NSError *error, NSData *decryptedMessage))completion;

@end

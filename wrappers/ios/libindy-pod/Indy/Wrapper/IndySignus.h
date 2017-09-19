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
 
 @param walletHandle Wallet handler (created by IndyWallet::OpenWalletWithName).
 @param didJson Identity information as json. See example above.
 @param handler Callback that takes command result as parameter. Returns DID, verkey (for verification of signature) and public_key (for decryption).

 @return Error Code
 */
+ (NSError *)createAndStoreMyDidWithWalletHandle:(IndyHandle)walletHandle
                                         didJSON:(NSString *)didJson
                                      completion:(void (^)(NSError *error,
                                                           NSString *did,
                                                           NSString *verkey,
                                                           NSString *pk)) handler;

/**
 Generated new keys (signing and encryption keys) for an existing
 DID (owned by the caller of the library).
 
 
 @code
 Example identityJSON:
 {
      "seed": string, (optional; if not provide then a random one will be created)
      "crypto_type": string, (optional; if not set then ed25519 curve is used;
                currently only 'ed25519' value is supported for this field)
 }
 @endcode
 
 @param walletHandle Wallet handler (created by IndyWallet::OpenWalletWithName).
 @param did Existing DID (owned by the caller of the library).
 @param identityJSON  Identity information as json. See example above.
 @param handler Callback that takes command result as parameter. Returns verkey (for verification of signature) and public_key (for decryption).
 
 @return Error Code
 */
+ (NSError *)replaceKeysWithWalletHandle:(IndyHandle)walletHandle
                                     did:(NSString *)did
                            identityJSON:(NSString *)identityJSON
                              completion:(void (^)(NSError *error,
                                                   NSString *verkey,
                                                   NSString *pk)) handler;

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
 
 @param walletHandle Wallet handler (created by IndyWallet::OpenWalletWithName).
 @param identityJSON Identity information as json. See example above.
 @param handler Callback that takes command result as parameter.Returns error code.

 @return Error Code
 */
+ (NSError *)storeTheirDidWithWalletHandle:(IndyHandle)walletHandle
                              identityJSON:(NSString *)identityJSON
                                completion:(void (^)(NSError *error)) handler;

/**
 Signs a message by a signing key associated with my DID.  
 
 The DID with a signing key
 must be already created and stored in a secured wallet (see create_and_store_my_identity)
 
 @param walletHandle Wallet handler (created by IndyWallet::OpenWalletWithName).
 @param did Signing DID
 @param message Message to be signed as NSData
 @param handler Callback that takes command result as parameter. Returns a signature string.

 @return Error Code
 */
+ (NSError *)signWithWalletHandle:(IndyHandle)walletHandle
                              did:(NSString *)did
                          message:(NSData*)message
                       completion:(void (^)(NSError *error, NSData *signature)) handler;

/**
 Verify a signature created by a key associated with a DID.  
 
 If a secure wallet doesn't contain a verkey associated with the given DID,
 then verkey is read from the Ledger.
 Otherwise either an existing verkey from wallet is used (see wallet_store_their_identity),
 or it checks the Ledger (according to freshness settings set during initialization)
 whether verkey is still the same and updates verkey for the DID if needed.
 
 @param walletHandle Wallet handler (created by IndyWallet::OpenWalletWithName).
 @param poolHandle Pool handle.
 @param did DID that signed the message.
 @param message Message to be signed.
 @param signature Signature to be verified.
 @param handler Callback that takes command result as parameter. Returns flag valid: true - if signature is valid, false - otherwise

 @return Error Code

 */
+ (NSError *)verifySignatureWithWalletHandle:(IndyHandle)walletHandle
                                  poolHandle:(IndyHandle)poolHandle
                                         did:(NSString *)did
                                     message:(NSData *)message
                                   signature:(NSData *)signature
                                  completion:(void (^)(NSError *error, BOOL valid)) handler;

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
 @param handler Callback that takes command result as parameter. Returns an encrypted message and nonce.
 
 @return Error Code

 */
+ (NSError *)encryptWithWalletHandle:(IndyHandle)walletHandle
                                pool:(IndyHandle)poolHandle
                               myDid:(NSString *)myDid
                                 did:(NSString *)did
                             message:(NSData *)message
                          completion:(void (^)(NSError *error, NSData *encryptedMsg, NSData *nonce)) handler;

/**
 Decrypts a message encrypted by a public key associated with my DID.
 The DID with a secret key must be already created and
 stored in a secured wallet (see wallet_create_and_store_my_identity)

 
 @param walletHandle Wallet handler (created by IndyWallet::OpenWalletWithName).
 @param myDid DID
 @param did DID that signed the message
 @param encryptedMessage Message that is to be decrypted.
 @param nonce Nonce
 @param handler Callback that takes command result as parameter. Returns decrypted message.
 
 @return Error Code
*/
+ (NSError *)decryptWithWalletHandle:(IndyHandle)walletHandle
                               myDid:(NSString *)myDid
                                 did:(NSString *)did
                    encryptedMessage:(NSData *)encryptedMessage
                               nonce:(NSData *)nonce
                          completion:(void (^)(NSError *error, NSData *decryptedMessage)) handler;

@end

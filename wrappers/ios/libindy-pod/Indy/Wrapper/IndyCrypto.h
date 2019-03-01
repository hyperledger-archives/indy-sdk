//
// Created by DSR on 03/11/2017.
// Copyright (c) 2017 Hyperledger. All rights reserved.
//

#import <Foundation/Foundation.h>
#import "IndyTypes.h"

@interface IndyCrypto : NSObject

/**
 Creates keys pair and stores in the wallet.

 @param  keyJson: Key information as json. Example:
  {
      "seed": string, (optional) Seed that allows deterministic key creation (if not set random one will be created).
                                 Can be UTF-8, base64 or hex string.
      "crypto_type": string, // Optional (if not set then ed25519 curve is used);
              Currently only 'ed25519' value is supported for this field.
  }
 @param  walletHandle: Wallet handle (created by openWallet).
 @param completion Callback that takes command result as parameter.
 Returns verkey: Ver key of generated key pair, also used as key identifier
 */
+ (void)createKey:(NSString *)keyJson
     walletHandle:(IndyHandle)walletHandle
       completion:(void (^)(NSError *error, NSString *verkey))completion;

/**
 Saves/replaces the meta information for the giving key in the wallet.

 @param  metadata: The meta information that will be store with the key.
 @param  verkey: The key (verkey, key id) to store metadata.
 @param  walletHandle: Wallet handle (created by openWallet).
 @param completion Callback that takes command result as parameter.
 */
+ (void)setMetadata:(NSString *)metadata
             forKey:(NSString *)verkey
       walletHandle:(IndyHandle)walletHandle
         completion:(void (^)(NSError *error))completion;

/**
 Retrieves the meta information for the giving key in the wallet.

 @param  verkey: The key (verkey, key id) to store metadata.
 @param  walletHandle: Wallet handle (created by open_wallet).
 @param completion Callback that takes command result as parameter.
 Returns metadata: The meta information stored with the key; Can be null if no metadata was saved for this key.
 */
+ (void)getMetadataForKey:(NSString *)key
             walletHandle:(IndyHandle)walletHandle
               completion:(void (^)(NSError *error, NSString *metadata))completion;

/**
 Signs a message with a key.

 Note to use DID keys with this function you can call keyForDid to get key id (verkey) for specific DID.

 @param  message: The message to be signed
 @param  key: Id (verkey) of my key. The key must be created by calling createKey or CreateAndStoreMyDid
 @param  walletHandle: Wallet handle (created by open_wallet).
 @param completion Callback that takes command result as parameter.
 Returns a signature string.
 */
+ (void)signMessage:(NSData *)message
                key:(NSString *)key
       walletHandle:(IndyHandle)walletHandle
         completion:(void (^)(NSError *error, NSData *signature))completion;

/**
 Verify a signature with a verkey.

 Note to use DID keys with this function you can call keyForDid to get key id (verkey) for specific DID.

 @param  signature: A signature to be verified
 @param  message: Message that has been signed
 @param  key: Verkey of signer of the message
 @param completion Callback that takes command result as parameter.
 Returns valid: true - if signature is valid, false - otherwise
 */
+ (void)verifySignature:(NSData *)signature
             forMessage:(NSData *)message
                    key:(NSString *)key
             completion:(void (^)(NSError *error, BOOL valid))completion;

/**
  **** THIS FUNCTION WILL BE DEPRECATED USE unpackMessage() INSTEAD ****
  Encrypt a message by authenticated-encryption scheme.

  Sender can encrypt a confidential message specifically for Recipient, using Sender's public key.
  Using Recipient's public key, Sender can compute a shared secret key.
  Using Sender's public key and his secret key, Recipient can compute the exact same shared secret key.
  That shared secret key can be used to verify that the encrypted message was not tampered with,
  before eventually decrypting it.

  Recipient only needs Sender's public key, the nonce and the ciphertext to peform decryption.
  The nonce doesn't have to be confidential.

  Note to use DID keys with this function you can call keyForDid to get key id (verkey)
  for specific DID.

  @param walletHandle  The wallet.
  @param myKey    id (verkey) of my key. The key must be created by calling createKey or createAndStoreMyDid
  @param theirKey id (verkey) of their key
  @param message a message to be decrypted
  Returns encrypted message as an array of bytes.
 */
+ (void)authCrypt:(NSData *)message
            myKey:(NSString *)myKey
         theirKey:(NSString *)theirKey
     walletHandle:(IndyHandle)walletHandle
       completion:(void (^)(NSError *error, NSData *encryptedMsg))completion;

/**
  **** THIS FUNCTION WILL BE DEPRECATED USE packMessage() INSTEAD ****
  Decrypt a message by authenticated-encryption scheme.

  Sender can encrypt a confidential message specifically for Recipient, using Sender's public key.
  Using Recipient's public key, Sender can compute a shared secret key.
  Using Sender's public key and his secret key, Recipient can compute the exact same shared secret key.
  That shared secret key can be used to verify that the encrypted message was not tampered with,
  before eventually decrypting it.

  Recipient only needs Sender's public key, the nonce and the ciphertext to peform decryption.
  The nonce doesn't have to be confidential.

  Note to use DID keys with this function you can call indy_key_for_did to get key id (verkey)
  for specific DID.

  @param walletHandle       The wallet.
  @param myKey  Id (verkey) of my key. The key must be created by calling createKey or createAndStoreMyDid
  @param encryptedMessage Encrypted message
  Returns sender verkey and decrypted message.
 */
+ (void)authDecrypt:(NSData *)encryptedMessage
              myKey:(NSString *)myKey
       walletHandle:(IndyHandle)walletHandle
         completion:(void (^)(NSError *error, NSString *theirKey, NSData *decryptedMessage))completion;

/**
  Encrypts a message by anonymous-encryption scheme.

  Sealed boxes are designed to anonymously send messages to a Recipient given its public key.
  Only the Recipient can decrypt these messages, using its private key.
  While the Recipient can verify the integrity of the message, it cannot verify the identity of the Sender.

  Note to use DID keys with this function you can call keyForDid to get key id (verkey)
  for specific DID.

  Note: use packMessage function for A2A goals.

  @param theirKey verkey of message recipient
  @param message a message to be encrypted
  Returns encrypted message
 */
+ (void)anonCrypt:(NSData *)message
         theirKey:(NSString *)theirKey
       completion:(void (^)(NSError *error, NSData *encryptedMsg))completion;

/**
  Decrypts a message by anonymous-encryption scheme.

  Sealed boxes are designed to anonymously send messages to a Recipient given its public key.
  Only the Recipient can decrypt these messages, using its private key.
  While the Recipient can verify the integrity of the message, it cannot verify the identity of the Sender.

  Note to use DID keys with this function you can call keyForDid to get key id (verkey)
  for specific DID.

  Note: use unpackMessage function for A2A goals.

  @param walletHandle       The wallet.
  @param myKey  Id (verkey) of my key. The key must be created by calling createKey or createAndStoreMyDid
  @param encryptedMessage encrypted message
  Returns decrypted message as an array of bytes.
 */
+ (void)anonDecrypt:(NSData *)encryptedMessage
              myKey:(NSString *)myKey
       walletHandle:(IndyHandle)walletHandle
         completion:(void (^)(NSError *error, NSData *decryptedMessage))completion;

/**
  Packs a message by encrypting the message and serializes it in a JWE-like format (Experimental)
 
  Note to use DID keys with this function you can call IndyDid.keyForDid to get key id (verkey)
  for specific DID.

  @param walletHandle  The wallet.
  @param message a message to be packed
  @param receivers   a string in the format of a json list which will contain the list of receiver's keys the message is being encrypted for.
            Example:  "[<receiver edge_agent_1 verkey>, <receiver edge_agent_2 verkey>]"
  @param sender      the sender's verkey as a string When null pointer is used in this parameter, anoncrypt is used
  
  Returns a JWE using authcrypt alg is defined below:
  {
      "protected": "b64URLencoded({
         "enc": "xsalsa20poly1305",
         "typ": "JWM/1.0",
         "alg": "Authcrypt",
         "recipients": [
             {
                 "encrypted_key": base64URLencode(libsodium.crypto_box(my_key, their_vk, cek, cek_iv))
                 "header": {
                      "kid": "base58encode(recipient_verkey)",
                      "sender" : base64URLencode(libsodium.crypto_box_seal(their_vk, base58encode(sender_vk)),
                      "iv" : base64URLencode(cek_iv)
                 }
             },
         ],
      })",
      "iv": <b64URLencode(iv)>,
      "ciphertext": b64URLencode(encrypt_detached({'@type'...}, protected_value_encoded, iv, cek),
      "tag": <b64URLencode(tag)>
  }
 
  Alternative example in using anoncrypt alg is defined below:
  {
      "protected": "b64URLencoded({
         "enc": "xsalsa20poly1305",
         "typ": "JWM/1.0",
         "alg": "Anoncrypt",
         "recipients": [
             {
                 "encrypted_key": base64URLencode(libsodium.crypto_box_seal(their_vk, cek)),
                 "header": {
                     "kid": base58encode(recipient_verkey),
                 }
             },
         ],
      })",
      "iv": b64URLencode(iv),
      "ciphertext": b64URLencode(encrypt_detached({'@type'...}, protected_value_encoded, iv, cek),
      "tag": b64URLencode(tag)
  }
 */
+ (void)packMessage:(NSData *)message
          receivers:(NSString *)receivers
             sender:(NSString *)sender
       walletHandle:(IndyHandle)walletHandle
         completion:(void (^)(NSError *error, NSData *jwe))completion;


/**
  Unpacks a JWE-like formatted message outputted by packMessage (Experimental)

  @param walletHandle  The wallet.
  @param jwe a JWE to be unpacked
  
  Returns 
  if authcrypt was used to pack the message returns this json structure:
  {
      message: <decrypted message>,
      sender_verkey: <sender_verkey>,
      recipient_verkey: <recipient_verkey>
  }
 
  OR
 
  if anoncrypt was used to pack the message returns this json structure:
  {
      message: <decrypted message>,
      recipient_verkey: <recipient_verkey>
  }
 */
+ (void)unpackMessage:(NSData *)jwe
       walletHandle:(IndyHandle)walletHandle
         completion:(void (^)(NSError *error, NSData *res))completion;

@end
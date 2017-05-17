//
//  SovrinSignus.h
//  libsovrin
//


#import <Foundation/Foundation.h>
#import "SovrinTypes.h"

@interface SovrinSignus : NSObject

+ (NSError*) createAndStoreMyDid:(SovrinHandle) walletHandle
                         didJSON:(NSString*) didJson
                      completion:(void (^)(NSError* error, NSString* did, NSString* verkey, NSString* pk)) handler;

+ (NSError*) replaceKeys:(SovrinHandle) walletHandle
                     did:(NSString*) did
            identityJSON:(NSString*) json
              completion:(void (^)(NSError* error, NSString* verkey, NSString* pk)) handler;

+ (NSError*) storeTheirDid:(SovrinHandle) walletHandle
              identityJSON:(NSString*) json
                completion:(void (^)(NSError* error)) handler;

+ (NSError*) sign:(SovrinHandle) walletHandle
              did:(NSString*) did
              msg:(NSString*) msg
       completion:(void (^)(NSError* error, NSString* signature)) handler;

+ (NSError*) verifySignature:(SovrinHandle) walletHandle
                        pool:(SovrinHandle) poolHandle
                         did:(NSString*) did
                         msg:(NSString*) msg
                   signature:(NSString*) signature
                  completion:(void (^)(NSError* error, BOOL valid)) handler;

+ (NSError*) encrypt:(SovrinHandle) walletHandle
                pool:(SovrinHandle) poolHandle
               myDid:(NSString*) myDid
                 did:(NSString*) did
                 msg:(NSString*) msg
          completion:(void (^)(NSError* error, NSString* encryptedMsg, NSString* nonce)) handler;

+ (NSError*) decrypt:(SovrinHandle) walletHandle
               myDid:(NSString*) myDid
                 did:(NSString*) did
        encryptedMsg:(NSString*) msg
               nonce:(NSString*) nonce
          completion:(void (^)(NSError* error, NSString* decryptedMsg)) handler;


@end

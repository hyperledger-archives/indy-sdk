//
//  SovrinSignus.m
//  libsovrin
//

#import "SovrinSignus.h"
#import "SovrinCallbacks.h"
#import "sovrin_core.h"
#import "NSError+SovrinError.h"

@implementation SovrinSignus

+ (NSError*) createAndStoreMyDid:(SovrinHandle) walletHandle
                         didJSON:(NSString*) didJson
                      completion:(void (^)(NSError* error, NSString* did, NSString* verkey, NSString* pk)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] add: (void*) handler];
    
    ret = sovrin_create_and_store_my_did( handle,
                                          walletHandle,
                                          [didJson UTF8String],
                                          SovrinWrapperCommon5PCallback );
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] remove: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
    
}

+ (NSError*) replaceKeys:(SovrinHandle) walletHandle
                     did:(NSString*) did
            identityJSON:(NSString*) json
              completion:(void (^)(NSError* error, NSString* verkey, NSString* pk)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] add: (void*) handler];
    
    ret = sovrin_replace_keys( handle,
                               walletHandle,
                               [did UTF8String],
                               [json UTF8String],
                               SovrinWrapperCommon4PCallback
                              );
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] remove: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) storeTheirDid:(SovrinHandle) walletHandle
              identityJSON:(NSString*) json
                completion:(void (^)(NSError* error)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] add: (void*) handler];
    
    ret = sovrin_store_their_did( handle,
                                  walletHandle,
                                  [json UTF8String],
                                  SovrinWrapperCommon2PCallback
                                );
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] remove: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) sign:(SovrinHandle) walletHandle
              did:(NSString*) did
              msg:(NSString*) msg
       completion:(void (^)(NSError* error, NSString* signature)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] add: (void*) handler];
    
    ret = sovrin_sign( handle,
                       walletHandle,
                       [did UTF8String],
                       [msg UTF8String],
                       SovrinWrapperCommon3PSCallback
                     );
    
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] remove: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) verifySignature:(SovrinHandle) walletHandle
                        pool:(SovrinHandle) poolHandle
                         did:(NSString*) did
                         msg:(NSString*) msg
                   signature:(NSString*) signature
                  completion:(void (^)(NSError* error, BOOL valid)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] add: (void*) handler];
    
    ret = sovrin_verify_signature( handle,
                                   walletHandle,
                                   poolHandle,
                                   [did UTF8String],
                                   [msg UTF8String],
                                   [signature UTF8String],
                                   SovrinWrapperCommon3PBCallback
                                 );

    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] remove: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) encrypt:(SovrinHandle) walletHandle
                pool:(SovrinHandle) poolHandle
               myDid:(NSString*) myDid
                 did:(NSString*) did
                 msg:(NSString*) msg
          completion:(void (^)(NSError* error, NSString* encryptedMsg, NSString* nonce)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] add: (void*) handler];
    
    ret = sovrin_encrypt( handle,
                          walletHandle,
                          poolHandle,
                          [myDid UTF8String],
                          [did UTF8String],
                          [msg UTF8String],
                          SovrinWrapperCommon4PCallback
                        );
    
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] remove: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) decrypt:(SovrinHandle) walletHandle
               myDid:(NSString*) myDid
                 did:(NSString*) did
        encryptedMsg:(NSString*) msg
               nonce:(NSString*) nonce
          completion:(void (^)(NSError* error, NSString* decryptedMsg)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] add: (void*) handler];
    
    ret = sovrin_decrypt( handle,
                          walletHandle,
                          [myDid UTF8String],
                          [did UTF8String],
                          [msg UTF8String],
                          [nonce UTF8String],
                          SovrinWrapperCommon3PSCallback
                        );
    
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] remove: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

@end

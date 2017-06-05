//
//  SovrinLedger.m
//  libsovrin
//


#import "SovrinLedger.h"
#import "SovrinCallbacks.h"
#import "sovrin_core.h"
#import "NSError+SovrinError.h"

@implementation SovrinLedger


+ (NSError*) signAndSubmitRequest:(SovrinHandle) walletHandle
                       poolHandle:(SovrinHandle) poolHandle
                     submitterDID:(NSString*) submitterDid
                      requestJSON:(NSString*) request
                       completion:(void (^)(NSError* error, NSString* requestResultJSON)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    
    ret = sovrin_sign_and_submit_request( handle,
                                          walletHandle,
                                          [submitterDid UTF8String],
                                          [request UTF8String],
                                          SovrinWrapperCommon3PSCallback );
    
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
    
}

+ (NSError*) submitRequest:(SovrinHandle) poolHandle
               requestJSON:(NSString*) request
                completion:(void (^)(NSError* error, NSString* requestResultJSON)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];

    ret = sovrin_submit_request( handle,
                                 poolHandle,
                                 [request UTF8String],
                                 SovrinWrapperCommon3PSCallback );
    
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) buildGetDdoRequest:(NSString*) submitterDid
                      targetDID:(NSString*) targetDid
                     completion:(void (^)(NSError* error, NSString* requestResultJSON)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = sovrin_build_get_ddo_request( handle,
                                        [submitterDid UTF8String],
                                        [targetDid UTF8String],
                                        SovrinWrapperCommon3PSCallback );
    
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }

    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) buildNymRequest:(NSString*) submitterDid
                   targetDID:(NSString*) targetDid
                      verkey:(NSString*) key
                        xref:(NSString*) ref
                        data:(NSString*) data
                        role:(NSString*) role
                  completion:(void (^)(NSError* error, NSString* requestJSON)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = sovrin_build_nym_request( handle,
                                    [submitterDid UTF8String],
                                    [targetDid UTF8String],
                                    [key UTF8String],
                                    [ref UTF8String],
                                    [data UTF8String],
                                    [role UTF8String],
                                    SovrinWrapperCommon3PSCallback );

    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) buildAttribRequest:(NSString*) submitterDid
                      targetDID:(NSString*) targetDid
                           hash:(NSString*) hash
                            raw:(NSString*) raw
                            enc:(NSString*) enc
                     completion:(void (^)(NSError* error, NSString* requestJSON)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = sovrin_build_attrib_request( handle,
                                       [submitterDid UTF8String],
                                       [targetDid UTF8String],
                                       [hash UTF8String],
                                       [raw UTF8String],
                                       [enc UTF8String],
                                       SovrinWrapperCommon3PSCallback );
    
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) buildGetAttribRequest:(NSString*) submitterDid
                         targetDID:(NSString*) targetDid
                              data:(NSString*) data
                        completion:(void (^)(NSError* error, NSString* requestJSON)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = sovrin_build_get_attrib_request(handle,
                                          [submitterDid UTF8String],
                                          [targetDid UTF8String],
                                          [data UTF8String],
                                          SovrinWrapperCommon3PSCallback);
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) buildGetNymRequest:(NSString*) submitterDid
                      targetDID:(NSString*) targetDid
                     completion:(void (^)(NSError* error, NSString* requestJSON)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = sovrin_build_get_nym_request( handle,
                                       [submitterDid UTF8String],
                                       [targetDid UTF8String],
                                       SovrinWrapperCommon3PSCallback );
    
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) buildSchemaRequest:(NSString*) submitterDid
                           data:(NSString*) data
                     completion:(void (^)(NSError* error, NSString* requestJSON)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = sovrin_build_schema_request( handle,
                                       [submitterDid UTF8String],
                                       [data UTF8String],
                                       SovrinWrapperCommon3PSCallback );
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) buildGetSchemaRequest:(NSString*) submitterDid
                              dest:(NSString*) dest
                              data:(NSString*) data
                        completion:(void (^)(NSError* error, NSString* requestJSON)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
 
    ret = sovrin_build_get_schema_request( handle,
                                           [submitterDid UTF8String],
                                           [dest UTF8String],
                                           [data UTF8String],
                                           SovrinWrapperCommon3PSCallback );
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) buildClaimDefTxn:(NSString*) submitterDid
                         xref:(NSString*) xref
                         data:(NSString*) data
                   completion:(void (^)(NSError* error, NSString* requestJSON)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = sovrin_build_claim_def_txn( handle,
                                      [submitterDid UTF8String],
                                      [xref UTF8String],
                                      [data UTF8String],
                                      SovrinWrapperCommon3PSCallback );
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}


+ (NSError*) buildGetClaimDefTxn:(NSString*) submitterDid
                            xref:(NSString*) xref
                   signatureType:(NSString *) signatureType
                          origin:(NSString *) origin
                      completion:(void (^)(NSError* error, NSString* requestJSON)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = sovrin_build_get_claim_def_txn(handle,
                                         [submitterDid UTF8String],
                                         [xref UTF8String],
                                         [signatureType UTF8String],
                                         [origin UTF8String],
                                         SovrinWrapperCommon3PSCallback);
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) buildNodeRequest:(NSString*) submitterDid
                    targetDid:(NSString*) targetDid
                         data:(NSString*) data
                   completion:(void (^)(NSError* error, NSString* requestJSON)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = sovrin_build_node_request( handle,
                                     [submitterDid UTF8String],
                                     [targetDid UTF8String],
                                     [data UTF8String],
                                     SovrinWrapperCommon3PSCallback );
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}


@end

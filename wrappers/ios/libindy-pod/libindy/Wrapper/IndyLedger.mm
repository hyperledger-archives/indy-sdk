//
//  IndyLedger.m
//  libindy
//


#import "IndyLedger.h"
#import "IndyCallbacks.h"
#import "indy_core.h"
#import "NSError+IndyError.h"

@implementation IndyLedger


+ (NSError *)signAndSubmitRequestWithWalletHandle:(IndyHandle)walletHandle
                                       poolHandle:(IndyHandle)poolHandle
                                     submitterDID:(NSString *)submitterDid
                                      requestJSON:(NSString *)request
                                       completion:(void (^)(NSError *error, NSString *requestResultJSON)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    
    ret = indy_sign_and_submit_request(handle,
                                       poolHandle,
                                       walletHandle,
                                       [submitterDid UTF8String],
                                       [request UTF8String],
                                       IndyWrapperCommon3PSCallback );
    
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

+ (NSError *)signRequestWithWalletHandle:(IndyHandle)walletHandle
                            submitterDid:(NSString *)submitterDid
                             requestJson:(NSString *)requestJson
                              completion:(void (^)(NSError *error, NSString *requestResultJSON)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    
    ret = indy_sign_request(handle,
                            walletHandle,
                            [submitterDid UTF8String],
                            [requestJson UTF8String],
                            IndyWrapperCommon3PSCallback);
    
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

+ (NSError *)submitRequestWithPoolHandle:(IndyHandle)poolHandle
                             requestJSON:(NSString *)request
                              completion:(void (^)(NSError *error, NSString *requestResultJSON)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];

    ret = indy_submit_request(handle,
                              poolHandle,
                              [request UTF8String],
                              IndyWrapperCommon3PSCallback );
    
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

// MARK: - Nym request

+ (NSError *)buildNymRequestWithSubmitterDid:(NSString *)submitterDid
                                   targetDID:(NSString *)targetDid
                                      verkey:(NSString *)key
                                       alias:(NSString *)alias
                                        role:(NSString *)role
                                  completion:(void (^)(NSError *error, NSString *requestJSON)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    
    
    ret = indy_build_nym_request(handle,
                                 [submitterDid UTF8String],
                                 [targetDid UTF8String],
                                 [key UTF8String],
                                 [alias UTF8String],
                                 [role UTF8String],
                                 IndyWrapperCommon3PSCallback);
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

+ (NSError *)buildGetNymRequestWithSubmitterDid:(NSString *)submitterDid
                                      targetDID:(NSString *)targetDid
                                     completion:(void (^)(NSError *error, NSString *requestJSON)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    
    ret = indy_build_get_nym_request(handle,
                                     [submitterDid UTF8String],
                                     [targetDid UTF8String],
                                     IndyWrapperCommon3PSCallback );
    
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

// MARK: - Attribute request

+ (NSError *)buildAttribRequestWithSubmitterDid:(NSString *)submitterDid
                                      targetDID:(NSString *)targetDid
                                           hash:(NSString *)hash
                                            raw:(NSString *)raw
                                            enc:(NSString *)enc
                                     completion:(void (^)(NSError *error, NSString *requestJSON)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    
    ret = indy_build_attrib_request( handle,
                                    [submitterDid UTF8String],
                                    [targetDid UTF8String],
                                    [hash UTF8String],
                                    [raw UTF8String],
                                    [enc UTF8String],
                                    IndyWrapperCommon3PSCallback );
    
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

+ (NSError *)buildGetAttribRequestWithSubmitterDid:(NSString *)submitterDid
                                         targetDID:(NSString *)targetDid
                                              data:(NSString *)data
                                        completion:(void (^)(NSError *error, NSString *requestJSON)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    
    ret = indy_build_get_attrib_request(handle,
                                        [submitterDid UTF8String],
                                        [targetDid UTF8String],
                                        [data UTF8String],
                                        IndyWrapperCommon3PSCallback);
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

// MARK: - Schema request

+ (NSError *)buildSchemaRequestWithSubmitterDid:(NSString *)submitterDid
                                           data:(NSString *)data
                                     completion:(void (^)(NSError *error, NSString *requestJSON)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    
    ret = indy_build_schema_request( handle,
                                    [submitterDid UTF8String],
                                    [data UTF8String],
                                    IndyWrapperCommon3PSCallback );
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

+ (NSError *)buildGetSchemaRequestWithSubmitterDid:(NSString *)submitterDid
                                              dest:(NSString *)dest
                                              data:(NSString *)data
                                        completion:(void (^)(NSError *error, NSString *requestJSON)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    
 
    ret = indy_build_get_schema_request( handle,
                                        [submitterDid UTF8String],
                                        [dest UTF8String],
                                        [data UTF8String],
                                        IndyWrapperCommon3PSCallback );
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

// MARK: - ClaimDefTxn request

+ (NSError *)buildClaimDefTxnWithSubmitterDid:(NSString *)submitterDid
                                         xref:(NSString *)xref
                                signatureType:(NSString *)signatureType
                                         data:(NSString *)data
                                   completion:(void (^)(NSError *error, NSString *requestJSON)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    
    ret = indy_build_claim_def_txn( handle,
                                   [submitterDid UTF8String],
                                   [xref UTF8String],
                                   [signatureType UTF8String],
                                   [data UTF8String],
                                   IndyWrapperCommon3PSCallback );
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}


+ (NSError *)buildGetClaimDefTxnWithSubmitterDid:(NSString *)submitterDid
                                            xref:(NSString *)xref
                                   signatureType:(NSString *)signatureType
                                          origin:(NSString *)origin
                                      completion:(void (^)(NSError *error, NSString *requestJSON)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    
    NSString *xrefStr;
    if ([xref isKindOfClass:[NSNumber class]])
    {
        xrefStr = [(NSNumber *)xref stringValue];
    }
    else
    {
        xrefStr = xref;
    }
    
    ret = indy_build_get_claim_def_txn(handle,
                                       [submitterDid UTF8String],
                                       [xrefStr UTF8String],
                                       [signatureType UTF8String],
                                       [origin UTF8String],
                                       IndyWrapperCommon3PSCallback);
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

// MARK: - Ddo request

+ (NSError *)buildGetDdoRequestWithSubmitterDid:(NSString *)submitterDid
                                      targetDID:(NSString *)targetDid
                                     completion:(void (^)(NSError *error, NSString *requestResultJSON)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    
    ret = indy_build_get_ddo_request( handle,
                                     [submitterDid UTF8String],
                                     [targetDid UTF8String],
                                     IndyWrapperCommon3PSCallback );
    
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

// MARK: - Node request

+ (NSError *)buildNodeRequestWithSubmitterDid:(NSString *)submitterDid
                                    targetDid:(NSString *)targetDid
                                         data:(NSString *)data
                                   completion:(void (^)(NSError *error, NSString *requestJSON)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    
    ret = indy_build_node_request( handle,
                                  [submitterDid UTF8String],
                                  [targetDid UTF8String],
                                  [data UTF8String],
                                  IndyWrapperCommon3PSCallback );
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

// MARK: - Txn request

+ (NSError *)buildGetTxnRequestWithSubmitterDid:(NSString *)submitterDid
                                           data:(NSNumber *)data
                                     completion:(void (^)(NSError *error, NSString *requestJSON)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    
    ret = indy_build_get_txn_request(handle,
                                     [submitterDid UTF8String],
                                     [data intValue],
                                     IndyWrapperCommon3PSCallback);
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

@end

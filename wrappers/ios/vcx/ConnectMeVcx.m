//
//  init.m
//  vcx
//
//  Created by GuestUser on 4/30/18.
//  Copyright © 2018 GuestUser. All rights reserved.
//

#import <Foundation/Foundation.h>
#import "ConnectMeVcx.h"
#import "utils/NSError+VcxError.h"
#import "utils/VcxCallbacks.h"
#import "vcx.h"
#include "vcx.h"

void VcxWrapperCommonCallback(vcx_command_handle_t xcommand_handle,
                              vcx_error_t err) {
    id block = [[VcxCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[VcxCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];
    
    void (^completion)(NSError *) = (void (^)(NSError *)) block;
    
    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromVcxError:err];
            completion(error);
        });
    }
}

void VcxWrapperCommonHandleCallback(vcx_command_handle_t xcommand_handle,
                                    vcx_error_t err,
                                    vcx_command_handle_t pool_handle) {
    id block = [[VcxCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[VcxCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];
    
    void (^completion)(NSError *, VcxHandle) = (void (^)(NSError *, VcxHandle)) block;
    
    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromVcxError:err];
            completion(error, (VcxHandle) pool_handle);
        });
    }
}

void VcxWrapperCommonNumberCallback(vcx_command_handle_t xcommand_handle,
                                    vcx_error_t err,
                                    int32_t handle) {
    id block = [[VcxCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[VcxCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];
    
    void (^completion)(NSError *, NSNumber *) = (void (^)(NSError *, NSNumber *arg1)) block;
    NSNumber *sarg1 = @(handle);
    
    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromVcxError:err];
            completion(error, (NSNumber *) sarg1);
        });
    }
}

void VcxWrapperCommonStringCallback(vcx_command_handle_t xcommand_handle,
                                    vcx_error_t err,
                                    const char *const arg1) {
    id block = [[VcxCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[VcxCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];
    
    void (^completion)(NSError *, NSString *) = (void (^)(NSError *, NSString *arg1)) block;
    NSString *sarg1 = [NSString stringWithUTF8String:arg1];
    
    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromVcxError:err];
            completion(error, sarg1);
        });
    }
}

void VcxWrapperCommonBoolCallback(vcx_command_handle_t xcommand_handle,
                                  vcx_error_t err,
                                  unsigned int arg1) {
    id block = [[VcxCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[VcxCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];
    
    void (^completion)(NSError *, BOOL) = (void (^)(NSError *, BOOL arg1)) block;
    
    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromVcxError:err];
            completion(error, (BOOL) arg1);
        });
    }
}

void VcxWrapperCommonStringStringCallback(vcx_command_handle_t xcommand_handle,
                                          vcx_error_t err,
                                          const char *const arg1,
                                          const char *const arg2) {
    id block = [[VcxCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[VcxCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];
    
    void (^completion)(NSError *, NSString *arg1, NSString *arg2) = (void (^)(NSError *, NSString *arg1, NSString *arg2)) block;
    
    NSString *sarg1 = [NSString stringWithUTF8String:arg1];
    NSString *sarg2 = [NSString stringWithUTF8String:arg2];
    
    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromVcxError:err];
            completion(error, sarg1, sarg2);
        });
    }
}

void VcxWrapperCommonStringOptStringCallback(vcx_command_handle_t xcommand_handle,
                                             vcx_error_t err,
                                             const char *const arg1,
                                             const char *const arg2) {
    id block = [[VcxCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[VcxCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];
    
    void (^completion)(NSError *, NSString *arg1, NSString *arg2) = (void (^)(NSError *, NSString *arg1, NSString *arg2)) block;
    
    NSString *sarg1 = [NSString stringWithUTF8String:arg1];
    NSString *sarg2 = nil;
    if (arg1) {
        sarg2 = [NSString stringWithUTF8String:arg2];
    }
    
    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromVcxError:err];
            completion(error, sarg1, sarg2);
        });
    }
}

void VcxWrapperCommonStringOptStringOptStringCallback(vcx_command_handle_t xcommand_handle,
                                                      vcx_error_t err,
                                                      const char *const arg1,
                                                      const char *const arg2,
                                                      const char *const arg3) {
    id block = [[VcxCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[VcxCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];
    
    void (^completion)(NSError *, NSString *arg1, NSString *arg2, NSString *arg3) = (void (^)(NSError *, NSString *arg1, NSString *arg2, NSString *arg3)) block;
    
    NSString *sarg1 = [NSString stringWithUTF8String:arg1];
    NSString *sarg2 = nil;
    if (arg2) {
        sarg2 = [NSString stringWithUTF8String:arg2];
    }
    NSString *sarg3 = nil;
    if (arg3) {
        sarg3 = [NSString stringWithUTF8String:arg3];
    }
    
    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromVcxError:err];
            completion(error, sarg1, sarg2, sarg3);
        });
    }
}

void VcxWrapperCommonStringStringStringCallback(vcx_command_handle_t xcommand_handle,
                                                vcx_error_t err,
                                                const char *const arg1,
                                                const char *const arg2,
                                                const char *const arg3) {
    id block = [[VcxCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[VcxCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];
    
    void (^completion)(NSError *, NSString *arg1, NSString *arg2, NSString *arg3) = (void (^)(NSError *, NSString *arg1, NSString *arg2, NSString *arg3)) block;
    
    NSString *sarg1 = [NSString stringWithUTF8String:arg1];
    NSString *sarg2 = [NSString stringWithUTF8String:arg2];
    NSString *sarg3 = [NSString stringWithUTF8String:arg3];
    
    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromVcxError:err];
            completion(error, sarg1, sarg2, sarg3);
        });
    }
}

/// Arguments arg1 and arg2 will be converted to nsdata
void VcxWrapperCommonDataCallback(vcx_command_handle_t xcommand_handle,
                                  vcx_error_t err,
                                  const uint8_t *const arg1,
                                  uint32_t arg2) {
    id block = [[VcxCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[VcxCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];
    
    void (^completion)(NSError *, NSData *arg) = (void (^)(NSError *, NSData *arg)) block;
    
    NSData *sarg = [NSData dataWithBytes:arg1 length:arg2];
    
    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromVcxError:err];
            completion(error, sarg);
        });
    }
}

void VcxWrapperCommonStringDataCallback(vcx_command_handle_t xcommand_handle,
                                        vcx_error_t err,
                                        const char *const arg1,
                                        const uint8_t *const arg2,
                                        uint32_t arg3) {
    id block = [[VcxCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[VcxCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];
    
    void (^completion)(NSError *, NSString *, NSData *) = (void (^)(NSError *, NSString *, NSData *)) block;
    
    NSString *sarg1 = nil;
    if (arg1) {
        sarg1 = [NSString stringWithUTF8String:arg1];
    }
    NSData *sarg2 = [NSData dataWithBytes:arg2 length:arg3];
    
    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromVcxError:err];
            completion(error, sarg1, sarg2);
        });
    }
}

void VcxWrapperCommonStringStringLongCallback(vcx_command_handle_t xcommand_handle,
                                              vcx_error_t err,
                                              const char *arg1,
                                              const char *arg2,
                                              unsigned long long arg3) {
    id block = [[VcxCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[VcxCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];
    
    void (^completion)(NSError *, NSString *, NSString *, NSNumber *) = (void (^)(NSError *, NSString *arg1, NSString *arg2, NSNumber *arg3)) block;
    NSString *sarg1 = [NSString stringWithUTF8String:arg1];
    NSString *sarg2 = [NSString stringWithUTF8String:arg2];
    NSNumber *sarg3 = [NSNumber numberWithInt:arg3];
    
    
    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromVcxError:err];
            completion(error, (NSString *) sarg1, (NSString *) sarg2, (NSNumber *) sarg3);
        });
    }
}

void VcxWrapperCommonNumberStringCallback(vcx_command_handle_t xcommand_handle,
                                          vcx_error_t err,
                                          unsigned int handle,
                                          const char *const arg2
                                          ) {
    id block = [[VcxCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[VcxCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];
    
    void (^completion)(NSError *, NSNumber *arg1, NSString *arg2) = (void (^)(NSError *, NSNumber *arg1, NSString *arg2)) block;
    
    NSNumber *sarg1 = @(handle);
    NSString *sarg2 = [NSString stringWithUTF8String:arg2];
    
    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromVcxError:err];
            completion(error, (NSNumber *) sarg1, (NSString *) sarg2);
        });
    }
}


@implementation ConnectMeVcx

- (void)initWithConfig:(NSString *)config 
completion:(void (^)(NSError *error))completion
{
    const char *config_char = [config cString];
    vcx_command_handle_t handle= [[VcxCallbacks sharedInstance] createCommandHandleFor:completion] ;
    vcx_error_t ret = vcx_init_with_config(handle, config_char, VcxWrapperCommonCallback);
    if( ret != 0 )
    {
        [[VcxCallbacks sharedInstance] deleteCommandHandleFor: handle];
        
        dispatch_async(dispatch_get_main_queue(), ^{
            NSLog(@"agentProvision: calling completion");
            completion([NSError errorFromVcxError: ret]);
        });
    }
    
}

- (void)agentProvisionAsync:(NSString *)config 
               completion:(void (^)(NSError *error, NSString *config))completion
{
    const char *config_char = [config cString];
    vcx_command_handle_t handle= [[VcxCallbacks sharedInstance] createCommandHandleFor:completion] ;
    vcx_error_t ret = vcx_agent_provision_async(handle, config_char, VcxWrapperCommonStringCallback);
    if( ret != 0 )
    {
        [[VcxCallbacks sharedInstance] deleteCommandHandleFor: handle];
        
        dispatch_async(dispatch_get_main_queue(), ^{
            NSLog(@"agentProvision: calling completion");
            completion([NSError errorFromVcxError: ret], false);
        });
    }
    
}

- (void)connectionCreateWithInvite:(NSString *)invitationId
                inviteDetails:(NSString *)inviteDetails
             completion:(void (^)(NSError *error, NSInteger connectionHandle)) completion
{
   vcx_error_t ret;

   vcx_command_handle_t handle = [[VcxCallbacks sharedInstance] createCommandHandleFor:completion];
   const char *invitationId_char = [invitationId cString];
   const char *inviteDetails_char = [inviteDetails cString];
   ret = vcx_connection_create_with_invite(handle, invitationId_char, inviteDetails_char, VcxWrapperCommonHandleCallback);
   if( ret != 0 )
   {
       [[VcxCallbacks sharedInstance] deleteCommandHandleFor: handle];

       dispatch_async(dispatch_get_main_queue(), ^{
           completion([NSError errorFromVcxError: ret], 0);
       });
   }
}

- (void)connectionConnect: (NSInteger) connectionHandle
        connectionType: (NSString *) connectionType
            completion: (void (^)(NSError *error, NSString *inviteDetails)) completion
{
   vcx_error_t ret;
   
   vcx_command_handle_t handle = [[VcxCallbacks sharedInstance] createCommandHandleFor:completion];
   const char *connectionType_char = [connectionType cString];
    ret = vcx_connection_connect(handle, connectionHandle, connectionType_char, VcxWrapperCommonStringCallback);
   if( ret != 0 )
   {
       [[VcxCallbacks sharedInstance] deleteCommandHandleFor: handle];
       
       dispatch_async(dispatch_get_main_queue(), ^{
           completion([NSError errorFromVcxError: ret], nil);
       });
   }
}
- (void)connectionSerialize:(NSInteger)connectionHandle
                  completion:(void (^)(NSError *error, NSString *serializedConnection))completion{
    vcx_error_t ret;
   vcx_command_handle_t handle = [[VcxCallbacks sharedInstance] createCommandHandleFor:completion];
   
    ret = vcx_connection_serialize(handle, connectionHandle, VcxWrapperCommonStringCallback);

   if( ret != 0 )
   {
       [[VcxCallbacks sharedInstance] deleteCommandHandleFor: handle];
       
       dispatch_async(dispatch_get_main_queue(), ^{
           completion([NSError errorFromVcxError: ret],nil);
       });
   }
}
- (void)connectionDeserialize:(NSString *)serializedConnection
                    completion:(void (^)(NSError *error, NSInteger connectionHandle))completion{
   vcx_error_t ret;
   vcx_command_handle_t handle = [[VcxCallbacks sharedInstance] createCommandHandleFor:completion];
    const char *serialized_connection=[serializedConnection cString];
    ret = vcx_connection_deserialize(handle, serialized_connection, VcxWrapperCommonHandleCallback);

   if( ret != 0 )
   {
       [[VcxCallbacks sharedInstance] deleteCommandHandleFor: handle];
       
       dispatch_async(dispatch_get_main_queue(), ^{
           completion([NSError errorFromVcxError: ret],0);
       });
   }
}
- (void)agentUpdateInfo: (NSString *) config
            completion: (void (^)(NSError *error)) completion
{
   vcx_error_t ret;
   
   vcx_command_handle_t handle = [[VcxCallbacks sharedInstance] createCommandHandleFor:completion];
   const char *config_char = [config cString];
    ret = vcx_agent_update_info(handle, config_char, VcxWrapperCommonCallback);
   if( ret != 0 )
   {
       [[VcxCallbacks sharedInstance] deleteCommandHandleFor: handle];
       
       dispatch_async(dispatch_get_main_queue(), ^{
           completion([NSError errorFromVcxError: ret]);
       });
   }
}

- (void)getCredential:(NSInteger)credentialHandle
           completion:(void (^)(NSError *error, NSString *credential))completion{
   vcx_error_t ret;
   vcx_command_handle_t handle = [[VcxCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = vcx_get_credential(handle, credentialHandle, VcxWrapperCommonStringCallback);

   if( ret != 0 )
   {
       [[VcxCallbacks sharedInstance] deleteCommandHandleFor: handle];
       
       dispatch_async(dispatch_get_main_queue(), ^{
           completion([NSError errorFromVcxError: ret], nil);
       });
   }
}

- (void)credentialCreateWithOffer:(NSString *)sourceId
            offer:(NSString *)credentialOffer
           completion:(void (^)(NSError *error, NSInteger credentialHandle))completion{
   vcx_error_t ret;
   vcx_command_handle_t handle = [[VcxCallbacks sharedInstance] createCommandHandleFor:completion];
   const char * credential_offer=[credentialOffer cString];
   const char * source_id = [sourceId cString];
    ret = vcx_credential_create_with_offer(handle, source_id,credential_offer, VcxWrapperCommonNumberCallback);

   if( ret != 0 )
   {
       [[VcxCallbacks sharedInstance] deleteCommandHandleFor: handle];
       
       dispatch_async(dispatch_get_main_queue(), ^{
           completion([NSError errorFromVcxError: ret], 0);
       });
   }
}
- (void)credentialCreateWithMsgid:(NSString *)sourceId
                 connectionHandle:(VcxHandle)connectionHandle
                            msgId:(NSString *)msgId
                       completion:(void (^)(NSError *error, NSInteger credentialHandle, NSString *credentialOffer))completion;{
   vcx_error_t ret;
   vcx_command_handle_t handle = [[VcxCallbacks sharedInstance] createCommandHandleFor:completion];
   const char * source_id = [sourceId cString];
   const char * msg_id=[msgId cString];
    ret = vcx_credential_create_with_msgid(handle, source_id,connectionHandle,msg_id, VcxWrapperCommonNumberStringCallback);

   if( ret != 0 )
   {
       [[VcxCallbacks sharedInstance] deleteCommandHandleFor: handle];
       
       dispatch_async(dispatch_get_main_queue(), ^{
           completion([NSError errorFromVcxError: ret], 0, nil);
       });
   }
}

- (void)credentialSendRequest:(NSInteger)credentialHandle
             connectionHandle:(VcxHandle)connectionHandle
                   completion:(void (^)(NSError *error, NSString *credential))completion{
    vcx_error_t ret;
   vcx_command_handle_t handle = [[VcxCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = vcx_credential_send_request(handle, credentialHandle,connectionHandle, VcxWrapperCommonCallback);

   if( ret != 0 )
   {
       [[VcxCallbacks sharedInstance] deleteCommandHandleFor: handle];
       
       dispatch_async(dispatch_get_main_queue(), ^{
           completion([NSError errorFromVcxError: ret],nil);
       });
   }
}
- (void)credentialGetState:(NSInteger)credentialHandle
                completion:(void (^)(NSError *error, NSInteger state))completion{
   vcx_error_t ret;
   vcx_command_handle_t handle = [[VcxCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = vcx_credential_get_state(handle, credentialHandle, VcxWrapperCommonNumberCallback);

   if( ret != 0 )
   {
       [[VcxCallbacks sharedInstance] deleteCommandHandleFor: handle];
       
       dispatch_async(dispatch_get_main_queue(), ^{
           completion([NSError errorFromVcxError: ret],0);
       });
   }
}

- (void)credentialUpdateState:(NSInteger)credentialHandle
                completion:(void (^)(NSError *error,NSInteger state))completion{
   vcx_error_t ret;
   vcx_command_handle_t handle = [[VcxCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = vcx_credential_update_state(handle, credentialHandle, VcxWrapperCommonNumberCallback);

   if( ret != 0 )
   {
       [[VcxCallbacks sharedInstance] deleteCommandHandleFor: handle];
       
       dispatch_async(dispatch_get_main_queue(), ^{
           completion([NSError errorFromVcxError: ret],nil);
       });
   }
}

- (void)credentialGetOffers:(VcxHandle)connectionHandle
                   completion:(void (^)(NSError *error, NSString *offers))completion{
   vcx_error_t ret;
   vcx_command_handle_t handle = [[VcxCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = vcx_credential_get_offers(handle,connectionHandle, VcxWrapperCommonStringCallback);

   if( ret != 0 )
   {
       [[VcxCallbacks sharedInstance] deleteCommandHandleFor: handle];
       
       dispatch_async(dispatch_get_main_queue(), ^{
           completion([NSError errorFromVcxError: ret],nil);
       });
   }
}
- (void)generateProof:(NSString *)proofRequestId
       requestedAttrs:(NSString *)requestedAttrs
  requestedPredicates:(NSString *)requestedPredicates
            proofName:(NSString *)proofName
           completion:(void (^)(NSError *error, NSString *proofHandle))completion;
{
   vcx_error_t ret;

   vcx_command_handle_t handle = [[VcxCallbacks sharedInstance] createCommandHandleFor:completion];
   const char *proofRequestId_char = [proofRequestId cString];
   const char *requestedAttrs_char = [requestedAttrs cString];
   const char *requestedPredicates_char = [requestedPredicates cString];
   const char *proofName_char = [proofName cString];
    ret = vcx_proof_create(handle, proofRequestId_char, requestedAttrs_char, requestedPredicates_char, proofName_char, VcxWrapperCommonStringCallback);
   if( ret != 0 )
   {
       [[VcxCallbacks sharedInstance] deleteCommandHandleFor: handle];

       dispatch_async(dispatch_get_main_queue(), ^{
           completion([NSError errorFromVcxError: ret], nil);
       });
   }
}

- (void)credentialSerialize:(NSInteger)credentialHandle
                  completion:(void (^)(NSError *error, NSString *state))completion{
    vcx_error_t ret;
   vcx_command_handle_t handle = [[VcxCallbacks sharedInstance] createCommandHandleFor:completion];
   
    ret = vcx_credential_serialize(handle, credentialHandle, VcxWrapperCommonStringCallback);

   if( ret != 0 )
   {
       [[VcxCallbacks sharedInstance] deleteCommandHandleFor: handle];
       
       dispatch_async(dispatch_get_main_queue(), ^{
           completion([NSError errorFromVcxError: ret],nil);
       });
   }
}
- (void)credentialDeserialize:(NSString *)serializedCredential
                    completion:(void (^)(NSError *error, NSInteger credentialHandle))completion{
   vcx_error_t ret;
   vcx_command_handle_t handle = [[VcxCallbacks sharedInstance] createCommandHandleFor:completion];
    const char *serialized_credential=[serializedCredential cString];
    ret = vcx_credential_deserialize(handle, serialized_credential, VcxWrapperCommonNumberCallback);

   if( ret != 0 )
   {
       [[VcxCallbacks sharedInstance] deleteCommandHandleFor: handle];
       
       dispatch_async(dispatch_get_main_queue(), ^{
           completion([NSError errorFromVcxError: ret],nil);
       });
   }
}
@end

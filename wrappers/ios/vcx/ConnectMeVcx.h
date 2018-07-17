//
//  init.h
//  vcx
//
//  Created by GuestUser on 4/30/18.
//  Copyright © 2018 GuestUser. All rights reserved.
//

#ifndef init_h
#define init_h
#import "libvcx.h"

extern void VcxWrapperCommonCallback(vcx_command_handle_t xcommand_handle,
                                     vcx_error_t err);

extern void VcxWrapperCommonHandleCallback(vcx_command_handle_t xcommand_handle,
                                           vcx_error_t err,
                                           vcx_command_handle_t pool_handle);

extern void VcxWrapperCommonStringCallback(vcx_command_handle_t xcommand_handle,
                                           vcx_error_t err,
                                           const char *const arg1);

extern void VcxWrapperCommonBoolCallback(vcx_command_handle_t xcommand_handle,
                                         vcx_error_t err,
                                         unsigned int arg1);

extern void VcxWrapperCommonStringStringCallback(vcx_command_handle_t xcommand_handle,
                                                 vcx_error_t err,
                                                 const char *const arg1,
                                                 const char *const arg2);

extern void VcxWrapperCommonStringOptStringCallback(vcx_command_handle_t xcommand_handle,
                                                    vcx_error_t err,
                                                    const char *const arg1,
                                                    const char *const arg2);

extern void VcxWrapperCommonDataCallback(vcx_command_handle_t xcommand_handle,
                                         vcx_error_t err,
                                         const uint8_t *const arg1,
                                         uint32_t arg2);

extern void VcxWrapperCommonStringStringStringCallback(vcx_command_handle_t xcommand_handle,
                                                       vcx_error_t err,
                                                       const char *const arg1,
                                                       const char *const arg2,
                                                       const char *const arg3);

extern void VcxWrapperCommonStringDataCallback(vcx_command_handle_t xcommand_handle,
                                               vcx_error_t err,
                                               const char *const arg1,
                                               const uint8_t *const arg2,
                                               uint32_t arg3);

extern void VcxWrapperCommonNumberCallback(vcx_command_handle_t xcommand_handle,
                                           vcx_error_t err,
                                           vcx_command_handle_t handle);

extern void VcxWrapperCommonStringOptStringOptStringCallback(vcx_command_handle_t xcommand_handle,
                                                             vcx_error_t err,
                                                             const char *const arg1,
                                                             const char *const arg2,
                                                             const char *const arg3);

extern void VcxWrapperCommonStringStringLongCallback(vcx_command_handle_t xcommand_handle,
                                                     vcx_error_t err,
                                                     const char *arg1,
                                                     const char *arg2,
                                                     unsigned long long arg3);

extern void VcxWrapperCommonNumberStringCallback(vcx_command_handle_t xcommand_handle,
                                                 vcx_error_t err,
                                                 vcx_command_handle_t handle,
                                                 const char *const arg2);

@interface ConnectMeVcx : NSObject

- (void)initWithConfig:(NSString *)config
            completion:(void (^)(NSError *error))completion;

- (void)agentProvisionAsync:(NSString *)config
                 completion:(void (^)(NSError *error, NSString *config))completion;

- (void)connectionCreateWithInvite:(NSString *)invitationId
                     inviteDetails:(NSString *)inviteDetails
                        completion:(void (^)(NSError *error, NSInteger connectionHandle))completion;

- (void)connectionConnect:(VcxHandle)connectionHandle
           connectionType:(NSString *)connectionType
               completion:(void (^)(NSError *error, NSString *inviteDetails))completion;

- (void)connectionSerialize:(NSInteger)connectionHandle
                 completion:(void (^)(NSError *error, NSString *serializedConnection))completion;

- (void)connectionDeserialize:(NSString *)serializedConnection
                   completion:(void (^)(NSError *error, NSInteger connectionHandle))completion;

- (void)agentUpdateInfo:(NSString *)config
             completion:(void (^)(NSError *error))completion;

- (void)getCredential:(NSInteger )credentailHandle
           completion:(void (^)(NSError *error, NSString *credential))completion;

- (void)credentialCreateWithOffer:(NSString *)sourceId
                            offer:(NSString *)credentialOffer
                       completion:(void (^)(NSError *error, NSInteger credentailHandle))completion;

- (void)credentialCreateWithMsgid:(NSString *)sourceId
                 connectionHandle:(VcxHandle)connectionHandle
                            msgId:(NSString *)msgId
                       completion:(void (^)(NSError *error, NSInteger credentialHandle, NSString *credentialOffer))completion;

- (void)credentialSendRequest:(NSInteger)credentialHandle
             connectionHandle:(VcxHandle)connectionHandle
                paymentHandle:(vcx_payment_handle_t)paymentHandle
                   completion:(void (^)(NSError *error))completion;

- (void)credentialGetState:(NSInteger )credentialHandle
                completion:(void (^)(NSError *error, NSInteger state))completion;

- (void)credentialUpdateState:(NSInteger )credentailHandle
                completion:(void (^)(NSError *error, NSInteger state))completion;

- (void)credentialGetOffers:(VcxHandle)connectionHandle
                 completion:(void (^)(NSError *error, NSString *offers))completion;

- (void)credentialSerialize:(NSInteger)credentialHandle
                 completion:(void (^)(NSError *error, NSString *state))completion;

- (void)credentialDeserialize:(NSString *)serializedCredential
                   completion:(void (^)(NSError *error, NSInteger credentialHandle))completion;

- (void)generateProof:(NSString *)proofRequestId
         requestedAttrs:(NSString *)requestedAttrs
    requestedPredicates:(NSString *)requestedPredicates
              proofName:(NSString *)proofName
             completion:(void (^)(NSError *error, NSString *proofHandle))completion;

- (void)exportWallet:(NSString *)exportPath
            encryptWith:(NSString *)encryptionKey
           completion:(void (^)(NSError *error, NSInteger exportHandle))completion;

- (void)importWallet:(NSString *)importPath
            decryptWith:(NSString *)decryptionKey
           completion:(void (^)(NSError *error, NSInteger importHandle))completion;

@end

#endif /* init_h */

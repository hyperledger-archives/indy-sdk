//
//  SovrinCallbacks.h
//  libsovrin
//


#import <Foundation/Foundation.h>
#import "sovrin_core.h"

extern void SovrinWrapperCommon2PCallback(sovrin_handle_t xcommand_handle,
                                          sovrin_error_t err);

extern void SovrinWrapperCommon3PHCallback(sovrin_handle_t xcommand_handle,
                                           sovrin_error_t err,
                                           sovrin_handle_t pool_handle);

extern void SovrinWrapperCommon3PSCallback(sovrin_handle_t xcommand_handle,
                                           sovrin_error_t err,
                                           const char* arg1);

extern void SovrinWrapperCommon3PBCallback(sovrin_handle_t xcommand_handle,
                                           sovrin_error_t err,
                                           sovrin_bool_t arg1);

extern void SovrinWrapperCommon4PCallback(sovrin_handle_t xcommand_handle,
                                          sovrin_error_t err,
                                          const char* arg1,
                                          const char *arg2);

extern void SovrinWrapperCommon5PCallback(sovrin_handle_t xcommand_handle,
                                          sovrin_error_t err,
                                          const char* arg1,
                                          const char *arg2,
                                          const char *arg3);

extern void SovrinWrapperCommon5PSCallback(sovrin_handle_t xcommand_handle,
                                           sovrin_error_t err,
                                           sovrin_handle_t connection_handle,
                                           const char* arg1,
                                           const char *arg2);

extern void SovrinWrapperCommonAgentOutgoingConnectionCallback(sovrin_handle_t xcommand_handle,
                                                               sovrin_error_t  err,
                                                               sovrin_handle_t connection_handle);

extern void SovrinWrapperCommonAgentMessageCallback(sovrin_handle_t xconnection_handle,
                                                    sovrin_error_t  err,
                                                    const char *    message);

extern void SovrinWrapperCloseConnectionCallback(sovrin_handle_t xcommand_handle,
                                                 sovrin_error_t err);

extern void SovrinWrapperCommonAgentListenerCallback(sovrin_handle_t xcommand_handle,
                                                     sovrin_error_t  err,
                                                     sovrin_handle_t listener_handle);


extern void SovrinWrapperCommonAgentListenerConnectionCallback(sovrin_handle_t xlistener_handle,
                                                               sovrin_error_t  err,
                                                               sovrin_handle_t connection_handle,
                                                               const char *    sender_did,
                                                               const char *    receiver_did);

extern void SovrinWrapperCommonAgentListenerMessageCallback(sovrin_handle_t xconnection_handle,
                                                            sovrin_error_t  err,
                                                            const char *    message);

@interface SovrinCallbacks : NSObject

- (sovrin_handle_t) createCommandHandleFor:(void*) cb;

- (sovrin_handle_t) createCommandHandleFor:(void *)callback
                       withMessageCallback:(void *)messageCallback;

- (sovrin_handle_t) createCommandHandleFor:(void *)callback
                      withConnectionHandle:(sovrin_handle_t)connectionHandle;

- (sovrin_handle_t) createCommandHandleFor:(void *)callback
                    withConnectionCallback:(void *)connectionCallback
                        andMessageCallback:(void *)messageCallback;

- (void)            deleteCommandHandleFor:(sovrin_handle_t) handle;
- (void)            forgetListenHandle:(sovrin_handle_t) listenHandle;

+ (SovrinCallbacks*) sharedInstance;

@end

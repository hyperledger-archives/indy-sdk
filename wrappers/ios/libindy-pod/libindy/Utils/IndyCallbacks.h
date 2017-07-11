//
//  IndyCallbacks.h
//  libindy
//


#import <Foundation/Foundation.h>
#import "indy_core.h"

extern void IndyWrapperCommon2PCallback(indy_handle_t xcommand_handle,
                                          indy_error_t err);

extern void IndyWrapperCommon3PHCallback(indy_handle_t xcommand_handle,
                                           indy_error_t err,
                                           indy_handle_t pool_handle);

extern void IndyWrapperCommon3PSCallback(indy_handle_t xcommand_handle,
                                           indy_error_t err,
                                           const char* arg1);

extern void IndyWrapperCommon3PBCallback(indy_handle_t xcommand_handle,
                                           indy_error_t err,
                                           indy_bool_t arg1);

extern void IndyWrapperCommon4PCallback(indy_handle_t xcommand_handle,
                                          indy_error_t err,
                                          const char* arg1,
                                          const char *arg2);

extern void IndyWrapperCommon5PCallback(indy_handle_t xcommand_handle,
                                          indy_error_t err,
                                          const char* arg1,
                                          const char *arg2,
                                          const char *arg3);

extern void IndyWrapperCommon5PSCallback(indy_handle_t xcommand_handle,
                                           indy_error_t err,
                                           indy_handle_t connection_handle,
                                           const char* arg1,
                                           const char *arg2);

extern void IndyWrapperCommonAgentOutgoingConnectionCallback(indy_handle_t xcommand_handle,
                                                               indy_error_t  err,
                                                               indy_handle_t connection_handle);

extern void IndyWrapperCommonAgentMessageCallback(indy_handle_t xconnection_handle,
                                                    indy_error_t  err,
                                                    const char *    message);

extern void IndyWrapperCloseConnectionCallback(indy_handle_t xcommand_handle,
                                                 indy_error_t err);

extern void IndyWrapperCommonAgentListenerCallback(indy_handle_t xcommand_handle,
                                                     indy_error_t  err,
                                                     indy_handle_t listener_handle);


extern void IndyWrapperCommonAgentListenerConnectionCallback(indy_handle_t xlistener_handle,
                                                               indy_error_t  err,
                                                               indy_handle_t connection_handle,
                                                               const char *    sender_did,
                                                               const char *    receiver_did);

extern void IndyWrapperCommonAgentListenerMessageCallback(indy_handle_t xconnection_handle,
                                                            indy_error_t  err,
                                                            const char *    message);

@interface IndyCallbacks : NSObject

- (indy_handle_t) createCommandHandleFor:(void*) cb;

- (indy_handle_t) createCommandHandleFor:(void *)callback
                       withMessageCallback:(void *)messageCallback;

- (indy_handle_t) createCommandHandleFor:(void *)callback
                      withConnectionHandle:(indy_handle_t)connectionHandle;

- (indy_handle_t)createCommandHandleForListenerCallback:(void *)listenerCallback
                                   withConnectionCallback:(void *)connectionCallback
                                       andMessageCallback:(void *)messageCallback;

- (void)            deleteCommandHandleFor:(indy_handle_t) handle;
- (void)            forgetListenHandle:(indy_handle_t) listenHandle;

+ (IndyCallbacks*) sharedInstance;

@end

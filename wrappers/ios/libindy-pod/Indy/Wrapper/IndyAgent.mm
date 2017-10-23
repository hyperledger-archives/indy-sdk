//
//  IndyAgent.m
//  libindy
//

#import "IndyAgent.h"
#import "IndyCallbacks.h"
#import "indy_core.h"
#import "NSError+IndyError.h"

@implementation IndyAgent

+ (void)connectSenderDid:(NSString *)senderDid
         withReceiverDid:(NSString *)receiverDid
              poolHandle:(IndyHandle)poolHandle
            walletHandle:(IndyHandle)walletHandle
       connectionHandler:(void (^)(NSError *error, IndyHandle connectionHandle)) connectionHandler
          messageHandler:(void (^)(IndyHandle connectionHandle, NSError *error, NSString *message))messageHandler
{
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:  connectionHandler
                                                              withMessageCallback:  messageHandler];
    
    ret = indy_agent_connect(handle,
                             poolHandle,
                             walletHandle,
                             [senderDid UTF8String],
                             [receiverDid UTF8String],
                             IndyWrapperCommonAgentOutgoingConnectionCallback,
                             IndyWrapperCommonAgentMessageCallback);
    
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
        
        dispatch_async(dispatch_get_main_queue(), ^{
            connectionHandler([NSError errorFromIndyError: ret], 0);
        });
    }
}

+ (void)listenForEndpoint:(NSString *)endpoint
          listenerHandler:(void (^)(NSError *error,
                                    IndyHandle listenerHandle))listenerCompletion
        connectionHandler:(void (^)(IndyHandle xlistenerHandle,
                                    NSError *error,
                                    IndyHandle connectionHandle,
                                    NSString *senderDid,
                                    NSString *receiverDid))connectionCompletion
           messageHandler:(void (^)(IndyHandle xconnectionHandle,
                                    NSError *error,
                                    NSString *message))messageCompletion
{
    indy_error_t ret;
    indy_handle_t listener_handle = [[IndyCallbacks sharedInstance] createCommandHandleForListenerCallback:listenerCompletion
                                                                                        withConnectionCallback:connectionCompletion
                                                                                            andMessageCallback:messageCompletion];
    
    ret = indy_agent_listen(listener_handle,
                            [endpoint UTF8String],
                            IndyWrapperCommonAgentListenerCallback,
                            IndyWrapperCommonAgentListenerConnectionCallback,
                            IndyWrapperCommonAgentListenerMessageCallback);
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: listener_handle];
        
        dispatch_async(dispatch_get_main_queue(), ^{
            listenerCompletion([NSError errorFromIndyError: ret], 0);
        });
    }
}

+ (void)addIdentity:(NSString *)did
  forListenerHandle:(IndyHandle)listenerHandle
         poolHandle:(IndyHandle)poolHandle
       walletHandle:(IndyHandle)walletHandle
         completion:(void (^)(NSError *error)) completion
{
    indy_error_t ret;
    
    indy_handle_t commandHandle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    
    ret = indy_agent_add_identity(commandHandle,
                                  listenerHandle,
                                  poolHandle,
                                  walletHandle,
                                  [did UTF8String],
                                  IndyWrapperCommon2PCallback);
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: commandHandle];
        
        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError: ret]);
        });
    }
}

+ (void)removeIdentity:(NSString *)did
     forListenerHandle:(IndyHandle)listenerHandle
          walletHandle:(IndyHandle)walletHandle
            completion:(void (^)(NSError *error)) completion
{
    indy_error_t ret;
    
    indy_handle_t commandHandle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    
    ret = indy_agent_remove_identity(commandHandle,
                                     listenerHandle,
                                     walletHandle,
                                     [did UTF8String],
                                     IndyWrapperCommon2PCallback);
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: commandHandle];
        
        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError: ret]);
        });
    }
}

+ (void)sendMessage:(NSString *)message
   connectionHandle:(IndyHandle)connectionHandle
         completion:(void (^)(NSError *error)) completion
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    
    ret = indy_agent_send(handle,
                            connectionHandle,
                            [message UTF8String], IndyWrapperCommon2PCallback);
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
        
        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError: ret]);
        });
    }
}

+ (void)closeConnection:(IndyHandle)connectionHandle
             completion:(void (^)(NSError *error)) completion
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion
                                                             withConnectionHandle:connectionHandle];
    
    ret = indy_agent_close_connection(handle,
                                        connectionHandle,
                                        IndyWrapperCloseConnectionCallback);
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
        
        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError: ret]);
        });
    }
}

+ (void)closeListener:(IndyHandle)listenerHandle
           completion:(void (^)(NSError *error)) completion
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    [[IndyCallbacks sharedInstance] forgetListenHandle:listenerHandle];
    
    ret = indy_agent_close_listener(handle,
                                      listenerHandle,
                                      IndyWrapperCommon2PCallback);
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
        
        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError: ret]);
        });
    }
}

@end

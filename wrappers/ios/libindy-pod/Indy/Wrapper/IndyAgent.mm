//
//  IndyAgent.m
//  libindy
//

#import "IndyAgent.h"
#import "IndyCallbacks.h"
#import "indy_core.h"
#import "NSError+IndyError.h"

@implementation IndyAgent

+ (NSError *)connectWithPoolHandle:(IndyHandle)poolHandle
                      walletHandle:(IndyHandle)walletHandle
                         senderDId:(NSString *)senderDid
                       receiverDId:(NSString *)receiverDid
                 connectionHandler:(void (^)(NSError *error, IndyHandle connection)) connectionHandler
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
    }

    return [NSError errorFromIndyError: ret];
}

+ (NSError *)listenForEndpoint:(NSString *)endpoint
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
    }
    
    return [NSError errorFromIndyError: ret];}

+ (NSError *)addIdentity:(NSString *)did
       forListenerHandle:(IndyHandle)listenerHandle
              poolHandle:(IndyHandle)poolHandle
            walletHandle:(IndyHandle)walletHandle
              completion:(void (^)(NSError *error)) handler
{
    indy_error_t ret;
    
    indy_handle_t commandHandle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    
    ret = indy_agent_add_identity(commandHandle,
                                    listenerHandle,
                                    poolHandle,
                                    walletHandle,
                                    [did UTF8String],
                                    IndyWrapperCommon2PCallback);
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: commandHandle];
    }
    
    return [NSError errorFromIndyError: ret];
}

+ (NSError *)removeIdentity:(NSString *)did
          forListenerHandle:(IndyHandle)listenerHandle
               walletHandle:(IndyHandle)walletHandle
                 completion:(void (^)(NSError *error)) handler
{
    indy_error_t ret;
    
    indy_handle_t commandHandle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    
    ret = indy_agent_remove_identity(commandHandle,
                                       listenerHandle,
                                       walletHandle,
                                       [did UTF8String],
                                       IndyWrapperCommon2PCallback);
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: commandHandle];
    }
    
    return [NSError errorFromIndyError: ret];
}

+ (NSError *)sendWithConnectionHandle:(IndyHandle)connectionHandle
                             messsage:(NSString *)message
                           completion:(void (^)(NSError *error)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    
    ret = indy_agent_send(handle,
                            connectionHandle,
                            [message UTF8String], IndyWrapperCommon2PCallback);
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

+ (NSError *)closeConnection:(IndyHandle)connectionHandle
                  completion:(void (^)(NSError *error)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler
                                                                 withConnectionHandle: connectionHandle];
    
    ret = indy_agent_close_connection(handle,
                                        connectionHandle,
                                        IndyWrapperCloseConnectionCallback);
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];

}

+ (NSError *)closeListener:(IndyHandle)listenerHandle
                completion:(void (^)(NSError *error)) handler
{
    indy_error_t ret;
    
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:handler];
    [[IndyCallbacks sharedInstance] forgetListenHandle: listenerHandle];
    
    ret = indy_agent_close_listener(handle,
                                      listenerHandle,
                                      IndyWrapperCommon2PCallback);
    if( ret != Success )
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromIndyError: ret];
}

@end

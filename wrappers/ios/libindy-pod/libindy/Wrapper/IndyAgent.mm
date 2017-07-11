//
//  SovrinAgent.m
//  libsovrin
//

#import "IndyAgent.h"
#import "IndyCallbacks.h"
#import "sovrin_core.h"
#import "NSError+IndyError.h"

@implementation SovrinAgent

+ (NSError *)connectWithPoolHandle:(SovrinHandle)poolHandle
                      walletHandle:(SovrinHandle)walletHandle
                         senderDId:(NSString *)senderDid
                       receiverDId:(NSString *)receiverDid
                 connectionHandler:(void (^)(NSError *error, SovrinHandle connection)) connectionHandler
                    messageHandler:(void (^)(SovrinHandle connectionHandle, NSError *error, NSString *message))messageHandler
{
    sovrin_error_t ret;

    // closure_map_ids?
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) connectionHandler
                                                                  withMessageCallback: (void*) messageHandler];
    
    ret = sovrin_agent_connect(handle,
                               poolHandle,
                               walletHandle,
                               [senderDid UTF8String],
                               [receiverDid UTF8String],
                               SovrinWrapperCommonAgentOutgoingConnectionCallback,
                               SovrinWrapperCommonAgentMessageCallback);
    
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }

    return [NSError errorFromSovrinError: ret];
}

+ (NSError *)listenForEndpoint:(NSString *)endpoint
               listenerHandler:(void (^)(NSError *error,
                                         SovrinHandle listenerHandle))listenerCompletion
             connectionHandler:(void (^)(SovrinHandle xlistenerHandle,
                                         NSError *error,
                                         SovrinHandle connectionHandle,
                                         NSString *senderDid,
                                         NSString *receiverDid))connectionCompletion
                messageHandler:(void (^)(SovrinHandle xconnectionHandle,
                                              NSError *error,
                                         NSString *message))messageCompletion
{
    sovrin_error_t ret;
    sovrin_handle_t listener_handle = [[SovrinCallbacks sharedInstance] createCommandHandleForListenerCallback:(void*)[listenerCompletion copy]
                                                                                        withConnectionCallback:(void*)[connectionCompletion copy]
                                                                                            andMessageCallback:(void*)[messageCompletion copy] ];
    
    ret = sovrin_agent_listen(listener_handle,
                              [endpoint UTF8String],
                              SovrinWrapperCommonAgentListenerCallback,
                              SovrinWrapperCommonAgentListenerConnectionCallback,
                              SovrinWrapperCommonAgentListenerMessageCallback);
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: listener_handle];
    }
    
    return [NSError errorFromSovrinError: ret];}

+ (NSError *)addIdentity:(NSString *)did
       forListenerHandle:(SovrinHandle)listenerHandle
              poolHandle:(SovrinHandle)poolHandle
            walletHandle:(SovrinHandle)walletHandle
              completion:(void (^)(NSError *error)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t commandHandle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = sovrin_agent_add_identity(commandHandle,
                                    listenerHandle,
                                    poolHandle,
                                    walletHandle,
                                    [did UTF8String],
                                    SovrinWrapperCommon2PCallback);
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: commandHandle];
    }
    
    return [NSError errorFromSovrinError: ret];

}

+ (NSError *)removeIdentity:(NSString *)did
          forListenerHandle:(SovrinHandle)listenerHandle
               walletHandle:(SovrinHandle)walletHandle
                 completion:(void (^)(NSError *error)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t commandHandle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = sovrin_agent_remove_identity(commandHandle,
                                       listenerHandle,
                                       walletHandle,
                                       [did UTF8String],
                                       SovrinWrapperCommon2PCallback);
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: commandHandle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError *)sendWithConnectionHandle:(SovrinHandle)connectionHandle
                             messsage:(NSString *)message
                           completion:(void (^)(NSError *error)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    
    ret = sovrin_agent_send(handle,
                            connectionHandle,
                            [message UTF8String], SovrinWrapperCommon2PCallback);
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError *)closeConnection:(SovrinHandle)connectionHandle
                  completion:(void (^)(NSError *error)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler
                                                                 withConnectionHandle: connectionHandle];
    
    ret = sovrin_agent_close_connection(handle,
                                        connectionHandle,
                                        SovrinWrapperCloseConnectionCallback);
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];

}

+ (NSError *)closeListener:(SovrinHandle)listenerHandle
                completion:(void (^)(NSError *error)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) handler];
    [[SovrinCallbacks sharedInstance] forgetListenHandle: listenerHandle];
    
    ret = sovrin_agent_close_listener(handle,
                                      listenerHandle,
                                      SovrinWrapperCommon2PCallback);
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

@end

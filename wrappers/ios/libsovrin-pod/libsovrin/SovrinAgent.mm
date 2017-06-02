//
//  SovrinAgent.m
//  libsovrin
//

#import "SovrinAgent.h"
#import "SovrinCallbacks.h"
#import "sovrin_core.h"
#import "NSError+SovrinError.h"

@implementation SovrinAgent

+ (NSError*) agentConnect:(SovrinHandle) walletHandle
                 senderId:(NSString *) senderDid
               receiverId:(NSString *) receiverDid
        connectionHandler:(void (^)(NSError* error, SovrinHandle connection)) connectionHandler
           messageHandler:(void (^)(NSError* error, NSString* message)) messageHandler
{
    sovrin_error_t ret;

    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor: (void*) connectionHandler
                                                                  withMessageCallback: (void*) messageHandler];
    
    ret = sovrin_agent_connect(handle,
                               walletHandle,
                               [senderDid UTF8String],
                               [receiverDid UTF8String],
                               SovrinWrapperCommonAgentOutgoingConnectionCallback,
                               SovrinWrapperCommonAgentMessageCallback
                              );
    
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: handle];
    }

    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) agentListen:(SovrinHandle) walletHandle
         listenerHandler:(void (^)(NSError* error,
                                   SovrinHandle listenerHandle)) listenerCompletion
       connectionHandler:(void (^)(SovrinHandle xlistenerHandle,
                                   NSError*     error,
                                   SovrinHandle connectionHandle,
                                   NSString*    senderDid,
                                   NSString*    receiverDid)) connectionCompletion
          messageHandler:(void (^)(SovrinHandle xconnectionHandle,
                                   NSError*     error,
                                   NSString*    message)) messageCompletion
{
    sovrin_error_t ret;
    
    sovrin_handle_t listener_handle = [[SovrinCallbacks sharedInstance] createCommandHandleFor:(void*)listenerCompletion
                                                                        withConnectionCallback:(void*)connectionCompletion
                                                                            andMessageCallback:(void*)messageCompletion ];
    
    ret = sovrin_agent_listen(listener_handle,
                              walletHandle,
                              SovrinWrapperCommonAgentListenerCallback,
                              SovrinWrapperCommonAgentListenerConnectionCallback,
                              SovrinWrapperCommonAgentListenerMessageCallback);
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] deleteCommandHandleFor: listener_handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) agentSend:(SovrinHandle) connectionHandle
              messsage:(NSString*) message
            completion:(void (^)(NSError* error)) handler
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

+ (NSError*) agentCloseConnection:(SovrinHandle) connectionHandle
                       completion:(void (^)(NSError* error)) handler
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

+ (NSError*) agentCloseListener:(SovrinHandle) listenerHandle
                     completion:(void (^)(NSError* error)) handler
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

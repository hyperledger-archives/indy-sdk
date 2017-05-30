//
//  SovrinAgent.m
//  libsovrin
//
//  Created by Anastasiya Tarasova on 30.05.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import "SovrinAgent.h"
#import "SovrinCallbacks.h"
#import "sovrin_core.h"
#import "NSError+SovrinError.h"

@implementation SovrinAgent

+ (NSError*) agentConnect: (SovrinHandle) commandHandle
             walletHandle: (SovrinHandle) walletHandle
                 senderId: (NSString *) senderId
               receiverId: (NSString *) receiverId
               completion: (void (^)(SovrinHandle xcommandHandle,
                                     NSError* error,
                                     SovrinHandle connectionHandle)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] add: (void*) handler];
    
    ret = sovrin_agent_connect(commandHandle,
                               walletHandle,
                               [senderId UTF8String],
                               [receiverId UTF8String],SovrinWrapperCommon3PHCallback);
    
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] remove: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) agentListen: (SovrinHandle) commandHandle
            walletHandle: (SovrinHandle) walletHandle
      listenerCompletion: (void (^)(SovrinHandle xcommandHandle,
                                    NSError* error,
                                    SovrinHandle listenerHandler)) listenerHandler
    connectionCompletion: (void (^)(SovrinHandle xlistenerHandle,
                                    NSError* error,
                                    SovrinHandle connectionHandle)) connectionHandler
       messageCompletion: (void (^)(SovrinHandle xconnectionHandle,
                                    NSError* error,
                                    NSString* message)) messageHandler
{
    sovrin_error_t ret;
    
    sovrin_handle_t listener_handle = [[SovrinCallbacks sharedInstance] add: (void*) listenerHandler];
    
    sovrin_handle_t connection_handler = [[SovrinCallbacks sharedInstance] add: (void*) connectionHandler];
    
    sovrin_handle_t message_handler = [[SovrinCallbacks sharedInstance] add: (void*) messageHandler];
    
    ret = sovrin_agent_listen(commandHandle,
                              walletHandle,
                              SovrinWrapperCommon3PHCallback,
                              SovrinWrapperCommon5PSCallback,
                              SovrinWrapperCommon3PSCallback);
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] remove: listener_handle];
        [[SovrinCallbacks sharedInstance] remove: connection_handler];
        [[SovrinCallbacks sharedInstance] remove: message_handler];
    }
    
    return [NSError errorFromSovrinError: ret];

}

+ (NSError*) agentSend: (SovrinHandle) commandHandle
      connectionHandle: (SovrinHandle) connectionHandle
              messsage: (NSString*) message
            completion: (void (^)(SovrinHandle xcommandHandle,
                                  NSError* error)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] add: (void*) handler];
    
    ret = sovrin_agent_send(commandHandle,
                            connectionHandle,
                            [message UTF8String], SovrinWrapperCommon2PCallback);
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] remove: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) agentCloseConnection: (SovrinHandle) commandHandle
                 connectionHandle: (SovrinHandle) connectionHandle
                       completion: (void (^)(SovrinHandle xcommandHandle,
                                             NSError* error)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] add: (void*) handler];
    
    ret = sovrin_agent_close_connection(commandHandle,
                                        connectionHandle,
                                        SovrinWrapperCommon2PCallback);
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] remove: handle];
    }
    
    return [NSError errorFromSovrinError: ret];

}

+ (NSError*) agentCloseListener: (SovrinHandle) commandHandle
                 listenerHandle: (SovrinHandle) listenerHandle
                     completion: (void (^)(SovrinHandle xcommandHandle,
                                           NSError* error)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] add: (void*) handler];
    
    ret = sovrin_agent_close_listener(commandHandle,
                                      listenerHandle,
                                      SovrinWrapperCommon2PCallback);
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] remove: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

@end

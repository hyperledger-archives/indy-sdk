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

+ (NSError*) agentConnect: (SovrinHandle) walletHandle
                 senderId: (NSString *) senderId
               receiverId: (NSString *) receiverId
               completion: (void (^)(NSError* error,
                                     SovrinHandle connectionHandle)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] add: (void*) handler];
    
    ret = sovrin_agent_connect(handle,
                               walletHandle,
                               [senderId UTF8String],
                               [receiverId UTF8String],SovrinWrapperCommon3PHCallback);
    
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] remove: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) agentListen: (SovrinHandle) walletHandle
      listenerCompletion: (void (^)(NSError* error,
                                    SovrinHandle listenerHandle)) listenerHandler

    connectionCompletion: (void (^)(SovrinHandle xlistenerHandle,
                                    NSError* error,
                                    SovrinHandle connectionHandle)) connectionHandler

       messageCompletion: (void (^)(SovrinHandle xconnectionHandle,
                                    NSError* error,
                                    NSString* message)) messageHandler
{
    sovrin_error_t ret;
    
    sovrin_handle_t listener_handle = [[SovrinCallbacks sharedInstance] add: (void*) listenerHandler];
    
    sovrin_handle_t connection_handle = [[SovrinCallbacks sharedInstance] add: (void*) connectionHandler];
    
    sovrin_handle_t message_handle = [[SovrinCallbacks sharedInstance] add: (void*) messageHandler];
    
    // Is it a right command handle? Check it.
    ret = sovrin_agent_listen(listener_handle,
                              walletHandle,
                              SovrinWrapperCommon3PHCallback,
                              SovrinWrapperCommon5PSCallback,
                              SovrinWrapperCommon3PSCallback);
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] remove: listener_handle];
        [[SovrinCallbacks sharedInstance] remove: connection_handle];
        [[SovrinCallbacks sharedInstance] remove: message_handle];
    }
    
    return [NSError errorFromSovrinError: ret];

}

+ (NSError*) agentSend: (SovrinHandle) connectionHandle
              messsage: (NSString*) message
            completion: (void (^)(NSError* error)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] add: (void*) handler];
    
    ret = sovrin_agent_send(handle,
                            connectionHandle,
                            [message UTF8String], SovrinWrapperCommon2PCallback);
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] remove: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

+ (NSError*) agentCloseConnection: (SovrinHandle) connectionHandle
                       completion: (void (^)(NSError* error)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] add: (void*) handler];
    
    ret = sovrin_agent_close_connection(handle,
                                        connectionHandle,
                                        SovrinWrapperCommon2PCallback);
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] remove: handle];
    }
    
    return [NSError errorFromSovrinError: ret];

}

+ (NSError*) agentCloseListener: (SovrinHandle) listenerHandle
                     completion: (void (^)(NSError* error)) handler
{
    sovrin_error_t ret;
    
    sovrin_handle_t handle = [[SovrinCallbacks sharedInstance] add: (void*) handler];
    
    ret = sovrin_agent_close_listener(handle,
                                      listenerHandle,
                                      SovrinWrapperCommon2PCallback);
    if( ret != Success )
    {
        [[SovrinCallbacks sharedInstance] remove: handle];
    }
    
    return [NSError errorFromSovrinError: ret];
}

@end

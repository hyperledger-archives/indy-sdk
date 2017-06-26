//
//  AgentUtils.m
//  libsovrin-demo
//
//  Created by Anastasia Tarasova on 22.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import "AgentUtils.h"
#import <libsovrin/libsovrin.h>
#import "TestUtils.h"

@implementation AgentUtils

+ (AgentUtils *)sharedInstance
{
    static AgentUtils *instance = nil;
    static dispatch_once_t dispatch_once_block;
    
    dispatch_once(&dispatch_once_block, ^ {
        instance = [AgentUtils new];
    });
    
    return instance;
}


- (NSError *)connectWithPoolHandle:(SovrinHandle)poolHandle
                      walletHandle:(SovrinHandle)walletHandle
                         senderDid:(NSString *)senderDid
                       receiverDid:(NSString *)receiverDid
                     messageCallback:(void (^)(NSError *error, NSString *message))messageCallback
                connectionCallback:(void (^)(NSError *error, SovrinHandle connection))connectionCallback
               outConnectionHandle:(SovrinHandle *)connectionHandle
{
//    NSError *ret = [SovrinAgent connectWithPoolHandle:poolHandle
//                                         walletHandle:walletHandle
//                                             senderId:senderDid
//                                           receiverId:receiverDid
//                                    connectionHandler:connectionCallback
//                                       messageHandler:messageCallback];
//    if (ret.code != Success)
//    {
//        return ret;
//    }
//    
//    if( err.code != Success)
//    {
//        return err;
//    }
    return nil;
}

- (NSError *)listenWithWalletHandle:(SovrinHandle) walletHandle
                           endpoint:(NSString *)endpoint
                 connectionCallback:( void (^)(SovrinHandle listenerHandle, SovrinHandle connectionHandle))connectionCallback
                    messageCallback:(void (^)(SovrinHandle connectionHandle, NSString *message))messageCallback
                  outListenerHandle:(SovrinHandle *)listenerHandle
{
    // connection callback
    XCTestExpectation* connectCompletionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"listener completion finished"];
    __block NSError *connectionErr;
    void (^onConnectCallback)(SovrinHandle, NSError*, SovrinHandle, NSString*, NSString* ) = ^(SovrinHandle xListenerHandle, NSError *error, SovrinHandle connectionHandle, NSString *senderDid, NSString *receiverDid) {
        connectionErr = error;
        if (connectionCallback) {connectionCallback(xListenerHandle, connectionHandle);}
        [connectCompletionExpectation fulfill];
    };
    
    
    // listener callback
    XCTestExpectation* listenerCompletionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"listener completion finished"];
    __block SovrinHandle tempListenerHandle = 0;
    __block NSError *listenerErr;
    void (^onListenerCallback)(NSError*, SovrinHandle) = ^(NSError *error, SovrinHandle xListenerHandle) {
        listenerErr = error;
        tempListenerHandle = xListenerHandle;
        [listenerCompletionExpectation fulfill];
    };
    
    // message callback
    XCTestExpectation* messageCompletionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"message completion finished"];
    __block NSError *messageErr;
    void (^onMessageCallback)(SovrinHandle, NSError*, NSString*) = ^(SovrinHandle xConnectionHandle, NSError *error, NSString *message) {
        messageErr = error;
        if (messageCallback != nil) { messageCallback(xConnectionHandle, message);}
        [messageCompletionExpectation fulfill];
    };
    
    // listen
    NSError *ret = [SovrinAgent listenWithWalletHandle:walletHandle
                                              endpoint:endpoint
                                       listenerHandler:onListenerCallback
                                     connectionHandler:onConnectCallback
                                        messageHandler:onMessageCallback];
    if (ret.code != Success)
    {
        NSLog(@"SovrinAgent::listenWithWalletHandle failed with code: %ld", ret.code);
        return ret;
    }
    
    // wait for messageCallback
    [self waitForExpectations: @[messageCompletionExpectation] timeout:[TestUtils defaultTimeout]];
    if (messageErr.code != Success)
    {
        NSLog(@"Message callback returned error code: %ld", messageErr.code);
        return messageErr;
    }
    
    // wait for listenerCallback
    [self waitForExpectations: @[listenerCompletionExpectation] timeout:[TestUtils defaultTimeout]];
    if (listenerErr.code != Success)
    {
        NSLog(@"Message callback returned error code: %ld", listenerErr.code);
        return listenerErr;
    }
    
    if (listenerHandle) { *listenerHandle = tempListenerHandle;};
    
    // wait for connection callback
    [self waitForExpectations: @[connectCompletionExpectation] timeout:[TestUtils defaultTimeout]];
    if (connectionErr.code != Success)
    {
        NSLog(@"Connection callback returned error code: %ld", connectionErr.code);
        return connectionErr;
    }
    
    return ret;
}


//
//- (NSError *)sendWithConnectionHandler:(SovrinHandle)connectionHandle
//                               message:(NSString *)message
//{
//    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
//    __block NSError *err;
//    
//    ret = [SovrinAgent sendWithConnectionHandle:connectionHandle
//                                       messsage:message
//                                     completion:^(NSError *error)
//    {
//        err = error;
//        [completionExpectation fulfill];
//    }];
//    
//    if (ret.code != Success)
//    {
//        return ret;
//    }
//    
//    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
//    
//    if( err.code != Success)
//    {
//        return err;
//    }
//}
@end

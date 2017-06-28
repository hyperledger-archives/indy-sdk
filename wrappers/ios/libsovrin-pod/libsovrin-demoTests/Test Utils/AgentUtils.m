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
                     messageCallback:(void (^)(SovrinHandle connectHandle, NSString *message))messageCallback
               outConnectionHandle:(SovrinHandle *)outConnectionHandle
{
    // connection callback
    XCTestExpectation* connectCompletionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"listener completion finished"];
    __block NSError *connectionErr;
    __block SovrinHandle tempConnectionHandle;
    void (^onConnectCallback)(NSError*, SovrinHandle) = ^(NSError *error, SovrinHandle connectionHandle) {
        NSLog(@"OnConnectCallback triggered.");
        tempConnectionHandle = connectionHandle;
        connectionErr = error;
        [connectCompletionExpectation fulfill];
    };
    
    
    // message callback
    XCTestExpectation* messageCompletionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"message completion finished"];
    __block NSError *messageErr;
    void (^onMessageCallback)(SovrinHandle, NSError*, NSString*) = ^(SovrinHandle xConnectionHandle, NSError *error, NSString *message) {
        NSLog(@"OnMessageCallback triggered invoced with error code: %ld", error.code);
        messageErr = error;
        if (messageCallback != nil) { messageCallback(xConnectionHandle, message);}
        [messageCompletionExpectation fulfill];
    };
    
    NSError *ret = [SovrinAgent connectWithPoolHandle:poolHandle
                                         walletHandle:walletHandle
                                   senderDId:senderDid
                                 receiverDId:receiverDid
                           connectionHandler:onConnectCallback
                              messageHandler:onMessageCallback];

    if (ret.code != Success)
    {
        return ret;
    }
    
    // wait for connection callback
    [self waitForExpectations: @[connectCompletionExpectation] timeout:[TestUtils defaultTimeout]];
    if (connectionErr.code != Success)
    {
        NSLog(@"Connection callback returned error code: %ld", connectionErr.code);
        return connectionErr;
    }
    
    if (outConnectionHandle) { *outConnectionHandle = tempConnectionHandle;}

    return connectionErr;
}

- (NSError *)listenWithWalletHandle:(SovrinHandle) walletHandle
                           endpoint:(NSString *)endpoint
                 connectionCallback:( void (^)(SovrinHandle listenerHandle, SovrinHandle connectionHandle))connectionCallback
                    messageCallback:(void (^)(SovrinHandle connectionHandle, NSString *message))messageCallback
                  outListenerHandle:(SovrinHandle *)listenerHandle
{
    // connection callback
    void (^onConnectCallback)(SovrinHandle, NSError*, SovrinHandle, NSString*, NSString* ) = ^(SovrinHandle xListenerHandle, NSError *error, SovrinHandle connectionHandle, NSString *senderDid, NSString *receiverDid) {
        XCTAssertEqual(error.code, Success, @"onConnectCallback in AgentUtiles");
        NSLog(@"OnConnectCallback triggered with error code: %ld", (long)error.code);
        if (connectionCallback) {connectionCallback(xListenerHandle, connectionHandle);}
    };
    
    
    // listener callback
    XCTestExpectation* listenerCompletionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"listener completion finished"];
    __block SovrinHandle tempListenerHandle = 0;
    __block NSError *listenerErr;
    void (^onListenerCallback)(NSError*, SovrinHandle) = ^(NSError *error, SovrinHandle xListenerHandle) {
        NSLog(@"OnListenerCallback triggered.");
        listenerErr = error;
        tempListenerHandle = xListenerHandle;
        [listenerCompletionExpectation fulfill];
    };
    
    // message callback
    void (^onMessageCallback)(SovrinHandle, NSError*, NSString*) = ^(SovrinHandle xConnectionHandle, NSError *error, NSString *message) {
        NSLog(@"OnMessageCallback triggered with error code: %ld.", (long)error.code);
        if (messageCallback != nil) { messageCallback(xConnectionHandle, message);}
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
    
    // wait for listenerCallback
    [self waitForExpectations: @[listenerCompletionExpectation] timeout:[TestUtils defaultTimeout]];
    
    if (listenerHandle) { *listenerHandle = tempListenerHandle;};

    return listenerErr;
}



- (NSError *)sendWithConnectionHandler:(SovrinHandle)connectionHandle
                               message:(NSString *)message
{
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err;
    
    NSError *ret = [SovrinAgent sendWithConnectionHandle:connectionHandle
                                                messsage:message
                                              completion:^(NSError *error)
    {
        err = error;
        [completionExpectation fulfill];
    }];
    
    if (ret.code != Success)
    {
        return ret;
    }
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    return err;
}

- (NSError *)closeConnection:(SovrinHandle)connectionHandle
{
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err;
    
    NSError *ret = [SovrinAgent closeConnection:connectionHandle
                                     completion:^(NSError *error)
                    {
                        err = error;
                        [completionExpectation fulfill];
                    }];
    
    if (ret.code != Success)
    {
        return ret;
    }
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    return err;
}
@end

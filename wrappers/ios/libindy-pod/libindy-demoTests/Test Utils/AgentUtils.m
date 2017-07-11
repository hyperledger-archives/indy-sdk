//
//  AgentUtils.m
//  libsovrin-demo
//
//  Created by Anastasia Tarasova on 22.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import "AgentUtils.h"
#import <libindy/libindy.h>
#import "TestUtils.h"


@interface AgentUtils ()

@property (atomic, strong) NSMutableDictionary* connectionCallbacks;

@end

@implementation AgentUtils

+ (AgentUtils *)sharedInstance
{
    static AgentUtils *instance = nil;
    static dispatch_once_t dispatch_once_block;
    
    dispatch_once(&dispatch_once_block, ^ {
        instance = [AgentUtils new];
        instance.connectionCallbacks = [NSMutableDictionary new];
    });
    
    return instance;
}

- (void)addMessageCallbackForConnection:(SovrinHandle)connectionHandle
{
    if (self.connectionCallbacks[@(connectionHandle)] == nil)
    {
        self.connectionCallbacks[@(connectionHandle)] = [NSArray new];
    }
}

- (NSError *)connectWithPoolHandle:(SovrinHandle)poolHandle
                      walletHandle:(SovrinHandle)walletHandle
                         senderDid:(NSString *)senderDid
                       receiverDid:(NSString *)receiverDid
                     messageCallback:(void (^)(SovrinHandle connectHandle, NSString *message))messageCallback
               outConnectionHandle:(SovrinHandle *)outConnectionHandle
{
    // connection callback. waiting for completion
    XCTestExpectation* connectCompletionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"listener completion finished"];
    __block NSError *connectionErr;
    __block SovrinHandle tempConnectionHandle;
    
    void (^onConnectCallback)(NSError*, SovrinHandle) = ^(NSError *error, SovrinHandle connectionHandle) {
        NSLog(@"AgentUtils::connectWithPoolHandle::OnConnectCallback triggered with code: %d", error.code);
        tempConnectionHandle = connectionHandle;
        connectionErr = error;
        [connectCompletionExpectation fulfill];
    };
    
    
    // message callback
    void (^onMessageCallback)(SovrinHandle, NSError*, NSString*) = ^(SovrinHandle xConnectionHandle, NSError *error, NSString *message) {
        NSLog(@"AgentUtils::connectWithPoolHandle::OnMessageCallback triggered invoced with error code: %ld", (long)error.code);
        if (messageCallback != nil) { messageCallback(xConnectionHandle, message);}
    };
    
    __weak typeof(self)weakSelf = self;
    weakSelf.connectionCallbacks[@(tempConnectionHandle)] = ^(SovrinHandle xConnectionHandle, NSError *error, NSString *message) {
        NSLog(@"AgentUtils::connectWithPoolHandle::OnMessageCallback triggered invoced with error code: %d", error.code);
        if (messageCallback != nil) { messageCallback(xConnectionHandle, message);}
    };
    
    NSError *ret = [SovrinAgent connectWithPoolHandle:poolHandle
                                         walletHandle:walletHandle
                                            senderDId:senderDid
                                          receiverDId:receiverDid
                                    connectionHandler:onConnectCallback
                                       messageHandler:(void (^)(SovrinHandle, NSError*, NSString*))weakSelf.connectionCallbacks[@(tempConnectionHandle)]];

    if (ret.code != Success)
    {
        return ret;
    }
    
    // wait for connection callback
    [self waitForExpectations: @[connectCompletionExpectation] timeout:[TestUtils defaultTimeout]];
    
    if (outConnectionHandle) { *outConnectionHandle = tempConnectionHandle;}

    return connectionErr;
}

__strong void (^onListenerCallback)(NSError*, SovrinHandle) = nil;
__strong void (^onMessageCallback)(SovrinHandle, NSError*, NSString*) = nil;
__strong void (^onConnectCallback)(SovrinHandle, NSError*, SovrinHandle, NSString*, NSString* ) = nil;

- (NSError *)listenForEndpoint:(NSString *)endpoint
             connectionCallback:( void (^)(SovrinHandle listenerHandle, SovrinHandle connectionHandle))connectionCallback
                messageCallback:(void (^)(SovrinHandle connectionHandle, NSString *message))messageCallback
              outListenerHandle:(SovrinHandle *)listenerHandle
{
    
    #if 0
    // connection callback
    void (^onConnectCallback)(SovrinHandle, NSError*, SovrinHandle, NSString*, NSString* ) = ^(SovrinHandle xListenerHandle, NSError *error, SovrinHandle connectionHandle, NSString *senderDid, NSString *receiverDid) {
        NSLog(@"AgentUtils::listen::New connection %d on listener %d, err %ld, sender DID %@, receiver DID: %@", (int)connectionHandle, (int)xListenerHandle, (long)error.code, senderDid, receiverDid);
        if (connectionCallback) {connectionCallback(xListenerHandle, connectionHandle);}
    };
    
    
    // listener callback. We need to obtain listenerHandle, so we wait for completion. Connection and message callnacks can be triggered multiple times later, so we just pass them to register.
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
        NSLog(@"AgentUtils::listen::On connection %d received (with error %ld) agent message (CLI->SRV): %@", (int)xConnectionHandle, (long)error.code, message);
        if (messageCallback != nil) { messageCallback(xConnectionHandle, message);}
    };
#endif
    
    // listener callback. We need to obtain listenerHandle, so we wait for completion. Connection and message callnacks can be triggered multiple times later, so we just pass them to register.
    XCTestExpectation* listenerCompletionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"listener completion finished"];
    __block SovrinHandle tempListenerHandle = 0;
    __block NSError *listenerErr;
    
    onConnectCallback = ^(SovrinHandle xListenerHandle, NSError *error, SovrinHandle connectionHandle, NSString *senderDid, NSString *receiverDid) {
        NSLog(@"AgentUtils::listen::New connection %d on listener %d, err %ld, sender DID %@, receiver DID: %@", (int)connectionHandle, (int)xListenerHandle, (long)error.code, senderDid, receiverDid);
        if (connectionCallback) {connectionCallback(xListenerHandle, connectionHandle);}
    };

    onListenerCallback = ^(NSError *error, SovrinHandle xListenerHandle) {
        NSLog(@"OnListenerCallback triggered.");
        listenerErr = error;
        tempListenerHandle = xListenerHandle;
        [listenerCompletionExpectation fulfill];
    };
    
    // message callback
   onMessageCallback = ^(SovrinHandle xConnectionHandle, NSError *error, NSString *message) {
        NSLog(@"AgentUtils::listen::On connection %d received (with error %ld) agent message (CLI->SRV): %@", (int)xConnectionHandle, (long)error.code, message);
        if (messageCallback != nil) { messageCallback(xConnectionHandle, message);}
    };

    // listen
    NSError *ret = [SovrinAgent listenForEndpoint:endpoint
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

- (NSError *)closeListener:(SovrinHandle)listenerHandle
{
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err;
    
    NSError *ret = [SovrinAgent closeListener:listenerHandle
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

- (NSError *)addIdentityForListenerHandle:(SovrinHandle)listenerHandle
                               poolHandle:(SovrinHandle)poolHandle
                             walletHandle:(SovrinHandle)walletHandle
                                      did:(NSString *)did
{
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err;
    
    NSError *ret = [SovrinAgent addIdentity:did
                          forListenerHandle:listenerHandle
                                 poolHandle:poolHandle
                               walletHandle:walletHandle
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

- (NSError *)removeIdentity:(NSString *) did
             listenerHandle:(SovrinHandle)listenerHandle
               walletHandle:(SovrinHandle)walletHandle
{
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err;
    
    NSError *ret = [SovrinAgent removeIdentity:did
                             forListenerHandle:listenerHandle
                                  walletHandle:walletHandle
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

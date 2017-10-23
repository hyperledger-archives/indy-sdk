//
//  AgentUtils.m
//  Indy-demo
//
//  Created by Anastasia Tarasova on 22.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import "AgentUtils.h"
#import <Indy/Indy.h>
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

- (void)addMessageCallbackForConnection:(IndyHandle)connectionHandle
{
    if (self.connectionCallbacks[@(connectionHandle)] == nil)
    {
        self.connectionCallbacks[@(connectionHandle)] = [NSArray new];
    }
}

- (NSError *)connectWithPoolHandle:(IndyHandle)poolHandle
                      walletHandle:(IndyHandle)walletHandle
                         senderDid:(NSString *)senderDid
                       receiverDid:(NSString *)receiverDid
                     messageCallback:(void (^)(IndyHandle connectHandle, NSString *message))messageCallback
               outConnectionHandle:(IndyHandle *)outConnectionHandle
{
    // connection callback. waiting for completion
    XCTestExpectation* connectCompletionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"listener completion finished"];
    __block NSError *connectionErr;
    __block IndyHandle tempConnectionHandle;
    
    void (^onConnectCallback)(NSError*, IndyHandle) = ^(NSError *error, IndyHandle connectionHandle) {
        NSLog(@"AgentUtils::connectWithPoolHandle::OnConnectCallback triggered with code: %ld", (long)error.code);
        tempConnectionHandle = connectionHandle;
        connectionErr = error;
        [connectCompletionExpectation fulfill];
    };
    
    __weak typeof(self)weakSelf = self;
    weakSelf.connectionCallbacks[@(tempConnectionHandle)] = ^(IndyHandle xConnectionHandle, NSError *error, NSString *message) {
        NSLog(@"AgentUtils::connectWithPoolHandle::OnMessageCallback triggered invoced with error code: %ld", (long)error.code);
        if (messageCallback != nil) { messageCallback(xConnectionHandle, message);}
    };
    
    [IndyAgent connectSenderDid:senderDid
                withReceiverDid:receiverDid
                     poolHandle:poolHandle
                   walletHandle:walletHandle
              connectionHandler:onConnectCallback
                 messageHandler:^(IndyHandle xConnectionHandle, NSError *error, NSString *message) {
                     NSLog(@"AgentUtils::connectWithPoolHandle::OnMessageCallback triggered invoced with error code: %ld", (long)error.code);
                     if (messageCallback != nil) { messageCallback(xConnectionHandle, message);}
                 }];
    
    // wait for connection callback
    [self waitForExpectations: @[connectCompletionExpectation] timeout:[TestUtils defaultTimeout]];
    
    if (outConnectionHandle) { *outConnectionHandle = tempConnectionHandle;}

    return connectionErr;
}


- (NSError *)listenForEndpoint:(NSString *)endpoint
             connectionCallback:( void (^)(IndyHandle listenerHandle, IndyHandle connectionHandle))connectionCallback
                messageCallback:(void (^)(IndyHandle connectionHandle, NSString *message))messageCallback
              outListenerHandle:(IndyHandle *)listenerHandle
{
    // listener callback. We need to obtain listenerHandle, so we wait for completion. Connection and message callnacks can be triggered multiple times later, so we just pass them to register.
    XCTestExpectation* listenerCompletionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"listener completion finished"];
    __block IndyHandle tempListenerHandle = 0;
    __block NSError *listenerErr;
    
    void (^onConnectCallback)(IndyHandle, NSError*, IndyHandle, NSString*, NSString* ) = ^(IndyHandle xListenerHandle, NSError *error, IndyHandle connectionHandle, NSString *senderDid, NSString *receiverDid) {
        NSLog(@"AgentUtils::listen::New connection %d on listener %d, err %ld, sender DID %@, receiver DID: %@", (int)connectionHandle, (int)xListenerHandle, (long)error.code, senderDid, receiverDid);
        if (connectionCallback) {connectionCallback(xListenerHandle, connectionHandle);}
    };

    void (^onListenerCallback)(NSError*, IndyHandle) = ^(NSError *error, IndyHandle xListenerHandle) {
        NSLog(@"OnListenerCallback triggered.");
        listenerErr = error;
        tempListenerHandle = xListenerHandle;
        [listenerCompletionExpectation fulfill];
    };
    
    // message callback
   void (^onMessageCallback)(IndyHandle, NSError*, NSString*) = ^(IndyHandle xConnectionHandle, NSError *error, NSString *message) {
        NSLog(@"AgentUtils::listen::On connection %d received (with error %ld) agent message (CLI->SRV): %@", (int)xConnectionHandle, (long)error.code, message);
        if (messageCallback != nil) { messageCallback(xConnectionHandle, message);}
    };

    // listen
    [IndyAgent listenForEndpoint:endpoint
                 listenerHandler:onListenerCallback
               connectionHandler:onConnectCallback
                  messageHandler:onMessageCallback];

    [self waitForExpectations: @[listenerCompletionExpectation] timeout:[TestUtils defaultTimeout]];
    
    if (listenerHandle) { *listenerHandle = tempListenerHandle;};

    return listenerErr;
}



- (NSError *)sendWithConnectionHandler:(IndyHandle)connectionHandle
                               message:(NSString *)message
{
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err;
    
    [IndyAgent sendMessage:message
          connectionHandle:connectionHandle
                completion:^(NSError *error)
     {
         err = error;
         [completionExpectation fulfill];
     }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    return err;
}

- (NSError *)closeConnection:(IndyHandle)connectionHandle
{
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err;
    
    [IndyAgent closeConnection:connectionHandle
                    completion:^(NSError *error)
     {
         err = error;
         [completionExpectation fulfill];
     }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    return err;
}

- (NSError *)closeListener:(IndyHandle)listenerHandle
{
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err;
    
    [IndyAgent closeListener:listenerHandle
                  completion:^(NSError *error)
     {
         err = error;
         [completionExpectation fulfill];
     }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    return err;
}

- (NSError *)addIdentityForListenerHandle:(IndyHandle)listenerHandle
                               poolHandle:(IndyHandle)poolHandle
                             walletHandle:(IndyHandle)walletHandle
                                      did:(NSString *)did
{
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err;
    
    [IndyAgent addIdentity:did
         forListenerHandle:listenerHandle
                poolHandle:poolHandle
              walletHandle:walletHandle
                completion:^(NSError *error)
     {
         err = error;
         [completionExpectation fulfill];
     }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    return err;
}

- (NSError *)removeIdentity:(NSString *) did
             listenerHandle:(IndyHandle)listenerHandle
               walletHandle:(IndyHandle)walletHandle
{
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err;
    
    [IndyAgent removeIdentity:did
            forListenerHandle:listenerHandle
                 walletHandle:walletHandle
                   completion:^(NSError *error)
     {
         err = error;
         [completionExpectation fulfill];
     }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    return err;
}

- (NSError *)connectHangUpExpectedForPoolHandle:(IndyHandle)poolHandle
                                   walletHandle:(IndyHandle)walletHandle
                                      senderDid:(NSString *)senderDid
                                    receiverDid:(NSString *)receiverDid
                                      isTimeout:(BOOL *)isTimeout
{
   dispatch_semaphore_t semaphore = dispatch_semaphore_create(0);
    
    // connection callback. waiting for completion
    __block NSError *connectionErr;
    __block IndyHandle tempConnectionHandle;
    
    void (^onConnectCallback)(NSError*, IndyHandle) = ^(NSError *error, IndyHandle connectionHandle) {
        NSLog(@"AgentUtils::connectWithPoolHandle::OnConnectCallback triggered with code: %ld", (long)error.code);
        tempConnectionHandle = connectionHandle;
        connectionErr = error;
        dispatch_semaphore_signal(semaphore);
    };
    
    void (^messageHandler)(IndyHandle, NSError*, NSString*) = ^(IndyHandle xConnectionHandle, NSError *error, NSString *message) {
        NSLog(@"AgentUtils::connectWithPoolHandle::OnMessageCallback triggered invoced with error code: %ld", (long)error.code);
    };
    
        [IndyAgent connectSenderDid:senderDid
                    withReceiverDid:receiverDid
                         poolHandle:poolHandle
                       walletHandle:walletHandle
                  connectionHandler:onConnectCallback
                     messageHandler:messageHandler];
   
    dispatch_semaphore_wait(semaphore, dispatch_time(DISPATCH_TIME_NOW, (5.0 * NSEC_PER_SEC)));
    
    if (connectionErr == nil)
    {
        if (isTimeout) { *isTimeout = true; }
    }
    else
    {
        if (isTimeout) { *isTimeout = false; }
    }

    return connectionErr;
}

@end

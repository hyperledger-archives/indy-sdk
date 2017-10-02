//
//  AgentUtils.h
//  Indy-demo
//
//  Created by Anastasia Tarasova on 22.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <Indy/Indy.h>

@interface AgentUtils : XCTestCase

+ (AgentUtils *)sharedInstance;

- (NSError *)connectWithPoolHandle:(IndyHandle)poolHandle
                      walletHandle:(IndyHandle)walletHandle
                         senderDid:(NSString *)senderDid
                       receiverDid:(NSString *)receiverDid
                   messageCallback:(void (^)(IndyHandle connectHandle, NSString *message))messageCallback
               outConnectionHandle:(IndyHandle *)outConnectionHandle;

- (NSError *)listenForEndpoint:(NSString *)endpoint
            connectionCallback:( void (^)(IndyHandle listenerHandle, IndyHandle connectionHandle))connectionCallback
               messageCallback:(void (^)(IndyHandle connectionHandle, NSString *message))messageCallback
             outListenerHandle:(IndyHandle *)listenerHandle;

- (NSError *)sendWithConnectionHandler:(IndyHandle)connectionHandle
                               message:(NSString *)message;

- (NSError *)closeConnection:(IndyHandle)connectionHandle;

- (NSError *)closeListener:(IndyHandle)listenerHandle;

- (NSError *)addIdentityForListenerHandle:(IndyHandle)listenerHandle
                               poolHandle:(IndyHandle)poolHandle
                             walletHandle:(IndyHandle)walletHandle
                                      did:(NSString *)did;

- (NSError *)removeIdentity:(NSString *)did
             listenerHandle:(IndyHandle)listenerHandle
               walletHandle:(IndyHandle)walletHandle;

- (NSError *)connectHangUpExpectedForPoolHandle:(IndyHandle)poolHandle
                                   walletHandle:(IndyHandle)walletHandle
                                      senderDid:(NSString *)senderDid
                                    receiverDid:(NSString *)receiverDid
                                      isTimeout:(BOOL *)isTimeout;

@end

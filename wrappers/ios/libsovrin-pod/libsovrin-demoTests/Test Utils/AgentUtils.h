//
//  AgentUtils.h
//  libsovrin-demo
//
//  Created by Anastasia Tarasova on 22.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <libsovrin/libsovrin.h>

@interface AgentUtils : XCTestCase

+ (AgentUtils *)sharedInstance;

//- (NSError *)listenWithWalletHandle:(SovrinHandle) walletHandle
//                           endpoint:(NSString *)endpoint
//                       onConnection:(NSDictionary *)connection
//                          onMessage:(NSDictionary *)message
//                  outListenerHandle:(SovrinHandle *)listenerHandle;
//
//

@end

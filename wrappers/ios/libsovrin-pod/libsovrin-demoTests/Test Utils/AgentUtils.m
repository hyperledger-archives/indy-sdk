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

//- (NSError *)listenWithWalletHandle:(SovrinHandle) walletHandle
//                           endpoint:(NSString *)endpoint
//                       onConnection:(NSDictionary *)connection
//                          onMessage:(NSDictionary *)message
//                  outListenerHandle:(SovrinHandle *)listenerHandle
//{
//    
//}
//
//- (NSError *)sendWithConnectionHandler:(SovrinHandle)connectionHandle
//                               message:(NSString *)message
//{
//    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
//    __block NSError *err;
//    
//    
//}
@end

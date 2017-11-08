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

- (NSError *)prepareMsg:(NSData *)msg
       withWalletHandle:(IndyHandle)walletHandle
               senderVk:(NSString *)senderVk
            recipientVk:(NSString *)recipientVk
                 outMsg:(NSData **)outMsg
{
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;

    [IndyAgent prepareMsg:msg
         withWalletHandle:walletHandle
                 senderVk:senderVk
              recipientVk:recipientVk
               completion:^(NSError *error, NSData *encryptedMsg)
    {
        err = error;
        if (encryptedMsg) *outMsg= encryptedMsg;
        [completionExpectation fulfill];
    }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils shortTimeout]];

    return err;
}

@end

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

- (NSError *)prepareMsg:(NSData *)msg
       withWalletHandle:(IndyHandle)walletHandle
               senderVk:(NSString *)senderVk
            recipientVk:(NSString *)recipientVk
                 outMsg:(NSData **)outMsg;

- (NSError *)prepareAnonymousMsg:(NSData *)msg
                     recipientVk:(NSString *)recipientVk
                          outMsg:(NSData **)outMsg;

- (NSError *)parseMsg:(NSData *)msg
     withWalletHandle:(IndyHandle)walletHandle
          recipientVk:(NSString *)recipientVk
          outSenderVk:(NSString **)outSenderVk
               outMsg:(NSData **)outMsg;
@end

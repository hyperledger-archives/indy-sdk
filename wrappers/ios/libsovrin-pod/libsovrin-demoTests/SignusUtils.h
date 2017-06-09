//
//  SignusUtils.h
//  libsovrin-demo
//
//  Created by Anastasia Tarasova on 02.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <libsovrin/libsovrin.h>

@interface SignusUtils : XCTestCase

+ (SignusUtils *)sharedInstance;

- (NSError *) sign:(SovrinHandle)walletHandle
        theirDid:(NSString*)theirDid
         message:(NSString*)message
       outString:(NSString**)outString;

- (NSError *)createMyDidWithWalletHandle:(SovrinHandle)walletHandle
                               myDidJson:(NSString *)myDidJson
                                outMyDid:(NSString **)myDid
                             outMyVerkey:(NSString **)myVerkey
                                 outMyPk:(NSString **)myPk;

- (NSError *)storeTheirDid: (SovrinHandle) walletHandle
              identityJson: (NSString *)identityJson;
@end

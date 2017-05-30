//
//  WalletUtils.h
//  libsovrin-demo
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <libsovrin/libsovrin.h>

@interface WalletUtils : XCTestCase

+ (WalletUtils *)sharedInstance;

-(NSError*) createWallet:(NSString*) poolName
              walletName:(NSString*) walletName
                   xtype:(NSString*) xtype
                  handle:(SovrinHandle*) handle;

-(NSError*) walletSetSeqNoForValue:(SovrinHandle) walletHandle
                      claimDefUUID:(NSString*) uuid
                     claimDefSeqNo:(NSNumber*) seqNo;

@end

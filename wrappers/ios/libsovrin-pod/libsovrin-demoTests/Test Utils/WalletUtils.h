//
//  WalletUtils.h
//  libsovrin-demo
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <libsovrin/libsovrin.h>

@interface WalletUtils : XCTestCase

+ (WalletUtils *)sharedInstance;

- (NSError *)createAndOpenWalletWithPoolName:(NSString *)poolName
                                  walletName:(NSString *)walletName
                                       xtype:(NSString *)xtype
                                      handle:(SovrinHandle *)handle;

- (NSError *)createWalletWithPoolName:(NSString *)poolName
                           walletName:(NSString *)walletName
                                xtype:(NSString *)xtype
                               config:(NSString *)config;

- (NSError *)deleteWalletWithName:(NSString *)walletName;

- (NSError *)openWalletWithName:(NSString *)walletName
                         config:(NSString *)config
                      outHandle:(SovrinHandle *)handle;

- (NSError *)closeWalletWithHandle:(SovrinHandle)walletHandle;



- (NSError *)walletSetSeqNoForValue:(SovrinHandle)walletHandle
                       claimDefUUID:(NSString *)uuid
                      claimDefSeqNo:(NSNumber *)seqNo;

@end

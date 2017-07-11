//
//  WalletUtils.h
//  libindy-demo
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <libindy/libindy.h>

@interface WalletUtils : XCTestCase

+ (WalletUtils *)sharedInstance;

- (NSError *)createAndOpenWalletWithPoolName:(NSString *)poolName
                                  walletName:(NSString *)walletName
                                       xtype:(NSString *)xtype
                                      handle:(IndyHandle *)handle;

- (NSError *)createWalletWithPoolName:(NSString *)poolName
                           walletName:(NSString *)walletName
                                xtype:(NSString *)xtype
                               config:(NSString *)config;

- (NSError *)deleteWalletWithName:(NSString *)walletName;

- (NSError *)openWalletWithName:(NSString *)walletName
                         config:(NSString *)config
                      outHandle:(IndyHandle *)handle;

- (NSError *)closeWalletWithHandle:(IndyHandle)walletHandle;



- (NSError *)walletSetSeqNoForValue:(IndyHandle)walletHandle
                       claimDefUUID:(NSString *)uuid
                      claimDefSeqNo:(NSNumber *)seqNo;

@end

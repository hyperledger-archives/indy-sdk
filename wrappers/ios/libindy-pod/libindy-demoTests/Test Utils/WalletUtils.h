//
//  WalletUtils.h
//  libindy-demo
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <libindy/libindy.h>

@interface WalletUtils : XCTestCase

+ (WalletUtils *)sharedInstance;

- (NSError *)registerWalletType: (NSString *)xtype
                    forceCreate: (BOOL)forceCreate;

- (NSError *)createAndOpenWalletWithPoolName:(NSString *)poolName
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

@end

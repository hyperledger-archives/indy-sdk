//
//  WalletUtils.h
//  Indy-demo
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <Indy/Indy.h>

@interface WalletUtils : XCTestCase

+ (WalletUtils *)sharedInstance;

- (NSError *)registerWalletType:(NSString *)xtype;

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

- (NSError *)exportWalletWithHandle:(IndyHandle)walletHandle
                   exportConfigJson:(NSString *)exportConfigJson;


- (NSError *)importWalletWithPoolName:(NSString *)poolName
                           walletName:(NSString *)walletName
                                xtype:(NSString *)xtype
                               config:(NSString *)config
                     importConfigJson:(NSString *)importConfigJson;

@end

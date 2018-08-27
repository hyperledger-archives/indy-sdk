//
//  WalletUtils.h
//  Indy-demo
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <Indy/Indy.h>

@interface WalletUtils : XCTestCase

+ (WalletUtils *)sharedInstance;

- (NSError *)createAndOpenWalletWithHandle:(IndyHandle *)handle;

- (NSError *)createWalletWithConfig:(NSString *)config;

- (NSError *)deleteWalletWithConfig:(NSString *)config;

- (NSError *)openWalletWithConfig:(NSString *)config
                        outHandle:(IndyHandle *)handle;

- (NSError *)closeWalletWithHandle:(IndyHandle)walletHandle;

- (NSError *)exportWalletWithHandle:(IndyHandle)walletHandle
                   exportConfigJson:(NSString *)exportConfigJson;

- (NSError *)importWalletWithConfig:(NSString *)config
                   importConfigJson:(NSString *)importConfigJson;


- (NSError *)generateWalletKeyForConfig:(NSString *)configJson
                                    key:(NSString **)key;

@end

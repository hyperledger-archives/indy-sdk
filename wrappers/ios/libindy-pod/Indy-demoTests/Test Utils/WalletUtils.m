//
//  WalletUtils.m
//  Indy-demo
//

#import "WalletUtils.h"
#import <Indy/Indy.h>
#import "TestUtils.h"

@interface WalletUtils ()

@property(strong, readwrite) NSMutableArray *registeredWalletTypes;
@end

@implementation WalletUtils

NSString *credentials = @"{\"key\":\"key\"}";

+ (WalletUtils *)sharedInstance {
    static WalletUtils *instance = nil;
    static dispatch_once_t dispatch_once_block;

    dispatch_once(&dispatch_once_block, ^{
        instance = [WalletUtils new];
        instance.registeredWalletTypes = [NSMutableArray new];
    });

    return instance;
}

- (NSError *)registerWalletType:(NSString *)xtype {
    if ([self.registeredWalletTypes containsObject:xtype]) {
        return [NSError errorWithDomain:@"IndyErrorDomain" code:WalletTypeAlreadyRegisteredError userInfo:nil];;
    }

    __block NSError *err = nil;
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];


    [[IndyWallet sharedInstance] registerIndyKeychainWalletType:xtype
                                                     completion:^(NSError *error) {
                                                         err = error;
                                                         [completionExpectation fulfill];
                                                     }];

    [self.registeredWalletTypes addObject:xtype];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];

    return err;
}

- (NSError *)createAndOpenWalletWithHandle:(IndyHandle *)handle {
    __block NSError *err = nil;

    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    NSString *walletName = [NSString stringWithFormat:@"default-wallet-name-%lu", (unsigned long) [[SequenceUtils sharedInstance] getNextId]];
    NSString *config = [NSString stringWithFormat:@"{\"id\": \"%@\"}", walletName];

    [[IndyWallet sharedInstance] createWalletWithConfig:config
                                            credentials:credentials
                                             completion:^(NSError *error) {
                                                 err = error;
                                                 [completionExpectation fulfill];
                                             }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (err.code != Success) {
        return err;
    }

    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block IndyHandle walletHandle = 0;

    [[IndyWallet sharedInstance] openWalletWithConfig:config
                                          credentials:credentials
                                           completion:^(NSError *error, IndyHandle h) {
                                               err = error;
                                               walletHandle = h;
                                               [completionExpectation fulfill];
                                           }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];

    if (handle) {*handle = walletHandle;}

    return err;
}

- (NSError *)createWalletWithConfig:(NSString *)config {
    __block NSError *err = nil;

    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    [[IndyWallet sharedInstance] createWalletWithConfig:config
                                            credentials:credentials
                                             completion:^(NSError *error) {
                                                 err = error;
                                                 [completionExpectation fulfill];
                                             }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];
    return err;
}

- (NSError *)deleteWalletWithConfig:(NSString *)config {
    __block NSError *err;

    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    [[IndyWallet sharedInstance] deleteWalletWithConfig:config
                                            credentials:credentials
                                             completion:^(NSError *error) {
                                                 err = error;
                                                 [completionExpectation fulfill];
                                             }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];

    return err;
}

- (NSError *)openWalletWithConfig:(NSString *)config
                        outHandle:(IndyHandle *)handle {
    __block NSError *err;
    __block IndyHandle outHandle = 0;

    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    [[IndyWallet sharedInstance] openWalletWithConfig:config
                                          credentials:credentials
                                           completion:^(NSError *error, IndyHandle h) {
                                               err = error;
                                               outHandle = h;
                                               [completionExpectation fulfill];
                                           }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];

    if (handle) {*handle = outHandle;}
    return err;
}

- (NSError *)closeWalletWithHandle:(IndyHandle)walletHandle {
    __block NSError *err;

    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    [[IndyWallet sharedInstance] closeWalletWithHandle:walletHandle
                                            completion:^(NSError *error) {
                                                err = error;
                                                [completionExpectation fulfill];
                                            }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils shortTimeout]];

    return err;
}

- (NSError *)exportWalletWithHandle:(IndyHandle)walletHandle
                   exportConfigJson:(NSString *)exportConfigJson {
    __block NSError *err;

    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    [[IndyWallet sharedInstance] exportWalletWithHandle:walletHandle
                                       exportConfigJson:exportConfigJson
                                             completion:^(NSError *error) {
                                                 err = error;
                                                 [completionExpectation fulfill];
                                             }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    return err;
}

- (NSError *)importWalletWithConfig:(NSString *)config
                   importConfigJson:(NSString *)importConfigJson {
    __block NSError *err = nil;

    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    [[IndyWallet sharedInstance] importWalletWithConfig:config
                                            credentials:credentials
                                       importConfigJson:importConfigJson
                                             completion:^(NSError *error) {
                                                 err = error;
                                                 [completionExpectation fulfill];
                                             }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];
    return err;
}

@end

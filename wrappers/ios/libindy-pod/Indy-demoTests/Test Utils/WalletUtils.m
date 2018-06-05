//
//  WalletUtils.m
//  Indy-demo
//

#import "WalletUtils.h"
#import <Indy/Indy.h>
#import "TestUtils.h"

@interface  WalletUtils()

@property (strong, readwrite) NSMutableArray *registeredWalletTypes;
@end

@implementation WalletUtils

NSString *credentials = @"{\"key\":\"key\"}";

+ (WalletUtils *)sharedInstance
{
    static WalletUtils *instance = nil;
    static dispatch_once_t dispatch_once_block;
    
    dispatch_once(&dispatch_once_block, ^ {
        instance = [WalletUtils new];
        instance.registeredWalletTypes = [NSMutableArray new];
    });
    
    return instance;
}

- (NSError *)registerWalletType: (NSString *)xtype
{
    if ([self.registeredWalletTypes containsObject:xtype])
    {
        return [NSError errorWithDomain:@"IndyErrorDomain" code: WalletTypeAlreadyRegisteredError userInfo:nil];;
    }
    
    __block NSError *err = nil;
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    
    [[IndyWallet sharedInstance] registerIndyKeychainWalletType:xtype
                                                     completion:^(NSError* error)
           {
               err = error;
               [completionExpectation fulfill];
           }];
    
    [self.registeredWalletTypes addObject:xtype];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    return err;
}

-(NSError *)createAndOpenWalletWithPoolName:(NSString *) poolName
                                      xtype:(NSString *) xtype
                                     handle:(IndyHandle *) handle
{
    __block NSError *err = nil;
    
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    NSString *walletName = [NSString stringWithFormat:@"default-wallet-name-%lu", (unsigned long)[[SequenceUtils sharedInstance] getNextId]];
    NSString *xTypeStr = (xtype) ? xtype : @"default";

    [[IndyWallet sharedInstance] createWalletWithName:  walletName
                                             poolName:  poolName
                                                 type:  xTypeStr
                                               config:  nil
                                          credentials:  credentials
                                           completion: ^(NSError* error)
           {
               err = error;
               [completionExpectation fulfill];
           }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    if( err.code != Success)
    {
        return err;
    }
    
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block IndyHandle walletHandle = 0;
    
    [[IndyWallet sharedInstance] openWalletWithName:walletName
                                      runtimeConfig:nil
                                        credentials:credentials
                                         completion:^(NSError *error, IndyHandle h)
           {
               err = error;
               walletHandle = h;
               [completionExpectation fulfill];
           }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    if (handle) { *handle = walletHandle; }
    
    return err;
}

- (NSError *)createWalletWithPoolName:(NSString *)poolName
                           walletName:(NSString *)walletName
                                xtype:(NSString *)xtype
                               config:(NSString *)config
{
    __block NSError *err = nil;
    
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];

    [[IndyWallet sharedInstance] createWalletWithName:  walletName
                                             poolName:  poolName
                                                 type:  xtype
                                               config:  config
                                          credentials:  credentials
                                           completion: ^(NSError *error)
           {
               err = error;
               [completionExpectation fulfill];
           }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    return err;
}

- (NSError *)deleteWalletWithName:(NSString *)walletName
{
    __block NSError *err;
    
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    [[IndyWallet sharedInstance] deleteWalletWithName:walletName
                                          credentials:credentials
                                           completion:^(NSError *error)
           {
               err = error;
               [completionExpectation fulfill];
           }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    return err;
}

- (NSError *)openWalletWithName:(NSString *)walletName
                         config:(NSString *)config
                      outHandle:(IndyHandle *)handle
{
    __block NSError *err;
    __block IndyHandle outHandle = 0;
    
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];

    [[IndyWallet sharedInstance] openWalletWithName:walletName
                                      runtimeConfig:config
                                        credentials:credentials
                                         completion:^(NSError *error, IndyHandle h)
           {
               err = error;
               outHandle = h;
               [completionExpectation fulfill];
           }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    if (handle) { *handle = outHandle; }
    return err;
}

- (NSError *)closeWalletWithHandle:(IndyHandle)walletHandle
{
    __block NSError *err;
    
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    [[IndyWallet sharedInstance] closeWalletWithHandle:walletHandle
                                            completion:^(NSError *error)
     {
         err = error;
         [completionExpectation fulfill];
     }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils shortTimeout]];
    
    return err;
}

@end

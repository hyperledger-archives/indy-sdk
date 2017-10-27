//
//  PairwiseUtils.m
//  Indy-demo
//

#import "PairwiseUtils.h"
#import "TestUtils.h"

@implementation PairwiseUtils

+ (PairwiseUtils *)sharedInstance
{
    static PairwiseUtils *instance = nil;
    static dispatch_once_t dispatch_once_block;
    
    dispatch_once(&dispatch_once_block, ^{
        instance = [PairwiseUtils new];
    });
    
    return instance;
}

- (NSError *)pairwiseExistsForDid:(NSString *)theirDid
                     walletHandle:(IndyHandle)walletHandle
                        outExists:(BOOL *)exists
{
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err = nil;
    
    [IndyPairwise isPairwiseExistsForDid:theirDid
                            walletHandle:walletHandle
                              completion:^(NSError* error, BOOL isExists)
     {
         err = error;
         if (exists) {*exists = isExists;}
         [completionExpectation fulfill];
     }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    return err;
}

- (NSError *)createPairwiseForTheirDid:(NSString *)theirDid
                             withMyDid:(NSString *)myDid
                              metadata:(NSString *)metadata
                          walletHandle:(IndyHandle)walletHandle
{
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err = nil;
    
    [IndyPairwise createPairwiseForTheirDid:theirDid
                                      myDid:myDid
                                   metadata:metadata
                               walletHandle:walletHandle
                                 completion:^(NSError *error) {
                                     err = error;
                                     [completionExpectation fulfill];
                                 }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    return err;
}

- (NSError *)listPairwiseFromWallet:(IndyHandle)walletHandle
                    outPairwiseList:(NSString**)pairwiseList
{
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err = nil;

    [IndyPairwise listPairwiseFromWalletHandle:walletHandle
                                    completion:^(NSError *error, NSString *listPairwise) {
                                        err = error;
                                        if (pairwiseList) {*pairwiseList = listPairwise;}
                                        [completionExpectation fulfill];
                                    }];
  
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    return err;
}

- (NSError *)getPairwiseForDid:(NSString *)theirDid
                  walletHandle:(IndyHandle)walletHandle
               outPairwiseJson:(NSString**)pairwiseJson
{
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err = nil;
    
    [IndyPairwise getPairwiseForTheirDid:theirDid
                            walletHandle:walletHandle
                              completion:^(NSError *error, NSString *pairwiseInfoJson) {
                                  err = error;
                                  if (pairwiseJson) {*pairwiseJson = pairwiseInfoJson;}
                                  [completionExpectation fulfill];
                              }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    return err;
}

- (NSError *)setPairwiseMetadata:(NSString *)metadata
                     forTheirDid:(NSString *)theirDid
                    walletHandle:(IndyHandle)walletHandle
{
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSError *err = nil;
    
    [IndyPairwise setPairwiseMetadata:metadata
                          forTheirDid:theirDid
                         walletHandle:walletHandle
                           completion:^(NSError *error) {
                               err = error;
                               [completionExpectation fulfill];
                           }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    return err;
}

@end

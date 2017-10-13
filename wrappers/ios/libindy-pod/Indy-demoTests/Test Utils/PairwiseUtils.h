//
//  PairwiseUtils.h
//  Indy-demo
//
//  Created by Anastasia Tarasova on 04/10/2017.
//  Copyright © 2017 Hyperledger. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <Indy/Indy.h>

@interface PairwiseUtils : XCTestCase

+ (PairwiseUtils *)sharedInstance;

- (NSError *)pairwiseExistsForDid:(NSString *)theirDid
                     walletHandle:(IndyHandle)walletHandle
                        outExists:(BOOL *)exists;

- (NSError *)createPairwiseForTheirDid:(NSString *)theirDid
                             withMyDid:(NSString *)myDid
                              metadata:(NSString *)metadata
                          walletHandle:(IndyHandle)walletHandle;

- (NSError *)listPairwiseFromWallet:(IndyHandle)walletHandle
                    outPairwiseList:(NSString**)pairwiseList;

- (NSError *)getPairwiseForDid:(NSString *)theirDid
                  walletHandle:(IndyHandle)walletHandle
               outPairwiseJson:(NSString**)pairwiseJson;

- (NSError *)setPairwiseMetadata:(NSString *)metadata
                     forTheirDid:(NSString *)theirDid
                    walletHandle:(IndyHandle)walletHandle;

@end

//
//  PoolUtils.h
//  Indy-demo
//
//  Created by Kirill Neznamov on 15/05/2017.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <Indy/Indy.h>

@interface PoolUtils : XCTestCase

+ (PoolUtils *)sharedInstance;

- (NSNumber *)getRequestId;

// MARK: - Txn file
- (NSString *)createGenesisTxnFileForTestPool:(NSString *)poolName
                                   nodesCount:(NSNumber *)nodesCount
                                  txnFilePath:(NSString *)txnFilePath;

- (NSString *)createGenesisTxnFileWithPoolName:(NSString *)poolName
                                   txnFileData:(NSString *)txnFileData
                                   txnFilePath:(NSString *)txnFilePath;

// MARK: - Config

- (NSString *)poolConfigJsonForTxnFilePath:(NSString *)txnFilePath;

- (NSString *)createDefaultPoolConfig:(NSString *)poolName
                          txnFileData:(NSString *)txnFileData;

- (NSError *)createPoolLedgerConfigWithPoolName:(NSString *)poolName
                                     poolConfig:(NSString *)config;

// MARK: - Pool ledger

- (NSError *)openPoolLedger:(NSString *)poolName
                     config:(NSString *)config
                poolHandler:(IndyHandle *)handle;

- (NSError *)createAndOpenPoolLedgerWithPoolName:(NSString *)poolName
                                      poolHandle:(IndyHandle *)handle;

// MARK: - Actions

- (NSError *)refreshPoolHandle:(IndyHandle)poolHandle;

- (NSError *)closeHandle:(IndyHandle)poolHandle;

- (NSError *)deletePoolWithName:(NSString *)poolName;

- (NSError *)setProtocolVersion:(NSNumber *)protocolVersion;

- (NSError *)sendRequestWithPoolHandle:(IndyHandle)poolHandle
                               request:(NSString *)request
                              response:(NSString **)response;

@end

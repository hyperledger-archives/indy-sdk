//
//  PoolUtils.h
//  libindy-demo
//
//  Created by Kirill Neznamov on 15/05/2017.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <libindy/libindy.h>

@interface PoolUtils : XCTestCase

+ (PoolUtils *)sharedInstance;

- (NSNumber *)getRequestId;

- (NSString *)createDefaultPoolConfig:(NSString *)poolName;

- (NSError *)createPoolLedgerConfigWithPoolName:(NSString *)poolName
                                          nodes:(NSString *)nodes
                                     poolConfig:(NSString *)config
                                 genTxnFileName:(NSString *)genTxnFileName;

- (NSError*)createAndOpenPoolLedgerConfigWithName: (NSString *) poolName
                                       poolHandle: (IndyHandle *) handle;

- (NSError *)openPoolLedger:(NSString*)poolName
                     config:(NSString*)config
                poolHandler:(IndyHandle*)handle;

- (NSError *)sendRequestWithPoolHandle:(IndyHandle)poolHandle
                               request:(NSString *)request
                              response:(NSString **)response;

- (NSError *)refreshPoolHandle:(IndyHandle)poolHandle;

- (NSError *)closeHandle:(IndyHandle)poolHandle;

- (NSError *)deletePoolWithName:(NSString *)poolName;

+ (NSString *) nodeIp;

@end

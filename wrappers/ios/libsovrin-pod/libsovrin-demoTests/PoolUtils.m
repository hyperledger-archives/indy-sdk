//
//  PoolUtils.m
//  libsovrin-demo
//
//  Created by Kirill Neznamov on 15/05/2017.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import "PoolUtils.h"
#import "TestUtils.h"
#import <libsovrin/libsovrin.h>
#import <XCTest/XCTest.h>

@implementation PoolUtils

+ (PoolUtils *)sharedInstance
{
    static PoolUtils *instance = nil;
    static dispatch_once_t dispatch_once_block;

    dispatch_once(&dispatch_once_block, ^ {
        instance = [PoolUtils new];
    });

    return instance;
}

+ (NSString *) nodeIp
{
    //return @"192.168.53.190";
    return @"192.168.52.38";
}

- (void)createGenesisTXNFile:(NSString *)poolName
{
    NSString *nodeIp = [PoolUtils nodeIp];
    NSString *genesisTXNs = [NSString stringWithFormat: @"{\"data\":{\"alias\":\"Node1\",\"client_ip\":\"%@\",\"client_port\":9702,\"node_ip\":\"%@\",\"node_"
                             "port\":9701,\"services\":[\"VALIDATOR\"]},\"dest\":\"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv\","
                             "\"identifier\":\"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4\",\"txnId\":"
                             "\"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62\",\"type\":\"0\"}\n"
                             "{\"data\":{\"alias\":\"Node2\",\"client_ip\":\"%@\",\"client_port\":9704,\"node_ip\":\"%@\",\"node_"
                             "port\":9703,\"services\":[\"VALIDATOR\"]},\"dest\":\"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb\","
                             "\"identifier\":\"8QhFxKxyaFsJy4CyxeYX34dFH8oWqyBv1P4HLQCsoeLy\",\"txnId\":"
                             "\"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc\",\"type\":\"0\"}\n"
                             "{\"data\":{\"alias\":\"Node3\",\"client_ip\":\"%@\",\"client_port\":9706,\"node_ip\":\"%@\",\"node_"
                             "port\":9705,\"services\":[\"VALIDATOR\"]},\"dest\":\"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya\","
                             "\"identifier\":\"2yAeV5ftuasWNgQwVYzeHeTuM7LwwNtPR3Zg9N4JiDgF\",\"txnId\":"
                             "\"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4\",\"type\":\"0\"}\n"
                             "{\"data\":{\"alias\":\"Node4\",\"client_ip\":\"%@\",\"client_port\":9708,\"node_ip\":\"%@\",\"node_"
                             "port\":9707,\"services\":[\"VALIDATOR\"]},\"dest\":\"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA\","
                             "\"identifier\":\"FTE95CVthRtrBnK2PYCBbC9LghTcGwi9Zfi1Gz2dnyNx\",\"txnId\":"
                             "\"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008\",\"type\":\"0\"}\n", nodeIp, nodeIp, nodeIp, nodeIp, nodeIp, nodeIp, nodeIp, nodeIp];
    
    [[NSFileManager defaultManager] createFileAtPath:[NSString stringWithFormat:@"%@/%@.txn", [TestUtils getUserTmpDir], poolName]
                                            contents:[NSData dataWithBytes:[genesisTXNs UTF8String] length:[genesisTXNs length]]
                                          attributes:nil];
}

- (NSString *)createPoolConfig:(NSString *)poolName
{
    NSString *filePath = [NSString stringWithFormat:@"%@%@.txn", [TestUtils getUserTmpDir], poolName];
    return [NSString stringWithFormat:@"{\"genesis_txn\":\"%@\"}", filePath];
}

- (NSError *)createPoolLedgerConfig:(NSString *)poolName
{
    NSError *ret = nil;
    [self createGenesisTXNFile:poolName];
    NSString *config = [self createPoolConfig:poolName];

    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *ret2 = nil;

    ret = [SovrinPool createPoolWithName:poolName
                               andConfig:config
                              completion:^ (NSError *error)
    {
        ret2 = error;
        [completionExpectation fulfill];
    }];
    
    if (ret.code != Success)
    {
        return ret;
    }

    [self waitForExpectations:@[ completionExpectation ] timeout:[TestUtils defaultTimeout]];

    return ret2;
}

- (NSError *)openPoolLedger:(NSString*)poolName
                     config:(NSString*)config
                poolHandler:(SovrinHandle*)handle
{
    NSError *ret = nil;
      XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block SovrinHandle poolHandle = 0;
    
    ret = [SovrinPool openPoolWithName:poolName
                             andConfig:config
                            completion:^(NSError *error, SovrinHandle handle)
           {
               err = error;
               poolHandle = handle;
               [completionExpectation fulfill];
           }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    if( ret.code != Success )
    {
        return ret;
    }
    
    *handle = poolHandle;
    return err;
}


- (NSError*)createAndOpenPoolLedgerConfigWithName: (NSString *) poolName
                                       poolHandle: (SovrinHandle *) handle
{
    NSError *ret = nil;
    
    // 1. Generate Genesis file
    [self createGenesisTXNFile:poolName];
    
    // 2. Create pool config
    NSString *poolConfig = [self createPoolConfig:poolName];
    
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *closureError = nil;
    
    // 3. Create pool ledger config
    ret = [SovrinPool createPoolWithName:poolName
                               andConfig:poolConfig
                              completion:^ (NSError *error)
           {
               closureError = error;
               [completionExpectation fulfill];
           }];
    
    if (ret.code != Success)
    {
        return ret;
    }
    
    [self waitForExpectations:@[ completionExpectation ] timeout:[TestUtils defaultTimeout]];
    
    if (closureError.code != Success)
    {
        return closureError;
    }
    
    // 4. Open Pool ledger config
    
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block SovrinHandle poolHandle = 0;
    
    ret = [SovrinPool openPoolWithName:poolName
                             andConfig:poolConfig
                            completion:^ (NSError* error, SovrinHandle handle)
           {
               closureError = error;
               poolHandle = handle;
               [completionExpectation fulfill];
           }];
   
    [self waitForExpectations:@[ completionExpectation ] timeout:[TestUtils shortTimeout]];
    
    if (ret.code != Success)
    {
        return ret;
    }
    
    *handle = poolHandle;
    return closureError;
}

- (NSError *)sendRequestWithPoolHandle:(SovrinHandle)poolHandle
                               request:(NSString *)request
                              response:(NSString **)response
{
    
    NSError *ret = nil;
    
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString* outResponse = nil;
    
    
    ret = [SovrinLedger submitRequest:poolHandle
                          requestJSON:request
                           completion:^(NSError* error, NSString* result)
    {
        err = error;
        outResponse = result;
        [completionExpectation fulfill];
    }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    if( ret.code != Success )
    {
        return ret;
    }
    
    *response = outResponse;
    return err;
}

@end

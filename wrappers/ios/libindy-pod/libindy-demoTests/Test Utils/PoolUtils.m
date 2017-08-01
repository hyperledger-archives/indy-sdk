//
//  PoolUtils.m
//  libindy-demo
//
//  Created by Kirill Neznamov on 15/05/2017.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import "PoolUtils.h"
#import "TestUtils.h"
#import <libindy/libindy.h>
#import <XCTest/XCTest.h>

@interface PoolUtils ()

@property (assign) int requestIdOffset;

@end


@implementation PoolUtils

+ (PoolUtils *)sharedInstance
{
    static PoolUtils *instance = nil;
    static dispatch_once_t dispatch_once_block;

    dispatch_once(&dispatch_once_block, ^ {
        instance = [PoolUtils new];
        instance.requestIdOffset = 1;
    });

    return instance;
}

+ (NSString *) nodeIp
{
    //return @"192.168.53.190";
    //return @"192.168.52.38";
    return @"127.0.0.1";
    //return @"10.0.0.2";
}

- (NSNumber *) getRequestId
{
    NSTimeInterval timeInSeconds = [[NSDate date] timeIntervalSince1970];
    return @(timeInSeconds + self.requestIdOffset++);
}

- (void)createGenesisTXNFile:(NSString *)fileName
              predefinedData:(NSString *)predefinedData
{
    NSString *nodeIp = [PoolUtils nodeIp];
    NSString *node1 = [NSString stringWithFormat:@"{"
                       "\"data\":{"
                            "\"alias\":\"Node1\","
                            "\"client_ip\":\"%@\","
                            "\"client_port\":9702,"
                            "\"node_ip\":\"%@\","
                            "\"node_port\":9701,"
                            "\"services\":[\"VALIDATOR\"]},"
                       "\"dest\":\"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv\","
                       "\"identifier\":\"Th7MpTaRZVRYnPiabds81Y\","
                       "\"txnId\":\"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62\","
                       "\"type\":\"0\"}", nodeIp, nodeIp];
    
    NSString *node2 = [NSString stringWithFormat:@"{"
                       "\"data\":{"
                            "\"alias\":\"Node2\","
                            "\"client_ip\":\"%@\","
                            "\"client_port\":9704,"
                            "\"node_ip\":\"%@\","
                            "\"node_port\":9703,"
                            "\"services\":[\"VALIDATOR\"]},"
                       "\"dest\":\"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb\","
                       "\"identifier\":\"EbP4aYNeTHL6q385GuVpRV\","
                       "\"txnId\":\"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc\","
                       "\"type\":\"0\"}", nodeIp, nodeIp];
    
    NSString *node3 = [NSString stringWithFormat:@"{"
                       "\"data\":{"
                            "\"alias\":\"Node3\","
                            "\"client_ip\":\"%@\","
                            "\"client_port\":9706,"
                            "\"node_ip\":\"%@\","
                            "\"node_port\":9705,"
                            "\"services\":[\"VALIDATOR\"]},"
                       "\"dest\":\"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya\","
                       "\"identifier\":\"4cU41vWW82ArfxJxHkzXPG\","
                       "\"txnId\":\"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4\","
                       "\"type\":\"0\"}", nodeIp, nodeIp];
    
    NSString *node4 = [NSString stringWithFormat:@"{"
                       "\"data\":{"
                       "\"alias\":\"Node4\","
                       "\"client_ip\":\"%@\","
                       "\"client_port\":9708,"
                       "\"node_ip\":\"%@\","
                       "\"node_port\":9707,"
                       "\"services\":[\"VALIDATOR\"]},"
                       "\"dest\":\"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA\","
                       "\"identifier\":\"TWwCRQRZ2ZHMJFn9TzLp7W\","
                       "\"txnId\":\"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008\","
                       "\"type\":\"0\"}", nodeIp, nodeIp];
    
    
    NSString *genesisTXNs = [NSString stringWithFormat: @"%@\n%@\n%@\n%@\n", node1, node2, node3, node4];
    
    genesisTXNs = (predefinedData) ? predefinedData : genesisTXNs;
    
    [[NSFileManager defaultManager] createFileAtPath:[NSString stringWithFormat:@"%@/%@.txn", [TestUtils getUserTmpDir], fileName]
                                            contents:[NSData dataWithBytes:[genesisTXNs UTF8String] length:[genesisTXNs length]]
                                          attributes:nil];
}

- (NSString *)createDefaultPoolConfig:(NSString *)poolName
{
    NSString *filePath = [NSString stringWithFormat:@"%@%@.txn", [TestUtils getUserTmpDir], poolName];
    return [NSString stringWithFormat:@"{\"genesis_txn\":\"%@\"}", filePath];
}

- (NSError *)createPoolLedgerConfigWithPoolName:(NSString *)poolName
                                          nodes:(NSString *)nodes
                                     poolConfig:(NSString *)config
                                 genTxnFileName:(NSString *)genTxnFileName

{
    NSError *ret = nil;
    NSString *fileName = (genTxnFileName) ? genTxnFileName : poolName;
    [self createGenesisTXNFile:fileName predefinedData: nodes];
    NSString *configStr = (config) ? config : [self createDefaultPoolConfig:poolName];

    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *ret2 = nil;

    ret = [IndyPool createPoolLedgerConfigWithPoolName:poolName
                                            poolConfig:configStr
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
                poolHandler:(IndyHandle*)handle
{
    NSError *ret = nil;
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block IndyHandle poolHandle = 0;
    
    ret = [IndyPool openPoolLedgerWithName:poolName
                                poolConfig:config
                                completion:^(NSError *error, IndyHandle blockHandle)
           {
               err = error;
               poolHandle = blockHandle;
               [completionExpectation fulfill];
           }];
    
    if( ret.code != Success )
    {
        return ret;
    }
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    if (handle) { *handle = poolHandle; }
    return err;
}


- (NSError*)createAndOpenPoolLedgerConfigWithName: (NSString *) poolName
                                       poolHandle: (IndyHandle*) handle
{
    NSError *ret;
    ret = [self createPoolLedgerConfigWithPoolName:poolName
                                             nodes:nil
                                        poolConfig:nil
                                    genTxnFileName:nil];
    
    if (ret.code != Success)
    {
        return ret;
    }
    
    ret = [self openPoolLedger:poolName
                        config:nil
                   poolHandler:handle];
    return ret;
}

- (NSError *)sendRequestWithPoolHandle:(IndyHandle)poolHandle
                               request:(NSString *)request
                              response:(NSString **)response
{
    
    NSError *ret = nil;
    
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString* outResponse = nil;
    
    
    ret = [IndyLedger submitRequestWithPoolHandle:poolHandle
                                        requestJSON:request
                                         completion:^(NSError* error, NSString* result)
    {
        err = error;
        outResponse = result;
        [completionExpectation fulfill];
    }];
    
    if( ret.code != Success )
    {
        return ret;
    }
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    if (response){ *response = outResponse; }
    return err;
}

- (NSError *)refreshPoolHandle:(IndyHandle)poolHandle
{
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    
    
    NSError * ret = [IndyPool refreshPoolLedgerWithHandle:poolHandle
                                                 completion:^(NSError* error)
                     {
                         err = error;
                         [completionExpectation fulfill];
                     }];
    
    if( ret.code != Success )
    {
        return ret;
    }
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    return err;
}

- (NSError *)closeHandle:(IndyHandle)poolHandle
{
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    
    
    NSError * ret = [IndyPool closePoolLedgerWithHandle:poolHandle
                                               completion:^(NSError* error)
                     {
                         err = error;
                         [completionExpectation fulfill];
                     }];
    
    if( ret.code != Success )
    {
        return ret;
    }
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    return err;
}

- (NSError *)deletePoolWithName:(NSString *)poolName
{
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    
    
    NSError * ret = [IndyPool deletePoolLedgerConfigWithName:poolName
                                                    completion:^(NSError* error)
                     {
                         err = error;
                         [completionExpectation fulfill];
                     }];
    
    if( ret.code != Success )
    {
        return ret;
    }
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    return err;

}
@end

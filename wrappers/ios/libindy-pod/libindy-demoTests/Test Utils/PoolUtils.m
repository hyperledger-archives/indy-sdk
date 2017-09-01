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


- (NSNumber *) getRequestId
{
    NSTimeInterval timeInSeconds = [[NSDate date] timeIntervalSince1970];
    return @(timeInSeconds + self.requestIdOffset++);
}

// MARK: - TXN File

- (NSString *)createGenesisTxnFileForTestPool:(NSString *)poolName
                             nodesCount:(NSNumber *)nodesCount
                            txnFilePath:(NSString *)txnFilePath
{
    int nodes = (nodesCount != nil) ? [nodesCount intValue] : 4;
    if (nodes <= 0 || nodes > 4)
    {
        return nil;
    }
    
    NSString *nodeIp = [TestUtils testPoolIp];
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
    
    
    NSArray *nodesArray = [NSArray arrayWithObjects:node1, node2, node3, node4, nil];
    NSArray *requiredNodes = [nodesArray subarrayWithRange:NSMakeRange(0, nodes)];
   
    NSString *genesisTXNs = [requiredNodes componentsJoinedByString:@"\n"];
    
    return [self createGenesisTxnFileWithPoolName:poolName
                                      txnFileData:genesisTXNs
                                      txnFilePath:txnFilePath];
}


- (NSString *)createGenesisTxnFileWithPoolName:(NSString *)poolName
                                   txnFileData:(NSString *)txnFileData
                                   txnFilePath:(NSString *)txnFilePath
{
    NSString *filePath;// = [NSString stringWithFormat:@"%@/%@.txn", [TestUtils getUserTmpDir], txnFilePath];
    if (!txnFilePath)
    {
        filePath = [NSString stringWithFormat:@"%@%@.txn", [TestUtils getUserTmpDir], poolName];
    }
    else
    {
        filePath = txnFilePath;
    }


    BOOL isSuccess =  [[NSFileManager defaultManager] createFileAtPath:filePath
                                             contents:[NSData dataWithBytes:[txnFileData UTF8String] length:[txnFileData length]]
                                           attributes:nil];
    
    if (isSuccess)
    {
        return filePath;
    }
    
    return nil;
}


- (NSString *)createGenesisTxnFileForTestPoolWithInvalidNodesForPoolName:(NSString *)poolName
                                                             txnFilePath:(NSString *)txnFilePath
{
    NSString *testPoolIp = [TestUtils testPoolIp];
    NSString *node1 = [NSString stringWithFormat:@"{"
                       "\"data\":{"
                       "\"client_ip\":\"%@\","
                       "\"client_port\":9702,"
                       "\"node_ip\":\"%@\","
                       "\"node_port\":9701,"
                       "\"services\":[\"VALIDATOR\"]},"
                       "\"dest\":\"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv\","
                       "\"identifier\":\"Th7MpTaRZVRYnPiabds81Y\","
                       "\"txnId\":\"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62\","
                       "\"type\":\"0\"}", testPoolIp, testPoolIp];
    
    NSString *node2 = [NSString stringWithFormat:@"{"
                       "\"data\":{"
                       "\"client_ip\":\"%@\","
                       "\"client_port\":9704,"
                       "\"node_ip\":\"%@\","
                       "\"node_port\":9703,"
                       "\"services\":[\"VALIDATOR\"]},"
                       "\"dest\":\"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb\","
                       "\"identifier\":\"EbP4aYNeTHL6q385GuVpRV\","
                       "\"txnId\":\"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc\","
                       "\"type\":\"0\"}", testPoolIp, testPoolIp];
    
    NSString *node3 = [NSString stringWithFormat:@"{"
                       "\"data\":{"
                       "\"client_ip\":\"%@\","
                       "\"client_port\":9706,"
                       "\"node_ip\":\"%@\","
                       "\"node_port\":9705,"
                       "\"services\":[\"VALIDATOR\"]},"
                       "\"dest\":\"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya\","
                       "\"identifier\":\"4cU41vWW82ArfxJxHkzXPG\","
                       "\"txnId\":\"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4\","
                       "\"type\":\"0\"}", testPoolIp, testPoolIp];
    
    NSString *node4 = [NSString stringWithFormat:@"{"
                       "\"data\":{"
                       "\"client_ip\":\"%@\","
                       "\"client_port\":9708,"
                       "\"node_ip\":\"%@\","
                       "\"node_port\":9707,"
                       "\"services\":[\"VALIDATOR\"]},"
                       "\"dest\":\"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA\","
                       "\"identifier\":\"TWwCRQRZ2ZHMJFn9TzLp7W\","
                       "\"txnId\":\"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008\","
                       "\"type\":\"0\"}", testPoolIp, testPoolIp];
    
    
    NSString *genesisTXNs = [NSString stringWithFormat: @"%@\n%@\n%@\n%@\n", node1, node2, node3, node4];
    
    return [self createGenesisTxnFileWithPoolName:poolName
                                      txnFileData:genesisTXNs
                                      txnFilePath:txnFilePath];
    
}

- (NSString *)createGenesisTxnFileForTestPoolWithWrongAliasForPoolName:(NSString *)poolName
                                                            txnFilePath:(NSString *)txnFilePath
{
    NSString *testPoolIp = [TestUtils testPoolIp];
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
                       "\"type\":\"0\"}", testPoolIp, testPoolIp];
    
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
                       "\"type\":\"0\"}", testPoolIp, testPoolIp];
    
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
                       "\"type\":\"0\"}", testPoolIp, testPoolIp];
    
    NSString *node4 = [NSString stringWithFormat:@"{"
                       "\"data\":{"
                            "\"alias\":\"ALIAS_NODE\","
                            "\"client_ip\":\"%@\","
                            "\"client_port\":9708,"
                            "\"node_ip\":\"%@\","
                            "\"node_port\":9707,"
                            "\"services\":[\"VALIDATOR\"]},"
                       "\"dest\":\"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA\","
                       "\"identifier\":\"TWwCRQRZ2ZHMJFn9TzLp7W\","
                       "\"txnId\":\"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008\","
                       "\"type\":\"0\"}", testPoolIp, testPoolIp];
    
    
    NSString *genesisTXNs = [NSString stringWithFormat: @"%@\n%@\n%@\n%@\n", node1, node2, node3, node4];
    
    return [self createGenesisTxnFileWithPoolName:poolName
                                      txnFileData:genesisTXNs
                                      txnFilePath:txnFilePath];

}

// MARK: - Config

// Note that to be config valid it assumes genesis txt file is already exists
- (NSString *)poolConfigJsonForTxnFilePath:(NSString *)txnFilePath
{
    NSString *config = [NSString stringWithFormat:@"{\"genesis_txn\":\"%@\"}", txnFilePath];
    
    return config;
}

- (NSString *)createDefaultPoolConfig:(NSString *)poolName
                          txnFileData:(NSString *)txnFileData

{
    NSString *filePath = [NSString stringWithFormat:@"%@%@.txn", [TestUtils getUserTmpDir], poolName];
    return [NSString stringWithFormat:@"{\"genesis_txn\":\"%@\"}", filePath];
}

- (NSError *)createPoolLedgerConfigWithPoolName:(NSString *)poolName
                                     poolConfig:(NSString *)config

{
    NSError *ret = nil;
    NSString *configStr = (config) ? config : @"";

    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;

    ret = [IndyPool createPoolLedgerConfigWithPoolName:poolName
                                            poolConfig:configStr
                                            completion:^ (NSError *error)
           {
               err = error;
               [completionExpectation fulfill];
           }];
    
    if (ret.code != Success)
    {
        return ret;
    }

    [self waitForExpectations:@[ completionExpectation ] timeout:[TestUtils shortTimeout]];

    return err;
}

// MARK: - Pool ledger

- (NSError *)openPoolLedger:(NSString*)poolName
                     config:(NSString*)config
                poolHandler:(IndyHandle*)handle
{
    NSError *ret = nil;
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block IndyHandle poolHandle = 0;
    
    NSString *configStr = (config) ? config : @"";
    
    ret = [IndyPool openPoolLedgerWithName:poolName
                                poolConfig:configStr
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


- (NSError*)createAndOpenPoolLedgerWithPoolName: (NSString *) poolName
                                     poolHandle: (IndyHandle*) handle
{
    NSError *ret;
    
    NSString *txnFilePath = [self createGenesisTxnFileForTestPool:poolName
                                                       nodesCount:nil
                                                      txnFilePath:nil];
    
    NSString *poolConfig = [self poolConfigJsonForTxnFilePath:txnFilePath];
    
    ret = [self createPoolLedgerConfigWithPoolName:poolName
                                        poolConfig:poolConfig];
    
    ret = [self openPoolLedger:poolName
                        config:nil
                   poolHandler:handle];
    return ret;
}

// MARK: - Actions

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
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils shortTimeout]];
    
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

@end

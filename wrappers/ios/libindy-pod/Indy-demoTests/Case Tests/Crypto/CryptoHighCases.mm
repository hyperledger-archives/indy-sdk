//
// Created by DSR on 07/11/2017.
// Copyright (c) 2017 Hyperledger. All rights reserved.
//


#import <XCTest/XCTestCase.h>
#import "TestUtils.h"
#import "CryptoUtils.h"

@interface CryptoHighCases : XCTestCase

@end

@implementation CryptoHighCases

// MARK: - Create key

- (void)testCreateKeyWorksWithSeed
{
    [TestUtils cleanupStorage];

    IndyHandle walletHandle = 0;
    NSError *ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                           xtype:nil
                                                                          handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");

    NSString *verkey = nil;
    ret = [[CryptoUtils sharedInstance] createKeyWithWalletHandle:walletHandle
                                                          keyJson:[NSString stringWithFormat:@"{\"seed\":\"%@\"}", [TestUtils mySeed1]]
                                                        outVerkey:&verkey];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:createKeyWithWalletHandle failed");
    XCTAssertEqualObjects(verkey, [TestUtils myVerkey1]);

    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testCreateKeyWorksWithoutSeed
{
    [TestUtils cleanupStorage];

    IndyHandle walletHandle = 0;
    NSError *ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                           xtype:nil
                                                                          handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");

    NSString *verkey = nil;
    ret = [[CryptoUtils sharedInstance] createKeyWithWalletHandle:walletHandle
                                                          keyJson:@"{}"
                                                        outVerkey:&verkey];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:createKeyWithWalletHandle failed");
    XCTAssertNotNil(verkey);

    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testCreateKeyWorksForInvalidWalletHandle
{
    [TestUtils cleanupStorage];

    int invalidWalletHandle = -1;
    NSError *ret = [[CryptoUtils sharedInstance] createKeyWithWalletHandle:invalidWalletHandle
                                                                   keyJson:@"{}"
                                                                 outVerkey:nil];
    XCTAssertEqual(ret.code, WalletInvalidHandle);

    [TestUtils cleanupStorage];
}

// MARK: - Set key metadata

- (void)testSetKeyMetadataWorks
{
    [TestUtils cleanupStorage];

    IndyHandle walletHandle = 0;
    NSError *ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                           xtype:nil
                                                                          handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");

    ret = [[CryptoUtils sharedInstance] setMetadata:[TestUtils someMetadata]
                                             forKey:[TestUtils myVerkey1]
                                       walletHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:setMetadata failed");

    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testSetKeyMetadataWorksForReplace
{
    [TestUtils cleanupStorage];

    IndyHandle walletHandle = 0;
    NSError *ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                           xtype:nil
                                                                          handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");

    ret = [[CryptoUtils sharedInstance] setMetadata:[TestUtils someMetadata]
                                             forKey:[TestUtils myVerkey1]
                                       walletHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:setMetadata failed");

    NSString *outMetadata = nil;
    ret = [[CryptoUtils sharedInstance] getMetadataForKey:[TestUtils myVerkey1]
                                             walletHandle:walletHandle
                                              outMetadata:&outMetadata];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:getMetadata failed");
    XCTAssertEqualObjects(outMetadata, [TestUtils someMetadata]);

    NSString *newMetadata = @"updated metadata";
    ret = [[CryptoUtils sharedInstance] setMetadata:newMetadata
                                             forKey:[TestUtils myVerkey1]
                                       walletHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:setMetadata failed");

    ret = [[CryptoUtils sharedInstance] getMetadataForKey:[TestUtils myVerkey1]
                                             walletHandle:walletHandle
                                              outMetadata:&outMetadata];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:getMetadata failed");
    XCTAssertEqualObjects(outMetadata, newMetadata);

    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testSetKeyMetadataWorksForInvalidHandle
{
    [TestUtils cleanupStorage];

    IndyHandle invalidWalletHandle = -1;
    NSError *ret = [[CryptoUtils sharedInstance] setMetadata:[TestUtils someMetadata]
                                                      forKey:[TestUtils myVerkey1]
                                                walletHandle:invalidWalletHandle];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"CryptoUtils:setMetadata failed with unexpected error code");

    [[WalletUtils sharedInstance] closeWalletWithHandle:invalidWalletHandle];
    [TestUtils cleanupStorage];
}

- (void)testSetKeyMetadataWorksForEmptyString
{
    [TestUtils cleanupStorage];

    IndyHandle walletHandle = 0;
    NSError *ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                           xtype:nil
                                                                          handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");

    ret = [[CryptoUtils sharedInstance] setMetadata:@""
                                             forKey:[TestUtils myVerkey1]
                                       walletHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:setMetadata failed");

    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testSetKeyMetadataWorksForInvalidKey
{
    [TestUtils cleanupStorage];

    IndyHandle walletHandle = 0;
    NSError *ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                           xtype:nil
                                                                          handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");

    ret = [[CryptoUtils sharedInstance] setMetadata:[TestUtils someMetadata]
                                             forKey:[TestUtils invalidBase58Verkey]
                                       walletHandle:walletHandle];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"CryptoUtils:setMetadata failed with unexpected error code");

    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

// MARK: - Get key metadata

- (void)testGetKeyMetadataWorks
{
    [TestUtils cleanupStorage];

    IndyHandle walletHandle = 0;
    NSError *ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                           xtype:nil
                                                                          handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");

    ret = [[CryptoUtils sharedInstance] setMetadata:[TestUtils someMetadata]
                                             forKey:[TestUtils myVerkey1]
                                       walletHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:setMetadata failed");

    NSString *outMetadata = nil;
    ret = [[CryptoUtils sharedInstance] getMetadataForKey:[TestUtils myVerkey1]
                                             walletHandle:walletHandle
                                              outMetadata:&outMetadata];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:getMetadata failed");
    XCTAssertEqualObjects(outMetadata, [TestUtils someMetadata]);

    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testGetKeyMetadataWorksForEmptyString
{
    [TestUtils cleanupStorage];

    IndyHandle walletHandle = 0;
    NSError *ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                           xtype:nil
                                                                          handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");

    ret = [[CryptoUtils sharedInstance] setMetadata:@""
                                             forKey:[TestUtils myVerkey1]
                                       walletHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:setMetadata failed");

    NSString *outMetadata = nil;
    ret = [[CryptoUtils sharedInstance] getMetadataForKey:[TestUtils myVerkey1]
                                             walletHandle:walletHandle
                                              outMetadata:&outMetadata];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:getMetadata failed");
    XCTAssertEqualObjects(outMetadata, @"");

    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testGetKeyMetadataWorksForNoMetadata
{
    [TestUtils cleanupStorage];

    IndyHandle walletHandle = 0;
    NSError *ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                           xtype:nil
                                                                          handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");

    NSString *outMetadata = nil;
    ret = [[CryptoUtils sharedInstance] getMetadataForKey:[TestUtils myVerkey1]
                                             walletHandle:walletHandle
                                              outMetadata:&outMetadata];
    XCTAssertEqual(ret.code, WalletNotFoundError, @"CryptoUtils:getMetadata failed with unexpected error code");

    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testGetKeyMetadataWorksForInvalidHandle
{
    [TestUtils cleanupStorage];

    IndyHandle walletHandle = 0;
    NSError *ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                           xtype:nil
                                                                          handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");

    ret = [[CryptoUtils sharedInstance] setMetadata:[TestUtils someMetadata]
                                             forKey:[TestUtils myVerkey1]
                                       walletHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:setMetadata failed");

    NSString *outMetadata = nil;
    ret = [[CryptoUtils sharedInstance] getMetadataForKey:[TestUtils myVerkey1]
                                             walletHandle:walletHandle + 1
                                              outMetadata:&outMetadata];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"CryptoUtils:getMetadata failed");

    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

// MARK: - Crypto sign

- (void)testCryptoSignWorks
{
    [TestUtils cleanupStorage];

    IndyHandle walletHandle = 0;
    NSError *ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                           xtype:nil
                                                                          handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");

    NSString *verkey = nil;
    ret = [[CryptoUtils sharedInstance] createKeyWithWalletHandle:walletHandle
                                                          keyJson:[NSString stringWithFormat:@"{\"seed\":\"%@\"}", [TestUtils mySeed1]]
                                                        outVerkey:&verkey];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:createKeyWithWalletHandle failed");

    NSData *signature = nil;
    [[CryptoUtils sharedInstance] signMessage:[TestUtils message] key:verkey walletHandle:walletHandle outSignature:&signature];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:signMessage failed");
    XCTAssertEqualObjects(signature, [TestUtils signature]);

    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

// MARK: - Crypto verify

- (void)testCryptoVerifyWorks
{
    [TestUtils cleanupStorage];

    BOOL isValid = NO;
    NSError *ret = [[CryptoUtils sharedInstance] verifySignature:[TestUtils signature]
                                                      forMessage:[TestUtils message]
                                                             key:[TestUtils myVerkey1]
                                                      outIsValid:&isValid];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:verifySignature failed");
    XCTAssert(isValid);

    [TestUtils cleanupStorage];
}

// MARK: - Auth crypt

- (void)testAuthCryptWorks
{
    [TestUtils cleanupStorage];

    IndyHandle walletHandle = 0;
    NSError *ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                           xtype:nil
                                                                          handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");

    NSString *verkey = nil;
    ret = [[CryptoUtils sharedInstance] createKeyWithWalletHandle:walletHandle
                                                          keyJson:[NSString stringWithFormat:@"{\"seed\":\"%@\"}", [TestUtils mySeed1]]
                                                        outVerkey:&verkey];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:createKeyWithWalletHandle failed");

    NSData *encrypted = nil;
    [[CryptoUtils sharedInstance] authCrypt:[TestUtils message]
                                      myKey:verkey
                                   theirKey:[TestUtils trusteeVerkey]
                               walletHandle:walletHandle
                               outEncrypted:&encrypted];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:authCrypt failed");
    XCTAssertNotNil(encrypted);

    [TestUtils cleanupStorage];
}

// MARK: - Auth decrypt

- (void)testAuthDecryptWorks
{
    [TestUtils cleanupStorage];

    IndyHandle walletHandle = 0;
    NSError *ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                           xtype:nil
                                                                          handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");

    NSString *myVerkey = nil;
    ret = [[CryptoUtils sharedInstance] createKeyWithWalletHandle:walletHandle
                                                          keyJson:[NSString stringWithFormat:@"{\"seed\":\"%@\"}", [TestUtils mySeed1]]
                                                        outVerkey:&myVerkey];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:createKeyWithWalletHandle failed");

    NSString *theirVerkey = nil;
    ret = [[CryptoUtils sharedInstance] createKeyWithWalletHandle:walletHandle
                                                          keyJson:[NSString stringWithFormat:@"{\"seed\":\"%@\"}", [TestUtils mySeed2]]
                                                        outVerkey:&theirVerkey];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:createKeyWithWalletHandle failed");

    NSData *encryptedMessage = nil;
    ret = [[CryptoUtils sharedInstance] authCrypt:[TestUtils message]
                                      myKey:myVerkey
                                   theirKey:theirVerkey
                               walletHandle:walletHandle
                               outEncrypted:&encryptedMessage];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:createKeyWithWalletHandle failed");
    XCTAssertNotNil(encryptedMessage);

    NSData *decryptedMessage = nil;
    NSString *outTheirKey = nil;
    ret = [[CryptoUtils sharedInstance] authDecrypt:encryptedMessage
                                         myKey:theirVerkey
                                         walletHandle:walletHandle
                                   outTheirKey: &outTheirKey
                           outDecryptedMessage:&decryptedMessage];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:authDecrypt failed");
    XCTAssertEqualObjects(outTheirKey, myVerkey);
    XCTAssertEqualObjects(decryptedMessage, [TestUtils message]);

    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

// MARK: - Anon crypt

- (void)testAnonCryptWorks
{
    [TestUtils cleanupStorage];

    NSData *encrypted = nil;
    NSError *ret = [[CryptoUtils sharedInstance] anonCrypt:[TestUtils message]
                                                      theirKey:[TestUtils myVerkey1]
                                                  outEncrypted:&encrypted];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:anonCrypt failed");
    XCTAssertNotNil(encrypted);

    [TestUtils cleanupStorage];
}

// MARK: - Anon decrypt

- (void)testAnonDecryptWorks
{
    [TestUtils cleanupStorage];

    IndyHandle walletHandle = 0;
    NSError *ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                           xtype:nil
                                                                          handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");

    NSString *verkey = nil;
    ret = [[CryptoUtils sharedInstance] createKeyWithWalletHandle:walletHandle
                                                          keyJson:[NSString stringWithFormat:@"{\"seed\":\"%@\"}", [TestUtils mySeed1]]
                                                        outVerkey:&verkey];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:createKeyWithWalletHandle failed");
    XCTAssertNotNil(verkey);

    NSData *encrypted = nil;
    ret = [[CryptoUtils sharedInstance] anonCrypt:[TestUtils message]
                                             theirKey:verkey
                                         outEncrypted:&encrypted];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:anonCrypt failed");
    XCTAssertNotNil(encrypted);

    NSData *decryptedMessage = nil;
    [[CryptoUtils sharedInstance] anonDecrypt:encrypted
                                        myKey:verkey
                                 walletHandle:walletHandle
                          outDecryptedMessage:&decryptedMessage];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:anonDecrypt failed");
    XCTAssertEqualObjects(decryptedMessage, [TestUtils message]);

    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

@end
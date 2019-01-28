//
// Created by DSR on 07/11/2017.
// Copyright (c) 2017 Hyperledger. All rights reserved.
//


#import <XCTest/XCTestCase.h>
#import "TestUtils.h"
#import "CryptoUtils.h"

@interface CryptoHighCases : XCTestCase

@end

@implementation CryptoHighCases {
    IndyHandle walletHandle;
    NSError *ret;
}

- (void)setUp {
    [super setUp];
    [TestUtils cleanupStorage];

    ret = [[PoolUtils sharedInstance] setProtocolVersion:[TestUtils protocolVersion]];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::setProtocolVersion() failed!");

    [[WalletUtils sharedInstance] createAndOpenWalletWithHandle:&walletHandle];
    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
    [super tearDown];
}

// MARK: - Create key

- (void)testCreateKeyWorksWithSeed {
    NSString *verkey = nil;
    ret = [[CryptoUtils sharedInstance] createKeyWithWalletHandle:walletHandle
                                                          keyJson:[NSString stringWithFormat:@"{\"seed\":\"%@\"}", [TestUtils mySeed1]]
                                                        outVerkey:&verkey];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:createKeyWithWalletHandle failed");
    XCTAssertEqualObjects(verkey, [TestUtils myVerkey1]);
}

- (void)testCreateKeyWorksWithoutSeed {
    NSString *verkey = nil;
    ret = [[CryptoUtils sharedInstance] createKeyWithWalletHandle:walletHandle
                                                          keyJson:@"{}"
                                                        outVerkey:&verkey];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:createKeyWithWalletHandle failed");
    XCTAssertNotNil(verkey);
}

// MARK: - Set key metadata

- (void)testSetKeyMetadataWorks {
    NSString *verkey = nil;
    ret = [[CryptoUtils sharedInstance] createKeyWithWalletHandle:walletHandle
                                                          keyJson:@"{}"
                                                        outVerkey:&verkey];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:createKeyWithWalletHandle failed");

    ret = [[CryptoUtils sharedInstance] setMetadata:[TestUtils someMetadata]
                                             forKey:verkey
                                       walletHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:setMetadata failed");
}

- (void)testSetKeyMetadataWorksForReplace {
    NSString *verkey = nil;
    ret = [[CryptoUtils sharedInstance] createKeyWithWalletHandle:walletHandle
                                                          keyJson:@"{}"
                                                        outVerkey:&verkey];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:createKeyWithWalletHandle failed");

    ret = [[CryptoUtils sharedInstance] setMetadata:[TestUtils someMetadata]
                                             forKey:verkey
                                       walletHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:setMetadata failed");

    NSString *outMetadata = nil;
    ret = [[CryptoUtils sharedInstance] getMetadataForKey:verkey
                                             walletHandle:walletHandle
                                              outMetadata:&outMetadata];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:getMetadata failed");
    XCTAssertEqualObjects(outMetadata, [TestUtils someMetadata]);

    NSString *newMetadata = @"updated metadata";
    ret = [[CryptoUtils sharedInstance] setMetadata:newMetadata
                                             forKey:verkey
                                       walletHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:setMetadata failed");

    ret = [[CryptoUtils sharedInstance] getMetadataForKey:verkey
                                             walletHandle:walletHandle
                                              outMetadata:&outMetadata];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:getMetadata failed");
    XCTAssertEqualObjects(outMetadata, newMetadata);
}

- (void)testSetKeyMetadataWorksForInvalidKey {
    ret = [[CryptoUtils sharedInstance] setMetadata:[TestUtils someMetadata]
                                             forKey:[TestUtils invalidBase58Verkey]
                                       walletHandle:walletHandle];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"CryptoUtils:setMetadata failed with unexpected error code");
}

// MARK: - Get key metadata

- (void)testGetKeyMetadataWorks {
    NSString *verkey = nil;
    ret = [[CryptoUtils sharedInstance] createKeyWithWalletHandle:walletHandle
                                                          keyJson:@"{}"
                                                        outVerkey:&verkey];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:createKeyWithWalletHandle failed");

    ret = [[CryptoUtils sharedInstance] setMetadata:[TestUtils someMetadata]
                                             forKey:verkey
                                       walletHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:setMetadata failed");

    NSString *outMetadata = nil;
    ret = [[CryptoUtils sharedInstance] getMetadataForKey:verkey
                                             walletHandle:walletHandle
                                              outMetadata:&outMetadata];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:getMetadata failed");
    XCTAssertEqualObjects(outMetadata, [TestUtils someMetadata]);
}

- (void)testGetKeyMetadataWorksForEmptyString {
    NSString *verkey = nil;
    ret = [[CryptoUtils sharedInstance] createKeyWithWalletHandle:walletHandle
                                                          keyJson:@"{}"
                                                        outVerkey:&verkey];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:createKeyWithWalletHandle failed");

    ret = [[CryptoUtils sharedInstance] setMetadata:@""
                                             forKey:verkey
                                       walletHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:setMetadata failed");

    NSString *outMetadata = nil;
    ret = [[CryptoUtils sharedInstance] getMetadataForKey:verkey
                                             walletHandle:walletHandle
                                              outMetadata:&outMetadata];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:getMetadata failed");
    XCTAssertEqualObjects(outMetadata, @"");
}

- (void)testGetKeyMetadataWorksForNoMetadata {
    NSString *outMetadata = nil;
    ret = [[CryptoUtils sharedInstance] getMetadataForKey:[TestUtils myVerkey1]
                                             walletHandle:walletHandle
                                              outMetadata:&outMetadata];
    XCTAssertEqual(ret.code, WalletItemNotFound, @"CryptoUtils:getMetadata failed with unexpected error code");
}

// MARK: - Crypto sign

- (void)testCryptoSignWorks {
    NSString *verkey = nil;
    ret = [[CryptoUtils sharedInstance] createKeyWithWalletHandle:walletHandle
                                                          keyJson:[NSString stringWithFormat:@"{\"seed\":\"%@\"}", [TestUtils mySeed1]]
                                                        outVerkey:&verkey];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:createKeyWithWalletHandle failed");

    NSData *signature = nil;
    [[CryptoUtils sharedInstance] signMessage:[TestUtils message] key:verkey walletHandle:walletHandle outSignature:&signature];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:signMessage failed");
    XCTAssertEqualObjects(signature, [TestUtils signature]);
}

// MARK: - Crypto verify

- (void)testCryptoVerifyWorks {
    BOOL isValid = NO;
    NSError *ret = [[CryptoUtils sharedInstance] verifySignature:[TestUtils signature]
                                                      forMessage:[TestUtils message]
                                                             key:[TestUtils myVerkey1]
                                                      outIsValid:&isValid];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:verifySignature failed");
    XCTAssert(isValid);
}

// MARK: - Auth crypt

- (void)testAuthCryptWorks {
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
}

// MARK: - Auth decrypt

- (void)testAuthDecryptWorks {
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
                                        outTheirKey:&outTheirKey
                                outDecryptedMessage:&decryptedMessage];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:authDecrypt failed");
    XCTAssertEqualObjects(outTheirKey, myVerkey);
    XCTAssertEqualObjects(decryptedMessage, [TestUtils message]);
}

// MARK: - Anon crypt

- (void)testAnonCryptWorks {
    NSData *encrypted = nil;
    NSError *ret = [[CryptoUtils sharedInstance] anonCrypt:[TestUtils message]
                                                  theirKey:[TestUtils myVerkey1]
                                              outEncrypted:&encrypted];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:anonCrypt failed");
    XCTAssertNotNil(encrypted);
}

// MARK: - Anon decrypt

- (void)testAnonDecryptWorks {
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

    NSData *decryptedMessage = nil;
    [[CryptoUtils sharedInstance] anonDecrypt:encrypted
                                        myKey:verkey
                                 walletHandle:walletHandle
                          outDecryptedMessage:&decryptedMessage];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:anonDecrypt failed");
    XCTAssertEqualObjects(decryptedMessage, [TestUtils message]);
}

// MARK: - Pack Unpack message

- (void)testPackUnpackAuthMessageWorks {
    NSString *senderVerkey = nil;
    ret = [[CryptoUtils sharedInstance] createKeyWithWalletHandle:walletHandle
                                                          keyJson:@"{}"
                                                        outVerkey:&senderVerkey];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:createKeyWithWalletHandle failed");


    IndyHandle walletHandleReceiver;
    [[WalletUtils sharedInstance] createAndOpenWalletWithHandle:&walletHandleReceiver];


    NSString *receiverVerkey = nil;
    ret = [[CryptoUtils sharedInstance] createKeyWithWalletHandle:walletHandleReceiver
                                                          keyJson:@"{}"
                                                        outVerkey:&receiverVerkey];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:createKeyWithWalletHandle failed");

    NSArray *receivers = @[[TestUtils trusteeVerkey], receiverVerkey];
    NSData *jsonData = [NSJSONSerialization dataWithJSONObject:receivers options:0 error:nil];
    NSString *receiversJson = [[NSString alloc] initWithData:jsonData encoding:NSUTF8StringEncoding];

    NSData *packedMessage = nil;
    ret = [[CryptoUtils sharedInstance] packMessage:[TestUtils message]
                                          receivers:receiversJson
                                             sender:senderVerkey
                                       walletHandle:walletHandle
                                                jwe:&packedMessage];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:packMessage failed");
    XCTAssertNotNil(packedMessage);

    NSData *unpackedMessageData = nil;
    ret = [[CryptoUtils sharedInstance] unpackMessage:packedMessage
                                         walletHandle:walletHandleReceiver
                                      unpackedMessage:&unpackedMessageData];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:unpackMessage failed");

    NSString *unpackedMessageJson = [[NSString alloc] initWithData:unpackedMessageData encoding:NSUTF8StringEncoding];
    NSDictionary *unpackedMessage = [NSDictionary fromString:unpackedMessageJson];

    XCTAssertTrue([[[NSString alloc] initWithData:[TestUtils message] encoding:NSUTF8StringEncoding] isEqualToString:unpackedMessage[@"message"]]);
    XCTAssertTrue([senderVerkey isEqualToString:unpackedMessage[@"sender_verkey"]]);
    XCTAssertTrue([receiverVerkey isEqualToString:unpackedMessage[@"recipient_verkey"]]);
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandleReceiver];
}

- (void)testPackUnpackAnonMessageWorks {
    NSString *senderVerkey = nil;
    ret = [[CryptoUtils sharedInstance] createKeyWithWalletHandle:walletHandle
                                                          keyJson:@"{}"
                                                        outVerkey:&senderVerkey];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:createKeyWithWalletHandle failed");
    
    
    IndyHandle walletHandleReceiver;
    [[WalletUtils sharedInstance] createAndOpenWalletWithHandle:&walletHandleReceiver];
    
    
    NSString *receiverVerkey = nil;
    ret = [[CryptoUtils sharedInstance] createKeyWithWalletHandle:walletHandleReceiver
                                                          keyJson:@"{}"
                                                        outVerkey:&receiverVerkey];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:createKeyWithWalletHandle failed");
    
    NSArray *receivers = @[[TestUtils trusteeVerkey], receiverVerkey];
    NSData *jsonData = [NSJSONSerialization dataWithJSONObject:receivers options:0 error:nil];
    NSString *receiversJson = [[NSString alloc] initWithData:jsonData encoding:NSUTF8StringEncoding];
    
    NSData *packedMessage = nil;
    ret = [[CryptoUtils sharedInstance] packMessage:[TestUtils message]
                                          receivers:receiversJson
                                             sender:nil
                                       walletHandle:walletHandle
                                                jwe:&packedMessage];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:packMessage failed");
    XCTAssertNotNil(packedMessage);
    
    NSData *unpackedMessageData = nil;
    ret = [[CryptoUtils sharedInstance] unpackMessage:packedMessage
                                         walletHandle:walletHandleReceiver
                                      unpackedMessage:&unpackedMessageData];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:unpackMessage failed");
    
    NSString *unpackedMessageJson = [[NSString alloc] initWithData:unpackedMessageData encoding:NSUTF8StringEncoding];
    NSDictionary *unpackedMessage = [NSDictionary fromString:unpackedMessageJson];
    
    XCTAssertTrue([[[NSString alloc] initWithData:[TestUtils message] encoding:NSUTF8StringEncoding] isEqualToString:unpackedMessage[@"message"]]);
    XCTAssertNil(unpackedMessage[@"sender_verkey"]);
    XCTAssertTrue([receiverVerkey isEqualToString:unpackedMessage[@"recipient_verkey"]]);
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandleReceiver];
}

@end

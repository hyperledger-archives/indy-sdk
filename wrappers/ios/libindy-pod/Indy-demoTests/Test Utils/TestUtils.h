//
//  TestUtlis.h
//  Indy-demo
//
//  Created by Kirill Neznamov on 11/05/2017.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <Foundation/Foundation.h>
#import "AnoncredsUtils.h"
#import "LedgerUtils.h"
#import "PoolUtils.h"
#import "CryptoUtils.h"
#import "DidUtils.h"
#import "WalletUtils.h"
#import "SequenceUtils.h"

#import "NSDictionary+JSON.h"
#import "NSString+Validation.h"
#import "NSArray+JSON.h"

@interface TestUtils : NSObject

+ (NSMutableString *)getUserDocumentDir;

+ (NSMutableString *)getUserTmpDir;

+ (NSTimeInterval)defaultTimeout;

+ (NSTimeInterval)shortTimeout;

+ (NSTimeInterval)longTimeout;

+ (void)cleanupStorage;

+ (NSString *)testPoolIp;

+ (NSString *)tmpFilePathAppending:(NSString *)fileName;

+ (NSString *)pool;

+ (NSData *)message;

+ (NSData *)encryptedMessage;

+ (NSData *)nonce;

+ (NSData *)signature;

+ (NSString *)trusteeSeed;

+ (NSString *)trusteeVerkey;

+ (NSString *)mySeed;

+ (NSString *)endpoint;

+ (NSString *)commonMasterSecretName;

+ (NSString *)issuerDid;

+ (NSString *)issuer2Did;

+ (NSString *)proverDid;

+ (IndyHandle)walletHandle;

+ (NSString *)mySeed1;

+ (NSString *)mySeed2;

+ (NSString *)myVerkey1;

+ (NSString *)invalidBase58Verkey;

+ (NSString *)someMetadata;

+ (NSString *)unknownDid;

+ (NSString *)defaultType;

+ (NSString *)gvtSchemaName;

+ (NSString *)schemaVersion;

+ (NSString *)gvtSchemaAttrs;

+ (NSString *)xyzSchemaName;

+ (NSString *)xyzSchemaAttrs;

+ (NSString *)tag;

@end

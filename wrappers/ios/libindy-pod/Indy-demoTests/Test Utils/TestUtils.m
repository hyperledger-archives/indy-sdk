//
//  TestUtlis.m
//  Indy-demo
//
//  Created by Kirill Neznamov on 11/05/2017.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import "TestUtils.h"

@implementation TestUtils

+ (NSMutableString *)getUserDocumentDir {
    NSArray *paths = NSSearchPathForDirectoriesInDomains(NSDocumentDirectory, NSUserDomainMask, YES);
    NSMutableString *path = [NSMutableString stringWithString:[paths objectAtIndex:0]];
    return path;
}

+ (NSMutableString *)getUserTmpDir {
    NSMutableString *path = [NSMutableString stringWithString:NSTemporaryDirectory()];
    return path;
}

+ (void)cleanupStorage {
    NSMutableString *path = [TestUtils getUserDocumentDir];
    [path appendString:@"/.indy_client"];
    [[NSFileManager defaultManager] removeItemAtPath:path error:nil];
}

+ (NSTimeInterval)defaultTimeout {
    return 25;
}

+ (NSTimeInterval)shortTimeout {
    return 5;
}

+ (NSTimeInterval)longTimeout {
    return 200;
}

+ (NSString *)tmpFilePathAppending:(NSString *)fileName {
    return [NSString stringWithFormat:@"%@/%@", [TestUtils getUserTmpDir], fileName];
}

+ (NSString *)testPoolIp {
    return @"127.0.0.1";
}

+ (NSString *)pool {
    return @"pool_1";
}

+ (NSString *)wallet {
    return @"wallet_1";
}

+ (NSString *)walletConfig {
    return @"{\"id\":\"wallet_1\"}";
}

+ (NSData *)message {
    NSString *messageJson = @"{\"reqId\":1496822211362017764}";
    return [messageJson dataUsingEncoding:NSUTF8StringEncoding];
}

+ (NSData *)encryptedMessage {
    const unsigned char bytes[] = {187, 227, 10, 29, 46, 178, 12, 179, 197, 69, 171, 70, 228, 204, 52, 22, 199, 54, 62, 13, 115, 5, 216, 66, 20, 131, 121, 29, 251, 224, 253, 201, 75, 73, 225, 237, 219, 133, 35, 217, 131, 135, 232, 129, 32};
    return [NSData dataWithBytes:bytes length:sizeof(bytes)];
}

+ (NSData *)nonce {
    const unsigned char bytes[] = {242, 246, 53, 153, 106, 37, 185, 65, 212, 14, 109, 131, 200, 169, 94, 110, 51, 47, 101, 89, 0, 171, 105, 183};
    return [NSData dataWithBytes:bytes length:sizeof(bytes)];
}

+ (NSData *)signature {
    const unsigned char bytes[] = {169, 215, 8, 225, 7, 107, 110, 9, 193, 162, 202, 214, 162, 66, 238, 211, 63, 209, 12, 196, 8, 211, 55, 27, 120, 94, 204, 147, 53, 104, 103, 61, 60, 249, 237, 127, 103, 46, 220, 223, 10, 95, 75, 53, 245, 210, 241, 151, 191, 41, 48, 30, 9, 16, 78, 252, 157, 206, 210, 145, 125, 133, 109, 11};
    return [NSData dataWithBytes:bytes length:sizeof(bytes)];
}

+ (NSString *)trusteeSeed {
    return @"000000000000000000000000Trustee1";
}

+ (NSString *)trusteeDid {
    return @"V4SGRU86Z58d6TV7PBUe6f";
}

+ (NSString *)trusteeVerkey {
    return @"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL";
}

+ (NSString *)mySeed {
    return @"00000000000000000000000000000My1";
}

+ (NSString *)endpoint {
    return @"127.0.0.1:9700";
}

+ (NSString *)commonMasterSecretName {
    return @"common_master_secret_name";
}

+ (NSString *)issuerDid {
    return @"NcYxiDXkpYi6ov5FcYDi1e";
}

+ (NSString *)issuer2Did {
    return @"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
}

+ (NSString *)proverDid {
    return @"VsKV7grR1BUE29mG2Fm2kX";
}

+ (IndyHandle)walletHandle {
    return 0;
}

+ (NSString *)mySeed1 {
    return @"00000000000000000000000000000My1";
}

+ (NSString *)mySeed2 {
    return @"00000000000000000000000000000My2";
}

+ (NSString *)myDid1 {
    return @"VsKV7grR1BUE29mG2Fm2kX";
}

+ (NSString *)myVerkey1 {
    return @"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa";
}

+ (NSString *)invalidBase58Verkey {
    return @"CnEDk___MnmiHXEV1WFgbV___eYnPqs___TdcZaNhFVW";
}

+ (NSString *)someMetadata {
    return @"some metadata";
}

+ (NSString *)unknownDid {
    return @"NcYxiDXkpYi6ov5FcYDi1e";
}

+ (NSString *)defaultType {
    return @"default";
}

+ (NSString *)gvtSchema {
    return @"{\"id\":\"id\", \"name\":\"gvt\",\"version\":\"1.0\",\"attrNames\":[\"name\"],\"ver\":\"1.0\"}";
}

+ (NSString *)gvtSchemaName {
    return @"gvt";
}

+ (NSString *)schemaVersion {
    return @"1.0";
}

+ (NSString *)gvtSchemaAttrs {
    return @"[\"age\",\"sex\",\"height\",\"name\"]";
}


+ (NSString *)xyzSchemaName {
    return @"xyz";
}

+ (NSString *)xyzSchemaAttrs {
    return @"[\"status\",\"period\"]";
}

+ (NSString *)tag {
    return @"TAG1";
}

+ (NSNumber *)protocolVersion {
    return @(2);
}


@end

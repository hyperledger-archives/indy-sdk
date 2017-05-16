//
//  TestUtlis.m
//  libsovrin-demo
//
//  Created by Kirill Neznamov on 11/05/2017.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import "TestUtils.h"

@implementation TestUtils

+ (NSMutableString*) getUserDocumentDir
{
    NSArray *paths = NSSearchPathForDirectoriesInDomains(NSDocumentDirectory, NSUserDomainMask, YES);
    NSMutableString *path = [NSMutableString stringWithString:[paths objectAtIndex:0]];
    return path;
}

+ (NSMutableString*) getUserTmpDir
{
    NSMutableString *path = [NSMutableString stringWithString:NSTemporaryDirectory()];
    return path;
}

+(void) cleanupStorage
{
    NSMutableString *path = [TestUtils getUserDocumentDir];
    [path appendString:@"/.sovrin"];
    [[NSFileManager defaultManager] removeItemAtPath:path error:nil];
}

+(NSTimeInterval) defaultTimeout
{
    return  1000;
}

@end

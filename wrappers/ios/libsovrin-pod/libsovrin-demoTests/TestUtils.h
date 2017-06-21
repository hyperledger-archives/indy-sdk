//
//  TestUtlis.h
//  libsovrin-demo
//
//  Created by Kirill Neznamov on 11/05/2017.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <Foundation/Foundation.h>

@interface TestUtils : NSObject

+ (NSMutableString*) getUserDocumentDir;
+ (NSMutableString*) getUserTmpDir;
+ (NSTimeInterval) defaultTimeout;
+(NSTimeInterval) shortTimeout;
+(NSTimeInterval) longTimeout;


+(void) cleanupStorage;

@end

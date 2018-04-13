//
//  PoolUtils.h
//  Indy-demo
//
//  Created by Kirill Neznamov on 15/05/2017.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <Indy/Indy.h>

@interface BlobStorageUtils : XCTestCase

+ (BlobStorageUtils *)sharedInstance;

- (NSError *)openReaderWithType:(NSString *)type
                         config:(NSString *)config
                         handle:(NSNumber **)handle;

- (NSError *)openWriterWithType:(NSString *)type
                         config:(NSString *)config
                         handle:(NSNumber **)handle;

@end

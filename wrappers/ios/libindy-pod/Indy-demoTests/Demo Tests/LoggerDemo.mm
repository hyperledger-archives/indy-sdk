#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <Indy/Indy.h>
#import <Indy/IndyLogger.h>
#import "TestUtils.h"
#import "BlobStorageUtils.h"

@interface Logger : XCTestCase

@end

@implementation Logger

- (void)setUp {
    [super setUp];
    [TestUtils cleanupStorage];
}

- (void)tearDown {
    [TestUtils cleanupStorage];
    [super tearDown];
}

#define levelMappings @{@"1": @"Error", @"2": @"Warning", @"3": @"Info", @"4": @"Debug", @"5": @"Trace"}

- (void)testSetLogger {
    [IndyLogger setLogger:^(NSObject *context, NSNumber *level, NSString *target, NSString *message, NSString *modulePath, NSString *file, NSNumber *line) {
        NSLog(@"%@    %@:%@ | %@", [levelMappings valueForKey:[NSString stringWithFormat:@"%@", level]], file, line, message);
    }];
    
    [[PoolUtils sharedInstance] setProtocolVersion:[TestUtils protocolVersion]];
}

@end


#import "BlobStorageUtils.h"
#import "TestUtils.h"
#import <Indy/Indy.h>
#import <XCTest/XCTest.h>

@interface BlobStorageUtils ()

@property(assign) int requestIdOffset;

@end


@implementation BlobStorageUtils

+ (BlobStorageUtils *)sharedInstance {
    static BlobStorageUtils *instance = nil;
    static dispatch_once_t dispatch_once_block;

    dispatch_once(&dispatch_once_block, ^{
        instance = [BlobStorageUtils new];
        instance.requestIdOffset = 1;
    });

    return instance;
}

// MARK: - Blob Storage Reader

- (NSError *)openReaderWithType:(NSString *)type
                         config:(NSString *)config
                         handle:(NSNumber **)handle {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSNumber *readerHandle = nil;

    NSString *configStr = (config) ? config : @"";

    [IndyBlobStorage openReaderWithType:type
                                 config:config
                             completion:^(NSError *error, NSNumber *blockHandle) {
                                 err = error;
                                 readerHandle = blockHandle;
                                 [completionExpectation fulfill];
                             }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (handle) {*handle = readerHandle;}
    return err;
}

// MARK: - Blob Storage Writer

- (NSError *)openWriterWithType:(NSString *)type
                         config:(NSString *)config
                         handle:(NSNumber **)handle {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSNumber *writerHandle = nil;

    NSString *configStr = (config) ? config : @"";

    [IndyBlobStorage openWriterWithType:type
                                 config:config
                             completion:^(NSError *error, NSNumber *blockHandle) {
                                 err = error;
                                 writerHandle = blockHandle;
                                 [completionExpectation fulfill];
                             }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (handle) {*handle = writerHandle;}
    return err;
}

@end

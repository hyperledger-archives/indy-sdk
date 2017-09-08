//
//  SequenceUtils.m
//  libindy
//

#import "SequenceUtils.h"

@interface SequenceUtils()

@property (atomic, strong) NSLock *lock;
@property (atomic, assign) NSUInteger idsCounter;

@end

@implementation SequenceUtils

+ (SequenceUtils *)sharedInstance
{
    static SequenceUtils *instance = nil;
    static dispatch_once_t dispatch_once_block;
    
    dispatch_once(&dispatch_once_block, ^{
        instance = [SequenceUtils new];
        instance.idsCounter = 0;
        instance.lock = [NSLock new];
    });
    
    return instance;
}


- (NSUInteger)getNextId
{
    [self.lock lock];
    self.idsCounter +=1;
    [self.lock unlock];
    
    return self.idsCounter;
}

@end

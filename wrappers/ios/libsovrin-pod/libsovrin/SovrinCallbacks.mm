//
//  SovrinCallbacks.m
//  libsovrin
//

#include "sovrin_core.h"
#import "SovrinCallbacks.h"
#import "NSError+SovrinError.h"
#import "SovrinTypes.h"

void SovrinWrapperCommon2PCallback(sovrin_handle_t xcommand_handle, sovrin_error_t err)
{
    void * block = [[SovrinCallbacks sharedInstance] get: xcommand_handle];
    [[SovrinCallbacks sharedInstance] remove: xcommand_handle];
    
    void (^completion)(NSError*) = (__bridge void (^)(NSError*)) block;

    if(completion)
    {
        dispatch_async(dispatch_get_main_queue(), ^
        {
            NSError *error = [ NSError errorFromSovrinError: err ];
            completion(error);
        });
    }
}

void SovrinWrapperCommon3PHCallback(sovrin_handle_t xcommand_handle, sovrin_error_t err, sovrin_handle_t pool_handle)
{
    void * block = [[SovrinCallbacks sharedInstance] get: xcommand_handle];
    [[SovrinCallbacks sharedInstance] remove: xcommand_handle];

    void (^completion)(NSError*, SovrinHandle) = (__bridge void (^)(NSError*, SovrinHandle))block;

    if (completion)
    {
        dispatch_async(dispatch_get_main_queue(), ^
        {
            NSError *error = [ NSError errorFromSovrinError: err ];
            completion(error, (SovrinHandle) pool_handle);
        });
    }
}

void SovrinWrapperCommon3PSCallback(sovrin_handle_t xcommand_handle, sovrin_error_t err, const char* arg1)
{
    void * block = [[SovrinCallbacks sharedInstance] get: xcommand_handle];
    [[SovrinCallbacks sharedInstance] remove: xcommand_handle];
    
    void (^completion)(NSError*, NSString *) = (__bridge void (^)(NSError*, NSString *arg1 ))block;
    
    if (completion)
    {
        dispatch_async(dispatch_get_main_queue(), ^
                       {
                           NSError *error = [ NSError errorFromSovrinError: err ];
                           NSString* sarg1 = [ NSString stringWithUTF8String: arg1];
                           completion(error, sarg1);
                       });
    }
}

void SovrinWrapperCommon3PBCallback(sovrin_handle_t xcommand_handle, sovrin_error_t err, sovrin_bool_t arg1)
{
    void * block = [[SovrinCallbacks sharedInstance] get: xcommand_handle];
    [[SovrinCallbacks sharedInstance] remove: xcommand_handle];
    
    void (^completion)(NSError*, BOOL ) = (__bridge void (^)(NSError*, BOOL arg1 ))block;
    
    if (completion)
    {
        dispatch_async(dispatch_get_main_queue(), ^
                       {
                           NSError *error = [ NSError errorFromSovrinError: err ];
                           completion(error, (BOOL) arg1);
                       });
    }
}

void SovrinWrapperCommon4PCallback(sovrin_handle_t xcommand_handle, sovrin_error_t err, const char* arg1, const char *arg2)
{
    void * block = [[SovrinCallbacks sharedInstance] get: xcommand_handle];
    [[SovrinCallbacks sharedInstance] remove: xcommand_handle];
    
    void (^completion)(NSError*, NSString* arg1, NSString *arg2) = (__bridge void (^)(NSError*, NSString* arg1, NSString *arg2))block;
    
    if (completion)
    {
        dispatch_async(dispatch_get_main_queue(), ^
                       {
                           NSError *error = [ NSError errorFromSovrinError: err ];
                           NSString* sarg1 = [ NSString stringWithUTF8String: arg1];
                           NSString* sarg2 = [ NSString stringWithUTF8String: arg2];
                           completion(error, sarg1, sarg2);
                       });
    }
}

@interface SovrinCallbacks ()

@property (strong, readwrite) NSMutableDictionary *callbacks;
@property sovrin_i32_t counter;
@end

@implementation SovrinCallbacks

- (SovrinCallbacks *)init
{
    self = [super init];
    if (self)
    {
        self.counter = 0;
        self.callbacks = [[NSMutableDictionary alloc] init];
    }
    return self;
}

- (sovrin_handle_t)add:(void *)ptr
{
    NSValue *val = [NSValue valueWithPointer:ptr];
    NSNumber *handle = nil;

    @synchronized(self)
    {
        handle = [NSNumber numberWithInt:self.counter];
        self.counter++;
        [self.callbacks setObject:val forKey:handle];
    }
    return (sovrin_handle_t)[handle integerValue];
}

- (void)remove:(sovrin_handle_t)handle
{
    NSNumber *key = [NSNumber numberWithInt:handle];
    @synchronized(self)
    {
        if ([self.callbacks objectForKey:key])
        {
            [self.callbacks removeObjectForKey:key];
        }
    }
}

- (void *)get:(sovrin_handle_t)handle
{
    NSNumber *key = [NSNumber numberWithInt:handle];
    NSValue *val = nil;
    @synchronized(self)
    {
        val = [self.callbacks objectForKey:key];
    }
    return val ? [val pointerValue] : NULL;
}

+ (SovrinCallbacks *)sharedInstance
{
    static SovrinCallbacks *instance = nil;
    static dispatch_once_t dispatch_once_block;
    
    dispatch_once(&dispatch_once_block, ^{
            instance = [SovrinCallbacks new];
    });

    return instance;
}

@end

//
//  IndyCallbacks.m
//  libindy
//

#include "indy_core.h"
#import "IndyCallbacks.h"
#import "NSError+IndyError.h"
#import "IndyTypes.h"

static NSString *commandCallbackKey = @"commandCallback";


@interface IndyCallbacks ()

@property(strong, readwrite) NSMutableDictionary *commandCompletions;
@property indy_i32_t commandHandleCounter;
@property(strong, readwrite) NSRecursiveLock *globalLock;

@end

@implementation IndyCallbacks

+ (IndyCallbacks *)sharedInstance {
    static IndyCallbacks *instance = nil;
    static dispatch_once_t dispatch_once_block;

    dispatch_once(&dispatch_once_block, ^{
        instance = [IndyCallbacks new];
    });

    return instance;
}

- (IndyCallbacks *)init {
    self = [super init];
    if (self) {
        self.commandHandleCounter = 0;
        self.commandCompletions = [[NSMutableDictionary alloc] init];
        self.globalLock = [NSRecursiveLock new];
    }
    return self;
}

// MARK: - Create command handle and store callback

- (indy_handle_t)createCommandHandleFor:(id)callback {
    NSNumber *handle = nil;

    @synchronized (self.globalLock) {
        handle = [NSNumber numberWithInt:self.commandHandleCounter];
        self.commandHandleCounter++;

        NSMutableDictionary *dict = [NSMutableDictionary new];
        dict[commandCallbackKey] = [callback copy];

        self.commandCompletions[handle] = dict;
    }
    return (indy_handle_t) [handle integerValue];
}

- (void)deleteCommandHandleFor:(indy_handle_t)handle {
    NSNumber *key = [NSNumber numberWithInt:handle];
    @synchronized (self.globalLock) {
        if ([self.commandCompletions objectForKey:key]) {
            [self.commandCompletions removeObjectForKey:key];
        }
    }
}

- (id)commandCompletionFor:(indy_handle_t)handle {
    NSNumber *key = [NSNumber numberWithInt:handle];
    id val = nil;
    @synchronized (self.globalLock) {
        NSMutableDictionary *dict = (NSMutableDictionary *) [self.commandCompletions objectForKey:key];
        val = [dict objectForKey:@"commandCallback"];
    }
    return val;
}

- (void)complete:(void (^)(NSError *))completion
       forHandle:(indy_handle_t)handle
         ifError:(indy_error_t)ret {
    if (ret != Success) {
        [self deleteCommandHandleFor:handle];
        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret]);
        });
    }
}

- (void)completeBool:(void (^)(NSError *, BOOL))completion
           forHandle:(indy_handle_t)handle
             ifError:(indy_error_t)ret {
    if (ret != Success) {
        [self deleteCommandHandleFor:handle];
        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], false);
        });
    }
}

- (void)completeStr:(void (^)(NSError *, NSString *))completion
          forHandle:(indy_handle_t)handle
            ifError:(indy_error_t)ret {
    if (ret != Success) {
        [self deleteCommandHandleFor:handle];
        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

- (void)complete2Str:(void (^)(NSError *, NSString *, NSString *))completion
           forHandle:(indy_handle_t)handle
             ifError:(indy_error_t)ret {
    if (ret != Success) {
        [self deleteCommandHandleFor:handle];
        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil, nil);
        });
    }
}

- (void)completeData:(void (^)(NSError *, NSData *))completion
           forHandle:(indy_handle_t)handle
             ifError:(indy_error_t)ret {
    if (ret != Success) {
        [self deleteCommandHandleFor:handle];
        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

- (void)complete2Data:(void (^)(NSError *, NSData *, NSData *))completion
            forHandle:(indy_handle_t)handle
              ifError:(indy_error_t)ret {
    if (ret != Success) {
        [self deleteCommandHandleFor:handle];
        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil, nil);
        });
    }
}

- (void)completeStringAndData:(void (^)(NSError *, NSString *, NSData *))completion
                    forHandle:(indy_handle_t)handle
                      ifError:(indy_error_t)ret {
    if (ret != Success) {
        [self deleteCommandHandleFor:handle];
        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], nil, nil);
        });
    }
}

@end

// MARK: - static indy C-callbacks

void IndyWrapperCommon2PCallback(indy_handle_t xcommand_handle,
        indy_error_t err) {
    id block = [[IndyCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[IndyCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];

    void (^completion)(NSError *) = (void (^)(NSError *)) block;

    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromIndyError:err];
            completion(error);
        });
    }
}

void IndyWrapperCommon3PHCallback(indy_handle_t xcommand_handle,
        indy_error_t err,
        indy_handle_t pool_handle) {
    id block = [[IndyCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[IndyCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];

    void (^completion)(NSError *, IndyHandle) = (void (^)(NSError *, IndyHandle)) block;

    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromIndyError:err];
            completion(error, (IndyHandle) pool_handle);
        });
    }
}

void IndyWrapperCommon3TRHCallback(indy_handle_t xcommand_handle,
        indy_error_t err,
        indy_i32_t handle) {
    id block = [[IndyCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[IndyCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];

    void (^completion)(NSError *, NSNumber *) = (void (^)(NSError *, NSNumber *arg1)) block;
    NSNumber *sarg1 = [NSNumber numberWithInt:handle];

    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromIndyError:err];
            completion(error, (NSNumber *) sarg1);
        });
    }
}

void IndyWrapperCommon3PSCallback(indy_handle_t xcommand_handle,
        indy_error_t err,
        const char *const arg1) {
    id block = [[IndyCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[IndyCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];

    void (^completion)(NSError *, NSString *) = (void (^)(NSError *, NSString *arg1)) block;
    NSString *sarg1 = [NSString stringWithUTF8String:arg1];

    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromIndyError:err];
            completion(error, sarg1);
        });
    }
}

void IndyWrapperCommon3PBCallback(indy_handle_t xcommand_handle,
        indy_error_t err,
        indy_bool_t arg1) {
    id block = [[IndyCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[IndyCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];

    void (^completion)(NSError *, BOOL) = (void (^)(NSError *, BOOL arg1)) block;

    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromIndyError:err];
            completion(error, (BOOL) arg1);
        });
    }
}

void IndyWrapperCommon4PCallback(indy_handle_t xcommand_handle,
        indy_error_t err,
        const char *const arg1,
        const char *const arg2) {
    id block = [[IndyCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[IndyCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];

    void (^completion)(NSError *, NSString *arg1, NSString *arg2) = (void (^)(NSError *, NSString *arg1, NSString *arg2)) block;

    NSString *sarg1 = [NSString stringWithUTF8String:arg1];
    NSString *sarg2 = [NSString stringWithUTF8String:arg2];

    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromIndyError:err];
            completion(error, sarg1, sarg2);
        });
    }
}

void IndyWrapperCommon4PSCallback(indy_handle_t xcommand_handle,
        indy_error_t err,
        const char *const arg1,
        const char *const arg2) {
    id block = [[IndyCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[IndyCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];

    void (^completion)(NSError *, NSString *arg1, NSString *arg2) = (void (^)(NSError *, NSString *arg1, NSString *arg2)) block;

    NSString *sarg1 = nil;
    if (arg1) {
        sarg1 = [NSString stringWithUTF8String:arg1];
    }
    NSString *sarg2 = [NSString stringWithUTF8String:arg2];

    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromIndyError:err];
            completion(error, sarg1, sarg2);
        });
    }
}

void IndyWrapperCommon5PStrOpStrOpStrCallback(indy_handle_t xcommand_handle,
        indy_error_t err,
        const char *const arg1,
        const char *const arg2,
        const char *const arg3) {
    id block = [[IndyCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[IndyCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];

    void (^completion)(NSError *, NSString *arg1, NSString *arg2, NSString *arg3) = (void (^)(NSError *, NSString *arg1, NSString *arg2, NSString *arg3)) block;

    NSString *sarg1 = [NSString stringWithUTF8String:arg1];
    NSString *sarg2 = nil;
    if (arg2) {
        sarg2 = [NSString stringWithUTF8String:arg2];
    }
    NSString *sarg3 = nil;
    if (arg3) {
        sarg3 = [NSString stringWithUTF8String:arg3];
    }

    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromIndyError:err];
            completion(error, sarg1, sarg2, sarg3);
        });
    }
}

void IndyWrapperCommon5PCallback(indy_handle_t xcommand_handle,
        indy_error_t err,
        const char *const arg1,
        const char *const arg2,
        const char *const arg3) {
    id block = [[IndyCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[IndyCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];

    void (^completion)(NSError *, NSString *arg1, NSString *arg2, NSString *arg3) = (void (^)(NSError *, NSString *arg1, NSString *arg2, NSString *arg3)) block;

    NSString *sarg1 = [NSString stringWithUTF8String:arg1];
    NSString *sarg2 = [NSString stringWithUTF8String:arg2];
    NSString *sarg3 = [NSString stringWithUTF8String:arg3];

    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromIndyError:err];
            completion(error, sarg1, sarg2, sarg3);
        });
    }
}

/// Arguments arg1 and arg2 will be converted to nsdata
void IndyWrapperCommon4PDataCallback(indy_handle_t xcommand_handle,
        indy_error_t err,
        const uint8_t *arg1,
        uint32_t arg2) {
    id block = [[IndyCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[IndyCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];

    void (^completion)(NSError *, NSData *arg) = (void (^)(NSError *, NSData *arg)) block;

    NSData *sarg = [NSData dataWithBytes:arg1 length:arg2];

    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromIndyError:err];
            completion(error, sarg);
        });
    }
}

void IndyWrapperCommon5PSCallback(indy_handle_t xcommand_handle,
        indy_error_t err,
        indy_handle_t connection_handle,
        const char *arg1,
        const char *arg2) {
    id block = [[IndyCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[IndyCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];

    void (^completion)(NSError *, IndyHandle, NSString *arg1, NSString *arg2) = (void (^)(NSError *, IndyHandle, NSString *arg1, NSString *arg2)) block;

    NSString *sarg1 = [NSString stringWithUTF8String:arg1];
    NSString *sarg2 = [NSString stringWithUTF8String:arg2];

    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromIndyError:err];
            completion(error, (IndyHandle) connection_handle, sarg1, sarg2);
        });
    }
}

void IndyWrapperCommon5PSDataCallback(indy_handle_t xcommand_handle,
        indy_error_t err,
        const char *arg1,
        const uint8_t *arg2,
        uint32_t arg3) {
    id block = [[IndyCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[IndyCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];

    void (^completion)(NSError *, NSString *, NSData *) = (void (^)(NSError *, NSString *, NSData *)) block;

    NSString *sarg1 = nil;
    if (arg1) {
        sarg1 = [NSString stringWithUTF8String:arg1];
    }
    NSData *sarg2 = [NSData dataWithBytes:arg2 length:arg3];

    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromIndyError:err];
            completion(error, sarg1, sarg2);
        });
    }
}

void IndyWrapperCommon6PDataCallback(indy_handle_t xcommand_handle,
        indy_error_t err,
        const uint8_t *arg1,
        uint32_t arg2,
        const uint8_t *arg3,
        uint32_t arg4) {
    id block = [[IndyCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[IndyCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];

    void (^completion)(NSError *, NSData *xArg1, NSData *xArg2) = (void (^)(NSError *, NSData *xArg1, NSData *xArg2)) block;

    NSData *sarg1 = [NSData dataWithBytes:arg1 length:arg2];
    NSData *sarg2 = [NSData dataWithBytes:arg3 length:arg4];

    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromIndyError:err];
            completion(error, sarg1, sarg2);
        });
    }
}

void IndyWrapperCommon5SSUCallback(indy_handle_t xcommand_handle,
        indy_error_t err,
        const char *arg1,
        const char *arg2,
        unsigned long long arg3) {
    id block = [[IndyCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[IndyCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];

    void (^completion)(NSError *, NSString *, NSString *, NSNumber *) = (void (^)(NSError *, NSString *arg1, NSString *arg2, NSNumber *arg3)) block;
    NSString *sarg1 = [NSString stringWithUTF8String:arg1];
    NSString *sarg2 = [NSString stringWithUTF8String:arg2];
    NSNumber *sarg3 = [NSNumber numberWithInt:arg3];


    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromIndyError:err];
            completion(error, (NSString *) sarg1, (NSString *) sarg2, (NSNumber *) sarg3);
        });
    }
}

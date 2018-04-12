//
//  IndyCallbacks.h
//  libindy
//


#import <Foundation/Foundation.h>
#import "indy_core.h"

extern void IndyWrapperCommon2PCallback(indy_handle_t xcommand_handle,
        indy_error_t err);

extern void IndyWrapperCommon3PHCallback(indy_handle_t xcommand_handle,
        indy_error_t err,
        indy_handle_t pool_handle);

extern void IndyWrapperCommon3PSCallback(indy_handle_t xcommand_handle,
        indy_error_t err,
        const char *const arg1);

extern void IndyWrapperCommon3PBCallback(indy_handle_t xcommand_handle,
        indy_error_t err,
        indy_bool_t arg1);

extern void IndyWrapperCommon4PCallback(indy_handle_t xcommand_handle,
        indy_error_t err,
        const char *const arg1,
        const char *const arg2);

extern void IndyWrapperCommon4PSCallback(indy_handle_t xcommand_handle,
        indy_error_t err,
        const char *const arg1,
        const char *const arg2);

extern void IndyWrapperCommon4PDataCallback(indy_handle_t xcommand_handle,
        indy_error_t err,
        const uint8_t *const arg1,
        uint32_t arg2);

extern void IndyWrapperCommon5PCallback(indy_handle_t xcommand_handle,
        indy_error_t err,
        const char *const arg1,
        const char *const arg2,
        const char *const arg3);

extern void IndyWrapperCommon5PSCallback(indy_handle_t xcommand_handle,
        indy_error_t err,
        indy_handle_t connection_handle,
        const char *const arg1,
        const char *const arg2);

extern void IndyWrapperCommon5PSDataCallback(indy_handle_t xcommand_handle,
        indy_error_t err,
        const char *const arg1,
        const uint8_t *const arg2,
        uint32_t arg3);

extern void IndyWrapperCommon6PDataCallback(indy_handle_t xcommand_handle,
        indy_error_t err,
        const uint8_t *const arg1,
        uint32_t arg2,
        const uint8_t *const arg3,
        uint32_t arg4);

extern void IndyWrapperCommon3TRHCallback(indy_handle_t xcommand_handle,
        indy_error_t err,
        indy_i32_t handle);

extern void IndyWrapperCommon5PStrOpStrOpStrCallback(indy_handle_t xcommand_handle,
        indy_error_t err,
        const char *const arg1,
        const char *const arg2,
        const char *const arg3);

void IndyWrapperCommon5SSUCallback(indy_handle_t xcommand_handle,
        indy_error_t err,
        const char *arg1,
        const char *arg2,
        unsigned long long arg3);

@interface IndyCallbacks : NSObject

// MARK: - Store callback and create command handle
- (indy_handle_t)createCommandHandleFor:(id)callback;

- (void)deleteCommandHandleFor:(indy_handle_t)handle;

- (void)complete:(void (^)(NSError *))completion
       forHandle:(indy_handle_t)handle
         ifError:(indy_error_t)ret;

- (void)completeBool:(void (^)(NSError *, BOOL))completion
           forHandle:(indy_handle_t)handle
             ifError:(indy_error_t)ret;

- (void)completeStr:(void (^)(NSError *, NSString *))completion
          forHandle:(indy_handle_t)handle
            ifError:(indy_error_t)ret;

- (void)complete2Str:(void (^)(NSError *, NSString *, NSString *))completion
           forHandle:(indy_handle_t)handle
             ifError:(indy_error_t)ret;

- (void)completeData:(void (^)(NSError *, NSData *))completion
           forHandle:(indy_handle_t)handle
             ifError:(indy_error_t)ret;

- (void)complete2Data:(void (^)(NSError *, NSData *, NSData *))completion
            forHandle:(indy_handle_t)handle
              ifError:(indy_error_t)ret;


- (void)completeStringAndData:(void (^)(NSError *, NSString *, NSData *))completion
                    forHandle:(indy_handle_t)handle
                      ifError:(indy_error_t)ret;

+ (IndyCallbacks *)sharedInstance;

@end

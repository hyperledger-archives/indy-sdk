//
//  VcxCallbacks.h
//


#import <Foundation/Foundation.h>
#import "vcx.h"



@interface VcxCallbacks : NSObject

// MARK: - Store callback and create command handle
- (vcx_command_handle_t)createCommandHandleFor:(id)callback;

- (id)commandCompletionFor:(vcx_command_handle_t)handle;

- (void)deleteCommandHandleFor:(vcx_command_handle_t)handle;

- (void)complete:(void (^)(NSError *))completion
       forHandle:(vcx_command_handle_t)handle
         ifError:(vcx_error_t)ret;

- (void)completeBool:(void (^)(NSError *, BOOL))completion
           forHandle:(vcx_command_handle_t)handle
             ifError:(vcx_error_t)ret;

- (void)completeStr:(void (^)(NSError *, NSString *))completion
          forHandle:(vcx_command_handle_t)handle
            ifError:(vcx_error_t)ret;

- (void)complete2Str:(void (^)(NSError *, NSString *, NSString *))completion
           forHandle:(vcx_command_handle_t)handle
             ifError:(vcx_error_t)ret;

- (void)completeData:(void (^)(NSError *, NSData *))completion
           forHandle:(vcx_command_handle_t)handle
             ifError:(vcx_error_t)ret;


- (void)completeStringAndData:(void (^)(NSError *, NSString *, NSData *))completion
                    forHandle:(vcx_command_handle_t)handle
                      ifError:(vcx_error_t)ret;

+ (VcxCallbacks *)sharedInstance;

@end

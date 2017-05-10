//
//  SovrinCallbacks.h
//  libsovrin
//


#import <Foundation/Foundation.h>
#import "sovrin_core.h"

extern void SovrinWrapperCommon2PCallback(sovrin_handle_t xcommand_handle, sovrin_error_t err);
extern void SovrinWrapperCommon3PHCallback(sovrin_handle_t xcommand_handle, sovrin_error_t err, sovrin_handle_t pool_handle);
extern void SovrinWrapperCommon3PSCallback(sovrin_handle_t xcommand_handle, sovrin_error_t err, const char* arg1);
extern void SovrinWrapperCommon3PBCallback(sovrin_handle_t xcommand_handle, sovrin_error_t err, sovrin_bool_t arg1);
extern void SovrinWrapperCommon4PCallback(sovrin_handle_t xcommand_handle, sovrin_error_t err, const char* arg1, const char *arg2);

@interface SovrinCallbacks : NSObject

- (sovrin_handle_t) add:(void*) cb;
- (void)            remove:(sovrin_handle_t) handle;
- (void*)           get:(sovrin_handle_t) handle;

+ (SovrinCallbacks*) sharedInstance;

@end

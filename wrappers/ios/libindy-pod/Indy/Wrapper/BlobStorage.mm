#import <Foundation/Foundation.h>
#import "IndyCallbacks.h"
#import "IndyBlobStorage.h"
#import "indy_core.h"
#import "NSError+IndyError.h"

@implementation IndyBlobStorage

+ (void)openReaderWithType:(NSString *)type
                configJson:(NSString *)configJson
                  location:(NSString *)location
                      hash:(NSString *)hash
                completion:(void (^)(NSError *error, IndyHandle handle))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_open_blob_storage_reader(handle,
            [type UTF8String],
            [configJson UTF8String],
            [location UTF8String],
            [hash UTF8String],
            IndyWrapperCommon3PHCallback
    );
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], 0);
        });
    }
}

@end


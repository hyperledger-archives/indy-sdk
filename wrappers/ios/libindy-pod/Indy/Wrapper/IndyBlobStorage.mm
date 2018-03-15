#import <Foundation/Foundation.h>
#import "IndyCallbacks.h"
#import "IndyBlobStorage.h"
#import "indy_core.h"
#import "NSError+IndyError.h"

@implementation IndyBlobStorage

+ (void)openReaderWithType:(NSString *)type_
                    config:(NSString *)config
                  location:(NSString *)location
                      hash:(NSString *)hash
                completion:(void (^)(NSError *error, NSNumber *handle))completion {
    indy_error_t ret;

    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    ret = indy_blob_storage_open_reader(handle,
            [type_ UTF8String],
            [config UTF8String],
            [location UTF8String],
            [hash UTF8String],
            IndyWrapperCommon3TRHCallback
    );
    if (ret != Success) {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor:handle];

        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromIndyError:ret], 0);
        });
    }
}

@end


#import "IndyAnoncreds.h"
#import "indy_core.h"
#import "NSError+IndyError.h"
#import "IndyTypes.h"

@implementation IndyUtils : NSObject

+ (void)setRuntimeConfig:(NSString *)config {
    indy_set_runtime_config([config UTF8String]);
}

@end

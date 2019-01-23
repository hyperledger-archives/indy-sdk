//
//  NSError+IndyError.m
//  libindy
//

#import "NSError+IndyError.h"
#import "NSDictionary+JSON.h"

static NSString *const IndyErrorDomain = @"IndyErrorDomain";

@implementation NSError (IndyError)

+ (NSError *)errorFromIndyError:(indy_error_t)error {

    NSMutableDictionary *userInfo = [NSMutableDictionary new];

    if (error != Success) {
        const char * error_json_p;
        indy_get_current_error(&error_json_p);
        NSString *errorDetailsJson = [NSString stringWithUTF8String:error_json_p];

        NSDictionary *errorDetails = [NSDictionary fromString:errorDetailsJson];
        [userInfo setValue:errorDetails[@"message"] forKey:@"message"];
        [userInfo setValue:errorDetails[@"backtrace"] forKey:@"indy_backtrace"];
    }

    return [NSError errorWithDomain:IndyErrorDomain code:error userInfo:userInfo];
}

@end

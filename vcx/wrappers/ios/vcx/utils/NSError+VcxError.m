//
//  NSError+VcxError.m
//  libindy
//

#import "NSError+VcxError.h"

static NSString *const VcxErrorDomain = @"VcxErrorDomain";

@implementation NSError (VcxError)

+ (NSError*) errorFromVcxError:(vcx_error_t) error
{
    return [NSError errorWithDomain:VcxErrorDomain code: error userInfo:nil];
}

@end

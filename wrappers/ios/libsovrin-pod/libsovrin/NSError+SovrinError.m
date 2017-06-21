//
//  NSError+SovrinError.m
//  libsovrin
//

#import "NSError+SovrinError.h"

static NSString *const SovrinErrorDomain = @"SovrinErrorDomain";

@implementation NSError (SovrinError)

+ (NSError*) errorFromSovrinError:(sovrin_error_t) error
{
    return [NSError errorWithDomain:SovrinErrorDomain code: error userInfo:nil];
}

@end

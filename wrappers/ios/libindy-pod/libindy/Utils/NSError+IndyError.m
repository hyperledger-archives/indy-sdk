//
//  NSError+IndyError.m
//  libindy
//

#import "NSError+IndyError.h"

static NSString *const IndyErrorDomain = @"IndyErrorDomain";

@implementation NSError (IndyError)

+ (NSError*) errorFromIndyError:(indy_error_t) error
{
    return [NSError errorWithDomain:IndyErrorDomain code: error userInfo:nil];
}

@end

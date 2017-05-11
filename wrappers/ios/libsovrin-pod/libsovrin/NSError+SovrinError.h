//
//  NSError+SovrinError.h
//  libsovrin
//

#import <Foundation/Foundation.h>
#import "sovrin_core.h"

@interface NSError (SovrinError)

+ (NSError*) errorFromSovrinError:(sovrin_error_t) error;

@end

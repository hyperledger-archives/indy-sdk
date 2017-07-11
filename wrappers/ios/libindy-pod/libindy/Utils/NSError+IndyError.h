//
//  NSError+IndyError.h
//  libindy
//

#import <Foundation/Foundation.h>
#import "indy_core.h"

@interface NSError (IndyError)

+ (NSError*) errorFromIndyError:(indy_error_t) error;

@end

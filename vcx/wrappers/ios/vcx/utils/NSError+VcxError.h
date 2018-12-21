//
//  NSError+VcxError.h
//  libindy
//

#import <Foundation/Foundation.h>
#import "vcx.h"

@interface NSError (VcxError)

+ (NSError*) errorFromVcxError:(vcx_error_t) error;

@end

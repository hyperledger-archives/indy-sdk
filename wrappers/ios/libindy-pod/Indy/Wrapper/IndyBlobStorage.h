//
//  IndyBlobStorage.h
//  libindy
//


#import <Foundation/Foundation.h>
#import "IndyTypes.h"

@interface IndyBlobStorage : NSObject

+ (void)openReaderWithType:(NSString *)type
                    config:(NSString *)config
                  location:(NSString *)location
                      hash:(NSString *)hash
                completion:(void (^)(NSError *error, NSNumber *handle))completion;

@end

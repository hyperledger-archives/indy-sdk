//
//  IndyBlobStorage.h
//  libindy
//


#import <Foundation/Foundation.h>
#import "IndyTypes.h"

@interface IndyBlobStorage : NSObject

+ (void)openReaderWithType:(NSString *)type
                    config:(NSString *)config
                completion:(void (^)(NSError *error, NSNumber *handle))completion;

+ (void)openWriterWithType:(NSString *)type
                    config:(NSString *)config
                completion:(void (^)(NSError *error, NSNumber *handle))completion;

@end

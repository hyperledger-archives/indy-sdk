//
//  IndyPool.h
//  libindy
//


#import <Foundation/Foundation.h>
#import "IndyTypes.h"

@interface IndyBlobStorage : NSObject

+ (void)openReaderWithType:(NSString *)type
                    configJson:(NSString *)configJson
                    location:(NSString *)location
                    hash:(NSString *)hash
                    completion:(void (^)(NSError *error, IndyHandle handle)) completion;

@end

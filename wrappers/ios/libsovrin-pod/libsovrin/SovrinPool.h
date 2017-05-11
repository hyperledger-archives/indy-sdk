//
//  SovrinPool.h
//  libsovrin
//


#import <Foundation/Foundation.h>
#import "SovrinTypes.h"

@interface SovrinPool : NSObject

- (NSError*) createPoolWithName:(NSString*) name
                      andConfig:(NSString*) config
                     completion:(void (^)(NSError* error)) handler;

- (NSError*) openPoolWithName:(NSString*) name
                    andConfig:(NSString*) config
                   completion:(void (^)(NSError* error, SovrinHandle handle)) handler;

- (NSError*) refreshPoolWithHandle:(SovrinHandle) SovrinHandle
                        completion:(void (^)(NSError* error)) handler;

- (NSError*) closePoolWithHandle:(SovrinHandle) SovrinHandle
                      completion:(void (^)(NSError* error)) handler;

- (NSError*) deletePoolWithName:(NSString*) name
                     completion:(void (^)(NSError* error)) handler;

+ (SovrinPool*) sharedInstance;

@end

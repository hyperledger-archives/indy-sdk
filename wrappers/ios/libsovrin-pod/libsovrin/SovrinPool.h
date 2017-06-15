//
//  SovrinPool.h
//  libsovrin
//


#import <Foundation/Foundation.h>
#import "SovrinTypes.h"

@interface SovrinPool : NSObject

+ (NSError *)createPoolLedgerConfigWithName:(NSString *)name
                                 poolConfig:(NSString *)config
                                 completion:(void (^)(NSError *error)) handler;

+ (NSError *)openPoolLedgerWithName:(NSString *)name
                         poolConfig:(NSString *)config
                         completion:(void (^)(NSError *error, SovrinHandle handle)) handler;

+ (NSError *)refreshPoolWithHandle:(SovrinHandle)SovrinHandle
                        completion:(void (^)(NSError *error)) handler;

+ (NSError *)closePoolWithHandle:(SovrinHandle)SovrinHandle
                      completion:(void (^)(NSError *error)) handler;

+ (NSError *)deletePoolWithName:(NSString *)name
                     completion:(void (^)(NSError *error)) handler;


@end

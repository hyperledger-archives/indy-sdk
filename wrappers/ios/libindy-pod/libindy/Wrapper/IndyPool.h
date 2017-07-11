//
//  SovrinPool.h
//  libsovrin
//


#import <Foundation/Foundation.h>
#import "IndyTypes.h"

@interface SovrinPool : NSObject

+ (NSError *)createPoolLedgerConfigWithPoolName:(NSString *)name
                                     poolConfig:(NSString *)config
                                     completion:(void (^)(NSError *error)) handler;

+ (NSError *)openPoolLedgerWithName:(NSString *)name
                         poolConfig:(NSString *)config
                         completion:(void (^)(NSError *error, SovrinHandle handle)) handler;

+ (NSError *)refreshPoolLedgerWithHandle:(SovrinHandle)poolHandle
                              completion:(void (^)(NSError *error)) handler;

+ (NSError *)closePoolLedgerWithHandle:(SovrinHandle)SovrinHandle
                            completion:(void (^)(NSError *error)) handler;

+ (NSError *)deletePoolLedgerConfigWithName:(NSString *)name
                                 completion:(void (^)(NSError *error)) handler;


@end

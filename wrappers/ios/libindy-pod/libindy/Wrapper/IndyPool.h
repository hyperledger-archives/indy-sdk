//
//  IndyPool.h
//  libindy
//


#import <Foundation/Foundation.h>
#import "IndyTypes.h"

@interface IndyPool : NSObject

+ (NSError *)createPoolLedgerConfigWithPoolName:(NSString *)name
                                     poolConfig:(NSString *)config
                                     completion:(void (^)(NSError *error)) handler;

+ (NSError *)openPoolLedgerWithName:(NSString *)name
                         poolConfig:(NSString *)config
                         completion:(void (^)(NSError *error, IndyHandle poolHandle)) handler;

+ (NSError *)refreshPoolLedgerWithHandle:(IndyHandle)poolHandle
                              completion:(void (^)(NSError *error)) handler;

+ (NSError *)closePoolLedgerWithHandle:(IndyHandle)IndyHandle
                            completion:(void (^)(NSError *error)) handler;

+ (NSError *)deletePoolLedgerConfigWithName:(NSString *)name
                                 completion:(void (^)(NSError *error)) handler;

@end

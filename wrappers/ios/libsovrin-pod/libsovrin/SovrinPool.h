//
//  SovrinPool.h
//  libsovrin
//
//  Created by Kirill Neznamov on 28/04/2017.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <Foundation/Foundation.h>
#import "SovrinTypes.h"

@interface SovrinPool : NSObject

- (NSError*) createPool:(CommandHandle) handle
               withName:(NSString*) name
              andConfig:(NSString*) config
             completion:(NSError* (^)(CommandHandle xhandle, NSError* error)) handler;

- (NSError*) openPool:(CommandHandle) handle
             withName:(NSString*) name
            andConfig:(NSString*) config
           completion:(NSError* (^)(CommandHandle xhandle, NSError* error, PoolHandle handle)) handler;

- (NSError*) refreshPool:(CommandHandle) handle
              withHandle:(PoolHandle) poolHandle
              completion:(NSError* (^)(CommandHandle xhandle, NSError* error)) handler;

- (NSError*) closePool:(CommandHandle) handle
            withHandle:(PoolHandle) poolHandle
            completion:(NSError* (^)(CommandHandle xhandle, NSError* error)) handler;

- (NSError*) deletePool:(CommandHandle) handle
               withName:(NSString*) name
             completion:(NSError* (^)(CommandHandle xhandle, NSError* error)) handler;

@end

//
//  SovrinPool.m
//  libsovrin
//
//  Created by Kirill Neznamov on 28/04/2017.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <Foundation/Foundation.h>
#import  "SovrinPool.h"
#include "mod.h"
#include "pool.h"



@implementation SovrinPool

- (NSError*) createPool:(CommandHandle) handle
               withName:(NSString*) name
              andConfig:(NSString*) config
             completion:(NSError* (^)(CommandHandle xhandle, NSError* error)) handler
{
    
    sovrin_create_pool_ledger_config(handle,
                                     [name UTF8String],
                                     [config UTF8String],
                                     NULL
                                     );
    return nil;
}
@end


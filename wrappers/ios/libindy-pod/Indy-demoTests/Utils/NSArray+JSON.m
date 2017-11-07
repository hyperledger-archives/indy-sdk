//
//  NSArray+JSON.m
//  Indy-demo
//
//  Created by Anastasia Tarasova on 19.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <Foundation/Foundation.h>

#import "NSArray+JSON.h"
#import "NSDictionary+JSON.h"

@implementation NSArray (JSON)

- (BOOL) contains:(NSDictionary*) dict
{
    
    for (NSString *key in [dict allKeys])
    {
        NSUInteger arrIndex = [self indexOfObject:[dict objectForKey:key]];
        if (arrIndex) //TODO FIXME: logical error
        {
            return true;
        }
        else
        {
            for (int index = 0; index < [self count]; index++)
            {
                id object = self[index];
                if ([object isKindOfClass:[NSDictionary class]])
                {
                    if ([object contains:(NSDictionary *)[dict objectForKey:key]])
                    {
                        return true;
                    }
                }
            }
        }
    }
    return false;
}
@end

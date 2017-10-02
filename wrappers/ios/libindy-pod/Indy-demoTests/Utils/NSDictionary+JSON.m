//
//  NSDictionary+JSON.m
//  Indy-demo
//
//  Created by Kirill Neznamov on 30/05/2017.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import "NSDictionary+JSON.h"

@implementation NSDictionary (JSON)

// WARNING: - PrettyPrinted flag is not suitable to configure JSON as RUST argument right now. Wait for fix and return it.
+ (NSString*) toString:(NSDictionary*) dict
{
    NSError *error;
    NSData *jsonData = [NSJSONSerialization dataWithJSONObject:dict
                                                       options:kNilOptions
                                                         error:&error];
    
    if (!jsonData)
    {
        return nil;
    }
    else
    {
        return [[NSString alloc] initWithData:jsonData encoding:NSUTF8StringEncoding];
    }
}

+ (NSDictionary*) fromString:(NSString*) s
{
    NSError *error;
    NSDictionary *dict = [NSJSONSerialization JSONObjectWithData:[NSData dataWithBytes:[s UTF8String]
                                                                                length:[s length]]
                                                         options:kNilOptions
                                                           error: &error];
    return dict;
}

- (BOOL) contains:(NSDictionary*) dict
{
    for (NSString *key in [dict allKeys])
    {
        id object = [self objectForKey:key];
        if ([object isKindOfClass:[NSDictionary class]])
        {
            [[self objectForKey:key] contains:(NSDictionary *)object];
        }
        else if ([self objectForKey:key] != nil
                 && ![[self objectForKey:key] isEqual:[dict objectForKey:key]])
        {
            return NO;
        }
    }
    return YES;
}
@end

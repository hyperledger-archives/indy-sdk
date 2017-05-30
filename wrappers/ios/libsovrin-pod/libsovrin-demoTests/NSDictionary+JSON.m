//
//  NSDictionary+JSON.m
//  libsovrin-demo
//
//  Created by Kirill Neznamov on 30/05/2017.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import "NSDictionary+JSON.h"

@implementation NSDictionary (JSON)

+ (NSString*) toString:(NSDictionary*) dict
{
    NSError *error;
    NSData *jsonData = [NSJSONSerialization dataWithJSONObject:dict
                                                       options:NSJSONWritingPrettyPrinted
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
@end

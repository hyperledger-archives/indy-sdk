//
//  NSDictionary+JSON.m
//  libindy-demo
//

#import "NSDictionary+JSON.h"

@implementation NSDictionary (JSON)

// WARNING: - PrettyPrinted flag is not suitable to configure JSON as RUST argument right now. Wait for fix and return it.
- (NSString*) toString
{
    NSError *error;
    NSData *jsonData = [NSJSONSerialization dataWithJSONObject:self
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

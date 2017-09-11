//
//  NSString+JSON.m
//  libindy
//

#import "NSString+JSON.h"

@implementation NSString (JSON)

- (NSDictionary*) toDictionary
{
    NSError *error;
    NSDictionary *dict = [NSJSONSerialization JSONObjectWithData:[NSData dataWithBytes:[self UTF8String]
                                                                                length:[self length]]
                                                         options:kNilOptions
                                                           error: &error];
    return dict;
    
}

@end

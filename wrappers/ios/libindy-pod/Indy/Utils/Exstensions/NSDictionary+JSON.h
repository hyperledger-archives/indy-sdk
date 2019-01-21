//
//  NSDictionary+JSON.h
//  libindy-demo
//

#import <Foundation/Foundation.h>

@interface NSDictionary (JSON)

- (NSString*) toString;
+ (NSDictionary*) fromString:(NSString*) s;
- (BOOL) contains:(NSDictionary*) dict;

@end

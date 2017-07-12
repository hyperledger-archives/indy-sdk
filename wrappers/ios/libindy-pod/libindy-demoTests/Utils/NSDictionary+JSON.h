//
//  NSDictionary+JSON.h
//  libindy-demo
//


#import <XCTest/XCTest.h>

@interface NSDictionary (JSON)

+ (NSString*) toString:(NSDictionary*) dict;
+ (NSDictionary*) fromString:(NSString*) s;
- (BOOL) contains:(NSDictionary*) dict;

@end

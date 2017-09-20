//
//  NSDictionary+JSON.h
//  libindy-demo
//

#import <Foundation/Foundation.h>

@interface NSDictionary (JSON)

- (NSString*) toString;
- (BOOL) contains:(NSDictionary*) dict;

@end

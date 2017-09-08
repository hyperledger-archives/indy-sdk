//
//  SequenceUtils.h
//  libindy
//

#import <Foundation/Foundation.h>

@interface SequenceUtils : NSObject

+ (SequenceUtils*) sharedInstance;

- (NSUInteger)getNextId;

@end

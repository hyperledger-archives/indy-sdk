//
//  IndySequenceUtils.h
//  libindy
//

#import <Foundation/Foundation.h>

@interface IndySequenceUtils : NSObject

+ (IndySequenceUtils*) sharedInstance;

- (NSUInteger)getNextId;

@end

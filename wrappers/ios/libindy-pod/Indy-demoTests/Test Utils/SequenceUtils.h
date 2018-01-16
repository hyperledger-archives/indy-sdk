//
//  SequenceUtils.h
//  Indy
//
//  Created by Anastasia Tarasova on 10/07/2017.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <Foundation/Foundation.h>

@interface SequenceUtils : NSObject

+ (SequenceUtils*) sharedInstance;

- (NSUInteger)getNextId;

@end

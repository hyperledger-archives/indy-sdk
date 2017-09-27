//
//  NSString+Validation.m
//  Indy-demo
//
//  Created by Anastasia Tarasova on 19.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import "NSString+Validation.h"
#import <Foundation/Foundation.h>

@implementation NSString (Validation)

- (BOOL)isValid
{
    if (self == nil || [self isEqualToString:@""])
    {
        return false;
    }
    
    return true;
}

@end


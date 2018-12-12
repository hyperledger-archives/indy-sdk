//
//  IndySequenceUtils.h
//  libindy
//

#import <Foundation/Foundation.h>

extern void LogCb(const void*  context,
        uint32_t level,
        const char *target,
        const char *message,
        const char *modulePath,
        const char *file,
        uint32_t line);

@interface IndyLogger : NSObject

+ (void)setLogger;

@end
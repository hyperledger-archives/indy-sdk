//
//  IndyAgent.h
//  libindy
//


#import <Foundation/Foundation.h>
#import "IndyTypes.h"

@interface IndyAgent: NSObject

+ (void)prepareMsg:(NSData *)msg
  withWalletHandle:(IndyHandle)walletHandle
          senderVk:(NSString*)senderVk
       recipientVk:(NSString*)recipientVK
        completion:(void (^)(NSError *error,
                             NSData *encryptedMsg)) completion;

+ (void)prepareAnonymousMsg:(NSData *)msg
            withrecipientVk:(NSString*)recipientVK
                 completion:(void (^)(NSError *error,
                                      NSData *encryptedMsg)) completion;

+ (void)parseMsg:(NSData *)msg
withWalletHandle:(IndyHandle)walletHandle
     recipientVk:(NSString*)recipientVK
      completion:(void (^)(NSError *error,
                           NSData *dencryptedMsg)) completion;

@end

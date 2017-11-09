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
       recipientVk:(NSString*)recipientVk
        completion:(void (^)(NSError *error,
                             NSData *encryptedMsg)) completion;

+ (void)prepareAnonymousMsg:(NSData *)msg
            withRecipientVk:(NSString *)recipientVk
                 completion:(void (^)(NSError *error,
                                      NSData *encryptedMsg)) completion;

+ (void)parseMsg:(NSData *)msg
withWalletHandle:(IndyHandle)walletHandle
     recipientVk:(NSString*)recipientVk
      completion:(void (^)(NSError *error,
                           NSString *senderVk,
                           NSData *dencryptedMsg)) completion;

@end

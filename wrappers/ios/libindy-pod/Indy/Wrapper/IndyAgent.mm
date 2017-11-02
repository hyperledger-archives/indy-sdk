//
//  IndyAgent.m
//  libindy
//

#import "IndyAgent.h"
#import "IndyCallbacks.h"
#import "indy_core.h"
#import "NSError+IndyError.h"

@implementation IndyAgent

+ (void)prepareMsg:(NSData *)msg
  withWalletHandle:(IndyHandle)walletHandle
          senderVk:(NSString*)senderVk
       recipientVk:(NSString*)recipientVK
        completion:(void (^)(NSError *error,
                             NSData *encryptedMsg)) completion
{
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    uint32_t messageLen = (uint32_t)[msg length];
    uint8_t *messageRaw = (uint8_t *)[msg bytes];

    indy_error_t ret = indy_prep_msg(handle, walletHandle,
                                     [senderVk UTF8String], [recipientVK UTF8String],
                                     messageRaw, messageLen, IndyWrapperCommon4PDataCallback);

    if (ret!= Success)
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];

        dispatch_async(dispatch_get_main_queue(),^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)prepareAnonymousMsg:(NSData *)msg
            withrecipientVk:(NSString*)recipientVK
                 completion:(void (^)(NSError *error,
                                      NSData *encryptedMsg)) completion
{
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];
    
    uint32_t messageLen = (uint32_t)[msg length];
    uint8_t *messageRaw = (uint8_t *)[msg bytes];
    
    indy_error_t ret = indy_prep_anonymous_msg(handle, [recipientVK UTF8String],
                                               messageRaw, messageLen,
                                               IndyWrapperCommon4PDataCallback);
    
    if (ret!= Success)
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];

        dispatch_async(dispatch_get_main_queue(),^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

+ (void)parseMsg:(NSData *)msg
withWalletHandle:(IndyHandle)walletHandle
     recipientVk:(NSString*)recipientVK
      completion:(void (^)(NSError *error,
                           NSData *dencryptedMsg)) completion
{
    indy_handle_t handle = [[IndyCallbacks sharedInstance] createCommandHandleFor:completion];

    uint32_t messageLen = (uint32_t)[msg length];
    uint8_t *messageRaw = (uint8_t *)[msg bytes];

    indy_error_t ret = indy_parse_msg(handle, walletHandle, [recipientVK UTF8String],
                                      messageRaw, messageLen, IndyWrapperCommon5PSDataCallback);

    if (ret!= Success)
    {
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: handle];

        dispatch_async(dispatch_get_main_queue(),^{
            completion([NSError errorFromIndyError:ret], nil);
        });
    }
}

@end

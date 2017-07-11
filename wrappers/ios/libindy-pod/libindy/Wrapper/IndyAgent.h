//
//  SovrinAgent.h
//  libsovrin
//


#import <Foundation/Foundation.h>
#import "IndyTypes.h"

@interface SovrinAgent: NSObject

+ (NSError *)connectWithPoolHandle:(SovrinHandle)poolHandle
                      walletHandle:(SovrinHandle)walletHandle
                         senderDId:(NSString *)senderDid
                       receiverDId:(NSString *)receiverDid
                 connectionHandler:(void (^)(NSError *error, SovrinHandle connection)) connectionHandler
                    messageHandler:(void (^)(SovrinHandle connectionHandle, NSError *error, NSString *message))messageHandler;

+ (NSError *)listenForEndpoint:(NSString *)endpoint
               listenerHandler:(void (^)(NSError *error,
                                         SovrinHandle listenerHandle))listenerCompletion
             connectionHandler:(void (^)(SovrinHandle xlistenerHandle,
                                         NSError *error,
                                         SovrinHandle connectionHandle,
                                         NSString *senderDid,
                                         NSString *receiverDid))connectionCompletion
                messageHandler:(void (^)(SovrinHandle xconnectionHandle,
                                         NSError *error,
                                         NSString *message))messageCompletion;

+ (NSError *)sendWithConnectionHandle:(SovrinHandle)connectionHandle
                             messsage:(NSString *)message
                           completion:(void (^)(NSError *error)) handler;

+ (NSError *)addIdentity:(NSString *)did
       forListenerHandle:(SovrinHandle)listenerHandle
              poolHandle:(SovrinHandle)poolHandle
            walletHandle:(SovrinHandle)walletHandle
              completion:(void (^)(NSError *error)) handler;

+ (NSError *)removeIdentity:(NSString *)did
             forListenerHandle:(SovrinHandle)listenerHandle
               walletHandle:(SovrinHandle)walletHandle
                 completion:(void (^)(NSError *error)) handler;

+ (NSError *)closeConnection:(SovrinHandle)connectionHandle
                  completion:(void (^)(NSError *error)) handler;

+ (NSError *)closeListener:(SovrinHandle)listenerHandle
                completion:(void (^)(NSError *error)) handler;
@end

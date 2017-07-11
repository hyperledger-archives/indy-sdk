//
//  IndyAgent.h
//  libindy
//


#import <Foundation/Foundation.h>
#import "IndyTypes.h"

@interface IndyAgent: NSObject

+ (NSError *)connectWithPoolHandle:(IndyHandle)poolHandle
                      walletHandle:(IndyHandle)walletHandle
                         senderDId:(NSString *)senderDid
                       receiverDId:(NSString *)receiverDid
                 connectionHandler:(void (^)(NSError *error, IndyHandle connection)) connectionHandler
                    messageHandler:(void (^)(IndyHandle connectionHandle, NSError *error, NSString *message))messageHandler;

+ (NSError *)listenForEndpoint:(NSString *)endpoint
               listenerHandler:(void (^)(NSError *error,
                                         IndyHandle listenerHandle))listenerCompletion
             connectionHandler:(void (^)(IndyHandle xlistenerHandle,
                                         NSError *error,
                                         IndyHandle connectionHandle,
                                         NSString *senderDid,
                                         NSString *receiverDid))connectionCompletion
                messageHandler:(void (^)(IndyHandle xconnectionHandle,
                                         NSError *error,
                                         NSString *message))messageCompletion;

+ (NSError *)sendWithConnectionHandle:(IndyHandle)connectionHandle
                             messsage:(NSString *)message
                           completion:(void (^)(NSError *error)) handler;

+ (NSError *)addIdentity:(NSString *)did
       forListenerHandle:(IndyHandle)listenerHandle
              poolHandle:(IndyHandle)poolHandle
            walletHandle:(IndyHandle)walletHandle
              completion:(void (^)(NSError *error)) handler;

+ (NSError *)removeIdentity:(NSString *)did
             forListenerHandle:(IndyHandle)listenerHandle
               walletHandle:(IndyHandle)walletHandle
                 completion:(void (^)(NSError *error)) handler;

+ (NSError *)closeConnection:(IndyHandle)connectionHandle
                  completion:(void (^)(NSError *error)) handler;

+ (NSError *)closeListener:(IndyHandle)listenerHandle
                completion:(void (^)(NSError *error)) handler;
@end

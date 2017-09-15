//
//  IndyAgent.h
//  libindy
//


#import <Foundation/Foundation.h>
#import "IndyTypes.h"

@interface IndyAgent: NSObject

/**
 
 Establishes agent to agent connection.
 
 Information about sender Identity must be saved in the wallet with IndySignus::CreateAndStoreMyDid
 call before establishing of connection.
 
 Information about receiver Identity can be saved in the wallet with IndySignus::StoreTheirDid
 call before establishing of connection. If there is no corresponded wallet record for receiver Identity
 than this call will lookup Identity Ledger and cache this information in the wallet.
 
 Note that messages encryption/decryption will be performed automatically.
 
 @param poolHandle Pool handle (created by IndyPool::openPoolLedgerWithName)
 @param walletHandle Wallet handle (created by IndyWallet::openWalletWithName)
 @param senderDid Id of sender Identity stored in secured Wallet.
 @param receiverDid Id of receiver Identity.
 @param connectionHandler Callback that will be called after establishing of connection or on error. Will be called exactly once with result of connect operation.
 @param messageHandler Callback that will be called on receiving of an incoming message.
 Can be called multiply times: once for each incoming message.
 
 
 /// connection_cb:
 /// - xcommand_handle: command handle to map callback to caller context.
 /// - err: Error code.
 /// - connection_handle: Connection handle to use for messages sending and mapping of incomming messages to this connection.
 /// message_cb:
 /// - xconnection_handle: Connection handle. Indetnifies connection.
 /// - err: Error code.
 /// - message: Received message.
 
 @return Error code
 */
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

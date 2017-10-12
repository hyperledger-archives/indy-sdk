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
 call before establishing of connection.
 
 If there is no corresponded wallet record for receiver Identities
 than this call will lookup Identity Ledger and cache this information in the wallet.
 
 Note that messages encryption/decryption will be performed automatically.
 
 @param poolHandle Pool handle (created by IndyPool::openPoolLedgerWithName)
 @param walletHandle Wallet handle (created by IndyWallet::openWalletWithName)
 @param senderDid Id of sender Identity stored in secured Wallet.
 @param receiverDid Id of receiver Identity.
 @param connectionHandler Callback that will be called after establishing of connection or on error. Will be called exactly once with result of connect operation.
 @param messageHandler Callback that will be called on receiving of an incoming message.
 Can be called multiply times: once for each incoming message.

 @return Error code
 */
+ (NSError *)connectWithPoolHandle:(IndyHandle)poolHandle
                      walletHandle:(IndyHandle)walletHandle
                         senderDId:(NSString *)senderDid
                       receiverDId:(NSString *)receiverDid
                 connectionHandler:(void (^)(NSError *error, IndyHandle connectionHandle)) connectionHandler
                    messageHandler:(void (^)(IndyHandle connectionHandle, NSError *error, NSString *message))messageHandler;


/**
 Starts listening of agent connections.
 
 Listener will accept only connections to registered DIDs by IndyAgent::addIdentity call.
 
 Information about sender Identity for incomming connection validation can be saved in the wallet
 with IndySignus::storeTheirDidWithWalletHandle call before establishing of connection.
 
 If there is no corresponding wallet record for sender Identity than listener will lookup Identity Ledger and cache this
 information in the wallet.
 
 Note that messages encryption/decryption will be performed automatically.
 
 @param endpoint Endpoint to use in starting listener.
 @param listenerCompletion Callback that will be called after listening started or on error.
        Will be called exactly once with result of start listen operation.
 @param connectionCompletion Callback that will be called after establishing of incoming connection.
        Can be called multiply times: once for each incoming connection.
 @param messageCompletion Callback that will be called on receiving of an incoming message.
        Can be called multiply times: once for each incoming message.
 
 @return Error code
 */
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


/**
 Sends message to connected agent.
 
 Note that this call works for both incoming and outgoing connections.
 Note that messages encryption/decryption will be performed automatically.
 

 @param connectionHandle Connection handle returned by IndyAgent::connectWithPoolHandle or IndyAgent::listenForEndpoint calls.
 @param message Message to send.
 @param handler Callback that will be called after message sent or on error. Will be called exactly once.
 
 @return Error code
 */
+ (NSError *)sendWithConnectionHandle:(IndyHandle)connectionHandle
                             messsage:(NSString *)message
                           completion:(void (^)(NSError *error)) handler;

/**
 Add identity to listener.
 
 Performs wallet lookup to find corresponding receiver Identity information.
 Information about receiver Identity must be saved in the wallet with
 IndySignus::createAndStoreMyDid call before this call.
 
 After successfull addIdentity call listener will start to accept incoming connection to added DID.
 
 @param listenerHandle Listener handle (created by indy_agent_listen).
 @param poolHandle Pool handle (created by open_pool_ledger).
 @param walletHandle Wallet handle (created by open_wallet).
 @param did DID of identity.
 
 @param handler Callback that will be called after identity added or on error.
         Will be called exactly once with result of start listen operation.
 
 @return Error code
 */
+ (NSError *)addIdentity:(NSString *)did
       forListenerHandle:(IndyHandle)listenerHandle
              poolHandle:(IndyHandle)poolHandle
            walletHandle:(IndyHandle)walletHandle
              completion:(void (^)(NSError *error)) handler;

/**
 Remove identity from listener.

 Performs wallet lookup to find corresponded receiver Identity information.
 Information about receiver Identity must be saved in the wallet with
 indy_create_and_store_my_did call before this call.

 After successfull removeIdentity call listener will stop accepting incoming connection to removed DID.
 
 @param listenerHandle Listener handle (created by indy_agent_listen).
 @param walletHandle Wallet handle (created by open_wallet).
 @param did DID of identity.
 @param handler Callback that will be called after identity removed or on error.
         Will be called exactly once with result of start listen operation.
 
 @return Error code
 */
+ (NSError *)removeIdentity:(NSString *)did
          forListenerHandle:(IndyHandle)listenerHandle
               walletHandle:(IndyHandle)walletHandle
                 completion:(void (^)(NSError *error)) handler;

/**
 Closes agent connection.
 
 Note that this call works for both incoming and outgoing connections.
 
 @param connectionHandle Connection handle returned by indy_agent_connect or indy_agent_listen calls.
 @param handler Callback that will be called after connection closed or on error. Will be called exactly once.
 
 @return Error code
 */
+ (NSError *)closeConnection:(IndyHandle)connectionHandle
                  completion:(void (^)(NSError *error)) handler;

/**
 Closes listener and stops listening for agent connections.
 
 Note that all opened incomming connections will be closed automatically.
 
 @param listenerHandle Listener handle returned by indy_agent_listen call.
 @param handler Callback that will be called after listener closed or on error. Will be called exactly once.
 
 @return Error code
 */
+ (NSError *)closeListener:(IndyHandle)listenerHandle
                completion:(void (^)(NSError *error)) handler;
@end

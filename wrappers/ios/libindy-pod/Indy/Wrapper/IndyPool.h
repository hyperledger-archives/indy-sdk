//
//  IndyPool.h
//  libindy
//


#import <Foundation/Foundation.h>
#import "IndyTypes.h"

@interface IndyPool : NSObject

/**
 Creates a new local pool ledger configuration that can be used later to connect pool nodes.
 
 @code
 Example poolConfig:
 {
    "genesis_txn": string (optional), A path to genesis transaction file. If NULL, then a default one will be used.
            If file doesn't exists default one will be created.
 }
 @endcode
 
 
 @param name Name of the pool ledger configuration.
 @param poolConfig Pool configuration json. if NULL, then default config will be used. See example above.
 @param completion Completion block, returns error code.
 */
+ (void)createPoolLedgerConfigWithPoolName:(NSString *)name
                                poolConfig:(NSString *)poolConfig
                                completion:(void (^)(NSError *error))completion;

/**
 Opens pool ledger and performs connecting to pool nodes.

 Pool ledger configuration with corresponded name must be previously created
 with indy_create_pool_ledger_config method.  
 
 It is impossible to open pool with the same name more than once.
 
 @code
 Example poolConfig:
  {
      "refresh_on_open": bool (optional), Forces pool ledger to be refreshed immediately after opening.
                       Defaults to true.
      "auto_refresh_time": int (optional), After this time in minutes pool ledger will be automatically refreshed.
                         Use 0 to disable automatic refresh. Defaults to 24*60.
      "network_timeout": int (optional), Network timeout for communication with nodes in milliseconds.
                        Defaults to 20000.
  }
 @endcode
 
 @param name Name of the pool ledger configuration.
 @param poolConfig Runtime pool configuration json. Optional. If NULL, then default config will be used. See example above.
 @param completion Callback returns handle to opened pool to use in methods that require pool connection.
 */
+ (void)openPoolLedgerWithName:(NSString *)name
                    poolConfig:(NSString *)poolConfig
                    completion:(void (^)(NSError *error, IndyHandle poolHandle))completion;

/**
 Refreshes a local copy of a pool ledger and updates pool nodes connections.
 
 @param poolHandle Pool handle returned by IndyPool::openPoolLedgerWithName
 @param completion Callback, returns error code.
 */
+ (void)refreshPoolLedgerWithHandle:(IndyHandle)poolHandle
                         completion:(void (^)(NSError *error))completion;

/**
 Closes opened pool ledger, opened nodes connections and frees allocated resources.
 
 @param poolHandle Pool handle returned by IndyPool::openPoolLedgerWithName.
 @param completion Completion callback, returns error code.
 */
+ (void)closePoolLedgerWithHandle:(IndyHandle)poolHandle
                       completion:(void (^)(NSError *error))completion;

/**
 Deletes created pool ledger configuration.

 @param name Name of the pool ledger configuration to delete.
 @param completion Completion callback, returns error code.
 */
+ (void)deletePoolLedgerConfigWithName:(NSString *)name
                            completion:(void (^)(NSError *error))completion;

/**
 Set PROTOCOL_VERSION to specific version.

 There is a global property PROTOCOL_VERSION that used in every request to the pool and
 specified version of Indy Node which Libindy works.

 By default PROTOCOL_VERSION=1.
 
 @param protocolVersion Protocol version will be used:
    1 - for Indy Node 1.3
    2 - for Indy Node 1.4
 @param completion Completion callback, returns error code.
 */
+ (void)setProtocolVersion:(NSNumber *)protocolVersion
                completion:(void (^)(NSError *error))completion;

@end

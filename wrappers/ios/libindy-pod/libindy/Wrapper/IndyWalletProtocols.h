//
//  IndyWalletProtocol.h
//  libindy
//

// MARK: - Indy Wallet type protocol

@protocol IndyWalletProtocol

@required
+ (id<IndyWalletProtocol>)sharedInstance;

@required
+ (NSString *)walletTypeName;

@required
- (void)cleanup;

@required
- (NSError *)createWithName:(NSString *)name
                     config:(NSString *)config
                credentials:(NSString *)credentials;

@required
- (NSError *)openWithName:(NSString *)name
                   config:(NSString *)config
            runtimeConfig:(NSString *)runtimeConfig
              credentials:(NSString *)credentials
                   handle:(IndyHandle *)handle;

@required
- (NSError *)deleteWalletWithName:(NSString *)name
                           config:(NSString *)config
                      credentials:(NSString *)credentials;

@required
- (NSError *)setValue:(NSString *)value
               forKey:(NSString *)key
           withHandle:(IndyHandle)handle;

@required
- (NSError *)getValue:(NSString **)value
               forKey:(NSString *)key
           withHandle:(IndyHandle)handle;

@required
- (NSError *)getNotExpired:(IndyHandle)walletHandle
                       key:(NSString *)key
                     value:(NSString**)value;

@required
- (NSError *)list:(IndyHandle)handle
              key:(NSString *)key
       valuesJson:(NSString**)valuesJson;

@required
- (NSError *)close:(IndyHandle)handle;

@required
- (NSError *)free:(IndyHandle)handle
              str:(NSString *)str;

@end


typedef indy_error_t (*createCb)(const char*, const char*, const char*);
typedef indy_error_t (*openCb)(const char* name, const char* config, const char* runtime_config, const char* credentials, indy_handle_t* handle);
typedef indy_error_t (*setCb)(indy_handle_t handle, const char* key, const char* value);
typedef indy_error_t (*getCb)(indy_handle_t handle, const char* key, const char *const *value_ptr);
typedef indy_error_t (*getNotExpiredCb)(indy_handle_t handle, const char* key, const char *const *value_ptr);
typedef indy_error_t (*listCb)(indy_handle_t handle, const char* key, const char *const *values_json_ptr);
typedef indy_error_t (*closeCb)(indy_handle_t handle);
typedef indy_error_t (*deleteCb)(const char* name, const char* config, const char* credentials);
typedef indy_error_t (*freeCb)(indy_handle_t handle, const char* str);



/**
 Methods returns c-function pointers of specified type
 */
@protocol IndyWalletCallbacks

@required
- (createCb)createCallback;

@required
- (openCb)openCallback;

@required
- (setCb)setCallback;

@required
- (getCb)getCallback;

@required
- (getNotExpiredCb)getNotExpiredCallback;

@required
- (listCb)listCallback;

@required
- (closeCb)closeCallback;

@required
- (deleteCb)deleteCallback;

@required
- (freeCb)freeCallback;

@end


//
//  IndyWalletImplementation.h
//  libindy
//
//  Created by Anastasia Tarasova on 04/09/2017.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

@protocol IndyWalletImplementation <NSObject>

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
                     value:(NSString *)value;

@required
- (NSError *)list:(IndyHandle)handle
              key:(NSString *)key
       valuesJson:(NSString *)valuesJson;

@required
- (NSError *)close:(IndyHandle)handle;

@required
- (NSError *)deleteWalletWithName:(NSString *)name
                           config:(NSString *)config
                      credentials:(NSString *)credentials;

@required
- (NSError *)free:(IndyHandle)handle
              str:(NSString *)str;

@end

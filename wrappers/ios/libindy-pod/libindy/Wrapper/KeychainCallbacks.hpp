//
//  KeychainCallbacks.hpp
//  libindy
//
//  Created by Anastasia Tarasova on 12/09/2017.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//


#import "IndyKeychainWallet.h"
#ifndef KeychainCallbacks_h
#define KeychainCallbacks_h

class KeychainCallbacks {
    
    
public:
    static indy_error_t createFn(const char* name,
                                 const char* config,
                                 const char* credentials);
};

#endif /* KeychainCallbacks_h */

indy_error_t KeychainCallbacks::createFn(const char* name, const char* config, const char* credentials)
{
    NSString *walletName = (name != NULL) ? [NSString stringWithUTF8String: name] : nil;
    NSString *walletConfig = (config != NULL) ? [NSString stringWithUTF8String:config] : nil;
    NSString *walletCredentials = (credentials != NULL) ? [NSString stringWithUTF8String:credentials] : nil;
    
    [[IndyKeychainWallet sharedInstance] createWithName:walletName config:walletConfig credentials:walletCredentials];
    return Success;
};

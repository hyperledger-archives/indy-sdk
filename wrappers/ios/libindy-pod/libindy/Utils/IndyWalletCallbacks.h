//
//  IndyWalletCallbacks.h
//  libindy
//
//  Created by Anastasia Tarasova on 01/09/2017.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <Foundation/Foundation.h>
#import "indy_core.h"

@interface IndyWalletCallbacks : NSObject
    
- (indy_handle_t) createCommandHandleFor:(void*) cb;
    
- (indy_handle_t) createCommandHandleFor:(void *)callback
                     withMessageCallback:(void *)messageCallback;
    
- (indy_handle_t) createCommandHandleFor:(void *)callback
                    withConnectionHandle:(indy_handle_t)connectionHandle;
    
- (indy_handle_t)createCommandHandleForListenerCallback:(void *)listenerCallback
                                 withConnectionCallback:(void *)connectionCallback
                                     andMessageCallback:(void *)messageCallback;
    
- (void)            deleteCommandHandleFor:(indy_handle_t) handle;
- (void)            forgetListenHandle:(indy_handle_t) listenHandle;
    
+ (IndyWalletCallbacks*) sharedInstance;
    
    @end

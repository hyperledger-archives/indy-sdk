//
//  KeychainCallbacks.hpp
//  libindy
//
//  Created by Anastasia Tarasova on 12/09/2017.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#ifndef KeychainCallbacks_h
#define KeychainCallbacks_h

class KeychainCallbacks {
    
    
public:
    static indy_error_t createFn(const char* name,
                                 const char* config,
                                 const char* credentials);
};

#endif /* KeychainCallbacks_h */

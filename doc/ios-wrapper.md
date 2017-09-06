# Installation

Wrapper is a private pod, so private podspec must be set. Put at the top of the Podfile:

```
source 'https://github.com/hyperledger/indy-sdk.git'
```
Cocoapos will search for spec files in the root Specs folder.

Add pod to target:

```
pod 'indy-objc'
```

# Usage

Import header:

```
#import <libindy/libindy.h>
```

All wrapper types and classes have prefix `Indy`.


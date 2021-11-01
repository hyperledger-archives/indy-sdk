Pod::Spec.new do |spec|
  spec.name     = 'Indy'
  spec.version  = '1.15.0'
  spec.license  =  'Apache License 2.0'
  spec.homepage = 'https://github.com/hyperledger/indy-sdk'
  spec.authors  = {'Hyperledger Indy Contributors' => 'hyperledger-indy@lists.hyperledger.org'}
  spec.summary  = 'libindy objective-C wrapper'
  spec.source   = {:git => 'https://github.com/hyperledger/indy-sdk.git', :tag => 'v1.15.0'}
  spec.source_files = 'wrappers/ios/libindy-pod/Indy/**/*'
  spec.exclude_files = [
    'wrappers/ios/libindy-pod/Indy/Info.plist',
    'wrappers/ios/libindy-pod/Indy/Wrapper/BlobStorage.*'
  ]
  spec.public_header_files = [
    'wrappers/ios/libindy-pod/Indy/Indy.h',
    'wrappers/ios/libindy-pod/Indy/Wrapper/*.h',
    'wrappers/ios/libindy-pod/Indy/Utils/IndyLogger.h'
  ]
  spec.libraries = 'sqlite3'
  spec.platform = :ios, '10.0'
  spec.static_framework = true
  spec.dependency   'libsodium'
  spec.dependency   'libzmq', '4.2.3'
  spec.dependency   'OpenSSL-XM'
  spec.dependency   'libindy', '1.15.0'
end

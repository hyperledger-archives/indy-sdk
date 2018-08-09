# VCX Pod for iOS
how to get a archive of vcx.framework

1) open /Users/norm/forge/work/code/evernym/sdk-evernym/vcx/wrappers/ios/vcx/vcx.xcodeproj in xcode
2) Select vcx as the target in Xcode
3) Select generic iOS device as Build only device
4) Select the menu Product -> archive
3) If every thing compiled successfully then folder with `vcx.framework` will be opened 
4) Copy libvcx.a file to vcx.framework/lib/
5) Copy ConnectMeVcx.h,libvcx.h,vcx.h files to vcx.framework/headers
6) Copy vcx.framework into a folder with name vcx

  $ mkdir -p /Users/username/path to sdk/vcx/wrappers/ios/vcx/tmp/vcx/
  $ cp -rp vcx.framewrok /Users/username/path to sdk/vcx/wrappers/ios/vcx/tmp/vcx/
  $ cd /Users/username/path to sdk/vcx/wrappers/ios/vcx/tmp/
  $ zip -r vcx.framework_[version]_[arch].zip vcx (i.e. zip -r vcx.framework_20180522.1635_universal.zip vcx)

7) upload vcx.framework_[version]_[arch].zip to repo to get a url.
$ curl --insecure -u normjarvis -X POST -F file=@./vcx.framework_20180522.1635_universal.zip https://kraken.corp.evernym.com/repo/ios/upload
8) Download the file at https://repo.corp.evernym.com/filely/ios/vcx.framework_20180522.1635_universal.zip

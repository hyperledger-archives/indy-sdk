#
# Be sure to run `pod lib lint vcx.podspec' to ensure this is a
# valid spec before submitting.
#
# Any lines starting with a # are optional, but their use is encouraged
# To learn more about a Podspec see https://guides.cocoapods.org/syntax/podspec.html
#

Pod::Spec.new do |s|
  s.name             = 'vcx'
  s.version          = '0.0.8'
  s.summary          = 'The Objective-C wrapper around the libvcx shared library.'

# This description is used to generate tags and improve search results.
#   * Think: What does it do? Why did you write it? What is the focus?
#   * Try to keep it short, snappy and to the point.
#   * Write the description between the DESC delimiters below.
#   * Finally, don't worry about the indent, CocoaPods strips it!

  s.description      = <<-DESC
The ConnectMe mobile app on the iOS platform will call into the libvcx shared library
from Objective-C. This pod is a very thin Objective-C wrapper that allows react native to call
through to the libvcx shared library.
                       DESC

  s.homepage         = 'https://www.evernym.com/'
  # s.screenshots     = 'www.example.com/screenshots_1', 'www.example.com/screenshots_2'
  s.license          = { :type => 'MIT', :file => 'LICENSE' }
  s.author           = { 'yaswanthsvist' => 'iosdev@evernym.com' }
  #s.source           = { :git => 'git@github.com:evernym/sdk.git', :tag => s.version.to_s }
  s.source           = { :http => 'https://repo.corp.evernym.com/filely/ios/vcx.framework_20180523.1433_universal.zip' }
  # s.social_media_url = 'https://twitter.com/<TWITTER_USERNAME>'

  s.ios.deployment_target = '8.0'
  s.ios.vendored_frameworks="vcx/vcx.framework"
  s.compiler_flags = '-ObjC -lvcx'
  
  # s.resource_bundles = {
  #   'vcx' => ['**/vcx/Assets/*.png']
  # }

  s.public_header_files = 'vcx/vcx.framework/include/*.h', 'vcx/vcx.framework/headers/*.h'
  s.ios.vendored_library = 'vcx/vcx.framework/lib/libvcx.a'
  # s.frameworks = 'UIKit', 'MapKit'
  # s.dependency 'AFNetworking', '~> 2.3'
end

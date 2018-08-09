//
//  AppDelegate.h
//  vcx-demo
//
//  Created by yaswanthsvist on 4/30/18.
//  Copyright © 2018 GuestUser. All rights reserved.
//

#import <UIKit/UIKit.h>
#import "vcx.h"

extern void VcxWrapperCommonStringCallback(vcx_command_handle_t xcommand_handle,
                                            vcx_error_t err,
                                            const char *const arg1);

@interface AppDelegate : UIResponder <UIApplicationDelegate>

@property (strong, nonatomic) UIWindow *window;


@end


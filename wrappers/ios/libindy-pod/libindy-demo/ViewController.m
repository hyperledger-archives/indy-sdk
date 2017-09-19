//
//  ViewController.m
//  libindy-demo
//
//  Created by Kirill Neznamov on 10/05/2017.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import "ViewController.h"
#import <libindy/libindy.h>


@implementation ViewController

- (void)viewDidLoad {
    [super viewDidLoad];
    // Do any additional setup after loading the view, typically from a nib.
    
    __block NSError *err;
    
    NSError *ret;
    ret = [[IndyWallet sharedInstance] createWalletWithPoolName:  @"pool1"
                                                           name:  @"wallet1"
                                                          xType:  @"default"
                                                         config:  nil
                                                    credentials:  nil
                                                     completion:  ^(NSError* error)
           {
               NSLog(@"inside completion");
               err = error;
           }];
}


- (void)didReceiveMemoryWarning {
    [super didReceiveMemoryWarning];
    // Dispose of any resources that can be recreated.
}


@end

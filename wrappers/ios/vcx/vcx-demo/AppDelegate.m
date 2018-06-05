//
//  AppDelegate.m
//  vcx-demo
//
//  Created by yaswanthsvist on 4/30/18.
//  Copyright © 2018 GuestUser. All rights reserved.
//

#import "AppDelegate.h"
#import "ConnectMeVcx.h"
/*
 typedef unsigned int vcx_error_t;
 typedef unsigned int vcx_schema_handle_t;
 typedef unsigned int vcx_credentialdef_handle_t;
 typedef unsigned int vcx_connection_handle_t;
 typedef unsigned int vcx_credential_handle_t;
 typedef unsigned int vcx_proof_handle_t;
 typedef unsigned int vcx_command_handle_t;
 typedef unsigned int vcx_bool_t;
 
 vcx_error_t vcx_init(vcx_command_handle_t handle, const char *config_path,void (*cb)(vcx_command_handle_t command_handle, vcx_error_t err));

 */


@interface AppDelegate ()

@end

//void VcxWrapperProvisionCallback(vcx_command_handle_t command_handle, vcx_error_t err, const char *config) {
//    NSData *dataData = [NSData dataWithBytes:config length:sizeof(config)];
//    [Console log:dataData];
//}



@implementation AppDelegate


- (BOOL)application:(UIApplication *)application didFinishLaunchingWithOptions:(NSDictionary *)launchOptions {
    // Override point for customization after application launch.
    return YES;
}


- (void)applicationWillResignActive:(UIApplication *)application {
    // Sent when the application is about to move from active to inactive state. This can occur for certain types of temporary interruptions (such as an incoming phone call or SMS message) or when the user quits the application and it begins the transition to the background state.
    // Use this method to pause ongoing tasks, disable timers, and invalidate graphics rendering callbacks. Games should use this method to pause the game.
}


- (void)applicationDidEnterBackground:(UIApplication *)application {
    // Use this method to release shared resources, save user data, invalidate timers, and store enough application state information to restore your application to its current state in case it is terminated later.
    // If your application supports background execution, this method is called instead of applicationWillTerminate: when the user quits.
}


- (void)applicationWillEnterForeground:(UIApplication *)application {
    // Called as part of the transition from the background to the active state; here you can undo many of the changes made on entering the background.
}


- (void)applicationDidBecomeActive:(UIApplication *)application {
    // Restart any tasks that were paused (or not yet started) while the application was inactive. If the application was previously in the background, optionally refresh the user interface.
    
    NSString  *config=@"{\"agency_url\": \"https://cagency.pdev.evernym.com\", \"agency_did\": \"dTLdJqRZLwMuWSogcKfBT\",\"wallet_name\":\"wallet1\",\"wallet_key\":\"wallet-key\",\"agent_seed\":null,\"enterprise_seed\":null, \"agency_verkey\": \"LsPQTDHi294TexkFmZK9Q9vW4YGtQRuLV8wuyZi94yH\"}";
    [[[ConnectMeVcx alloc] init] agentProvisionAsync:config completion:^(NSError *error, NSString *config) {
        NSLog(@"applicationDidBecomeActive callback:%@",config);
        if(error){
            
        }else{
            NSLog(@"%@",config);
        }
    }];
    
    //vcx_error_t vcx_init(vcx_command_handle_t handle, const char *config_path,void (*cb)(vcx_command_handle_t command_handle, vcx_error_t err));
    //err = vcx_init(handle,"{\"data\":{\"alias\":\"Node1\",\"blskey\":\"4N8aUNHSgjQVgkpm8nhNEfDf6txHznoYREg9kirmJrkivgL4oSEimFF6nsQ6M41QvhM2Z33nves5vfSn9n1UwNFJBYtWVnHYMATn76vLuL3zU88KyeAYcHfsih3He6UHcXDxcaecHVz6jhCYz1P2UZn2bDVruL5wXpehgBfBaLKm3Ba\",\"client_ip\":\"34.212.206.9\",\"client_port\":9702,\"node_ip\":\"34.212.206.9\",\"node_port\":9701,\"services\":[\"VALIDATOR\"]},\"dest\":\"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv\",\"identifier\":\"Th7MpTaRZVRYnPiabds81Y\",\"txnId\":\"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62\",\"type\":\"0\"}\n{\"data\":{\"alias\":\"Node2\",\"blskey\":\"37rAPpXVoxzKhz7d9gkUe52XuXryuLXoM6P6LbWDB7LSbG62Lsb33sfG7zqS8TK1MXwuCHj1FKNzVpsnafmqLG1vXN88rt38mNFs9TENzm4QHdBzsvCuoBnPH7rpYYDo9DZNJePaDvRvqJKByCabubJz3XXKbEeshzpz4Ma5QYpJqjk\",\"client_ip\":\"34.212.206.9\",\"client_port\":9704,\"node_ip\":\"34.212.206.9\",\"node_port\":9703,\"services\":[\"VALIDATOR\"]},\"dest\":\"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb\",\"identifier\":\"EbP4aYNeTHL6q385GuVpRV\",\"txnId\":\"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc\",\"type\":\"0\"}\n{\"data\":{\"alias\":\"Node3\",\"blskey\":\"3WFpdbg7C5cnLYZwFZevJqhubkFALBfCBBok15GdrKMUhUjGsk3jV6QKj6MZgEubF7oqCafxNdkm7eswgA4sdKTRc82tLGzZBd6vNqU8dupzup6uYUf32KTHTPQbuUM8Yk4QFXjEf2Usu2TJcNkdgpyeUSX42u5LqdDDpNSWUK5deC5\",\"client_ip\":\"34.212.206.9\",\"client_port\":9706,\"node_ip\":\"34.212.206.9\",\"node_port\":9705,\"services\":[\"VALIDATOR\"]},\"dest\":\"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya\",\"identifier\":\"4cU41vWW82ArfxJxHkzXPG\",\"txnId\":\"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4\",\"type\":\"0\"}\n{\"data\":{\"alias\":\"Node4\",\"blskey\":\"2zN3bHM1m4rLz54MJHYSwvqzPchYp8jkHswveCLAEJVcX6Mm1wHQD1SkPYMzUDTZvWvhuE6VNAkK3KxVeEmsanSmvjVkReDeBEMxeDaayjcZjFGPydyey1qxBHmTvAnBKoPydvuTAqx5f7YNNRAdeLmUi99gERUU7TD8KfAa6MpQ9bw\",\"client_ip\":\"34.212.206.9\",\"client_port\":9708,\"node_ip\":\"34.212.206.9\",\"node_port\":9707,\"services\":[\"VALIDATOR\"]},\"dest\":\"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA\",\"identifier\":\"TWwCRQRZ2ZHMJFn9TzLp7W\",\"txnId\":\"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008\",\"type\":\"0\"}", VcxWrapperCommonCallback);
    
}


- (void)applicationWillTerminate:(UIApplication *)application {
    // Called when the application is about to terminate. Save data if appropriate. See also applicationDidEnterBackground:.
}


@end



//
//  SovrinAgent.h
//  libsovrin
//


#import <Foundation/Foundation.h>
#import "SovrinTypes.h"

@interface SovrinAgent: NSObject

+ (NSError*) agentConnect: (SovrinHandle) walletHandle
                 senderId: (NSString *) senderId
               receiverId: (NSString *) receiverId
               completion: (void (^)(NSError* error,
                                     SovrinHandle connectionHandle)) handler;
//
//+ (NSError*) agentListen: (SovrinHandle) walletHandle
//      listenerCompletion: (void (^)(NSError* error,
//                                    SovrinHandle listenerHandle)) listenerHandler
//
//    connectionCompletion: (void (^)(SovrinHandle xlistenerHandle,
//                                    NSError* error,
//                                    SovrinHandle connectionHandle)) connectionHandler
//
//       messageCompletion: (void (^)(SovrinHandle xconnectionHandle,
//                                     NSError* error,
//                                     NSString* message)) messageHandler;

+ (NSError*) agentSend: (SovrinHandle) connectionHandle
              messsage: (NSString*) message
            completion: (void (^)(NSError* error)) handler;

+ (NSError*) agentCloseConnection: (SovrinHandle) connectionHandle
                       completion: (void (^)(NSError* error)) handler;

+ (NSError*) agentCloseListener: (SovrinHandle) listenerHandle
                     completion: (void (^)(NSError* error)) handler;
@end

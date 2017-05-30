//
//  SovrinAgent.h
//  libsovrin
//


#import <Foundation/Foundation.h>
#import "SovrinTypes.h"

@interface SovrinAgent: NSObject

+ (NSError*) agentConnect: (SovrinHandle) commandHandle
             walletHandle: (SovrinHandle) walletHandle
                 senderId: (NSString *) senderId
               receiverId: (NSString *) receiverId
               completion: (void (^)(SovrinHandle xcommandHandle,
                                     NSError* error,
                                     SovrinHandle connectionHandle)) handler;

+ (NSError*) agentListen: (SovrinHandle) commandHandle
            walletHandle: (SovrinHandle) walletHandle
      listenerCompletion: (void (^)(SovrinHandle xcommandHandle,
                                    NSError* error,
                                    SovrinHandle listenerHandler)) listenerHandler
       connectionCompletion: (void (^)(SovrinHandle xlistenerHandle,
                                    NSError* error,
                                    SovrinHandle connectionHandle)) connectionHandler
         messageCompletion: (void (^)(SovrinHandle xconnectionHandle,
                                    NSError* error,
                                    NSString* message)) messageHandler;

+ (NSError*) agentSend: (SovrinHandle) commandHandle
      connectionHandle: (SovrinHandle) connectionHandle
              messsage: (NSString*) message
            completion: (void (^)(SovrinHandle xcommandHandle,
                                  NSError* error)) handler;

+ (NSError*) agentCloseConnection: (SovrinHandle) commandHandle
                 connectionHandle: (SovrinHandle) connectionHandle
                       completion: (void (^)(SovrinHandle xcommandHandle,
                                             NSError* error)) handler;

+ (NSError*) agentCloseListener: (SovrinHandle) commandHandle
                 listenerHandle: (SovrinHandle) listenerHandle
                     completion: (void (^)(SovrinHandle xcommandHandle,
                                           NSError* error)) handler;
@end

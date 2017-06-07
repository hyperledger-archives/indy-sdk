//
//  SovrinAgent.h
//  libsovrin
//


#import <Foundation/Foundation.h>
#import "SovrinTypes.h"

@interface SovrinAgent: NSObject

+ (NSError*) agentConnect:(SovrinHandle) walletHandle
                 senderId:(NSString *) senderDid
               receiverId:(NSString *) receiverDid
        connectionHandler:(void (^)(NSError* error, SovrinHandle connection)) connectionHandler
           messageHandler:(void (^)(NSError* error, NSString* message)) messageHandler;

+ (NSError*) agentListen:(SovrinHandle) walletHandle
         listenerHandler:(void (^)(NSError* error,
                                   SovrinHandle listenerHandle)) listenerCompletion
       connectionHandler:(void (^)(SovrinHandle xlistenerHandle,
                                   NSError*     error,
                                   SovrinHandle connectionHandle,
                                   NSString*    senderDid,
                                   NSString*    receiverDid)) connectionCompletion
          messageHandler:(void (^)(SovrinHandle xconnectionHandle,
                                   NSError*     error,
                                   NSString*    message)) messageCompletion;

+ (NSError*) agentSend:(SovrinHandle) connectionHandle
              messsage:(NSString*) message
            completion:(void (^)(NSError* error)) handler;

+ (NSError*) agentCloseConnection:(SovrinHandle) connectionHandle
                       completion:(void (^)(NSError* error)) handler;

+ (NSError*) agentCloseListener:(SovrinHandle) listenerHandle
                     completion:(void (^)(NSError* error)) handler;
@end

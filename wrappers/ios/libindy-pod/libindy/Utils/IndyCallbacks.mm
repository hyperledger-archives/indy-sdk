//
//  IndyCallbacks.m
//  libindy
//

#include "indy_core.h"
#import "IndyCallbacks.h"
#import "NSError+IndyError.h"
#import "IndyTypes.h"

static NSString* commandCallbackKey    =  @"commandCallback";
static NSString* connectionCallbackKey =  @"connectionCallback";
static NSString* messageCallbackKey    =  @"messageCallback";
static NSString* connectionHandleKey   =  @"connectionHandle";
static NSString* connectionsKey        =  @"connections";


@interface IndyCallbacks ()

@property (strong, readwrite) NSMutableDictionary *commandCompletions;
@property                     indy_i32_t         commandHandleCounter;
@property (strong, readwrite) NSMutableDictionary *agentConnectCompletions;
@property (strong, readwrite) NSMutableDictionary *agentListenCompletions;
@property (strong, readwrite) NSMutableDictionary *listenerForConnection;      // used to determine listener handle for corresponded connection handle
@property (strong, readwrite) NSRecursiveLock     *globalLock;

@end

@implementation IndyCallbacks

- (IndyCallbacks *)init
{
    self = [super init];
    if (self)
    {
        self.commandHandleCounter = 0;
        self.commandCompletions = [[NSMutableDictionary alloc] init];
        self.agentConnectCompletions = [[NSMutableDictionary alloc] init];
        self.agentListenCompletions = [[NSMutableDictionary alloc] init];
        self.listenerForConnection = [[NSMutableDictionary alloc] init];
        self.globalLock = [NSRecursiveLock new];
    }
    return self;
}

- (void) addConnection:(indy_handle_t) connection  forListener:(indy_handle_t) listener
{
    NSNumber *nl = [NSNumber numberWithInt: listener];
    NSNumber *nc = [NSNumber numberWithInt: connection];
    
    @synchronized(self.globalLock)
    {
        NSMutableDictionary *dict = [self.agentConnectCompletions objectForKey: nl ];
        if(dict)
        {
            NSMutableDictionary *listenerParams = [dict objectForKey: connectionsKey];
            if(listenerParams)
            {
                [listenerParams setObject:nc forKey:nc];
            }
        }
    }
}


- (void) addListener:(indy_handle_t) listener forConnection:(indy_handle_t) connection
{
    NSNumber *nl = [NSNumber numberWithInt: listener];
    NSNumber *nc = [NSNumber numberWithInt: connection];

    @synchronized(self.globalLock)
    {
        [self.listenerForConnection setObject:nl forKey: nc];
    }
}

- (NSNumber*) listenerForConnection:(indy_handle_t) connection
{
    NSNumber *ret = nil;
    @synchronized(self.globalLock)
    {
        ret = [self.listenerForConnection objectForKey: [NSNumber numberWithInt:connection]];
    }
    return ret;
}

- (void) removeListenerForConnection:(indy_handle_t) connection
{
    NSNumber *nc = [NSNumber numberWithInt: connection];
    @synchronized(self.globalLock)
    {
        if( [self.listenerForConnection objectForKey: nc])
        {
            [self.listenerForConnection removeObjectForKey: nc];
        }
    }
}

- (void) rememberListenHandle:(indy_handle_t) listenHandle withDictionary:(NSMutableDictionary*) callbacks
{
    NSNumber *key = [NSNumber numberWithInt:listenHandle];
    
    @synchronized(self.globalLock)
    {
        [self.agentListenCompletions setObject:callbacks forKey:key];
    }
}

-(void) forgetListenHandle:(indy_handle_t) listenHandle
{
    NSNumber *key = [NSNumber numberWithInt:listenHandle];
    @synchronized(self.globalLock)
    {
        if ([self.agentListenCompletions objectForKey:key])
        {
            NSMutableDictionary *dict = [self.agentListenCompletions objectForKey:key];
            if(dict && [dict objectForKey:connectionsKey])
            {
                NSArray *connections = [[dict objectForKey:connectionsKey] allKeys];
                for(NSNumber *n in connections)
                {
                    [self removeListenerForConnection:[n intValue]];
                }
            }
            [self.agentListenCompletions removeObjectForKey:key];
        }
    }
}

- (NSMutableDictionary*)listenCompletionsFor:(indy_handle_t)handle
{
    NSNumber *key = [NSNumber numberWithInt:handle];
    NSMutableDictionary *val = nil;
    @synchronized(self.globalLock)
    {
        val = [self.agentListenCompletions objectForKey: key];
    }
    return val;
}

- (void) rememberConnectHandle:(indy_handle_t) connectionHandle withCallback:(void*) callback
{
    NSValue *val = [NSValue valueWithPointer:callback];
    NSNumber *key = [NSNumber numberWithInt:connectionHandle];

    @synchronized(self.globalLock)
    {
        [self.agentConnectCompletions setObject:val forKey:key];
    }
}

-(void) forgetConnectHandle:(indy_handle_t) connectionHandle
{
    NSNumber *key = [NSNumber numberWithInt:connectionHandle];
    @synchronized(self.globalLock)
    {
        if ([self.agentConnectCompletions objectForKey:key])
        {
            [self.agentConnectCompletions removeObjectForKey:key];
        }
    }
}

- (void *)connectCompletionFor:(indy_handle_t)handle
{
    NSNumber *key = [NSNumber numberWithInt:handle];
    NSValue *val = nil;
    @synchronized(self.globalLock)
    {
        val = [self.agentConnectCompletions objectForKey: key];
    }
    return val ? [val pointerValue] : NULL;
}


- (indy_handle_t)createCommandHandleFor:(void *)callback
{
    NSValue *cmdVal = [NSValue valueWithPointer:callback];
    NSNumber *handle = nil;

    @synchronized(self.globalLock)
    {
        handle = [NSNumber numberWithInt:self.commandHandleCounter];
        self.commandHandleCounter++;
        NSMutableDictionary *dict = [NSMutableDictionary dictionaryWithObjectsAndKeys: cmdVal, commandCallbackKey, nil];
        [self.commandCompletions setObject:dict forKey:handle];
    }
    return (indy_handle_t)[handle integerValue];
}

- (indy_handle_t)createCommandHandleFor:(void *)callback
                     withConnectionHandle:(indy_handle_t)connectionHandle
{
    NSValue  *cmdVal = [NSValue valueWithPointer:callback];
    NSNumber *conVal = [NSNumber numberWithInt:connectionHandle];

    NSNumber *handle = nil;
    
    @synchronized(self.globalLock)
    {
        handle = [NSNumber numberWithInt:self.commandHandleCounter];
        self.commandHandleCounter++;
        NSMutableDictionary *dict = [ NSMutableDictionary dictionaryWithObjectsAndKeys: cmdVal, commandCallbackKey,
                                                                                        conVal, connectionHandleKey, nil];
        [self.commandCompletions setObject:dict forKey:handle];
    }
    return (indy_handle_t)[handle integerValue];
}


/**
 Map passed callbacks to commandHandle.

 @param listenerCallback Callback that will be called after listening started or on error.
      Will be called exactly once with result of start listen operation.
 @param connectionCallback  Callback that will be called after establishing of incoming connection.
    Can be called multiply times: once for each incoming connection.
 @param messageCallback Callback that will be called on receiving of an incoming message. Can be called multiply times: once for each incoming message.
 @return commandHandle
 */
- (indy_handle_t)createCommandHandleForListenerCallback:(void *)listenerCallback
                                   withConnectionCallback:(void *)connectionCallback
                                       andMessageCallback:(void *)messageCallback
{
    NSValue *listenerCbVal = [NSValue valueWithPointer:listenerCallback];
    NSValue *connectionCbVal = [NSValue valueWithPointer:connectionCallback];
    NSValue *messageCbVal = [NSValue valueWithPointer:messageCallback];
    
    NSNumber *handle = nil;
    
    @synchronized(self.globalLock)
    {
        handle = [NSNumber numberWithInt:self.commandHandleCounter];
        self.commandHandleCounter++;
        NSMutableDictionary *dict = [ NSMutableDictionary dictionaryWithObjectsAndKeys: listenerCbVal, commandCallbackKey,
                                                                                        connectionCbVal, connectionCallbackKey,
                                                                                        messageCbVal, messageCallbackKey,    nil];
        
        [self.commandCompletions setObject:dict forKey:handle];
    }
    return (indy_handle_t)[handle integerValue];
}

- (indy_handle_t)createCommandHandleFor:(void *)callback
                       withMessageCallback:(void *)messageCallback
{
    NSValue *cmdVal = [NSValue valueWithPointer:callback];
    NSValue *mesVal = [NSValue valueWithPointer:messageCallback];
    
    NSNumber *handle = nil;
    
    @synchronized(self.globalLock)
    {
        handle = [NSNumber numberWithInt:self.commandHandleCounter];
        self.commandHandleCounter++;
        NSMutableDictionary *dict = [ NSMutableDictionary dictionaryWithObjectsAndKeys: cmdVal, commandCallbackKey,
                                                                                        mesVal, messageCallbackKey,    nil];
        [self.commandCompletions setObject:dict forKey:handle];
    }
    return (indy_handle_t)[handle integerValue];
}


- (void)deleteCommandHandleFor:(indy_handle_t)handle
{
    NSNumber *key = [NSNumber numberWithInt:handle];
    @synchronized(self.globalLock)
    {
        if ([self.commandCompletions objectForKey:key])
        {
            [self.commandCompletions removeObjectForKey:key];
        }
    }
}

- (void *)commandCompletionFor:(indy_handle_t)handle
{
    NSNumber *key = [NSNumber numberWithInt:handle];
    NSValue *val = nil;
    @synchronized(self.globalLock)
    {
        NSMutableDictionary *dict = (NSMutableDictionary*)[self.commandCompletions objectForKey:key];
        val = [dict objectForKey:@"commandCallback"];
    }
    return val ? [val pointerValue] : NULL;
}

- (NSMutableDictionary*) dictionaryFor:(indy_handle_t)handle
{
    NSNumber *key = [NSNumber numberWithInt:handle];
    NSMutableDictionary *dict = nil;
    @synchronized(self.globalLock)
    {
        dict = (NSMutableDictionary*)[self.commandCompletions objectForKey:key];
    }
    return dict;
}

+ (IndyCallbacks *)sharedInstance
{
    static IndyCallbacks *instance = nil;
    static dispatch_once_t dispatch_once_block;

    dispatch_once(&dispatch_once_block, ^ {
        instance = [IndyCallbacks new];
    });

    return instance;
}

@end


void IndyWrapperCommon2PCallback(indy_handle_t xcommand_handle,
                                   indy_error_t err)
{
    void * block = [[IndyCallbacks sharedInstance] commandCompletionFor: xcommand_handle];
    [[IndyCallbacks sharedInstance] deleteCommandHandleFor: xcommand_handle];
    
    void (^completion)(NSError*) = (__bridge void (^)(NSError*))block;
    
    if (completion)
    {
        dispatch_async(dispatch_get_main_queue(), ^
        {
            NSError *error = [ NSError errorFromIndyError: err ];
            completion(error);
        });
    }
}

void IndyWrapperCommon3PHCallback(indy_handle_t xcommand_handle,
                                    indy_error_t err,
                                    indy_handle_t pool_handle)
{
    void * block = [[IndyCallbacks sharedInstance] commandCompletionFor: xcommand_handle];
    [[IndyCallbacks sharedInstance] deleteCommandHandleFor: xcommand_handle];
    
    void (^completion)(NSError*, IndyHandle) = (__bridge void (^)(NSError*, IndyHandle))block;
    
    if (completion)
    {
        dispatch_async(dispatch_get_main_queue(), ^
                       {
                           NSError *error = [ NSError errorFromIndyError: err ];
                           completion(error, (IndyHandle) pool_handle);
                       });
    }
}

void IndyWrapperCommon3PSCallback(indy_handle_t xcommand_handle,
                                    indy_error_t err,
                                    const char* arg1)
{
    void * block = [[IndyCallbacks sharedInstance] commandCompletionFor: xcommand_handle];
    [[IndyCallbacks sharedInstance] deleteCommandHandleFor: xcommand_handle];
    
    void (^completion)(NSError*, NSString *) = (__bridge void (^)(NSError*, NSString *arg1 ))block;
    NSString* sarg1 = [ NSString stringWithUTF8String: arg1];
    
    if (completion)
    {
        dispatch_async(dispatch_get_main_queue(), ^
                       {
                           NSError *error = [ NSError errorFromIndyError: err ];
                           completion(error, sarg1);
                       });
    }
}

void IndyWrapperCommon3PBCallback(indy_handle_t xcommand_handle,
                                    indy_error_t err,
                                    indy_bool_t arg1)
{
    void * block = [[IndyCallbacks sharedInstance] commandCompletionFor: xcommand_handle];
    [[IndyCallbacks sharedInstance] deleteCommandHandleFor: xcommand_handle];
    
    void (^completion)(NSError*, BOOL ) = (__bridge void (^)(NSError*, BOOL arg1 ))block;
    
    if (completion)
    {
        dispatch_async(dispatch_get_main_queue(), ^
                       {
                           NSError *error = [ NSError errorFromIndyError: err ];
                           completion(error, (BOOL) arg1);
                       });
    }
}

void IndyWrapperCommon4PCallback(indy_handle_t xcommand_handle,
                                   indy_error_t err,
                                   const char* arg1,
                                   const char *arg2)
{
    void * block = [[IndyCallbacks sharedInstance] commandCompletionFor: xcommand_handle];
    [[IndyCallbacks sharedInstance] deleteCommandHandleFor: xcommand_handle];
    
    void (^completion)(NSError*, NSString* arg1, NSString *arg2) = (__bridge void (^)(NSError*, NSString* arg1, NSString *arg2))block;
    
    NSString* sarg1 = [ NSString stringWithUTF8String: arg1];
    NSString* sarg2 = [ NSString stringWithUTF8String: arg2];
    
    if (completion)
    {
        dispatch_async(dispatch_get_main_queue(), ^
                       {
                           NSError *error = [ NSError errorFromIndyError: err ];
                           completion(error, sarg1, sarg2);
                       });
    }
}

void IndyWrapperCommon5PCallback(indy_handle_t xcommand_handle,
                                   indy_error_t err,
                                   const char* arg1,
                                   const char *arg2,
                                   const char *arg3)
{
    void * block = [[IndyCallbacks sharedInstance] commandCompletionFor: xcommand_handle];
    [[IndyCallbacks sharedInstance] deleteCommandHandleFor: xcommand_handle];
    
    void (^completion)(NSError*, NSString* arg1, NSString *arg2, NSString *arg3) = (__bridge void (^)(NSError*, NSString* arg1, NSString *arg2, NSString *arg3))block;
    
    NSString* sarg1 = [ NSString stringWithUTF8String: arg1];
    NSString* sarg2 = [ NSString stringWithUTF8String: arg2];
    NSString* sarg3 = [ NSString stringWithUTF8String: arg3];
    
    if (completion)
    {
        dispatch_async(dispatch_get_main_queue(), ^
                       {
                           NSError *error = [ NSError errorFromIndyError: err ];
                           completion(error, sarg1, sarg2, sarg3);
                       });
    }
}

void IndyWrapperCommon5PSCallback(indy_handle_t xcommand_handle,
                                    indy_error_t err,
                                    indy_handle_t connection_handle,
                                    const char* arg1,
                                    const char *arg2)
{
    void * block = [[IndyCallbacks sharedInstance] commandCompletionFor: xcommand_handle];
    [[IndyCallbacks sharedInstance] deleteCommandHandleFor: xcommand_handle];
    
    void (^completion)(NSError*, IndyHandle, NSString* arg1, NSString *arg2) = (__bridge void (^)(NSError*, IndyHandle, NSString* arg1, NSString *arg2))block;
    
    NSString* sarg1 = [NSString stringWithUTF8String: arg1];
    NSString* sarg2 = [NSString stringWithUTF8String: arg2];
    
    if (completion)
    {
        dispatch_async(dispatch_get_main_queue(), ^
                       {
                           NSError *error = [ NSError errorFromIndyError: err ];
                           completion(error, (IndyHandle) connection_handle, sarg1, sarg2);
                       });
    }
}

void IndyWrapperCommonAgentOutgoingConnectionCallback(indy_handle_t xcommand_handle,
                                                        indy_error_t  err,
                                                        indy_handle_t connection_handle)
{
    NSMutableDictionary *dict = [[IndyCallbacks sharedInstance] dictionaryFor: xcommand_handle];
    if(dict && [dict objectForKey: commandCallbackKey])
    {
        void * commandBlock = [[dict objectForKey: commandCallbackKey] pointerValue];
        void * messageBlock = [[dict objectForKey: messageCallbackKey] pointerValue];
        [[IndyCallbacks sharedInstance] deleteCommandHandleFor: xcommand_handle];
        
        void (^completion)(NSError*, IndyHandle) = (__bridge void (^)(NSError*, IndyHandle))commandBlock;

        if(err == Success)
        {
            [[IndyCallbacks sharedInstance] rememberConnectHandle: connection_handle withCallback: messageBlock];
        }

        if (completion)
        {
            dispatch_async(dispatch_get_main_queue(), ^
            {
                               NSError *error = [NSError errorFromIndyError: err ];
                               completion(error, (IndyHandle) connection_handle);
            });
        }
    }
}

void IndyWrapperCommonAgentMessageCallback(indy_handle_t xconnection_handle,
                                             indy_error_t  err,
                                             const char *    message)
{
    NSString *messageArg = [NSString stringWithUTF8String: message];
    dispatch_async(dispatch_get_main_queue(), ^
    {
        void* block = [[IndyCallbacks sharedInstance] connectCompletionFor: xconnection_handle];
        if(block)
        {
            void (^completion)(IndyHandle, NSError*, NSString*) = (__bridge void (^)(IndyHandle, NSError*, NSString*))block;
            if(completion)
            {
                NSError *error = [NSError errorFromIndyError: err ];
                completion(xconnection_handle, error, messageArg);
            }
        }
    });
}

void IndyWrapperCloseConnectionCallback(indy_handle_t xcommand_handle,
                                          indy_error_t err)
{
    void * block = [[IndyCallbacks sharedInstance] commandCompletionFor: xcommand_handle];
    [[IndyCallbacks sharedInstance] deleteCommandHandleFor: xcommand_handle];
    void (^completion)(NSError*) = (__bridge void (^)(NSError*))block;

    NSMutableDictionary *dict = [[IndyCallbacks sharedInstance] dictionaryFor: xcommand_handle];

    if(dict && [dict objectForKey: connectionHandleKey])
    {
        NSNumber *connectionHandle = [dict objectForKey: connectionHandleKey];
        [[IndyCallbacks sharedInstance] forgetConnectHandle: [connectionHandle intValue]];
        [[IndyCallbacks sharedInstance] removeListenerForConnection: [connectionHandle intValue]];
    }
    
    if (completion)
    {
        dispatch_async(dispatch_get_main_queue(), ^
        {
            NSError *error = [ NSError errorFromIndyError: err ];
            completion(error);
        });
    }
}

void IndyWrapperCommonAgentListenerCallback(indy_handle_t xcommand_handle,
                                              indy_error_t  err,
                                              indy_handle_t listener_handle)
{
    void * block = [[IndyCallbacks sharedInstance] commandCompletionFor: xcommand_handle];
    // Dictionary of callbacks for this commandHandle
    NSMutableDictionary *dict = [[IndyCallbacks sharedInstance] dictionaryFor: xcommand_handle];
    
    // reset connections dictionary
    dict[connectionsKey] = [NSMutableDictionary new];
    // delete completions for this command handle
    [[IndyCallbacks sharedInstance] deleteCommandHandleFor: xcommand_handle];
    [[IndyCallbacks sharedInstance] rememberListenHandle:listener_handle withDictionary:dict];

    void (^completion)(NSError* error, IndyHandle) = (__bridge void (^)(NSError*,IndyHandle))block;

    if(completion)
    {
        dispatch_async(dispatch_get_main_queue(), ^
        {
            NSError *error = [ NSError errorFromIndyError: err ];
            completion(error, (IndyHandle) listener_handle);
        });
    }
}

void IndyWrapperCommonAgentListenerConnectionCallback(indy_handle_t xlistener_handle,
                                                        indy_error_t  err,
                                                        indy_handle_t connection_handle,
                                                        const char *    sender_did,
                                                        const char *    receiver_did)
{
    NSString* sarg1 = [NSString stringWithUTF8String: sender_did];
    NSString* sarg2 = [NSString stringWithUTF8String: receiver_did];

    dispatch_async(dispatch_get_main_queue(), ^
    {
        [[IndyCallbacks sharedInstance] addListener:xlistener_handle forConnection:connection_handle];
        [[IndyCallbacks sharedInstance] addConnection:connection_handle forListener:xlistener_handle];
        
        NSMutableDictionary *dict = [[IndyCallbacks sharedInstance] listenCompletionsFor: xlistener_handle];

        if(dict && [dict objectForKey: connectionCallbackKey])
        {
            NSValue *val = [dict objectForKey: connectionCallbackKey];
            if(val)
            {
                void* block = [val pointerValue];
                void (^completion)(IndyHandle,NSError*,IndyHandle,NSString*,NSString*) = (__bridge void (^)(IndyHandle,NSError*,IndyHandle,NSString*,NSString*))block;
                if(completion)
                {
                    NSError *error = [ NSError errorFromIndyError: err ];
  
                    completion((IndyHandle)xlistener_handle, error, (IndyHandle) connection_handle, sarg1, sarg2);
                }
            }
        }

    });
}

void IndyWrapperCommonAgentListenerMessageCallback(indy_handle_t xconnection_handle,
                                                     indy_error_t  err,
                                                     const char *    message)
{
    NSString* sarg = [NSString stringWithUTF8String: message];
    
    dispatch_async(dispatch_get_main_queue(), ^
    {
        NSNumber* listenerHandle = [[IndyCallbacks sharedInstance] listenerForConnection: xconnection_handle];
        NSMutableDictionary *dict = [[IndyCallbacks sharedInstance] listenCompletionsFor: [listenerHandle intValue]];

        if(listenerHandle && dict && [dict objectForKey: messageCallbackKey] )
        {
            NSValue *val = [dict objectForKey: messageCallbackKey];
            
            if(val)
            {
                void* block = [val pointerValue];
                void (^completion)(IndyHandle,NSError*,NSString*) = (__bridge void (^)(IndyHandle,NSError*,NSString*))block;
                if(completion)
                {
                    NSError *error = [ NSError errorFromIndyError: err ];
                    completion((IndyHandle)xconnection_handle, error, sarg);
                }
                
            }
        }
    });
}


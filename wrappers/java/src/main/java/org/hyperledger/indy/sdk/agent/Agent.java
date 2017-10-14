package org.hyperledger.indy.sdk.agent;

import java.util.Map;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ConcurrentHashMap;

import org.hyperledger.indy.sdk.IndyException;
import org.hyperledger.indy.sdk.IndyJava;
import org.hyperledger.indy.sdk.LibIndy;
import org.hyperledger.indy.sdk.ParamGuard;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.wallet.Wallet;

import com.sun.jna.Callback;

/**
 * agent.rs API
 */

/**
 * Agent related functionality.
 */
public class Agent extends IndyJava.API {

	private static Map<Integer, Agent.Connection> connections = new ConcurrentHashMap<Integer, Agent.Connection>();
	private static Map<Integer, Agent.Listener> listeners = new ConcurrentHashMap<Integer, Agent.Listener>();

	private Agent() {

	}

	/*
	 * OBSERVERS
	 */

	private static Map<Integer, AgentObservers.MessageObserver> messageObserver = new ConcurrentHashMap<Integer, AgentObservers.MessageObserver>();
	private static Map<Integer, AgentObservers.ConnectionObserver> connectionObservers = new ConcurrentHashMap<Integer, AgentObservers.ConnectionObserver>();

	/**
	 * Adds a message observer to track against the specified command handle.
	 * 
	 * @param commandHandle The command handle to track against.
	 * @param messageObserver	The message observer to track.
	 */
	private static void addMessageObserver(int commandHandle, AgentObservers.MessageObserver messageObserver) {

		assert(! Agent.messageObserver.containsKey(commandHandle));
		Agent.messageObserver.put(commandHandle, messageObserver);

	}

	/**
	 * Removes a message observer from tracking.
	 * 
	 * @param xcommand_handle The command handle the message observer is tracked against.
	 * @return The message observer.
	 */
	private static AgentObservers.MessageObserver removeMessageObserver(int xcommand_handle) {

		AgentObservers.MessageObserver future = messageObserver.remove(xcommand_handle);
		assert(future != null);

		return future;
	}

	/**
	 * Adds a connection observer to track against the specified command handle.
	 * 
	 * @param commandHandle The command handle to track against.
	 * @param connectionObserver The connection observer to track.
	 */
	private static void addConnectionObserver(int commandHandle, AgentObservers.ConnectionObserver connectionObserver) {

		assert(! connectionObservers.containsKey(commandHandle));
		connectionObservers.put(commandHandle, connectionObserver);

	}

	/**
	 * Removes a connection observer from tracking.
	 * 
	 * @param xcommand_handle The command handle the connection observer is tracked against.
	 * @return The connection observer.
	 */
	private static AgentObservers.ConnectionObserver removeConnectionObserver(int xcommand_handle) {

		AgentObservers.ConnectionObserver future = connectionObservers.remove(xcommand_handle);
		assert(future != null);

		return future;
	}

	/*
	 * STATIC CALLBACKS
	 */

	/**
	 * Callback used when an outgoing connection is established.
	 */
	private static Callback agentConnectConnectionCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err, int connection_handle) throws IndyException {

			CompletableFuture<Connection> future = (CompletableFuture<Connection>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			assert(! connections.containsKey(connection_handle));
			Agent.Connection connection = new Agent.Connection(connection_handle);
			connections.put(connection_handle, connection);

			connection.messageObserver = removeMessageObserver(xcommand_handle);

			future.complete(connection);
		}
	};

	/**
	 * Callback used when am outgoing connection receives a message.
	 */
	private static Callback agentConnectMessageCb = new Callback() {

		@SuppressWarnings("unused")
		public void callback(int xconnection_handle, int err, String message) throws IndyException {

			checkCallback(err);

			Agent.Connection connection = connections.get(xconnection_handle);
			if (connection == null) return;

			AgentObservers.MessageObserver messageObserver = connection.messageObserver;
			messageObserver.onMessage(connection, message);
		}
	};

	/**
	 * Callback used when a listener is ready to accept connections.
	 */
	private static Callback agentListenListenerCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err, int listener_handle) throws IndyException {

			CompletableFuture<Listener> future = (CompletableFuture<Listener>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			assert(! listeners.containsKey(listener_handle));
			Agent.Listener listener = new Agent.Listener(listener_handle);
			listeners.put(listener_handle, listener);

			listener.connectionObserver = removeConnectionObserver(xcommand_handle);

			future.complete(listener);
		}
	};

	/**
	 * Callback used when a listener receives an incoming connection.
	 */
	private static Callback agentListenConnectionCb = new Callback() {

		@SuppressWarnings("unused")
		public void callback(int xlistener_handle, int err, int connection_handle, String sender_did, String receiver_did) throws IndyException {

			checkCallback(err);

			Agent.Listener listener = listeners.get(xlistener_handle);
			if (listener == null) return;

			assert(! connections.containsKey(connection_handle));
			Agent.Connection connection = new Agent.Connection(connection_handle);
			connections.put(connection_handle, connection);

			AgentObservers.ConnectionObserver connectionObserver = listener.connectionObserver;
			connection.messageObserver = connectionObserver.onConnection(listener, connection, sender_did, receiver_did);
		}
	};

	/**
	 * Callback used when a listener connection receives a message.
	 */
	private static Callback agentListenMessageCb = new Callback() {

		@SuppressWarnings("unused")
		public void callback(int xconnection_handle, int err, String message) throws IndyException {

			checkCallback(err);

			Agent.Connection connection = connections.get(xconnection_handle);
			if (connection == null) return;

			AgentObservers.MessageObserver messageObserver = connection.messageObserver;
			messageObserver.onMessage(connection, message);
		}
	};

	/**
	 * Callback used when an identity is added to a listener.
	 */
	private static Callback agentAddIdentityCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, int listener_handle) {

			CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			future.complete(null);
		}
	};

	/**
	 * Callback used when an identity is removed from a listener.
	 */
	private static Callback agentRemoveIdentityCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, int listener_handle) {

			CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			future.complete(null);
		}
	};

	/**
	 * Callback used when a message is sent.
	 */
	private static Callback agentSendCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err) {

			CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			future.complete(null);
		}
	};

	/**
	 * Callback used when a connection is closed.
	 */
	private static Callback agentCloseConnectionCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err) {

			CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			future.complete(null);
		}
	};

	/**
	 * Callback used when a listener is closed.
	 */
	private static Callback agentCloseListenerCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err) {

			CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			future.complete(null);
		}
	};

	/*
	 * STATIC METHODS
	 */

	/**
	 * Establishes an outgoing connection.
	 * 
	 * @param pool The pool.
	 * @param wallet The wallet.
	 * @param senderDid The sender DID.
	 * @param receiverDid The receiver DID.
	 * @param messageObserver The message observer that will be notified when a message is received on the connection.
	 * @return A future that resolves to a Connection.
	 * @throws IndyException Thrown if a failure occurs when calling the SDK.
	 */
	public static CompletableFuture<Connection> agentConnect(
			Pool pool,
			Wallet wallet,
			String senderDid,
			String receiverDid,
			AgentObservers.MessageObserver messageObserver) throws IndyException {
		
		ParamGuard.notNull(pool, "pool");
		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(receiverDid, "senderDid");
		ParamGuard.notNullOrWhiteSpace(receiverDid, "senderDid");
		ParamGuard.notNull(messageObserver, "messageObserver");

		CompletableFuture<Connection> future = new CompletableFuture<>();
		int commandHandle = addFuture(future);
		addMessageObserver(commandHandle, messageObserver);

		int poolHandle = pool.getPoolHandle();
		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_agent_connect(
				commandHandle,
				poolHandle,
				walletHandle,
				senderDid,
				receiverDid,
				agentConnectConnectionCb,
				agentConnectMessageCb);

		checkResult(result);

		return future;
	}

	/**
	 * Starts a listener that listens for incoming connections.
	 * 
	 * @param endpoint The endpoint on which the listener should listen for connections.
	 * @param connectionObserver An observer that will be notified when new incoming connections are established.
	 * @return A future that resolves to a listener.
	 * @throws IndyException Thrown if an error occurs when calling the SDK.
	 */
	public static CompletableFuture<Listener> agentListen(
			String endpoint,
			AgentObservers.ConnectionObserver connectionObserver) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(endpoint, "endpoint");
		ParamGuard.notNull(connectionObserver, "connectionObserver");
		
		CompletableFuture<Listener> future = new CompletableFuture<>();
		int commandHandle = addFuture(future);
		addConnectionObserver(commandHandle, connectionObserver);

		int result = LibIndy.api.indy_agent_listen(
				commandHandle,
				endpoint,
				agentListenListenerCb,
				agentListenConnectionCb,
				agentListenMessageCb);

		checkResult(result);

		return future;
	}

	/**
	 * Adds an identity to the specified listener.
	 * 
	 * @param listener The listener to add the identity to.
	 * @param pool The pool.
	 * @param wallet The wallet.
	 * @param did The DID of the identity to add.
	 * @return A future that does not resolve a value.
	 * @throws IndyException Thrown if an error occurs when calling the SDK. 
	 */
	public static CompletableFuture<Void> agentAddIdentity(
			Agent.Listener listener,
			Pool pool,
			Wallet wallet,
			String did) throws IndyException {
		
		ParamGuard.notNull(listener, "listener");
		ParamGuard.notNull(pool, "pool");
		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(did, "did");		

		CompletableFuture<Void> future = new CompletableFuture<Void>();
		int commandHandle = addFuture(future);

		int listenerHandle = listener.getListenerHandle();
		int poolHandle = pool.getPoolHandle();
		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_agent_add_identity(
				commandHandle,
				listenerHandle,
				poolHandle,
				walletHandle,
				did,
				agentAddIdentityCb);

		checkResult(result);

		return future;
	}

	/**
	 * Removes an identity from the specified listener.
	 * 
	 * @param listener The listener to remove the identity from.
	 * @param wallet The wallet.
	 * @param did The DID of the identity to remove.
	 * @return A future that does not resolve a value.
	 * @throws IndyException Thrown if an error occurs when calling the SDK. 
	 */
	public static CompletableFuture<Void> agentRemoveIdentity(
			Agent.Listener listener,
			Wallet wallet,
			String did) throws IndyException {

		ParamGuard.notNull(listener, "listener");
		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(did, "did");		
		
		CompletableFuture<Void> future = new CompletableFuture<Void>();
		int commandHandle = addFuture(future);

		int listenerHandle = listener.getListenerHandle();
		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_agent_remove_identity(
				commandHandle,
				listenerHandle,
				walletHandle,
				did,
				agentRemoveIdentityCb);

		checkResult(result);

		return future;
	}

	/**
	 * Sends a message to the specified connection.
	 * 
	 * @param connection The connection to send the message to.
	 * @param message The message to send.
	 * @return A future that does not resolve a value.
	 * @throws IndyException Thrown if an error occurs when calling the SDK. 
	 */
	public static CompletableFuture<Void> agentSend(
			Agent.Connection connection,
			String message) throws IndyException {
		
		ParamGuard.notNull(connection, "connection");
		ParamGuard.notNull(message, "message");		

		CompletableFuture<Void> future = new CompletableFuture<Void>();
		int commandHandle = addFuture(future);

		int connectionHandle = connection.getConnectionHandle();

		int result = LibIndy.api.indy_agent_send(
				commandHandle,
				connectionHandle,
				message,
				agentSendCb);

		checkResult(result);

		return future;
	}

	/**
	 * Closes the provided connection.
	 * 
	 * @param connection The connection to close. 
	 * @return A future that does not resolve a value.
	 * @throws IndyException Thrown if an error occurs when calling the SDK. 
	 */
	public static CompletableFuture<Void> agentCloseConnection(
			Agent.Connection connection) throws IndyException {

		ParamGuard.notNull(connection, "connection");
		
		CompletableFuture<Void> future = new CompletableFuture<Void>();
		int commandHandle = addFuture(future);

		int connectionHandle = connection.getConnectionHandle();

		connections.remove(connectionHandle);

		int result = LibIndy.api.indy_agent_close_connection(
				commandHandle,
				connectionHandle,
				agentCloseConnectionCb);

		checkResult(result);

		return future;
	}

	/**
	 * Closes the specified listener.
	 * 
	 * @param listener The listener to close.
	 * @return A future that does not resolve a value.
	 * @throws IndyException Thrown if an error occurs when calling the SDK. 
	 */
	public static CompletableFuture<Void> agentCloseListener(
			Agent.Listener listener) throws IndyException {

		ParamGuard.notNull(listener, "listener");
		
		CompletableFuture<Void> future = new CompletableFuture<Void>();
		int commandHandle = addFuture(future);

		int listenerHandle = listener.getListenerHandle();

		listeners.remove(listenerHandle);

		int result = LibIndy.api.indy_agent_close_listener(
				commandHandle,
				listenerHandle,
				agentCloseListenerCb);

		checkResult(result);

		return future;
	}

	/*
	 * NESTED CLASSES WITH INSTANCE METHODS
	 */

	/**
	 * Listens for incoming connections.
	 */
	public static class Listener {

		private final int listenerHandle;
		private AgentObservers.ConnectionObserver connectionObserver;

		private Listener(int listenerHandle) {

			this.listenerHandle = listenerHandle;
		}

		/**
		 * Gets the handle of the listener.
		 * 
		 * @return The handle of the listener.
		 */
		public int getListenerHandle() {

			return this.listenerHandle;
		}

		/**
		 * Adds an identity to the listener.
		 * 
		 * @param pool The pool.
		 * @param wallet The wallet.
		 * @param did The DID of the identity to add.
		 * @return A future that does not resolve a value.
		 * @throws IndyException Thrown if an error occurs when calling the SDK. 
		 */
		public CompletableFuture<Void> agentAddIdentity(Pool pool, Wallet wallet, String did) throws IndyException {

			return Agent.agentAddIdentity(this, pool, wallet, did);
		}

		/**
		 * Removes an identity from the listener.
		 * 
		 * @param wallet The wallet.
		 * @param did The DID of the identity to remove.
		 * @return A future that does not resolve a value.
		 * @throws IndyException Thrown if an error occurs when calling the SDK. 
		 */
		public CompletableFuture<Void> agentRemoveIdentity(Wallet wallet, String did) throws IndyException {

			return Agent.agentRemoveIdentity(this, wallet, did);
		}

		/**
		 * Closes the listener.
		 * 
		 * @return A future that does not resolve a value.
		 * @throws IndyException Thrown if an error occurs when calling the SDK. 
		 */
		public CompletableFuture<Void> agentCloseListener() throws IndyException {

			return Agent.agentCloseListener(this);
		}
	}

	/**
	 * A connection between two agents.
	 */
	public static class Connection {

		private final int connectionHandle;
		private AgentObservers.MessageObserver messageObserver;

		private Connection(int connectionHandle) {

			this.connectionHandle = connectionHandle;
		}

		/**
		 * Gets the handle of the connection.
		 * 
		 * @return The handle of the connection.
		 */
		public int getConnectionHandle() {

			return this.connectionHandle;
		}

		/**
		 * Sends a message on the connection.
		 * 
		 * @param message The message to send.
		 * @return A future that does not resolve a value.
		 * @throws IndyException Thrown if an error occurs when calling the SDK. 
		 */
		public CompletableFuture<Void> agentSend(String message) throws IndyException {

			return Agent.agentSend(this, message);
		}

		/**
		 * Closes the connection.
		 * 
		 * @return A future that does not resolve a value.
		 * @throws IndyException Thrown if an error occurs when calling the SDK. 
		 */
		public CompletableFuture<Void> agentCloseConnection() throws IndyException {

			return Agent.agentCloseConnection(this);
		}
	}
}

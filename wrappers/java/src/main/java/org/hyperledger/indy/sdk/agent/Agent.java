package org.hyperledger.indy.sdk.agent;

import java.util.Map;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ConcurrentHashMap;

import org.hyperledger.indy.sdk.IndyException;
import org.hyperledger.indy.sdk.IndyJava;
import org.hyperledger.indy.sdk.LibIndy;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.wallet.Wallet;

import com.sun.jna.Callback;

/**
 * agent.rs API
 */
public class Agent extends IndyJava.API {

	private static Map<Integer, Agent.Connection> connections = new ConcurrentHashMap<Integer, Agent.Connection> ();
	private static Map<Integer, Agent.Listener> listeners = new ConcurrentHashMap<Integer, Agent.Listener> ();

	private Agent() {

	}

	/*
	 * OBSERVERS
	 */

	private static Map<Integer, AgentObservers.ListenerObserver> listenerObservers = new ConcurrentHashMap<Integer, AgentObservers.ListenerObserver> ();
	private static Map<Integer, AgentObservers.ConnectionObserver> connectionObservers = new ConcurrentHashMap<Integer, AgentObservers.ConnectionObserver> ();

	private static int addListenerObserver(AgentObservers.ListenerObserver listenerObserver) {

		int commandHandle = newCommandHandle();
		assert(! listenerObservers.containsKey(Integer.valueOf(commandHandle)));
		listenerObservers.put(Integer.valueOf(commandHandle), listenerObserver);

		return commandHandle;
	}

	private static AgentObservers.ListenerObserver removeListenerObserver(int xcommand_handle) {

		AgentObservers.ListenerObserver future = listenerObservers.remove(Integer.valueOf(xcommand_handle));
		assert(future != null);

		return future;
	}

	private static int addConnectionObserver(AgentObservers.ConnectionObserver connectionObserver) {

		int commandHandle = newCommandHandle();
		assert(! connectionObservers.containsKey(Integer.valueOf(commandHandle)));
		connectionObservers.put(Integer.valueOf(commandHandle), connectionObserver);

		return commandHandle;
	}

	private static AgentObservers.ConnectionObserver removeConnectionObserver(int xcommand_handle) {

		AgentObservers.ConnectionObserver future = connectionObservers.remove(Integer.valueOf(xcommand_handle));
		assert(future != null);

		return future;
	}

	/*
	 * STATIC CALLBACKS
	 */

	private static Callback agentConnectConnectionCb = new Callback() {

		@SuppressWarnings("unused")
		public void callback(int xcommand_handle, int err, int connection_handle) throws IndyException {

			checkCallback(err);

			assert(! connections.containsKey(Integer.valueOf(connection_handle)));
			Agent.Connection connection = new Agent.Connection(connection_handle);
			connections.put(Integer.valueOf(connection_handle), connection);

			AgentObservers.ConnectionObserver connectionObserver = removeConnectionObserver(xcommand_handle);
			AgentObservers.MessageObserver messageObserver = connectionObserver.onConnection(null, connection, null, null);
			connection.messageObserver = messageObserver;
		}
	};

	private static Callback agentConnectMessageCb = new Callback() {

		@SuppressWarnings("unused")
		public void callback(int xconnection_handle, int err, String message) throws IndyException {

			checkCallback(err);

			Agent.Connection connection = connections.get(Integer.valueOf(xconnection_handle));
			if (connection == null) return;

			AgentObservers.MessageObserver messageObserver = connection.messageObserver;
			messageObserver.onMessage(connection, message);
		}
	};

	private static Callback agentListenListenerCb = new Callback() {

		@SuppressWarnings("unused")
		public void callback(int xcommand_handle, int err, int listener_handle) throws IndyException {

			checkCallback(err);

			assert(! listeners.containsKey(Integer.valueOf(listener_handle)));
			Agent.Listener listener = new Agent.Listener(listener_handle);
			listeners.put(Integer.valueOf(listener_handle), listener);

			AgentObservers.ListenerObserver listenerObserver = removeListenerObserver(xcommand_handle);
			AgentObservers.ConnectionObserver connectionObserver = listenerObserver.onListener(listener);
			listener.connectionObserver = connectionObserver;
		}
	};

	private static Callback agentListenConnectionCb = new Callback() {

		@SuppressWarnings("unused")
		public void callback(int xlistener_handle, int err, int connection_handle, String sender_did, String receiver_did) throws IndyException {

			checkCallback(err);

			Agent.Listener listener = listeners.get(Integer.valueOf(xlistener_handle));
			if (listener == null) return;

			assert(! connections.containsKey(Integer.valueOf(connection_handle)));
			Agent.Connection connection = new Agent.Connection(connection_handle);
			connections.put(Integer.valueOf(connection_handle), connection);

			AgentObservers.ConnectionObserver connectionObserver = listener.connectionObserver;
			AgentObservers.MessageObserver messageObserver = connectionObserver.onConnection(listener, connection, sender_did, receiver_did);
			connection.messageObserver = messageObserver;
		}
	};

	private static Callback agentListenMessageCb = new Callback() {

		@SuppressWarnings("unused")
		public void callback(int xconnection_handle, int err, String message) throws IndyException {

			checkCallback(err);

			Agent.Connection connection = connections.get(Integer.valueOf(xconnection_handle));
			if (connection == null) return;

			AgentObservers.MessageObserver messageObserver = connection.messageObserver;
			messageObserver.onMessage(connection, message);
		}
	};

	private static Callback agentAddIdentityCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err, int listener_handle) {

			CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			Void result = null;
			future.complete(result);
		}
	};

	private static Callback agentRemoveIdentityCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err, int listener_handle) {

			CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			Void result = null;
			future.complete(result);
		}
	};

	private static Callback agentSendCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err) {

			CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			Void result = null;
			future.complete(result);
		}
	};

	private static Callback agentCloseConnectionCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err) {

			CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			Void result = null;
			future.complete(result);
		}
	};

	private static Callback agentCloseListenerCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err) {

			CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			Void result = null;
			future.complete(result);
		}
	};

	/*
	 * STATIC METHODS
	 */

	public static void agentConnect(
			Pool pool,
			Wallet wallet,
			String senderDid,
			String receiverDid,
			AgentObservers.ConnectionObserver connectionObserver) throws IndyException {

		int commandHandle = addConnectionObserver(connectionObserver);

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
	}

	public static void agentListen(
			String endpoint,
			AgentObservers.ListenerObserver listenerObserver) throws IndyException {

		int commandHandle = addListenerObserver(listenerObserver);

		int result = LibIndy.api.indy_agent_listen(
				commandHandle, 
				endpoint,
				agentListenListenerCb,
				agentListenConnectionCb,
				agentListenMessageCb);

		checkResult(result);
	}

	public static CompletableFuture<Void> agentAddIdentity(
			Agent.Listener listener,
			Pool pool,
			Wallet wallet,
			String did) throws IndyException {

		CompletableFuture<Void> future = new CompletableFuture<Void> ();
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

	public static CompletableFuture<Void> agentRemoveIdentity(
			Agent.Listener listener,
			Wallet wallet,
			String did) throws IndyException {

		CompletableFuture<Void> future = new CompletableFuture<Void> ();
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

	public static CompletableFuture<Void> agentSend(
			Agent.Connection connection,
			String message) throws IndyException {

		CompletableFuture<Void> future = new CompletableFuture<Void> ();
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

	public static CompletableFuture<Void> agentCloseConnection(
			Agent.Connection connection) throws IndyException {

		CompletableFuture<Void> future = new CompletableFuture<Void> ();
		int commandHandle = addFuture(future);

		int connectionHandle = connection.getConnectionHandle();

		connections.remove(Integer.valueOf(connectionHandle));

		int result = LibIndy.api.indy_agent_close_connection(
				commandHandle, 
				connectionHandle, 
				agentCloseConnectionCb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<Void> agentCloseListener(
			Agent.Listener listener) throws IndyException {

		CompletableFuture<Void> future = new CompletableFuture<Void> ();
		int commandHandle = addFuture(future);

		int listenerHandle = listener.getListenerHandle();

		listeners.remove(Integer.valueOf(listenerHandle));

		int result = LibIndy.api.indy_agent_close_connection(
				commandHandle, 
				listenerHandle, 
				agentCloseListenerCb);

		checkResult(result);

		return future;
	}

	/*
	 * NESTED CLASSES WITH INSTANCE METHODS
	 */

	public static class Listener {

		private final int listenerHandle;
		private AgentObservers.ConnectionObserver connectionObserver;

		private Listener(int listenerHandle) {

			this.listenerHandle = listenerHandle;
		}

		public int getListenerHandle() {

			return this.listenerHandle;
		}

		public CompletableFuture<Void> agentAddIdentity(Pool pool, Wallet wallet, String did) throws IndyException {

			return Agent.agentAddIdentity(this, pool, wallet, did);
		}

		public CompletableFuture<Void> agentRemoveIdentity(Wallet wallet, String did) throws IndyException {

			return Agent.agentRemoveIdentity(this, wallet, did);
		}

		public CompletableFuture<Void> agentCloseListener() throws IndyException {

			return Agent.agentCloseListener(this);
		}
	}

	public static class Connection {

		private final int connectionHandle;
		private AgentObservers.MessageObserver messageObserver;

		private Connection(int connectionHandle) {

			this.connectionHandle = connectionHandle;
		}

		public int getConnectionHandle() {

			return this.connectionHandle;
		}

		public CompletableFuture<Void> agentSend(String message) throws IndyException {

			return Agent.agentSend(this, message);
		}

		public CompletableFuture<Void> agentCloseConnection() throws IndyException {

			return Agent.agentCloseConnection(this);
		}
	}
}

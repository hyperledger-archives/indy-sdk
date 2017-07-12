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
	 * STATIC CALLBACKS
	 */

	private static Callback agentConnectCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err, int connection_handle) {

			CompletableFuture<Agent.Connection> future = (CompletableFuture<Agent.Connection>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			assert(! connections.containsKey(Integer.valueOf(connection_handle)));
			Agent.Connection connection = new Agent.Connection(connection_handle);
			connections.put(Integer.valueOf(connection_handle), connection);

			Agent.Connection result = connection;
			future.complete(result);
		}
	};

	private static Callback agentListenCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err, int listener_handle) {

			CompletableFuture<Agent.Listener> future = (CompletableFuture<Agent.Listener>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			assert(! listeners.containsKey(Integer.valueOf(listener_handle)));
			Agent.Listener listener = new Agent.Listener(listener_handle);
			listeners.put(Integer.valueOf(listener_handle), listener);

			Agent.Listener result = listener;
			future.complete(result);
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

	public static CompletableFuture<Agent.Connection> agentConnect(
			Pool pool,
			Wallet wallet,
			String senderDid,
			String receiverDid,
			final AgentObservers.AgentConnectObserver agentConnectObserver) throws IndyException {

		CompletableFuture<Agent.Connection> future = new CompletableFuture<Agent.Connection> ();
		int commandHandle = addFuture(future);

		Callback messageCb = new Callback() {

			@SuppressWarnings({ "unused", "unchecked" })
			public void callback(int xconnection_handle, int err, String message) throws IndyException {

				checkCallback(err);

				Agent.Connection connection = connections.get(Integer.valueOf(xconnection_handle));
				if (connection == null) return;

				agentConnectObserver.onMessage(connection, message);
			}
		};

		int poolHandle = pool.getPoolHandle();
		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_agent_connect(
				commandHandle, 
				poolHandle,
				walletHandle, 
				senderDid,
				receiverDid,
				agentConnectCb,
				messageCb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<Agent.Listener> agentListen(
			String endpoint,
			final AgentObservers.AgentListenObserver agentListenObserver) throws IndyException {

		CompletableFuture<Agent.Listener> future = new CompletableFuture<Agent.Listener> ();
		int commandHandle = addFuture(future);

		Callback connectionCb = new Callback() {

			@SuppressWarnings({ "unused", "unchecked" })
			public void callback(int xlistener_handle, int err, int connection_handle, String sender_did, String receiver_did) throws IndyException {

				checkCallback(err);

				Agent.Listener listener = listeners.get(Integer.valueOf(xlistener_handle));
				if (listener == null) return;

				assert(! connections.containsKey(Integer.valueOf(connection_handle)));
				Agent.Connection connection = new Agent.Connection(connection_handle);
				connections.put(Integer.valueOf(connection_handle), connection);

				agentListenObserver.onConnection(listener, connection, sender_did, receiver_did);
			}
		};

		Callback messageCb = new Callback() {

			@SuppressWarnings({ "unused", "unchecked" })
			public void callback(int xconnection_handle, int err, String message) throws IndyException {

				checkCallback(err);

				Agent.Connection connection = connections.get(Integer.valueOf(xconnection_handle));
				if (connection == null) return;

				agentListenObserver.onMessage(connection, message);
			}
		};

		int result = LibIndy.api.indy_agent_listen(
				commandHandle, 
				endpoint,
				agentListenCb,
				connectionCb,
				messageCb);

		checkResult(result);

		return future;
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

	public static class Connection {

		private final int connectionHandle;

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

	public static class Listener {

		private final int listenerHandle;

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
}

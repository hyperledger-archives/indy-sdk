package org.hyperledger.indy.sdk.agent;

import java.util.concurrent.CompletableFuture;

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

	private Agent() {

	}

	/*
	 * STATIC METHODS
	 */

	public static CompletableFuture<Agent.Connection> agentConnect(
			Pool pool,
			Wallet wallet,
			String senderDid,
			String receiverDid,
			Callback messageCb) throws IndyException {

		final CompletableFuture<Agent.Connection> future = new CompletableFuture<> ();

		Callback connectionCb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, int connection_handle) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				Agent.Connection result = new Agent.Connection(connection_handle);
				future.complete(result);
			}
		};

		int poolHandle = pool.getPoolHandle();
		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_agent_connect(
				FIXED_COMMAND_HANDLE, 
				poolHandle,
				walletHandle, 
				senderDid,
				receiverDid,
				connectionCb,
				messageCb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<Agent.Listener> agentListen(
			String endpoint,
			Callback connectionCb,
			Callback messageCb) throws IndyException {

		final CompletableFuture<Agent.Listener> future = new CompletableFuture<> ();

		Callback listenerCb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, int listener_handle) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				Agent.Listener result = new Agent.Listener(listener_handle);
				future.complete(result);
			}
		};

		int result = LibIndy.api.indy_agent_listen(
				FIXED_COMMAND_HANDLE, 
				endpoint,
				listenerCb,
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

		final CompletableFuture<Void> future = new CompletableFuture<> ();

		Callback addIdentityCb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, int listener_handle) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				Void result = null;
				future.complete(result);
			}
		};

		int listenerHandle = listener.getListenerHandle();
		int poolHandle = pool.getPoolHandle();
		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_agent_add_identity(
				FIXED_COMMAND_HANDLE, 
				listenerHandle,
				poolHandle,
				walletHandle, 
				did,
				addIdentityCb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<Void> agentRemoveIdentity(
			Agent.Listener listener,
			Wallet wallet,
			String did) throws IndyException {

		final CompletableFuture<Void> future = new CompletableFuture<> ();

		Callback rmIdentityCb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, int listener_handle) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				Void result = null;
				future.complete(result);
			}
		};

		int listenerHandle = listener.getListenerHandle();
		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_agent_remove_identity(
				FIXED_COMMAND_HANDLE, 
				listenerHandle,
				walletHandle, 
				did,
				rmIdentityCb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<Void> agentSend(
			Agent.Connection connection,
			String message) throws IndyException {

		final CompletableFuture<Void> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				Void result = null;
				future.complete(result);
			}
		};

		int connectionHandle = connection.getConnectionHandle();

		int result = LibIndy.api.indy_agent_send(
				FIXED_COMMAND_HANDLE, 
				connectionHandle, 
				message,
				cb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<Void> agentCloseConnection(
			Agent.Connection connection) throws IndyException {

		final CompletableFuture<Void> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				Void result = null;
				future.complete(result);
			}
		};

		int connectionHandle = connection.getConnectionHandle();

		int result = LibIndy.api.indy_agent_close_connection(
				FIXED_COMMAND_HANDLE, 
				connectionHandle, 
				cb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<Void> agentCloseListener(
			Agent.Listener listener) throws IndyException {

		final CompletableFuture<Void> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				Void result = null;
				future.complete(result);
			}
		};

		int listenerHandle = listener.getListenerHandle();

		int result = LibIndy.api.indy_agent_close_connection(
				FIXED_COMMAND_HANDLE, 
				listenerHandle, 
				cb);

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

package org.hyperledger.indy.sdk.agent;

import java.util.concurrent.CompletableFuture;

import org.hyperledger.indy.sdk.IndyException;
import org.hyperledger.indy.sdk.IndyJava;
import org.hyperledger.indy.sdk.LibIndy;
import org.hyperledger.indy.sdk.agent.AgentResults.AgentAddIdentityResult;
import org.hyperledger.indy.sdk.agent.AgentResults.AgentCloseConnectionResult;
import org.hyperledger.indy.sdk.agent.AgentResults.AgentCloseListenerResult;
import org.hyperledger.indy.sdk.agent.AgentResults.AgentConnectResult;
import org.hyperledger.indy.sdk.agent.AgentResults.AgentListenResult;
import org.hyperledger.indy.sdk.agent.AgentResults.AgentRemoveIdentityResult;
import org.hyperledger.indy.sdk.agent.AgentResults.AgentSendResult;
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

	public static CompletableFuture<AgentConnectResult> agentConnect(
			Pool pool,
			Wallet wallet,
			String senderDid,
			String receiverDid,
			Callback messageCb) throws IndyException {

		final CompletableFuture<AgentConnectResult> future = new CompletableFuture<> ();

		Callback connectionCb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, int connection_handle) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				Agent.Connection connection = new Agent.Connection(connection_handle);

				AgentConnectResult result = new AgentConnectResult(connection);
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

	public static CompletableFuture<AgentListenResult> agentListen(
			String endpoint,
			Callback connectionCb,
			Callback messageCb) throws IndyException {

		final CompletableFuture<AgentListenResult> future = new CompletableFuture<> ();

		Callback listenerCb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, int listener_handle) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				Agent.Listener connection = new Agent.Listener(listener_handle);

				AgentListenResult result = new AgentListenResult(connection);
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

	public static CompletableFuture<AgentAddIdentityResult> agentAddIdentity(
			Agent.Listener listener,
			Pool pool,
			Wallet wallet,
			String did) throws IndyException {

		final CompletableFuture<AgentAddIdentityResult> future = new CompletableFuture<> ();

		Callback addIdentityCb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, int listener_handle) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				AgentAddIdentityResult result = new AgentAddIdentityResult();
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

	public static CompletableFuture<AgentRemoveIdentityResult> agentRemoveIdentity(
			Agent.Listener listener,
			Wallet wallet,
			String did) throws IndyException {

		final CompletableFuture<AgentRemoveIdentityResult> future = new CompletableFuture<> ();

		Callback rmIdentityCb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, int listener_handle) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				AgentRemoveIdentityResult result = new AgentRemoveIdentityResult();
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

	public static CompletableFuture<AgentSendResult> agentSend(
			Agent.Connection connection,
			String message) throws IndyException {

		final CompletableFuture<AgentSendResult> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				AgentSendResult result = new AgentSendResult();
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

	public static CompletableFuture<AgentCloseConnectionResult> agentCloseConnection(
			Agent.Connection connection) throws IndyException {

		final CompletableFuture<AgentCloseConnectionResult> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				AgentCloseConnectionResult result = new AgentCloseConnectionResult();
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

	public static CompletableFuture<AgentCloseListenerResult> agentCloseListener(
			Agent.Listener listener) throws IndyException {

		final CompletableFuture<AgentCloseListenerResult> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				AgentCloseListenerResult result = new AgentCloseListenerResult();
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

		public CompletableFuture<AgentSendResult> agentSend(String message) throws IndyException {

			return Agent.agentSend(this, message);
		}

		public CompletableFuture<AgentCloseConnectionResult> agentCloseConnection() throws IndyException {

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

		public CompletableFuture<AgentAddIdentityResult> agentAddIdentity(Pool pool, Wallet wallet, String did) throws IndyException {

			return Agent.agentAddIdentity(this, pool, wallet, did);
		}

		public CompletableFuture<AgentRemoveIdentityResult> agentRemoveIdentity(Wallet wallet, String did) throws IndyException {

			return Agent.agentRemoveIdentity(this, wallet, did);
		}

		public CompletableFuture<AgentCloseListenerResult> agentCloseListener() throws IndyException {

			return Agent.agentCloseListener(this);
		}
	}
}

package org.hyperledger.indy.sdk.agent;

import java.util.concurrent.CompletableFuture;
import java.util.concurrent.Future;

import org.hyperledger.indy.sdk.LibSovrin;
import org.hyperledger.indy.sdk.SovrinException;
import org.hyperledger.indy.sdk.SovrinJava;
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
public class Agent extends SovrinJava.API {

	private Agent() {

	}

	/*
	 * STATIC METHODS
	 */

	public static Future<AgentConnectResult> agentConnect(
			Pool pool,
			Wallet wallet,
			String senderDid,
			String receiverDid,
			Callback messageCb) throws SovrinException {

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

		int result = LibSovrin.api.sovrin_agent_connect(
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

	public static Future<AgentListenResult> agentListen(
			String endpoint,
			Callback connectionCb,
			Callback messageCb) throws SovrinException {

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

		int result = LibSovrin.api.sovrin_agent_listen(
				FIXED_COMMAND_HANDLE, 
				endpoint,
				listenerCb,
				connectionCb,
				messageCb);

		checkResult(result);

		return future;
	}

	public static Future<AgentAddIdentityResult> agentAddIdentity(
			Agent.Listener listener,
			Pool pool,
			Wallet wallet,
			String did,
			Callback connectionCb,
			Callback messageCb) throws SovrinException {

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

		int result = LibSovrin.api.sovrin_agent_add_identity(
				FIXED_COMMAND_HANDLE, 
				listenerHandle,
				poolHandle,
				walletHandle, 
				did,
				addIdentityCb);

		checkResult(result);

		return future;
	}

	public static Future<AgentRemoveIdentityResult> agentRemoveIdentity(
			Agent.Listener listener,
			Wallet wallet,
			String did,
			Callback connectionCb,
			Callback messageCb) throws SovrinException {

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

		int result = LibSovrin.api.sovrin_agent_remove_identity(
				FIXED_COMMAND_HANDLE, 
				listenerHandle,
				walletHandle, 
				did,
				rmIdentityCb);

		checkResult(result);

		return future;
	}

	public static Future<AgentSendResult> agentSend(
			Agent.Connection connection,
			String message) throws SovrinException {

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

		int result = LibSovrin.api.sovrin_agent_send(
				FIXED_COMMAND_HANDLE, 
				connectionHandle, 
				message,
				cb);

		checkResult(result);

		return future;
	}

	public static Future<AgentCloseConnectionResult> agentCloseConnection(
			Agent.Connection connection) throws SovrinException {

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

		int result = LibSovrin.api.sovrin_agent_close_connection(
				FIXED_COMMAND_HANDLE, 
				connectionHandle, 
				cb);

		checkResult(result);

		return future;
	}

	public static Future<AgentCloseListenerResult> agentCloseListener(
			Agent.Listener listener) throws SovrinException {

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

		int result = LibSovrin.api.sovrin_agent_close_connection(
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

		public Future<AgentCloseConnectionResult> agentCloseConnection() throws SovrinException {

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

		public Future<AgentCloseListenerResult> agentCloseListener() throws SovrinException {

			return Agent.agentCloseListener(this);
		}
	}
}

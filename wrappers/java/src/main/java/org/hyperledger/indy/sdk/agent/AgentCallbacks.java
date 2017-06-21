package org.hyperledger.indy.sdk.agent;

import com.sun.jna.Callback;

/**
 * agent.rs callbacks
 */
public final class AgentCallbacks {

	private AgentCallbacks() {

	}

	public static class AgentMessageCallback implements Callback {

		public final void callback(int xconnection_handle, int err, String message) {

		}
	}

	public static class AgentConnectionCallback implements Callback {

		public final void callback(int xlistener_handle, int err, int connection_handle, String sender_did, String receiver_did) {

		}
	}
}

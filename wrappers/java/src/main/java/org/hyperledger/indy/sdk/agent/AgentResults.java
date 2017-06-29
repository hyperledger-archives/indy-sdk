package org.hyperledger.indy.sdk.agent;

import org.hyperledger.indy.sdk.SovrinJava;

/**
 * agent.rs results
 */
public final class AgentResults {

	private AgentResults() {

	}

	public static class AgentConnectResult extends SovrinJava.Result {

		private Agent.Connection connection;
		AgentConnectResult(Agent.Connection connection) { this.connection = connection; }
		public Agent.Connection getConnection() { return this.connection; }
	}

	public static class AgentListenResult extends SovrinJava.Result {

		private Agent.Listener listener;
		AgentListenResult(Agent.Listener listener) { this.listener = listener; }
		public Agent.Listener getListener() { return this.listener; }
	}

	public static class AgentSendResult extends SovrinJava.Result {

		AgentSendResult() { }
	}

	public static class AgentAddIdentityResult extends SovrinJava.Result {

		AgentAddIdentityResult() { }
	}

	public static class AgentRemoveIdentityResult extends SovrinJava.Result {

		AgentRemoveIdentityResult() { }
	}

	public static class AgentCloseConnectionResult extends SovrinJava.Result {

		AgentCloseConnectionResult() { }
	}

	public static class AgentCloseListenerResult extends SovrinJava.Result {

		AgentCloseListenerResult() { }
	}
}

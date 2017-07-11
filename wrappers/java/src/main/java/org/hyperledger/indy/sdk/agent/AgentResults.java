package org.hyperledger.indy.sdk.agent;

import org.hyperledger.indy.sdk.IndyJava;

/**
 * agent.rs results
 */
public final class AgentResults {

	private AgentResults() {

	}

	public static class AgentConnectResult extends IndyJava.Result {

		private Agent.Connection connection;
		AgentConnectResult(Agent.Connection connection) { this.connection = connection; }
		public Agent.Connection getConnection() { return this.connection; }
	}

	public static class AgentListenResult extends IndyJava.Result {

		private Agent.Listener listener;
		AgentListenResult(Agent.Listener listener) { this.listener = listener; }
		public Agent.Listener getListener() { return this.listener; }
	}

	public static class AgentSendResult extends IndyJava.Result {

		AgentSendResult() { }
	}

	public static class AgentAddIdentityResult extends IndyJava.Result {

		AgentAddIdentityResult() { }
	}

	public static class AgentRemoveIdentityResult extends IndyJava.Result {

		AgentRemoveIdentityResult() { }
	}

	public static class AgentCloseConnectionResult extends IndyJava.Result {

		AgentCloseConnectionResult() { }
	}

	public static class AgentCloseListenerResult extends IndyJava.Result {

		AgentCloseListenerResult() { }
	}
}

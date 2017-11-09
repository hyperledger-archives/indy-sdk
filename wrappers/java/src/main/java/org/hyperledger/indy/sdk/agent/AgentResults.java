package org.hyperledger.indy.sdk.agent;

import org.hyperledger.indy.sdk.IndyJava;

/**
 * agent.rs results
 */
public final class AgentResults {

	private AgentResults() {

	}

	/**
	 * Result from calling parseMsg.
	 */
	public static class ParseMsgResult extends IndyJava.Result {

		private String senderKey;
		private byte[] msg;
		ParseMsgResult(String senderKey, byte[] msg) { this.senderKey = senderKey; this.msg = msg;}

		/**
		 * Gets the sender key.
		 *
		 * @return The Sender key.
		 */
		public String getSenderKey() { return this.senderKey; }

		/**
		 * Gets the message.
		 *
		 * @return The message.
		 */
		public byte[] getMessage() { return this.msg; }
	}
}

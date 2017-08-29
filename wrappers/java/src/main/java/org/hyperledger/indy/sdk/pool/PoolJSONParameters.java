package org.hyperledger.indy.sdk.pool;

import org.hyperledger.indy.sdk.IndyJava;

/**
 * pool.rs JSON parameters
 */
public final class PoolJSONParameters {

	private PoolJSONParameters() {

	}

	public static class CreatePoolLedgerConfigJSONParameter extends IndyJava.JsonParameter {

		public CreatePoolLedgerConfigJSONParameter(String genesisTxn) {

			if (genesisTxn != null) this.map.put("genesis_txn", genesisTxn);
		}
	}

	public static class OpenPoolLedgerJSONParameter extends IndyJava.JsonParameter {

		public OpenPoolLedgerJSONParameter(Boolean refreshOnOpen, Boolean autoRefreshTime, Integer networkTimeout) {

			if (refreshOnOpen != null) this.map.put("refresh_on_open", refreshOnOpen);
			if (autoRefreshTime != null) this.map.put("auto_refresh_time", autoRefreshTime);
			if (networkTimeout != null) this.map.put("network_timeout", networkTimeout);
		}
	}
}

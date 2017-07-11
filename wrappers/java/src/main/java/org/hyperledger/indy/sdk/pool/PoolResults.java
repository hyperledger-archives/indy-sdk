package org.hyperledger.indy.sdk.pool;

import org.hyperledger.indy.sdk.IndyJava;

/**
 * pool.rs results
 */
public final class PoolResults {

	private PoolResults() {

	}

	public static class CreatePoolLedgerConfigResult extends IndyJava.Result {

		CreatePoolLedgerConfigResult() { }
	}

	public static class OpenPoolLedgerResult extends IndyJava.Result {

		private Pool pool;
		OpenPoolLedgerResult(Pool pool) { this.pool = pool; }
		public Pool getPool() { return this.pool; }
	}

	public static class RefreshPoolLedgerResult extends IndyJava.Result {

		RefreshPoolLedgerResult() { }
	}

	public static class ClosePoolLedgerResult extends IndyJava.Result {

		ClosePoolLedgerResult() { }
	}

	public static class DeletePoolLedgerConfigResult extends IndyJava.Result {

		DeletePoolLedgerConfigResult() { }
	}
}

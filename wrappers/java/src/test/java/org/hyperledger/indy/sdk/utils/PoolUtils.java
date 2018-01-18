package org.hyperledger.indy.sdk.utils;

import org.apache.commons.io.FileUtils;
import org.hyperledger.indy.sdk.IndyException;
import org.hyperledger.indy.sdk.ledger.Ledger;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.pool.PoolJSONParameters;
import org.json.JSONException;

import java.io.File;
import java.io.FileWriter;
import java.io.IOException;
import java.util.concurrent.ExecutionException;

public class PoolUtils {

	public static final String DEFAULT_POOL_NAME = "default_pool";
	public static final int TEST_TIMEOUT_FOR_REQUEST_ENSURE = 20_000;
	static final int RESUBMIT_REQUEST_TIMEOUT = 5_000;
	static final int RESUBMIT_REQUEST_CNT = 3;


	public static File createGenesisTxnFile(String filename) throws IOException {
		return createGenesisTxnFile(filename, 4);
	}

	private static File createGenesisTxnFile(String filename, int nodesCnt) throws IOException {
		String path = EnvironmentUtils.getTmpPath(filename);

		File file = new File(path);

		FileUtils.forceMkdirParent(file);

		writeTransactions(file, nodesCnt);
		return file;
	}

	public static void writeTransactions(File file, int nodesCnt) throws IOException {
		String testPoolIp = EnvironmentUtils.getTestPoolIP();

		String[] defaultTxns = new String[]{
				String.format("{\"data\":{\"alias\":\"Node1\",\"blskey\":\"4N8aUNHSgjQVgkpm8nhNEfDf6txHznoYREg9kirmJrkivgL4oSEimFF6nsQ6M41QvhM2Z33nves5vfSn9n1UwNFJBYtWVnHYMATn76vLuL3zU88KyeAYcHfsih3He6UHcXDxcaecHVz6jhCYz1P2UZn2bDVruL5wXpehgBfBaLKm3Ba\",\"client_ip\":\"%s\",\"client_port\":9702,\"node_ip\":\"%s\",\"node_port\":9701,\"services\":[\"VALIDATOR\"]},\"dest\":\"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv\",\"identifier\":\"Th7MpTaRZVRYnPiabds81Y\",\"txnId\":\"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62\",\"type\":\"0\"}", testPoolIp, testPoolIp),
				String.format("{\"data\":{\"alias\":\"Node2\",\"blskey\":\"37rAPpXVoxzKhz7d9gkUe52XuXryuLXoM6P6LbWDB7LSbG62Lsb33sfG7zqS8TK1MXwuCHj1FKNzVpsnafmqLG1vXN88rt38mNFs9TENzm4QHdBzsvCuoBnPH7rpYYDo9DZNJePaDvRvqJKByCabubJz3XXKbEeshzpz4Ma5QYpJqjk\",\"client_ip\":\"%s\",\"client_port\":9704,\"node_ip\":\"%s\",\"node_port\":9703,\"services\":[\"VALIDATOR\"]},\"dest\":\"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb\",\"identifier\":\"EbP4aYNeTHL6q385GuVpRV\",\"txnId\":\"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc\",\"type\":\"0\"}", testPoolIp, testPoolIp),
				String.format("{\"data\":{\"alias\":\"Node3\",\"blskey\":\"3WFpdbg7C5cnLYZwFZevJqhubkFALBfCBBok15GdrKMUhUjGsk3jV6QKj6MZgEubF7oqCafxNdkm7eswgA4sdKTRc82tLGzZBd6vNqU8dupzup6uYUf32KTHTPQbuUM8Yk4QFXjEf2Usu2TJcNkdgpyeUSX42u5LqdDDpNSWUK5deC5\",\"client_ip\":\"%s\",\"client_port\":9706,\"node_ip\":\"%s\",\"node_port\":9705,\"services\":[\"VALIDATOR\"]},\"dest\":\"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya\",\"identifier\":\"4cU41vWW82ArfxJxHkzXPG\",\"txnId\":\"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4\",\"type\":\"0\"}", testPoolIp, testPoolIp),
				String.format("{\"data\":{\"alias\":\"Node4\",\"blskey\":\"2zN3bHM1m4rLz54MJHYSwvqzPchYp8jkHswveCLAEJVcX6Mm1wHQD1SkPYMzUDTZvWvhuE6VNAkK3KxVeEmsanSmvjVkReDeBEMxeDaayjcZjFGPydyey1qxBHmTvAnBKoPydvuTAqx5f7YNNRAdeLmUi99gERUU7TD8KfAa6MpQ9bw\",\"client_ip\":\"%s\",\"client_port\":9708,\"node_ip\":\"%s\",\"node_port\":9707,\"services\":[\"VALIDATOR\"]},\"dest\":\"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA\",\"identifier\":\"TWwCRQRZ2ZHMJFn9TzLp7W\",\"txnId\":\"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008\",\"type\":\"0\"}", testPoolIp, testPoolIp)
		};

		FileWriter fw = new FileWriter(file);
		for (int i = 0; i < nodesCnt; i++) {
			fw.write(defaultTxns[i]);
			fw.write("\n");
		}

		fw.close();

	}

	public static String createPoolLedgerConfig() throws InterruptedException, ExecutionException, IndyException, IOException {
		return createPoolLedgerConfig(4);
	}

	public static String createPoolLedgerConfig(int nodesCnt) throws InterruptedException, ExecutionException, IndyException, IOException {
		createPoolLedgerConfig(DEFAULT_POOL_NAME, nodesCnt);
		return DEFAULT_POOL_NAME;
	}

	public static void createPoolLedgerConfig(String poolName, int nodesCnt) throws IOException, InterruptedException, java.util.concurrent.ExecutionException, IndyException {
		File genesisTxnFile = createGenesisTxnFile("temp.txn", nodesCnt);
		PoolJSONParameters.CreatePoolLedgerConfigJSONParameter createPoolLedgerConfigJSONParameter
				= new PoolJSONParameters.CreatePoolLedgerConfigJSONParameter(genesisTxnFile.getAbsolutePath());
		Pool.createPoolLedgerConfig(poolName, createPoolLedgerConfigJSONParameter.toJson()).get();
	}

	public static Pool createAndOpenPoolLedger() throws IndyException, InterruptedException, ExecutionException, IOException {
		String poolName = PoolUtils.createPoolLedgerConfig();

		PoolJSONParameters.OpenPoolLedgerJSONParameter config = new PoolJSONParameters.OpenPoolLedgerJSONParameter(true, null, null);
		return Pool.openPoolLedger(poolName, config.toJson()).get();
	}

	public interface PoolResponseChecker {
		boolean check(String response);
	}

	public interface ActionChecker {
		String action() throws IndyException, ExecutionException, InterruptedException;
	}

	public static String ensurePreviousRequestApplied(Pool pool, String checkerRequest, PoolResponseChecker checker) throws IndyException, ExecutionException, InterruptedException {
		for (int i = 0; i < RESUBMIT_REQUEST_CNT; i++) {
			String response = Ledger.submitRequest(pool, checkerRequest).get();
			try {
				if (checker.check(response)) {
					return response;
				}
			} catch (JSONException e) {
				e.printStackTrace();
				System.err.println(e.toString());
				System.err.println(response);
			}
			Thread.sleep(RESUBMIT_REQUEST_TIMEOUT);
		}
		throw new IllegalStateException();
	}

	public static boolean retryCheck(ActionChecker action, PoolResponseChecker checker) throws InterruptedException, ExecutionException, IndyException {
		for (int i = 0; i < RESUBMIT_REQUEST_CNT; i++) {
			if (checker.check(action.action())) {
				return true;
			}
		}
		return false;
	}
}

package org.hyperledger.indy.sdk.utils;

import org.apache.commons.io.FileUtils;
import org.hyperledger.indy.sdk.IndyException;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.pool.PoolJSONParameters;

import java.io.File;
import java.io.FileWriter;
import java.io.IOException;
import java.util.concurrent.ExecutionException;

public class PoolUtils {
	public static File createGenesisTxnFile(String filename) throws IOException {
		return createGenesisTxnFile(filename, 4);
	}

	public static File createGenesisTxnFile(String filename, int nodesCnt) throws IOException {
		String path = StorageUtils.getTmpPath(filename);
		String[] defaultTxns = new String[]{
				"{\"data\":{\"alias\":\"Node1\",\"client_ip\":\"10.0.0.2\",\"client_port\":9702,\"node_ip\":\"10.0.0.2\",\"node_port\":9701,\"services\":[\"VALIDATOR\"]},\"dest\":\"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv\",\"identifier\":\"Th7MpTaRZVRYnPiabds81Y\",\"txnId\":\"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62\",\"type\":\"0\"}",
				"{\"data\":{\"alias\":\"Node2\",\"client_ip\":\"10.0.0.2\",\"client_port\":9704,\"node_ip\":\"10.0.0.2\",\"node_port\":9703,\"services\":[\"VALIDATOR\"]},\"dest\":\"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb\",\"identifier\":\"EbP4aYNeTHL6q385GuVpRV\",\"txnId\":\"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc\",\"type\":\"0\"}",
				"{\"data\":{\"alias\":\"Node3\",\"client_ip\":\"10.0.0.2\",\"client_port\":9706,\"node_ip\":\"10.0.0.2\",\"node_port\":9705,\"services\":[\"VALIDATOR\"]},\"dest\":\"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya\",\"identifier\":\"4cU41vWW82ArfxJxHkzXPG\",\"txnId\":\"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4\",\"type\":\"0\"}",
				"{\"data\":{\"alias\":\"Node4\",\"client_ip\":\"10.0.0.2\",\"client_port\":9708,\"node_ip\":\"10.0.0.2\",\"node_port\":9707,\"services\":[\"VALIDATOR\"]},\"dest\":\"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA\",\"identifier\":\"TWwCRQRZ2ZHMJFn9TzLp7W\",\"txnId\":\"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008\",\"type\":\"0\"}"
		};

		File file = new File(path);

		FileUtils.forceMkdirParent(file);

		FileWriter fw = new FileWriter(file);
		for (int i = 0; i < nodesCnt; i++) {
			fw.write(defaultTxns[i]);
			fw.write("\n");
		}

		fw.close();

		return file;
	}

	public static String createPoolLedgerConfig() throws InterruptedException, ExecutionException, IndyException, IOException {
		return createPoolLedgerConfig(4);
	}

	public static String createPoolLedgerConfig(int nodesCnt) throws InterruptedException, ExecutionException, IndyException, IOException {
		String poolName = "test";
		createPoolLedgerConfig(poolName, nodesCnt);
		return poolName;
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
}

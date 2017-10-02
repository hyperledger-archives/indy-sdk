<a href="https://sovrin.org/" target="_blank"><img src="https://avatars2.githubusercontent.com/u/22057628?v=3&s=50" align="right"></a>

## Indy SDK for .NET

This is a **work-in-progress** .NET wrapper for [Indy](https://www.hyperledger.org/projects/indy). It is implemented using PInvoke calls to a native library written in Rust. Indy 
is the open-source codebase behind the Sovrin network for self-sovereign digital identity.

Pull requests welcome!

**Not ready for production use! Not all commands work properly! Use at your own risk!**

### How to build

First, build the native "indy" library at https://github.com/hyperledger/indy-sdk:

	cargo build

This will create the indy.dll library and its dependencies under the target directory.

Next, open the visual studio project and build.  When deploying the indy.dll and it's dependencies should be placed in the same directory as the the .NET assembly or should be available in a directory
 in the <a href="https://msdn.microsoft.com/en-us/library/windows/desktop/ms682586(v=vs.85).aspx">Windows DLL search order</a>

### Example use

	// 0. Create genesis txn file and pool ledger config
	var poolName = "temp.txn"
	var file = StorageUtils.GetTmpPath(poolName);

	Directory.CreateDirectory(Path.GetDirectoryName(file));
	using(var stream = new StreamWriter(file))
	{
		Console.WriteLine("{\"data\":{\"alias\":\"Node1\",\"blskey\":\"4N8aUNHSgjQVgkpm8nhNEfDf6txHznoYREg9kirmJrkivgL4oSEimFF6nsQ6M41QvhM2Z33nves5vfSn9n1UwNFJBYtWVnHYMATn76vLuL3zU88KyeAYcHfsih3He6UHcXDxcaecHVz6jhCYz1P2UZn2bDVruL5wXpehgBfBaLKm3Ba\",\"client_ip\":\"10.0.0.2\",\"client_port\":9702,\"node_ip\":\"10.0.0.2\",\"node_port\":9701,\"services\":[\"VALIDATOR\"]},\"dest\":\"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv\",\"identifier\":\"Th7MpTaRZVRYnPiabds81Y\",\"txnId\":\"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62\",\"type\":\"0\"}");
		Console.WriteLine("{\"data\":{\"alias\":\"Node2\",\"blskey\":\"37rAPpXVoxzKhz7d9gkUe52XuXryuLXoM6P6LbWDB7LSbG62Lsb33sfG7zqS8TK1MXwuCHj1FKNzVpsnafmqLG1vXN88rt38mNFs9TENzm4QHdBzsvCuoBnPH7rpYYDo9DZNJePaDvRvqJKByCabubJz3XXKbEeshzpz4Ma5QYpJqjk\",\"client_ip\":\"10.0.0.2\",\"client_port\":9704,\"node_ip\":\"10.0.0.2\",\"node_port\":9703,\"services\":[\"VALIDATOR\"]},\"dest\":\"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb\",\"identifier\":\"EbP4aYNeTHL6q385GuVpRV\",\"txnId\":\"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc\",\"type\":\"0\"}");
		Console.WriteLine("{\"data\":{\"alias\":\"Node3\",\"blskey\":\"3WFpdbg7C5cnLYZwFZevJqhubkFALBfCBBok15GdrKMUhUjGsk3jV6QKj6MZgEubF7oqCafxNdkm7eswgA4sdKTRc82tLGzZBd6vNqU8dupzup6uYUf32KTHTPQbuUM8Yk4QFXjEf2Usu2TJcNkdgpyeUSX42u5LqdDDpNSWUK5deC5\",\"client_ip\":\"10.0.0.2\",\"client_port\":9706,\"node_ip\":\"10.0.0.2\",\"node_port\":9705,\"services\":[\"VALIDATOR\"]},\"dest\":\"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya\",\"identifier\":\"4cU41vWW82ArfxJxHkzXPG\",\"txnId\":\"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4\",\"type\":\"0\"}");
		Console.WriteLine("{\"data\":{\"alias\":\"Node4\",\"blskey\":\"2zN3bHM1m4rLz54MJHYSwvqzPchYp8jkHswveCLAEJVcX6Mm1wHQD1SkPYMzUDTZvWvhuE6VNAkK3KxVeEmsanSmvjVkReDeBEMxeDaayjcZjFGPydyey1qxBHmTvAnBKoPydvuTAqx5f7YNNRAdeLmUi99gERUU7TD8KfAa6MpQ9bw\",\"client_ip\":\"10.0.0.2\",\"client_port\":9708,\"node_ip\":\"10.0.0.2\",\"node_port\":9707,\"services\":[\"VALIDATOR\"]},\"dest\":\"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA\",\"identifier\":\"TWwCRQRZ2ZHMJFn9TzLp7W\",\"txnId\":\"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008\",\"type\":\"0\"}");
		stream.Close();
	}
	
	var createPoolLedgerConfig = string.Format("{{\"genesis_txn\":\"{0}\"}}", file);
	Pool.CreatePoolLedgerConfigAsync(poolName, createPoolLedgerConfig).Wait();
			
	// 1. Create ledger config from genesis txn file
	var pool = Pool.OpenPoolLedgerAsync(poolName, "{}").Result;

	// 2. Create and Open My Wallet
	Wallet.CreateWalletAsync(poolName, "myWallet", "default", null, null).Wait();
	var myWallet = Wallet.OpenWalletAsync("myWallet", null, null).Result;

	// 3. Create and Open Trustee Wallet
	Wallet.CreateWalletAsync(poolName, "theirWallet", "default", null, null).Wait();
	var trusteeWallet = Wallet.OpenWalletAsync("theirWallet", null, null).Result;

	// 4. Create My Did
	var createMyDidResult = Signus.CreateAndStoreMyDidAsync(myWallet, "{}").Result;
	Assert.IsNotNull(createMyDidResult);
	var myDid = createMyDidResult.Did;
	var myVerkey = createMyDidResult.VerKey;

	// 5. Create Did from Trustee1 seed
	var theirDidJson = "{\"seed\":\"000000000000000000000000Trustee1\"}"; 

	var createTheirDidResult = Signus.CreateAndStoreMyDidAsync(trusteeWallet, theirDidJson).Result;
	Assert.IsNotNull(createTheirDidResult);
	var trusteeDid = createTheirDidResult.Did;

	// 6. Build Nym Request
	var nymRequest = Ledger.BuildNymRequestAsync(trusteeDid, myDid, myVerkey, null, null).Result;
	Assert.IsNotNull(nymRequest);

	// 7. Trustee Sign Nym Request
	var nymResponseJson = Ledger.SignAndSubmitRequestAsync(pool, trusteeWallet, trusteeDid, nymRequest).Result;
	Assert.IsNotNull(nymResponseJson);

	var nymResponse = JObject.Parse(nymResponseJson);

	Assert.AreEqual(myDid, nymResponse["result"].Value<string>("dest"));
	Assert.AreEqual(myVerkey, nymResponse["result"].Value<string>("verkey"));

	// 8. Close and delete My Wallet
	myWallet.CloseAsync().Wait();
	Wallet.DeleteWalletAsync("myWallet", null).Wait();

	// 9. Close and delete Their Wallet
	trusteeWallet.CloseAsync().Wait();
	Wallet.DeleteWalletAsync("theirWallet", null).Wait();

	// 10. Close Pool
	pool.CloseAsync().Wait();
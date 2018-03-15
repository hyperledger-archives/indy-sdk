## Indy SDK for Java

This is a **work-in-progress** Java wrapper for [Indy](https://www.hyperledger.org/projects/indy). It is implemented using a foreign function interface (FFI) to a native library written in Rust. Indy is the
open-source codebase behind the Sovrin network for self-sovereign digital identity.

This Java wrapper currently requires Java 8 (e.g. the openjdk-8-jdk package in Debian/Ubuntu).

Pull requests welcome!

**Not ready for production use! Not all commands work properly! Use at your own risk!**
### How to install
In your maven project add to pom.xml file next content:

1. Inside repositories tag block add:
    
    
    <repository>
        <id>evernym</id>
        <url>https://repo.evernym.com/artifactory/libindy-maven-local</url>
    </repository>

2. Inside dependencies tag block add:    
    
    
    <dependency>
        <groupId>org.hyperledger</groupId>
        <artifactId>indy</artifactId>
        <version>1.3.1-dev-410</version>
    </dependency>
     
Note that before you can use java wrapper you must install  c-callable SDK. 
See the section "How-to-install" in [Indy SDK](README.md)
### How to build

First, build the native "indy" library at https://github.com/hyperledger/indy-sdk:

	cargo build

Then copy the resulting `libindy.so` to `./lib/`.

Then run

    mvn clean install

### Example use

	public class TestCreate {
	
		public static final String TRUSTEE_DID = "V4SGRU86Z58d6TV7PBUe6f";
		public static final String TRUSTEE_VERKEY = "GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL";
		public static final String TRUSTEE_SEED = "000000000000000000000000Trustee1";
	
		public static void main(String[] args) throws Exception {
	
			if (! LibIndy.isInitialized()) LibIndy.init(new File("./lib/libindy.so"));
	
			// create pool
	
			System.out.println("=== CREATE POOL ===");
			CreatePoolLedgerConfigJSONParameter createPoolLedgerConfigJSONParameter = new CreatePoolLedgerConfigJSONParameter("localhost.txn");
			System.out.println("CreatePoolLedgerConfigJSONParameter: " + createPoolLedgerConfigJSONParameter);
			CreatePoolLedgerConfigResult createPoolLedgerConfigResult = Pool.createPoolLedgerConfig("localhost", createPoolLedgerConfigJSONParameter).get();
			System.out.println("CreatePoolLedgerConfigResult: " + createPoolLedgerConfigResult);
	
			// open pool
	
			System.out.println("=== OPEN POOL ===");
			OpenPoolLedgerJSONParameter openPoolLedgerJSONParameter = new OpenPoolLedgerJSONParameter(Boolean.TRUE, null, null);
			System.out.println("OpenPoolLedgerJSONParameter: " + openPoolLedgerJSONParameter);
			OpenPoolLedgerResult openPoolLedgerResult = Pool.openPoolLedger("localhost", openPoolLedgerJSONParameter).get();
			System.out.println("OpenPoolLedgerResult: " + openPoolLedgerResult);
	
			Pool pool = openPoolLedgerResult.getPool();
	
			// create TRUSTEE wallet
	
			System.out.println("=== CREATE TRUSTEE WALLET ===");
			CreateWalletResult createWalletResultTrustee = Wallet.createWallet("localhost", "trusteewallet", "default", null, null).get();
			System.out.println("CreateWalletResultTrustee: " + createWalletResultTrustee);
	
			// create USER wallet
	
			System.out.println("=== CREATE USER WALLET ===");
			CreateWalletResult createWalletResultUser = Wallet.createWallet("localhost", "userwallet", "default", null, null).get();
			System.out.println("CreateWalletResultUser: " + createWalletResultUser);
	
			// open TRUSTEE wallet
	
			System.out.println("=== OPEN TRUSTEE WALLET ===");
			OpenWalletResult openWalletResultTrustee = Wallet.openWallet("trusteewallet", null, null).get();
			System.out.println("OpenWalletResultTrustee: " + openWalletResultTrustee);
	
			Wallet walletTrustee = openWalletResultTrustee.getWallet();
	
			// create TRUSTEE DID
	
			System.out.println("=== CREATE TRUSTEE DID ===");
			CreateAndStoreMyDidJSONParameter createAndStoreMyDidJSONParameterTrustee = new CreateAndStoreMyDidJSONParameter(null, TRUSTEE_SEED, null, null);
			System.out.println("CreateAndStoreMyDidJSONParameterTrustee: " + createAndStoreMyDidJSONParameterTrustee);
			CreateAndStoreMyDidResult createAndStoreMyDidResultTrustee = Signus.createAndStoreMyDid(walletTrustee, createAndStoreMyDidJSONParameterTrustee).get();
			System.out.println("CreateAndStoreMyDidResultTrustee: " + createAndStoreMyDidResultTrustee);
	
			// create NYM request
	
			System.out.println("=== CREATE NYM REQUEST ===");
			BuildNymRequestResult buildNymRequestResult = Ledger.buildNymRequest(TRUSTEE_VERKEY, TRUSTEE_DID, TRUSTEE_VERKEY, null, null, IndyConstants.ROLE_TRUSTEE).get();
			System.out.println("BuildNymRequestResult: " + buildNymRequestResult);
	
			// sign
	
			System.out.println("=== SIGN ===");
			SignResult signResult = Signus.sign(walletTrustee, TRUSTEE_DID, buildNymRequestResult.getRequestJson()).get();
			System.out.println("SignResult: " + signResult);
	
			// submit request to ledger
	
			System.out.println("=== SUBMIT ===");
			SubmitRequestResult submitRequestResult = Ledger.submitRequest(pool, signResult.getSignature()).get();
			System.out.println("SubmitRequestResult: " + submitRequestResult);
	
			// close wallet
	
			System.out.println("=== CLOSE WALLET ===");
			walletTrustee.closeWallet().get();
	
			// close pool
	
			System.out.println("=== CLOSE POOL ===");
			pool.closePoolLedger().get();
		}
	}

Output:

	=== CREATE POOL ===
	CreatePoolLedgerConfigJSONParameter: {"genesis_txn":"localhost.txn"}
	CreatePoolLedgerConfigResult: PoolResults.CreatePoolLedgerConfigResult[]
	=== OPEN POOL ===
	OpenPoolLedgerJSONParameter: {"refresh_on_open":true}
	OpenPoolLedgerResult: PoolResults.OpenPoolLedgerResult[pool=Pool[poolHandle=2]]
	=== CREATE TRUSTEE WALLET ===
	CreateWalletResultTrustee: WalletResults.CreateWalletResult[]
	=== CREATE USER WALLET ===
	CreateWalletResultUser: WalletResults.CreateWalletResult[]
	=== OPEN TRUSTEE WALLET ===
	OpenWalletResultTrustee: WalletResults.OpenWalletResult[wallet=Wallet[walletHandle=3]]
	=== CREATE TRUSTEE DID ===
	CreateAndStoreMyDidJSONParameterTrustee: {"seed":"000000000000000000000000Trustee1"}
	CreateAndStoreMyDidResultTrustee: SignusResults.CreateAndStoreMyDidResult[did=V4SGRU86Z58d6TV7PBUe6f,verkey=GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL,pk=XWSNZUwj7Uc4KzBuTQjNwCZZFwXSMNGVqnfDgbwMiNP]
	=== CREATE NYM REQUEST ===
	BuildNymRequestResult: LedgerResults.BuildNymRequestResult[requestJson={"reqId":1496433560568786245,"identifier":"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL","operation":{"type":"1","dest":"V4SGRU86Z58d6TV7PBUe6f","verkey":"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL","role":"0"}}]
	=== SIGN ===
	SignResult: SignusResults.SignResult[signature={"identifier":"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL","operation":{"dest":"V4SGRU86Z58d6TV7PBUe6f","role":"0","type":"1","verkey":"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL"},"reqId":1496433560568786245,"signature":"2g5hkUm4zZfv91k6BftJvFuBxVwWYvWzuVx4G8WnUivwty9QLLqSzEeLdPveu9wctDmN3AMBosNziHm5HRic9aZR"}]
	=== SUBMIT ===
	SubmitRequestResult: LedgerResults.SubmitRequestResult[requestResultJson={"result":{"reqId":1496433560568786245,"signature":"2g5hkUm4zZfv91k6BftJvFuBxVwWYvWzuVx4G8WnUivwty9QLLqSzEeLdPveu9wctDmN3AMBosNziHm5HRic9aZR","auditPath":["5LAnZ9NxbBSE23WSBHhNExAC3BXMhUgvddWNcBZpSBta","8zaHB69xfcxZZL7rYEKYYVj5xUvBs2onGXzWJjfALWRE","2fhF8bWzNtPXuzV4wF9aTUz4JctzGXjB8uEjEE6ytZZc","FiVgaUHHJ9Nu842dcj7JBHXned5mkz6HZRyhE8kTeWUH"],"identifier":"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL","txnTime":1496433560615.697265625,"seqNo":16,"rootHash":"2J5WdLNnBSFyeASKr5YZouYzWn3KXZafrPrX2JBDVhKM","dest":"V4SGRU86Z58d6TV7PBUe6f","role":"0","verkey":"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL","type":"1"},"op":"REPLY"}]
	=== CLOSE WALLET ===
	=== CLOSE POOL ===

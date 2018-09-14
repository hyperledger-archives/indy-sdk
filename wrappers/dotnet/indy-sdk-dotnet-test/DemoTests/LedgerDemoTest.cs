using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.DidApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;
using Hyperledger.Indy.Test.Util;

namespace Hyperledger.Indy.Test.DemoTests
{
    [TestClass]
    public class LedgerDemoTest : IndyIntegrationTestBase
    {
        private const string WALLET_NAME = "commonWallet";
        private const string TRUSTEE_WALLET_NAME = "trusteeWallet";
        private const string TRUSTEE_WALLET_KEY = "trusteeWalletKey";

        [TestMethod]
        public async Task TestLedgerDemo()
        {
            // 1. Create ledger config from genesis txn file
            Pool.SetProtocolVersionAsync(PoolUtils.PROTOCOL_VERSION);

            var poolName = PoolUtils.CreatePoolLedgerConfig();
            var pool = await Pool.OpenPoolLedgerAsync(poolName, "{}");

            // 2. Create and Open My Wallet
            await WalletUtils.CreateWallet(WALLET_NAME, WALLET_KEY);
            var myWallet = await WalletUtils.OpenWallet(WALLET_NAME, WALLET_KEY);

            // 3. Create and Open Trustee Wallet
            await WalletUtils.CreateWallet(TRUSTEE_WALLET_NAME, TRUSTEE_WALLET_KEY);
            var trusteeWallet = await WalletUtils.OpenWallet(TRUSTEE_WALLET_NAME, TRUSTEE_WALLET_KEY);

            // 4. Create My Did
            var createMyDidResult = await Did.CreateAndStoreMyDidAsync(myWallet, "{}");
            Assert.IsNotNull(createMyDidResult);
            var myDid = createMyDidResult.Did;
            var myVerkey = createMyDidResult.VerKey;

            // 5. Create Did from Trustee1 seed
            var createTheirDidResult = await Did.CreateAndStoreMyDidAsync(trusteeWallet, TRUSTEE_IDENTITY_JSON);
            Assert.IsNotNull(createTheirDidResult);
            var trusteeDid = createTheirDidResult.Did;

            // 6. Build Nym Request
            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, myDid, myVerkey, null, null);
            Assert.IsNotNull(nymRequest);

            // 7. Trustee Sign Nym Request
            var nymResponseJson = await Ledger.SignAndSubmitRequestAsync(pool, trusteeWallet, trusteeDid, nymRequest);
            Assert.IsNotNull(nymResponseJson, "SignAndSubmitRequestAsync response is null");

            /*
             * The format of SignAndSubmitRequestAsync response is like this.
             * 
                {"result":{
                    "reqSignature":{
                        "type":"ED25519",
                        "values":[{"value":"7kDrVBrmrKAvSs1QoQWYq6F774ZN3bRXx5e3aaUFiNvmh4F1yNqQw1951Az35nfrnGjZ99vtCmSFXZ5GqS1zLiG","from":"V4SGRU86Z58d6TV7PBUe6f"}]
                    },
                    "txnMetadata":{
                        "txnTime":1536876204,
                        "seqNo":36,
                        "txnId":"5d38ac6a242239c97ee28884c2b5cadec62248b2256bce51afd814c7847a853e"
                    },
                    "ver":"1",
                    "auditPath":["DATtzSu9AMrArv8C2oribQh4wJ6TaD2K9o76t7EL2N7G","AbGuM7s9MudnT8M2eZe1yaG2EGUGxggMXSSbXCm4DFDx","3fjMoUdsbNrRfG5ZneHaQuX994oA4Z2pYPZtRRPmkngw"],
                    "rootHash":"A9LirjLuoBT59JJTJYvUgfQyEJA32Wb7njrbD9XqT2wc",
                    "txn":{
                        "data":{
                            "dest":"KQRpY4EmSG4MwH7md8gMoN","verkey":"B2nW4JfqZ2omHksoCmwD8zXXmtBsvbQk6WVSboazd8QB"
                        },
                        "protocolVersion":2,
                        "type":"1",
                        "metadata":{
                            "digest":"14594e0b31f751faf72d4bf4abdc6f54af34dab855fe1a0c67fe651b47bb93b5","reqId":1536876205519496000,"from":"V4SGRU86Z58d6TV7PBUe6f"
                        }
                    }
                },
                "op":"REPLY"}
            */

            var nymResponse = JObject.Parse(nymResponseJson);

            Assert.AreEqual(myDid, nymResponse["result"].Value<JObject>("txn").Value<JObject>("data").Value<string>("dest"));
            Assert.AreEqual(myVerkey, nymResponse["result"].Value<JObject>("txn").Value<JObject>("data").Value<string>("verkey"));

            // 8. Close and delete My Wallet
            await myWallet.CloseAsync();
            await WalletUtils.DeleteWallet(WALLET_NAME, WALLET_KEY);

            // 9. Close and delete Their Wallet
            await trusteeWallet.CloseAsync();
            await WalletUtils.DeleteWallet(TRUSTEE_WALLET_NAME, TRUSTEE_WALLET_KEY);

            // 10. Close Pool
            await pool.CloseAsync();
        }
       
    }
}

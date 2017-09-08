using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.LedgerTests
{
    [TestClass]
    public class GetTxnRequestTest : IndyIntegrationTestBase
    {
        private Pool _pool;
        private Wallet _wallet;
        private string _walletName = "ledgerWallet";

        [TestInitialize]
        public async Task OpenPool()
        {
            string poolName = PoolUtils.CreatePoolLedgerConfig();
            _pool = await Pool.OpenPoolLedgerAsync(poolName, null);

            await Wallet.CreateWalletAsync(poolName, _walletName, "default", null, null);
            _wallet = await Wallet.OpenWalletAsync(_walletName, null, null);
        }

        [TestCleanup]
        public async Task ClosePool()
        {
            if (_pool != null)
                await _pool.CloseAsync();

            if (_wallet != null)
                await _wallet.CloseAsync();

            await Wallet.DeleteWalletAsync(_walletName, null);
        }

        [TestMethod]
        public async Task TestBuildGetTxnRequestWorks()
        {
            var identifier = "Th7MpTaRZVRYnPiabds81Y";
            var data = 1;

            var expectedResult = string.Format("\"identifier\":\"{0}\"," +
                    "\"operation\":{{" +
                    "\"type\":\"3\"," +
                    "\"data\":{1}" +
                    "}}", identifier, data);

            var getTxnRequest = await Ledger.BuildGetTxnRequestAsync(identifier, data);

            Assert.IsTrue(getTxnRequest.Replace("\\", "").Contains(expectedResult));
        }

        [TestMethod] //This test fails here and in the Java version.
        public async Task TestGetTxnRequestWorks()
        {
            var didJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";

            var didResult = await Signus.CreateAndStoreMyDidAsync(_wallet, didJson);
            var did = didResult.Did;

            var schemaData = "{\"name\":\"gvt2\",\"version\":\"3.0\",\"attr_names\": [\"name\", \"male\"]}";

            var schemaRequest = await Ledger.BuildSchemaRequestAsync(did, schemaData);
            var schemaResponse = await Ledger.SignAndSubmitRequestAsync(_pool, _wallet, did, schemaRequest);

            var schemaResponseObj = JObject.Parse(schemaResponse);

            var seqNo = schemaResponseObj["result"].Value<int>("seqNo");

            var getTxnRequest = await Ledger.BuildGetTxnRequestAsync(did, seqNo);
            var getTxnResponse = await Ledger.SubmitRequestAsync(_pool, getTxnRequest);

            var getTxnResponseObj = JObject.Parse(getTxnResponse);

            var returnedSchemaData = getTxnResponseObj["result"]["data"]["data"];            
            var expectedSchemaData = JToken.Parse(schemaData);

            Assert.IsTrue(JToken.DeepEquals(expectedSchemaData, returnedSchemaData));
        }

        [TestMethod] 
        public async Task TestGetTxnRequestWorksForInvalidSeqNo()
        {
            var didJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";

            var didResult = await Signus.CreateAndStoreMyDidAsync(_wallet, didJson);
            var did = didResult.Did;

            var schemaData = "{\"name\":\"gvt2\",\"version\":\"3.0\",\"attr_names\": [\"name\", \"male\"]}";

            var schemaRequest = await Ledger.BuildSchemaRequestAsync(did, schemaData);
            var schemaResponse = await Ledger.SignAndSubmitRequestAsync(_pool, _wallet, did, schemaRequest);

            var schemaResponseObj = JObject.Parse(schemaResponse);

            var seqNo = (int)schemaResponseObj["result"]["seqNo"] + 1;

            var getTxnRequest = await Ledger.BuildGetTxnRequestAsync(did, seqNo);
            var getTxnResponse = await Ledger.SubmitRequestAsync(_pool, getTxnRequest);

            var getTxnResponseObj = JObject.Parse(getTxnResponse);

            Assert.IsFalse(getTxnResponseObj["result"]["data"].HasValues);
        }
    }
}

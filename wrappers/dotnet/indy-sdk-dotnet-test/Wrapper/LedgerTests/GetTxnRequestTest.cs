using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;

namespace Indy.Sdk.Dotnet.Test.Wrapper.LedgerTests
{
    [TestClass]
    public class GetTxnRequestTest : IndyIntegrationTestBase
    {
        private Pool _pool;
        private Wallet _wallet;
        private string _walletName = "ledgerWallet";

        [TestInitialize]
        public void OpenPool()
        {
            string poolName = PoolUtils.CreatePoolLedgerConfig();
            _pool = Pool.OpenPoolLedgerAsync(poolName, null).Result;

            Wallet.CreateWalletAsync(poolName, _walletName, "default", null, null).Wait();
            _wallet = Wallet.OpenWalletAsync(_walletName, null, null).Result;
        }

        [TestCleanup]
        public void ClosePool()
        {
            _pool.CloseAsync().Wait();
            _wallet.CloseAsync().Wait();
            Wallet.DeleteWalletAsync(_walletName, null).Wait();
        }

        [TestMethod]
        public void TestBuildGetTxnRequestWorks()
        {
            var identifier = "Th7MpTaRZVRYnPiabds81Y";
            var data = 1;

            var expectedResult = string.Format("\"identifier\":\"{0}\"," +
                    "\"operation\":{{" +
                    "\"type\":\"3\"," +
                    "\"data\":{1}" +
                    "}}", identifier, data);

            var getTxnRequest = Ledger.BuildGetTxnRequestAsync(identifier, data).Result;

            Assert.IsTrue(getTxnRequest.Replace("\\", "").Contains(expectedResult));
        }

        [TestMethod] //This test fails here and in the Java version.
        public void TestGetTxnRequestWorks()
        {
            var didJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";

            var didResult = Signus.CreateAndStoreMyDidAsync(_wallet, didJson).Result;
            var did = didResult.Did;

            var schemaData = "{\"name\":\"gvt2\",\"version\":\"3.0\",\"keys\": [\"name\", \"male\"]}";

            var schemaRequest = Ledger.BuildSchemaRequestAsync(did, schemaData).Result;
            var schemaResponse = Ledger.SignAndSubmitRequestAsync(_pool, _wallet, did, schemaRequest).Result;

            var schemaResponseObj = JObject.Parse(schemaResponse);

            var seqNo = schemaResponseObj["result"].Value<int>("seqNo");

            var getTxnRequest = Ledger.BuildGetTxnRequestAsync(did, seqNo).Result;
            var getTxnResponse = Ledger.SubmitRequestAsync(_pool, getTxnRequest).Result;

            var getTxnResponseObj = JObject.Parse(getTxnResponse);

            var schemaTransaction = getTxnResponseObj["result"].Value<string>("data");
            var schemaTransactionObj = JObject.Parse(schemaTransaction);

            Assert.AreEqual(schemaData, schemaTransactionObj.Value<string>("data"));
        }

        [TestMethod] 
        public void TestGetTxnRequestWorksForInvalidSeqNo()
        {
            var didJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";

            var didResult = Signus.CreateAndStoreMyDidAsync(_wallet, didJson).Result;
            var did = didResult.Did;

            var schemaData = "{\"name\":\"gvt2\",\"version\":\"3.0\",\"keys\": [\"name\", \"male\"]}";

            var schemaRequest = Ledger.BuildSchemaRequestAsync(did, schemaData).Result;
            var schemaResponse = Ledger.SignAndSubmitRequestAsync(_pool, _wallet, did, schemaRequest).Result;

            var schemaResponseObj = JObject.Parse(schemaResponse);

            var seqNo = (int)schemaResponseObj["result"]["seqNo"] + 1;

            var getTxnRequest = Ledger.BuildGetTxnRequestAsync(did, seqNo).Result;
            var getTxnResponse = Ledger.SubmitRequestAsync(_pool, getTxnRequest).Result;

            var getTxnResponseObj = JObject.Parse(getTxnResponse);

            Assert.IsFalse(getTxnResponseObj["result"]["data"].HasValues);
        }
    }
}

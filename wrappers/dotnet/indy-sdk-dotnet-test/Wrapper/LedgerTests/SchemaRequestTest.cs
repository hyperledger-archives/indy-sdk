using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.LedgerTests
{
    [TestClass]
    public class SchemaRequestTest : IndyIntegrationTest
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
        public void TestBuildSchemaRequestWorks()
        {
            var identifier = "Th7MpTaRZVRYnPiabds81Y";
            var data = "{\"name\":\"name\", \"version\":\"1.0\", \"keys\":[\"name\",\"male\"]}";

            var expectedResult = string.Format("\"identifier\":\"{0}\"," +
                    "\"operation\":{{" +
                    "\"type\":\"101\"," +
                    "\"data\":\"{1}\"" +
                    "}}", identifier, data);

            var schemaRequest = Ledger.BuildSchemaRequestAsync(identifier, data).Result;

            Assert.IsTrue(schemaRequest.Replace("\\", "").Contains(expectedResult));
        }

        [TestMethod]
        public void TestBuildGetSchemaRequestWorks()
        {
            var identifier = "Th7MpTaRZVRYnPiabds81Y";
            var data = "{\"name\":\"name\",\"version\":\"1.0\"}";

            var expectedResult = string.Format("\"identifier\":\"{0}\"," +
                    "\"operation\":{{" +
                    "\"type\":\"107\"," +
                    "\"dest\":\"{1}\"," +
                    "\"data\":{2}" +
                    "}}", identifier, identifier, data);

            var getSchemaRequest = Ledger.BuildGetSchemaRequestAsync(identifier, identifier, data).Result;

            Assert.IsTrue(getSchemaRequest.Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestSchemaRequestWorksWithoutSignature()
        {
            var didJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";
            var didResult = Signus.CreateAndStoreMyDidAsync(_wallet, didJson).Result;
            var did = didResult.Did;

            var schemaData = "{\"name\":\"gvt2\",\n" +
                    "             \"version\":\"2.0\",\n" +
                    "             \"keys\": [\"name\", \"male\"]}";

            var schemaRequest = Ledger.BuildSchemaRequestAsync(did, schemaData).Result;

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Ledger.SubmitRequestAsync(_pool, schemaRequest)
            );

            Assert.AreEqual(ErrorCode.LedgerInvalidTransaction, ex.ErrorCode);
        }

        [TestMethod]
        public void TestSchemaRequestWorks()
        {
            var didJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";
            var didResult = Signus.CreateAndStoreMyDidAsync(_wallet, didJson).Result;
            var did = didResult.Did;

            var schemaData = "{\"name\":\"gvt2\",\"version\":\"2.0\",\"keys\": [\"name\", \"male\"]}";

            var schemaRequest = Ledger.BuildSchemaRequestAsync(did, schemaData).Result;                
            Ledger.SignAndSubmitRequestAsync(_pool, _wallet, did, schemaRequest).Wait();

            var getSchemaData = "{\"name\":\"gvt2\",\"version\":\"2.0\"}";
            var getSchemaRequest = Ledger.BuildGetSchemaRequestAsync(did, did, getSchemaData).Result;
            var getSchemaResponse = Ledger.SubmitRequestAsync(_pool, getSchemaRequest).Result;

            var getSchemaResponseObject = JObject.Parse(getSchemaResponse);

            Assert.AreEqual("gvt2", (string)getSchemaResponseObject["result"]["data"]["name"]);
            Assert.AreEqual("2.0", (string)getSchemaResponseObject["result"]["data"]["version"]);
            Assert.AreEqual(did, (string)getSchemaResponseObject["result"]["data"]["origin"]);
        }

        [TestMethod]
        public void TestGetSchemaRequestsWorksForUnknownSchema()
        {
            var didJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";
            var didResult = Signus.CreateAndStoreMyDidAsync(_wallet, didJson).Result;
            var did = didResult.Did;

            var getSchemaData = "{\"name\":\"schema_name\",\"version\":\"2.0\"}";
            var getSchemaRequest = Ledger.BuildGetSchemaRequestAsync(did, did, getSchemaData).Result;
            var getSchemaResponse = Ledger.SubmitRequestAsync(_pool, getSchemaRequest).Result;

            var getSchemaResponseObject = JObject.Parse(getSchemaResponse);

            Assert.IsNotNull(getSchemaResponseObject["result"]["data"]);
        }

        [TestMethod]
        public async Task TestBuildSchemaRequestWorksForMissedFields()
        {
            var identifier = "Th7MpTaRZVRYnPiabds81Y";
            var data = "{\"name\":\"name\",\"version\":\"1.0\"}";

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Ledger.BuildSchemaRequestAsync(identifier, data)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

    }
}

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
    public class SchemaRequestTest : IndyIntegrationTestBase
    {
        private Pool _pool;
        private Wallet _wallet;
        private string _walletName = "ledgerWallet";
        private string _identifier = "Th7MpTaRZVRYnPiabds81Y";

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
            if(_pool != null)
                await _pool.CloseAsync();

            if(_wallet != null)
                await _wallet.CloseAsync();

            await Wallet.DeleteWalletAsync(_walletName, null);
        }

        [TestMethod]
        public async Task TestBuildSchemaRequestWorks()
        {
            var data = "{\"name\":\"name\",\"version\":\"1.0\",\"attr_names\":[\"name\",\"male\"]}";

            var expectedResult = string.Format("\"identifier\":\"{0}\"," +
                    "\"operation\":{{" +
                    "\"type\":\"101\"," +
                    "\"data\":{1}" +
                    "}}", _identifier, data);

            var schemaRequest = await Ledger.BuildSchemaRequestAsync(_identifier, data);

            Assert.IsTrue(schemaRequest.Replace("\\", "").Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestBuildGetSchemaRequestWorks()
        {
            var data = "{\"name\":\"name\",\"version\":\"1.0\"}";

            var expectedResult = string.Format("\"identifier\":\"{0}\"," +
                    "\"operation\":{{" +
                    "\"type\":\"107\"," +
                    "\"dest\":\"{1}\"," +
                    "\"data\":{2}" +
                    "}}", _identifier, _identifier, data);

            var getSchemaRequest = await Ledger.BuildGetSchemaRequestAsync(_identifier, _identifier, data);

            Assert.IsTrue(getSchemaRequest.Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestBuildSchemaRequestWorksWithoutSignature()
        {
            var didJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";
            var didResult = await Signus.CreateAndStoreMyDidAsync(_wallet, didJson);
            var did = didResult.Did;

            var schemaData = "{\"name\":\"gvt2\",\n" +
                    "             \"version\":\"2.0\",\n" +
                    "             \"attr_names\": [\"name\", \"male\"]}";

            var schemaRequest = await Ledger.BuildSchemaRequestAsync(did, schemaData);

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Ledger.SubmitRequestAsync(_pool, schemaRequest)
            );

            Assert.AreEqual(ErrorCode.LedgerInvalidTransaction, ex.ErrorCode);
        }

        [TestMethod] //Name of this test is bad.
        public async Task TestSchemaRequestWorks()
        {
            var didJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";
            var didResult = await Signus.CreateAndStoreMyDidAsync(_wallet, didJson);
            var did = didResult.Did;

            var schemaData = "{\"name\":\"gvt2\",\"version\":\"2.0\",\"attr_names\": [\"name\", \"male\"]}";

            var schemaRequest = await Ledger.BuildSchemaRequestAsync(did, schemaData);
            await Ledger.SignAndSubmitRequestAsync(_pool, _wallet, did, schemaRequest);

            var getSchemaData = "{\"name\":\"gvt2\",\"version\":\"2.0\"}";
            var getSchemaRequest = await Ledger.BuildGetSchemaRequestAsync(did, did, getSchemaData);
            var getSchemaResponse = await Ledger.SubmitRequestAsync(_pool, getSchemaRequest);

            var getSchemaResponseObject = JObject.Parse(getSchemaResponse);

            Assert.AreEqual("gvt2", (string)getSchemaResponseObject["result"]["data"]["name"]);
            Assert.AreEqual("2.0", (string)getSchemaResponseObject["result"]["data"]["version"]);           
        }

        [TestMethod]
        public async Task TestGetSchemaRequestsWorksForUnknownSchema()
        {
            var didJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";
            var didResult = await Signus.CreateAndStoreMyDidAsync(_wallet, didJson);
            var did = didResult.Did;

            var getSchemaData = "{\"name\":\"schema_name\",\"version\":\"2.0\"}";
            var getSchemaRequest = await Ledger.BuildGetSchemaRequestAsync(did, did, getSchemaData);
            var getSchemaResponse = await Ledger.SubmitRequestAsync(_pool, getSchemaRequest);

            var getSchemaResponseObject = JObject.Parse(getSchemaResponse);

            Assert.IsNotNull(getSchemaResponseObject["result"]["data"]);
        }

        [TestMethod]
        public async Task TestBuildSchemaRequestWorksForMissedFields()
        {
            var data = "{\"name\":\"name\",\"version\":\"1.0\"}";

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Ledger.BuildSchemaRequestAsync(_identifier, data)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

    }
}

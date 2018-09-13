using Hyperledger.Indy.AnonCredsApi;
using Hyperledger.Indy.Test.Util;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;


namespace Hyperledger.Indy.Test.AnonCredsTests
{
    [TestClass]
    public class IssuerCreateAndStoreClaimDefinitionTest : AnonCredsIntegrationTestBase
    {
        private const string WALLET_NAME = "createMasterSecretWallet";
        private const string WALLET_KEY = "issuerKey";
        private  Wallet _wallet;
        private const string _walletName = "createAndStoreClaimDefWallet";
        private const string _issuerDid = "NcYxiDXkpYi6ov5FcYDi1e";
        private const string _gvtSchemaJson = "{\n" +
                "                    \"seqNo\":1,\n" +
                "                    \"data\": {\n" +
                "                        \"name\":\"gvt\",\n" +
                "                        \"version\":\"1.0\",\n" +
                "                        \"attr_names\":[\"age\",\"sex\",\"height\",\"name\"]\n" +
                "                    }\n" +
                "                 }";

        [TestInitialize]
        public async Task CreateWallet()
        {
            await WalletUtils.CreateWallet(WALLET_NAME, WALLET_KEY);
            _wallet = await WalletUtils.OpenWallet(WALLET_NAME, WALLET_KEY);
        }

        [TestCleanup]
        public async Task DeleteWallet()
        {
            if(_wallet != null)
                await _wallet.CloseAsync();

            await WalletUtils.DeleteWallet(WALLET_NAME, WALLET_KEY);           
        }

        [TestMethod]
        public async Task TestIssuerCreateAndStoreClaimDefWorks()
        {
            var claimDef = ""; // TODO await AnonCreds.IssuerCreateAndStoreClaimDefAsync(_wallet, _issuerDid, _gvtSchemaJson, null, false);
            Assert.IsNotNull(claimDef);

            var claimDefObject = JObject.Parse(claimDef);
            var primary = claimDefObject["data"]["primary"];

            Assert.AreEqual(((JObject)primary["r"]).Count, 4);
            Assert.IsTrue(primary.Value<string>("n").Length > 0);
            Assert.IsTrue(primary.Value<string>("s").Length > 0);
            Assert.IsTrue(primary.Value<string>("z").Length > 0);
            Assert.IsTrue(primary.Value<string>("rms").Length > 0);
            Assert.IsTrue(primary.Value<string>("rctxt").Length > 0);
        }

        [TestMethod]
        public async Task TestIssuerCreateAndStoreClaimDefWorksForInvalidSchemaJson()
        {
            var schema = "{\"seqNo\":1, \"name\":\"name\",\"version\":\"1.0\", \"attr_names\":[\"name\"]}";

            // TODO 
            //var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
            //    AnonCreds.IssuerCreateAndStoreClaimDefAsync(_wallet, _issuerDid, schema, null, false)
            //);
        }

        [TestMethod]
        public async Task TestIssuerCreateAndStoreClaimDefWorksForEmptyKeys()
        {
            var schema = "{\n" +
                "                    \"seqNo\":1,\n" +
                "                    \"data\": {\n" +
                "                        \"name\":\"gvt\",\n" +
                "                        \"version\":\"1.0\",\n" +
                "                        \"attr_names\":[]\n" +
                "                    }\n" +
                "                 }";

            // TODO 
            //var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
            //    AnonCreds.IssuerCreateAndStoreClaimDefAsync(_wallet, _issuerDid, schema, null, false)
            //);
        }

        [TestMethod]
        public async Task TestIssuerCreateAndStoreClaimDefWorksForCorrectCryptoType()
        {
            var claimDef = ""; // TODO await AnonCreds.IssuerCreateAndStoreClaimDefAsync(_wallet, _issuerDid, _gvtSchemaJson, "CL", false);
            Assert.IsNotNull(claimDef);

            var claimDefObject = JObject.Parse(claimDef);
            var primary = claimDefObject["data"]["primary"];

            Assert.AreEqual(((JObject)primary["r"]).Count, 4);
            Assert.IsTrue(primary.Value<string>("n").Length > 0);
            Assert.IsTrue(primary.Value<string>("s").Length > 0);
            Assert.IsTrue(primary.Value<string>("z").Length > 0);
            Assert.IsTrue(primary.Value<string>("rms").Length > 0);
            Assert.IsTrue(primary.Value<string>("rctxt").Length > 0);
        }

        [TestMethod]
        public async Task TestIssuerCreateAndStoreClaimDefWorksForInvalidCryptoType()
        {
            // TODO 
            //var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
            //    AnonCreds.IssuerCreateAndStoreClaimDefAsync(_wallet, _issuerDid, _gvtSchemaJson, "type", false)
            //);
        }
    }
}

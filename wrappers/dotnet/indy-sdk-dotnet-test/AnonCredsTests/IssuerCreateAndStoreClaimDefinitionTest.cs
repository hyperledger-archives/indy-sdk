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
        private const string DEFAULT_SIGNATURE_TYPE = "CL";
        private const string WALLET_NAME = "createMasterSecretWallet";
        private const string WALLET_KEY = "issuerKey";
        private  Wallet _wallet;
        private const string _walletName = "createAndStoreClaimDefWallet";
        private const string _issuerDid = "NcYxiDXkpYi6ov5FcYDi1e";
        private const string _gvtSchemaJson = "{\"id\":\"id\",\"name\":\"gvt\",\"version\":\"1.0\",\"ver\":\"1.0\",\"attrNames\":[\"age\",\"sex\",\"height\",\"name\"]}";

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
        public async Task TestIssuerCreateAndStoreClaimDefWorksBob()
        {
            IssuerCreateAndStoreCredentialDefResult claimDefType = await AnonCreds.IssuerCreateAndStoreCredentialDefAsync(_wallet, _issuerDid, _gvtSchemaJson, DEFAULT_SIGNATURE_TYPE, null, null);
            Assert.IsNotNull(claimDefType);

            var claimDefObject = JObject.Parse(claimDefType.CredDefJson);
            var primary = claimDefObject["value"]["primary"];

            Assert.AreEqual(((JObject)primary["r"]).Count, 5, "length of primary.r didn't match");
            Assert.IsTrue(primary.Value<string>("n").Length > 0, "n had zero length");
            Assert.IsTrue(primary.Value<string>("s").Length > 0, "s had zero length");
            Assert.IsTrue(primary.Value<string>("z").Length > 0, "z had zero length");
            Assert.IsTrue(primary.Value<string>("rctxt").Length > 0, "rctxt had zero length");
        }

        [TestMethod]
        public async Task TestIssuerCreateAndStoreClaimDefWorksForInvalidSchemaJson()
        {
            var schema = "{\"seqNo\":1, \"name\":\"name\",\"version\":\"1.0\", \"attr_names\":[\"name\"]}";

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                AnonCreds.IssuerCreateAndStoreCredentialDefAsync(_wallet, _issuerDid, schema, DEFAULT_SIGNATURE_TYPE, null, null)
            );
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

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                AnonCreds.IssuerCreateAndStoreCredentialDefAsync(_wallet, _issuerDid, schema, DEFAULT_SIGNATURE_TYPE, null, null)
            );
        }

        [TestMethod]
        public async Task TestIssuerCreateAndStoreClaimDefWorksForCorrectCryptoType()
        {
            IssuerCreateAndStoreCredentialDefResult claimDefType = await AnonCreds.IssuerCreateAndStoreCredentialDefAsync(_wallet, _issuerDid, _gvtSchemaJson, DEFAULT_SIGNATURE_TYPE, null, null);
            Assert.IsNotNull(claimDefType);

            var claimDefObject = JObject.Parse(claimDefType.CredDefJson);
            var primary = claimDefObject["value"]["primary"];

            Assert.AreEqual(((JObject)primary["r"]).Count, 5, "length of primary.r didn't match");
            Assert.IsTrue(primary.Value<string>("n").Length > 0, "n had zero length");
            Assert.IsTrue(primary.Value<string>("s").Length > 0, "s had zero length");
            Assert.IsTrue(primary.Value<string>("z").Length > 0, "z had zero length");
            Assert.IsTrue(primary.Value<string>("rctxt").Length > 0, "rctxt had zero length");

        }

        [TestMethod]
        public async Task TestIssuerCreateAndStoreClaimDefWorksForInvalidCryptoType()
        {
            var ex = await Assert.ThrowsExceptionAsync<InvalidParameterException>(() =>
                AnonCreds.IssuerCreateAndStoreCredentialDefAsync(_wallet, _issuerDid, _gvtSchemaJson, null, "GG", null)
            );
        }
    }
}

using Hyperledger.Indy.AnonCredsApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.WalletTests
{
    [TestClass]
    public class RegisterWalletTypeTest : IndyIntegrationTestBase
    {
        private const string _type = "inmem";               

        [TestMethod]
        [Ignore] //Not a valid test since the wallet type is registered on init!
        public async Task TestRegisterWalletTypeWorks()
        {
            await Wallet.RegisterWalletTypeAsync(_type, new InMemWalletType());
        }

        [TestMethod]
        public async Task TestRegisterWalletTypeDoesNotWorkForTwiceWithSameName()
        {
            var ex = await Assert.ThrowsExceptionAsync<DuplicateWalletTypeException>(() =>
                Wallet.RegisterWalletTypeAsync(_type, new InMemWalletType())
            );
        }

        [TestMethod]
        public async Task TestExerciseCustomWallet()
        {
            StorageUtils.CleanupStorage();

            var walletName = "exerciseWalletTypeWorks";

            await Wallet.CreateWalletAsync(POOL, walletName, _type, null, null);

            var wallet = await Wallet.OpenWalletAsync(walletName, null, null);
            Assert.IsNotNull(wallet);

            var schema = "{\"seqNo\":1,\"dest\":\"{}\",\"data\": {\"name\":\"gvt\",\"version\":\"1.0\",\"attr_names\":[\"age\",\"sex\",\"height\",\"name\"]}}";
            var claimDef = await AnonCreds.IssuerCreateAndStoreClaimDefAsync(wallet, DID1, schema, null, false);

            var claimOfferTemplate = "{{\"issuer_did\":\"{0}\",\"schema_seq_no\":{1}}}";

            await AnonCreds.ProverStoreClaimOfferAsync(wallet, string.Format(claimOfferTemplate, DID1, 1));
            await AnonCreds.ProverStoreClaimOfferAsync(wallet, string.Format(claimOfferTemplate, DID1, 2));

            var issuerDid2 = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
            await AnonCreds.ProverStoreClaimOfferAsync(wallet, string.Format(claimOfferTemplate, issuerDid2, 2));

            string masterSecretName = "master_secret_name";
            await AnonCreds.ProverCreateMasterSecretAsync(wallet, masterSecretName);

            var claimOffer = string.Format("{{\"issuer_did\":\"{0}\",\"schema_seq_no\":{1}}}", DID1, 1);

            var claimRequest = await AnonCreds.ProverCreateAndStoreClaimReqAsync(wallet, "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW", claimOffer, claimDef, masterSecretName);

            var claim = "{\"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
                    "                 \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
                    "                 \"height\":[\"175\",\"175\"],\n" +
                    "                 \"age\":[\"28\",\"28\"]\n" +
                    "        }";

            var createClaimResult = await AnonCreds.IssuerCreateClaimAsync(wallet, claimRequest, claim, -1);
            var claimJson = createClaimResult.ClaimJson;

            await AnonCreds.ProverStoreClaimAsync(wallet, claimJson, createClaimResult.RevocRegUpdateJson);

            var filter = string.Format("{{\"issuer_did\":\"{0}\"}}", DID1);

            var claims = await AnonCreds.ProverGetClaimsAsync(wallet, filter);

            var claimsArray = JArray.Parse(claims);

            Assert.AreEqual(1, claimsArray.Count);

            await wallet.CloseAsync();
        }
    }
}

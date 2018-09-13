using Hyperledger.Indy.AnonCredsApi;
using Hyperledger.Indy.Test.Util;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AnonCredsTests
{
    [TestClass]
    public class ProverStoreClaimTest : AnonCredsIntegrationTestBase
    {
        private const string WALLET_NAME = "proverWallet";
        private const string WALLET_KEY = "proverWalletKEy";

        [TestMethod]
        public async Task TestProverStoreClaimWorks()
        {
            await InitCommonWallet();

            await WalletUtils.CreateWallet(WALLET_NAME, WALLET_KEY);
            var proverWallet = await WalletUtils.OpenWallet(WALLET_NAME, WALLET_KEY);

            await AnonCreds.ProverCreateMasterSecretAsync(proverWallet, masterSecretName);
            
            var claimOffer = string.Format(claimOfferTemplate, issuerDid, 1);

            var claimRequest = await AnonCreds.ProverCreateCredentialReqAsync(proverWallet, proverDid, claimOffer, claimDef, masterSecretName);

            var claim = "{\"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
                    "                 \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
                    "                 \"height\":[\"175\",\"175\"],\n" +
                    "                 \"age\":[\"28\",\"28\"]\n" +
                    "        }";

            // TODO var createClaimResult = await AnonCreds.IssuerCreateCredentialAsync(commonWallet, claimRequest, claim, -1);
            // var claimJson = createClaimResult.ClaimJson;

            // TODO await AnonCreds.ProverStoreClaimAsync(proverWallet, claimJson, createClaimResult.RevocRegUpdateJson);

            await proverWallet.CloseAsync();
            await WalletUtils.DeleteWallet(WALLET_NAME, WALLET_KEY);
        }

        [TestMethod]
        public async Task TestProverStoreClaimWorksWithoutClaim()
        {
            await InitCommonWallet();

            var claimJson = string.Format("{{\"claim\":{{\"sex\":[\"male\",\"1\"],\"age\":[\"28\",\"28\"],\"name\":[\"Alex\",\"1\"],\"height\":[\"175\",\"175\"]}},\n" +
                "                          \"issuer_did\":\"{0}\",\n" +
                "                          \"revoc_reg_seq_no\":null,\n" +
                "                          \"schema_seq_no\":2,\n" +
                "                          \"signature\":{{\"primary_claim\":{{\"m2\":\"1\",\"a\":\"1\",\"e\":\"2\",\"v\":\"3\"}}," +
                "                          \"non_revocation_claim\":null}}}}", issuerDid2);

            // TODO
            //var ex = await Assert.ThrowsExceptionAsync<WalletValueNotFoundException>(() =>
            //    AnonCreds.ProverStoreClaimAsync(commonWallet, claimJson, string.Empty)
            //);

        }

        [TestMethod]
        public async Task TestProverStoreClaimWorksForInvalidClaimJson()
        {
            await InitCommonWallet();

            var claimOffer = string.Format(claimOfferTemplate, issuerDid, 1);

            await AnonCreds.ProverCreateCredentialReqAsync(commonWallet, proverDid, claimOffer, claimDef, masterSecretName);

            var claimJson = "{\"claim\":{\"sex\":[\"male\",\"1\"],\"age\":[\"28\",\"28\"],\"name\":[\"Alex\",\"1\"],\"height\":[\"175\",\"175\"]},\n" +
                    "            \"issuer_did\":1,\"\n" +
                    "            \"revoc_reg_seq_no\":null,\n" +
                    "            \"schema_seq_no\":1}";

            // TODO
            //var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
            //    AnonCreds.ProverStoreClaimAsync(commonWallet, claimJson, string.Empty)
            //);
        }
    }
}

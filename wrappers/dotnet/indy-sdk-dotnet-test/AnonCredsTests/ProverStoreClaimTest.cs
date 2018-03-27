using Hyperledger.Indy.AnonCredsApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AnonCredsTests
{
    [TestClass]
    public class ProverStoreClaimTest : AnonCredsIntegrationTestBase
    {
        [TestMethod]
        public async Task TestProverStoreClaimWorks()
        {
            await InitCommonWallet();

            var proverWalletName = "proverWallet";
            await Wallet.CreateWalletAsync("default", proverWalletName, "default", null, null);
            var proverWallet = await Wallet.OpenWalletAsync(proverWalletName, null, null);

            await AnonCreds.ProverCreateMasterSecretAsync(proverWallet, masterSecretName);
            
            var claimOffer = string.Format(claimOfferTemplate, issuerDid, 1);

            var claimRequest = await AnonCreds.ProverCreateAndStoreClaimReqAsync(proverWallet, proverDid, claimOffer, claimDef, masterSecretName);

            var claim = "{\"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
                    "                 \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
                    "                 \"height\":[\"175\",\"175\"],\n" +
                    "                 \"age\":[\"28\",\"28\"]\n" +
                    "        }";

            var createClaimResult = await AnonCreds.IssuerCreateClaimAsync(commonWallet, claimRequest, claim, -1);
            var claimJson = createClaimResult.ClaimJson;

            await AnonCreds.ProverStoreClaimAsync(proverWallet, claimJson, createClaimResult.RevocRegUpdateJson);

            await proverWallet.CloseAsync();
            await Wallet.DeleteWalletAsync(proverWalletName, null);
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

            var ex = await Assert.ThrowsExceptionAsync<WalletValueNotFoundException>(() =>
                AnonCreds.ProverStoreClaimAsync(commonWallet, claimJson, string.Empty)
            );

        }

        [TestMethod]
        public async Task TestProverStoreClaimWorksForInvalidClaimJson()
        {
            await InitCommonWallet();

            var claimOffer = string.Format(claimOfferTemplate, issuerDid, 1);

            await AnonCreds.ProverCreateAndStoreClaimReqAsync(commonWallet, proverDid, claimOffer, claimDef, masterSecretName);

            var claimJson = "{\"claim\":{\"sex\":[\"male\",\"1\"],\"age\":[\"28\",\"28\"],\"name\":[\"Alex\",\"1\"],\"height\":[\"175\",\"175\"]},\n" +
                    "            \"issuer_did\":1,\"\n" +
                    "            \"revoc_reg_seq_no\":null,\n" +
                    "            \"schema_seq_no\":1}";

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                AnonCreds.ProverStoreClaimAsync(commonWallet, claimJson, string.Empty)
            );
        }
    }
}

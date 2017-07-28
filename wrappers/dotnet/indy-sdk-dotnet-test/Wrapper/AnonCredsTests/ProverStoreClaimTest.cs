using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.Wrapper.Agent;
using static Indy.Sdk.Dotnet.Wrapper.AgentObservers;

namespace Indy.Sdk.Dotnet.Test.Wrapper.AnonCredsTests
{
    [TestClass]
    public class ProverStoreClaimTest : AnonCredsIntegrationTestBase
    {
        [ClassCleanup]
        public static void CloseCommonWallet()
        {
            try
            {
                _commonWallet.CloseAsync().Wait();
            }
            catch (Exception)
            { }

        }

        [TestMethod]
        public void TestProverStoreClaimWorks()
        {
            InitCommonWallet();

            var proverWalletName = "proverWallet";
            Wallet.CreateWalletAsync("default", proverWalletName, "default", null, null).Wait();
            var proverWallet = Wallet.OpenWalletAsync(proverWalletName, null, null).Result;

            AnonCreds.ProverCreateMasterSecretAsync(proverWallet, _masterSecretName).Wait();
            
            var claimOffer = string.Format(_claimOfferTemplate, _issuerDid, 1);

            var claimRequest = AnonCreds.ProverCreateAndStoreClaimReqAsync(proverWallet, _proverDid, claimOffer, _claimDef, _masterSecretName).Result;

            var claim = "{\"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
                    "                 \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
                    "                 \"height\":[\"175\",\"175\"],\n" +
                    "                 \"age\":[\"28\",\"28\"]\n" +
                    "        }";

            var createClaimResult = AnonCreds.IssuerCreateClaimAsync(_commonWallet, claimRequest, claim, -1, -1).Result;
            var claimJson = createClaimResult.ClaimJson;

            AnonCreds.ProverStoreClaimAsync(proverWallet, claimJson).Wait();

            proverWallet.CloseAsync().Wait();
            Wallet.DeleteWalletAsync(proverWalletName, null).Wait();
        }

        [TestMethod]
        public async Task TestProverStoreClaimWorksWithoutClaim()
        {
            InitCommonWallet();

            var claimJson = string.Format("{{\"claim\":{{\"sex\":[\"male\",\"1\"],\"age\":[\"28\",\"28\"],\"name\":[\"Alex\",\"1\"],\"height\":[\"175\",\"175\"]}},\n" +
                "                          \"issuer_did\":\"{0}\",\n" +
                "                          \"revoc_reg_seq_no\":null,\n" +
                "                          \"schema_seq_no\":2,\n" +
                "                          \"signature\":{{\"primary_claim\":{{\"m2\":\"1\",\"a\":\"1\",\"e\":\"2\",\"v\":\"3\"}}," +
                "                          \"non_revocation_claim\":null}}}}", _issuerDid2);

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                AnonCreds.ProverStoreClaimAsync(_commonWallet, claimJson)
            );

            Assert.AreEqual(ErrorCode.WalletNotFoundError, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestProverStoreClaimWorksForInvalidClaimJson()
        {
            InitCommonWallet();

            var claimOffer = string.Format(_claimOfferTemplate, _issuerDid, 1);

            AnonCreds.ProverCreateAndStoreClaimReqAsync(_commonWallet, _proverDid, claimOffer, _claimDef, _masterSecretName).Wait();

            var claimJson = "{\"claim\":{\"sex\":[\"male\",\"1\"],\"age\":[\"28\",\"28\"],\"name\":[\"Alex\",\"1\"],\"height\":[\"175\",\"175\"]},\n" +
                    "            \"issuer_did\":1,\"\n" +
                    "            \"revoc_reg_seq_no\":null,\n" +
                    "            \"schema_seq_no\":1}";

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                AnonCreds.ProverStoreClaimAsync(_commonWallet, claimJson)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }
    }
}

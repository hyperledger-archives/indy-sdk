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
    public class ProverCreateMasterSecretTest : AnonCredsIntegrationTestBase
    {
        private Wallet _wallet;
        private string _walletName = "createMasterSecretWallet";
        

        [TestInitialize]
        public void CreateWallet()
        {
            Wallet.CreateWalletAsync("default", _walletName, "default", null, null).Wait();
            _wallet = Wallet.OpenWalletAsync(_walletName, null, null).Result;
        }

        [TestCleanup]
        public void DeleteWallet()
        {
            try
            {
                _wallet.CloseAsync().Wait();
                Wallet.DeleteWalletAsync(_walletName, null).Wait();
            }
            catch (Exception)
            { }
        }

        [TestMethod]
        public void TestProverCreateMasterSecretWorks()
        {
            AnonCreds.ProverCreateMasterSecretAsync(_wallet, "master_secret_name").Wait();
        }

        [TestMethod] //Does this test do what it's supposed to?
        public void TestProverCreateMasterSecretWorksForDuplicate()
        {
            AnonCreds.ProverCreateMasterSecretAsync(_wallet, "master_secret_name").Wait();
        }

        [TestMethod]
        public async Task TestProverStoreClaimOfferWorksForInvalidIssuerDid()
        {
            var claimOffer = "{\"issuer_did\":\"invalid_base58_string\",\"schema_seq_no\":1}";

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                AnonCreds.ProverStoreClaimOfferAsync(_wallet, claimOffer)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }
        
    }
}

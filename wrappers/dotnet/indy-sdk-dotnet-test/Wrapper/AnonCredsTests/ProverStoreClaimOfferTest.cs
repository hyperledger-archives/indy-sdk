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
    public class ProverStoreClaimOfferTest : AnonCredsIntegrationTestBase
    {
        private Wallet _wallet;
        private string _walletName = "storeClaimOfferWallet";
        
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
        public void TestProverStoreClaimOfferWorks()
        {
            var claimOffer = "{\"issuer_did\":\"NcYxiDXkpYi6ov5FcYDi1e\",\"schema_seq_no\":1 }";

            AnonCreds.ProverStoreClaimOfferAsync(_wallet, claimOffer).Wait();
        }


        [TestMethod]
        public async Task TestProverStoreClaimOfferWorksForInvalidJson()
        {
            var claimOffer = "{\"issuer_did\":\"NcYxiDXkpYi6ov5FcYDi1e\"}";

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                AnonCreds.ProverStoreClaimOfferAsync(_wallet, claimOffer)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

        [TestMethod]
        public async Task testProverStoreClaimOfferWorksForInvalidIssuerDid()
        {
            var claimOffer = "{\"issuer_did\":\"invalid_base58_string\",\"schema_seq_no\":1}";

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                AnonCreds.ProverStoreClaimOfferAsync(_wallet, claimOffer)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

    }
}

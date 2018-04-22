using Hyperledger.Indy.AnonCredsApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AnonCredsTests
{
    [TestClass]
    public class ProverStoreClaimOfferTest : AnonCredsIntegrationTestBase
    {
        private Wallet _wallet;
        private string _walletName = "storeClaimOfferWallet";
        
        [TestInitialize]
        public async Task CreateWallet()
        {
            await Wallet.CreateWalletAsync("default", _walletName, "default", null, null);
            _wallet = await Wallet.OpenWalletAsync(_walletName, null, null);
        }

        [TestCleanup]
        public async Task DeleteWallet()
        {
            if(_wallet != null)
                await _wallet.CloseAsync();

            await Wallet.DeleteWalletAsync(_walletName, null);
        }

       
        [TestMethod]
        public async Task TestProverStoreClaimOfferWorks()
        {
            var claimOffer = "{\"issuer_did\":\"NcYxiDXkpYi6ov5FcYDi1e\",\"schema_seq_no\":1 }";

            await AnonCreds.ProverStoreCredentialOfferAsync(_wallet, claimOffer);
        }


        [TestMethod]
        public async Task TestProverStoreClaimOfferWorksForInvalidJson()
        {
            var claimOffer = "{\"issuer_did\":\"NcYxiDXkpYi6ov5FcYDi1e\"}";

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                AnonCreds.ProverStoreCredentialOfferAsync(_wallet, claimOffer)
            );
        }

        [TestMethod]
        public async Task TestProverStoreClaimOfferWorksForInvalidIssuerDid()
        {
            var claimOffer = "{\"issuer_did\":\"invalid_base58_string\",\"schema_seq_no\":1}";

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                AnonCreds.ProverStoreCredentialOfferAsync(_wallet, claimOffer)
            );
        }

    }
}

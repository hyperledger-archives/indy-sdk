using Hyperledger.Indy.AnonCredsApi;
using Hyperledger.Indy.Test.Util;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AnonCredsTests
{
    [TestClass]
    public class ProverStoreClaimOfferTest : AnonCredsIntegrationTestBase
    {
        private const string WALLET_NAME = "commonWallet";
        private const string WALLET_KEY = "commonWalletKey";
        private Wallet _wallet;
        private string _walletName = "storeClaimOfferWallet";
        
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
        public async Task TestProverStoreClaimOfferWorks()
        {
            var claimOffer = "{\"issuer_did\":\"NcYxiDXkpYi6ov5FcYDi1e\",\"schema_seq_no\":1 }";

            // TODO await AnonCreds.ProverStoreCredentialOfferAsync(_wallet, claimOffer);
        }


        [TestMethod]
        public async Task TestProverStoreClaimOfferWorksForInvalidJson()
        {
            var claimOffer = "{\"issuer_did\":\"NcYxiDXkpYi6ov5FcYDi1e\"}";

            // TODO
            //var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
            //    AnonCreds.ProverStoreCredentialOfferAsync(_wallet, claimOffer)
            //);
        }

        [TestMethod]
        public async Task TestProverStoreClaimOfferWorksForInvalidIssuerDid()
        {
            var claimOffer = "{\"issuer_did\":\"invalid_base58_string\",\"schema_seq_no\":1}";

            // TODO
            //var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
            //    AnonCreds.ProverStoreCredentialOfferAsync(_wallet, claimOffer)
            //);
        }

    }
}

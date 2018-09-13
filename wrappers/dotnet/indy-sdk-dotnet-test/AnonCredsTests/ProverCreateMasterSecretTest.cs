using Hyperledger.Indy.AnonCredsApi;
using Hyperledger.Indy.Test.Util;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AnonCredsTests
{
    [TestClass]
    public class ProverCreateMasterSecretTest : AnonCredsIntegrationTestBase
    {
        private const string WALLET_NAME = "createMasterSecretWallet";
        private const string WALLET_KEY = "issuerKey";

        private Wallet _wallet;

        [TestInitialize]
        public async Task CreateWallet()
        {
            await WalletUtils.CreateWallet(WALLET_NAME, WALLET_KEY);
            _wallet = await WalletUtils.OpenWallet(WALLET_NAME, WALLET_KEY);
        }

        [TestCleanup]
        public async Task DeleteWallet()
        {
            if (_wallet != null)
                await _wallet.CloseAsync();

            await WalletUtils.DeleteWallet(WALLET_NAME, WALLET_KEY);
        }

        [TestMethod]
        public async Task TestProverCreateMasterSecretWorks()
        {
            await AnonCreds.ProverCreateMasterSecretAsync(_wallet, "master_secret_name");
        }

        [TestMethod] 
        public async Task TestProverCreateMasterSecretWorksForDuplicate()
        {
            await AnonCreds.ProverCreateMasterSecretAsync(_wallet, "master_secret_name_duplicate");

            var ex = await Assert.ThrowsExceptionAsync<DuplicateMasterSecretNameException>(() =>
               AnonCreds.ProverCreateMasterSecretAsync(_wallet, "master_secret_name_duplicate")
           );
        }
    }
}

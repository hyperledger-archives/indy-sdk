using Hyperledger.Indy.AnonCredsApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;


namespace Hyperledger.Indy.Test.AnonCredsTests
{
    [TestClass]
    public class ProverCreateMasterSecretTest : AnonCredsIntegrationTestBase
    {
        private Wallet _wallet;
        private string _walletName = "createMasterSecretWallet";
        

        [TestInitialize]
        public async Task CreateWallet()
        {
            await Wallet.CreateWalletAsync("default", _walletName, "default", null, null);
            _wallet = await Wallet.OpenWalletAsync(_walletName, null, null);
        }

        [TestCleanup]
        public async Task DeleteWallet()
        {
            if (_wallet != null)
                await _wallet.CloseAsync();

            await Wallet.DeleteWalletAsync(_walletName, null);            
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

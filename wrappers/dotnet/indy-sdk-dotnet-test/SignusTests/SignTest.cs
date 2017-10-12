using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.SignusTests
{
    [TestClass]
    public class SignTest : IndyIntegrationTestBase
    {
        private Wallet _wallet;
        private string _walletName = "SignusWallet";

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
        public async Task TestSignWorks()
        {
            var result = await Signus.CreateAndStoreMyDidAsync(_wallet, "{\"seed\":\"000000000000000000000000Trustee1\"}");
            var did = result.Did;
            
            var msg = Encoding.UTF8.GetBytes("{\"reqId\":1496822211362017764}");

            var expectedSignatureBytes = new byte[] { 20, 191, 100, 213, 101, 12, 197, 198, 203, 49, 89, 220, 205, 192, 224, 221, 97, 77, 220, 190, 90, 60, 142, 23, 16, 240, 189, 129, 45, 148, 245, 8, 102, 95, 95, 249, 100, 89, 41, 227, 213, 25, 100, 1, 232, 188, 245, 235, 186, 21, 52, 176, 236, 11, 99, 70, 155, 159, 89, 215, 197, 239, 138, 5 };
            var signatureBytes = await Signus.SignAsync(_wallet, did, msg);      

            Assert.IsTrue(expectedSignatureBytes.SequenceEqual(signatureBytes));
        }

        [TestMethod]
        public async Task TestSignWorksForUnknownDid()
        {
            var msg = Encoding.UTF8.GetBytes("{\"reqId\":1496822211362017764}");

            var ex = await Assert.ThrowsExceptionAsync<WalletValueNotFoundException>(() =>
                Signus.SignAsync(_wallet, "8wZcEriaNLNKtteJvx7f8i", msg)
            );
        }       
    }
}

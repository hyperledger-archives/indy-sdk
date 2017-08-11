using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.SignusTests
{
    [TestClass]
    public class SignTest : IndyIntegrationTestBase
    {
        private Wallet _wallet;
        private string _walletName = "signusWallet";

        [TestInitialize]
        public async Task CreateWallet()
        {
            await Wallet.CreateWalletAsync("default", _walletName, "default", null, null);
            _wallet = await Wallet.OpenWalletAsync(_walletName, null, null);
        }

        [TestCleanup]
        public async Task DeleteWallet()
        {
            await _wallet.CloseAsync();
            await Wallet.DeleteWalletAsync(_walletName, null);
        }
        
        [TestMethod]
        public async Task TestSignWorks()
        {
            var result = await Signus.CreateAndStoreMyDidAsync(_wallet, "{\"seed\":\"000000000000000000000000Trustee1\"}");
            var did = result.Did;
            
            var msg = "{\"reqId\":1496822211362017764}";
            
            var expectedSignature = "R4Rj68n4HZosQqEc3oMUbQh7MtG8tH7WmXE2Mok8trHJ67CrzyqahZn5ziJy4nebRtq6Qi6fVH9JkvVCM85XjFa";
            var signedMessage = await Signus.SignAsync(_wallet, did, msg);      

            Assert.AreEqual(expectedSignature, signedMessage);
        }

        [TestMethod]
        public async Task TestSignWorksForUnknownDid()
        {
            var msg = "{\"reqId\":1496822211362017764}";

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Signus.SignAsync(_wallet, "8wZcEriaNLNKtteJvx7f8i", msg)
            );

            Assert.AreEqual(ErrorCode.WalletNotFoundError, ex.ErrorCode);
        }       
    }
}

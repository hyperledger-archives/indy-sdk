using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.WalletTests
{
    [TestClass]
    public class RegisterWalletTypeTest : IndyIntegrationTestBase
    {
        [TestMethod]
        public void TestOpenWalletWorks()
        {
            Wallet.RegisterWalletTypeAsync("inmem", new InMemWalletType()).Wait();

            Wallet.CreateWalletAsync("default", "registerWalletTypeWorks", "inmem", null, null).Wait();

            var wallet = Wallet.OpenWalletAsync("registerWalletTypeWorks", null, null).Result;
            Assert.IsNotNull(wallet);

            var createAndStoreMyDidResult = Signus.CreateAndStoreMyDidAsync(wallet, "{}").Result;
            
            wallet.CloseAsync().Wait();
        }

        
    }
}

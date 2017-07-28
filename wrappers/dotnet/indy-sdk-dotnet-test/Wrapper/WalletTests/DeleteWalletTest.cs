using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.WalletTests
{
    [TestClass]
    public class DeleteWalletTest : IndyIntegrationTestBase
    {
        [TestMethod]
        public void TestDeleteWalletWorks()
        {
            var poolName = "default";
            var walletName = "deleteWalletWorks";
            var type = "default";

            Wallet.CreateWalletAsync(poolName, walletName, type, null, null).Wait();
            Wallet.DeleteWalletAsync(walletName, null).Wait();
            Wallet.CreateWalletAsync(poolName, walletName, type, null, null).Wait();
        }

        [TestMethod]
        public void TestDeleteWalletWorksForClosed()
        {
            var poolName = "default";
            var walletName = "deleteWalletWorks";

            Wallet.CreateWalletAsync(poolName, walletName, null, null, null).Wait();

            var wallet = Wallet.OpenWalletAsync(walletName, null, null).Result;
            Assert.IsNotNull(wallet);

            wallet.CloseAsync().Wait();
            Wallet.DeleteWalletAsync(walletName, null).Wait();
            Wallet.CreateWalletAsync(poolName, walletName, null, null, null).Wait();
        }

        [TestMethod]
        [Ignore] //Bug in Indy
        public async Task TestDeleteWalletWorksForOpened()
        {
            var walletName = "deleteWalletWorksForOpened";

            Wallet.CreateWalletAsync("default", walletName, null, null, null).Wait();
            var wallet = Wallet.OpenWalletAsync(walletName, null, null).Result;

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Wallet.DeleteWalletAsync(walletName, null)
            );

            Assert.AreEqual(ErrorCode.CommonIOError, ex.ErrorCode);            
        }

        [TestMethod]
        public async Task TestDeleteWalletWorksForTwice()
        {
            var walletName = "deleteWalletWorksForTwice";


            Wallet.CreateWalletAsync("default", walletName, null, null, null).Wait();

            var wallet = Wallet.OpenWalletAsync(walletName, null, null).Result;

            Assert.IsNotNull(wallet);

            wallet.CloseAsync().Wait();

            Wallet.DeleteWalletAsync(walletName, null).Wait();

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                 Wallet.DeleteWalletAsync(walletName, null)
            );

            Assert.AreEqual(ErrorCode.CommonIOError, ex.ErrorCode);
        
        }

        [TestMethod]
        public async Task TestDeleteWalletWorksForNotCreated()
        {
            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Wallet.DeleteWalletAsync("DeleteWalletAsyncWorksForNotCreated", null)
            );

            Assert.AreEqual(ErrorCode.CommonIOError, ex.ErrorCode);
        }

        [TestMethod]
        public void TestDeleteWalletWorksForPlugged()
        {
            var type = "inmem";
            var poolName = "default";
            var walletName = "wallet";

            Wallet.RegisterWalletTypeAsync(type, new InMemWalletType(), false).Wait();
            Wallet.CreateWalletAsync(poolName, walletName, type, null, null).Wait();
            Wallet.DeleteWalletAsync(walletName, null).Wait();
            Wallet.CreateWalletAsync(poolName, walletName, type, null, null).Wait();
        }
    }
}

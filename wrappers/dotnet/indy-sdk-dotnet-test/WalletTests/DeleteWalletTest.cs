using Hyperledger.Indy.Test.Util;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.WalletTests
{
    [TestClass]
    public class DeleteWalletTest : IndyIntegrationTestBase
    {
        string config = WalletUtils.GetCreateWalletConfig(WALLET);
        string cred = WalletUtils.GetOpenWalletCredentials(WALLET_KEY);

        [TestMethod]
        public async Task TestDeleteWalletWorks()
        {
            await Wallet.CreateWalletAsync(config, cred);
            await Wallet.DeleteWalletAsync(config, cred);
            await Wallet.CreateWalletAsync(config, cred);
            await Wallet.DeleteWalletAsync(config, cred);
        }

        [TestMethod]
        public async Task TestDeleteWalletWorksForClosed()
        {
            await Wallet.CreateWalletAsync(config, cred);

            var wallet = await Wallet.OpenWalletAsync(config, cred);
            Assert.IsNotNull(wallet);

            await wallet.CloseAsync();
            await Wallet.DeleteWalletAsync(config, cred);
            await Wallet.CreateWalletAsync(config, cred);
            await Wallet.DeleteWalletAsync(config, cred);
        }

        [TestMethod]
        public async Task TestDeleteWalletSucceedsForOpened()
        {
            await Wallet.CreateWalletAsync(config, cred);
            var wallet = await Wallet.OpenWalletAsync(config, cred);

            // delete wallet will not throw any exceptions if the wallet is open
            // it essentially does nothing
            Wallet.DeleteWalletAsync(config, cred);

            // safely clean up
            try
            {
                await wallet.CloseAsync();
                await Wallet.DeleteWalletAsync(config, cred);
            }
            catch 
            {
                // don't allow exceptions on cleanup to fail test
            }
        }

        [TestMethod]
        public async Task TestDeleteWalletThrowsExceptionOnSecondDelete()
        {
            await Wallet.CreateWalletAsync(config, cred);

            var wallet = await Wallet.OpenWalletAsync(config, cred);
            await wallet.CloseAsync();

            await Wallet.DeleteWalletAsync(config, cred);

            var ex = await Assert.ThrowsExceptionAsync<WalletValueNotFoundException>(() =>
                 Wallet.DeleteWalletAsync(config, cred)
            );        
        }

        [TestMethod]
        public async Task TestDeleteWalletThrowsExceptionNotCreatedWallet()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletValueNotFoundException>(() =>
                Wallet.DeleteWalletAsync(config, cred)
            );
        }

        //[TestMethod]
        //public async Task TestDeleteWalletWorksForPlugged()
        //{
        //    var walletName = "pluggedWalletDelete";

        //    await Wallet.CreateWalletAsync(POOL, walletName, "inmem", null, null);
        //    await Wallet.DeleteWalletAsync(walletName, null);
        //    await Wallet.CreateWalletAsync(POOL, walletName, "inmem", null, null);
        //}
    }
}

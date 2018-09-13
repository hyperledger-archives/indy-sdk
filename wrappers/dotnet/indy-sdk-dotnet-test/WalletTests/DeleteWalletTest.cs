using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.WalletTests
{
    [TestClass]
    public class DeleteWalletTest : IndyIntegrationTestBase
    {
        // TODO


        //[TestMethod]
        //public async Task TestDeleteWalletWorks()
        //{
        //    await Wallet.CreateWalletAsync(POOL, WALLET, TYPE, null, null);
        //    await Wallet.DeleteWalletAsync(WALLET, null);
        //    await Wallet.CreateWalletAsync(POOL, WALLET, TYPE, null, null);
        //    await Wallet.DeleteWalletAsync(WALLET, null);
        //}

        //[TestMethod]
        //public async Task TestDeleteWalletWorksForClosed()
        //{
        //    await Wallet.CreateWalletAsync(POOL, WALLET, null, null, null);

        //    var wallet = await Wallet.OpenWalletAsync(WALLET, null, null);
        //    Assert.IsNotNull(wallet);

        //    await wallet.CloseAsync();
        //    await Wallet.DeleteWalletAsync(WALLET, null);
        //    await Wallet.CreateWalletAsync(POOL, WALLET, null, null, null);
        //    await Wallet.DeleteWalletAsync(WALLET, null);
        //}

        //[TestMethod]
        //[Ignore] //TODO: Remove ignore when bug in Indy fixed.
        //public async Task TestDeleteWalletWorksForOpened()
        //{
        //    await Wallet.CreateWalletAsync(POOL, WALLET, null, null, null);
        //    var wallet = await Wallet.OpenWalletAsync(WALLET, null, null);

        //    var ex = await Assert.ThrowsExceptionAsync<IOException>(() =>
        //        Wallet.DeleteWalletAsync(WALLET, null)
        //    );           
        //}

        //[TestMethod]
        //public async Task TestDeleteWalletWorksForTwice()
        //{
        //    await Wallet.CreateWalletAsync(POOL, WALLET, null, null, null);

        //    var wallet = await Wallet.OpenWalletAsync(WALLET, null, null);
        //    await wallet.CloseAsync();

        //    await Wallet.DeleteWalletAsync(WALLET, null);

        //    var ex = await Assert.ThrowsExceptionAsync<IOException>(() =>
        //         Wallet.DeleteWalletAsync(WALLET, null)
        //    );        
        //}

        //[TestMethod]
        //public async Task TestDeleteWalletWorksForNotCreated()
        //{
        //    var ex = await Assert.ThrowsExceptionAsync<IOException>(() =>
        //        Wallet.DeleteWalletAsync(WALLET, null)
        //    );
        //}

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

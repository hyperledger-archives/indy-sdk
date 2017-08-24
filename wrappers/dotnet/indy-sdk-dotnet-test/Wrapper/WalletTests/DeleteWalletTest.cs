﻿using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.WalletTests
{
    [TestClass]
    public class DeleteWalletTest : IndyIntegrationTestBase
    {
        [TestMethod]
        public async Task TestDeleteWalletWorks()
        {
            var poolName = "default";
            var walletName = "deleteWalletWorks";
            var type = "default";

            await Wallet.CreateWalletAsync(poolName, walletName, type, null, null);
            await Wallet.DeleteWalletAsync(walletName, null);
            await Wallet.CreateWalletAsync(poolName, walletName, type, null, null);
        }

        [TestMethod]
        public async Task TestDeleteWalletWorksForClosed()
        {
            var poolName = "default";
            var walletName = "deleteWalletWorks";

            await Wallet.CreateWalletAsync(poolName, walletName, null, null, null);

            var wallet = await Wallet.OpenWalletAsync(walletName, null, null);
            Assert.IsNotNull(wallet);

            await wallet.CloseAsync();
            await Wallet.DeleteWalletAsync(walletName, null);
            await Wallet.CreateWalletAsync(poolName, walletName, null, null, null);
        }

        [TestMethod]
        [Ignore] //TODO: Remove ignore when bug in Indy fixed.
        public async Task TestDeleteWalletWorksForOpened()
        {
            var walletName = "deleteWalletWorksForOpened";

            await Wallet.CreateWalletAsync("default", walletName, null, null, null);
            var wallet = await Wallet.OpenWalletAsync(walletName, null, null);

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Wallet.DeleteWalletAsync(walletName, null)
            );

            Assert.AreEqual(ErrorCode.CommonIOError, ex.ErrorCode);            
        }

        [TestMethod]
        public async Task TestDeleteWalletWorksForTwice()
        {
            var walletName = "deleteWalletWorksForTwice";

            await Wallet.CreateWalletAsync("default", walletName, null, null, null);

            var wallet = await Wallet.OpenWalletAsync(walletName, null, null);

            Assert.IsNotNull(wallet);

            await wallet.CloseAsync();

            await Wallet.DeleteWalletAsync(walletName, null);

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
        public async Task TestDeleteWalletWorksForPlugged()
        {
            var type = "inmem";
            var poolName = "default";
            var walletName = "wallet";

            await Wallet.CreateWalletAsync(poolName, walletName, type, null, null);
            await Wallet.DeleteWalletAsync(walletName, null);
            await Wallet.CreateWalletAsync(poolName, walletName, type, null, null);
        }
    }
}

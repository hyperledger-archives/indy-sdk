using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
using System.Collections.Generic;
using System.Text;
using System.Threading;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.WalletTests
{
    [TestClass]
    public class DisposeWalletTest : IndyIntegrationTestBase
    {
        private string _walletName = "disposableWallet";

        [TestInitialize]
        public async Task CreateWallet()
        {
            await Wallet.CreateWalletAsync("default", _walletName, "default", null, null);
        }

        [TestCleanup]
        public async Task DeleteWallet()
        {
            await Wallet.DeleteWalletAsync(_walletName, null);
        }

        [TestMethod]
        public async Task CanDisposeClosedWallet()
        {
            using (var wallet = await Wallet.OpenWalletAsync(_walletName, null, null))
            {
                await wallet.CloseAsync();
            }
        }

        [TestMethod]
        public async Task DisposeCanBeCalledRepeatedly()
        {
            var wallet = await Wallet.OpenWalletAsync(_walletName, null, null);
            wallet.Dispose();
            wallet.Dispose();
        }

        [TestMethod]
        public async Task WalletCanBeReOpenedAfterDispose()
        {
            var wallet = await Wallet.OpenWalletAsync(_walletName, null, null);
            wallet.Dispose();

            using (var newWallet = await Wallet.OpenWalletAsync(_walletName, null, null))
            {
            }
        }

        [TestMethod]
        public async Task ClosingDisposedWalletStillProvidesSDKError()
        {
            var wallet = await Wallet.OpenWalletAsync(_walletName, null, null);
            wallet.Dispose();

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                wallet.CloseAsync()
            );

            Assert.AreEqual(ErrorCode.WalletInvalidHandle, ex.ErrorCode);
        }      
    }
}

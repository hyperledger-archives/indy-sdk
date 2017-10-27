using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.WalletTests
{
    [TestClass]
    public class CreateWalletTest : IndyIntegrationTestBase
    {
        [TestMethod]
        public async Task TestCreateWalletWorks()
        {
            await Wallet.CreateWalletAsync(POOL, WALLET, TYPE, null, null);
        }

        [TestMethod]
        public async Task TestCreateWalletWorksForPlugged()
        {
            await Wallet.CreateWalletAsync(POOL, "pluggedWalletCreate", "inmem", null, null);
        }

        [TestMethod]
        public async Task TestCreateWalletWorksForEmptyType()
        {
            await Wallet.CreateWalletAsync(POOL, WALLET, null, null, null);
        }

        [TestMethod]
        public async Task TestCreateWalletWorksForConfigJson()
        {
            await Wallet.CreateWalletAsync(POOL, WALLET, null, "{\"freshness_time\":1000}", null);
        }

        [TestMethod]
        public async Task TestCreateWalletWorksForUnknownType()
        {
            var ex = await Assert.ThrowsExceptionAsync<UnknownWalletTypeException>(() =>
                Wallet.CreateWalletAsync(POOL, WALLET, "unknown_type", null, null)
            );
        }

        [TestMethod]
        public async Task TestCreateWalletWorksForEmptyName()
        {
            var ex = await Assert.ThrowsExceptionAsync<ArgumentException>(() =>
                Wallet.CreateWalletAsync(POOL, string.Empty, TYPE, null, null)
            );

            Assert.AreEqual("name", ex.ParamName);
        }

        [TestMethod]
        public async Task TestCreateWalletFailsForDuplicateName()
        {
            await Wallet.CreateWalletAsync(POOL, WALLET, TYPE, null, null);

            var ex = await Assert.ThrowsExceptionAsync<WalletExistsException>(() =>
                Wallet.CreateWalletAsync(POOL, WALLET, TYPE, null, null)
            );
        }

        

    }
}

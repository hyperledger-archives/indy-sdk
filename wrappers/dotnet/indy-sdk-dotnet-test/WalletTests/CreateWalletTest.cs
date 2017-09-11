using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.WalletTests
{
    [TestClass]
    public class CreateWalletTest : IndyIntegrationTestBase
    {
        [TestMethod]
        public async Task TestCreateWalletWorks()
        {
            await Wallet.CreateWalletAsync("default", "createWalletWorks", "default", null, null);
        }

        [TestMethod]
        public async Task TestCreateWalletWorksForEmptyType()
        {
            await Wallet.CreateWalletAsync("default", "createWalletWorks", null, null, null);
        }

        [TestMethod]
        public async Task TestCreateWalletWorksForConfigJson()
        {
            await Wallet.CreateWalletAsync("default", "createWalletWorks", null, "{\"freshness_time\":1000}", null);
        }

        [TestMethod]
        public async Task TestCreateWalletWorksForUnknownType()
        {
            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Wallet.CreateWalletAsync("default", "createWalletWorks", "unknown_type", null, null)
            );

            Assert.AreEqual(ErrorCode.WalletUnknownTypeError, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestCreateWalletWorksForEmptyName()
        {
            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Wallet.CreateWalletAsync(string.Empty, "createWalletWorks", "default", null, null)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidParam2, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestCreateWalletWorksForDuplicateName()
        {
            var poolName = "default";
            var walletName = "deleteWalletWorks";
            var type = "default";

            await Wallet.CreateWalletAsync(poolName, walletName, type, null, null);

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Wallet.CreateWalletAsync(poolName, walletName, type, null, null)
            );

            Assert.AreEqual(ErrorCode.WalletAlreadyExistsError, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestCreateWalletWorksForPlugged()
        {       
            await Wallet.CreateWalletAsync("default", "createPluggedWalletWorks", "inmem", null, null);
        }

    }
}

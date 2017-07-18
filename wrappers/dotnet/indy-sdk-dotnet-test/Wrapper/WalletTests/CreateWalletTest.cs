using System;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Indy.Sdk.Dotnet.Wrapper;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.WalletTests
{
    [TestClass]
    public class CreateWalletTest : IndyIntegrationTest
    {
        [TestMethod]
        public void TestCreateWalletWorks()
        {
            Wallet.CreateWalletAsync("default", "createWalletWorks", "default", null, null).Wait();
        }

        [TestMethod]
        public void TestCreateWalletWorksForEmptyType()
        {
            Wallet.CreateWalletAsync("default", "createWalletWorks", null, null, null).Wait();
        }

        [TestMethod]
        public void TestCreateWalletWorksForConfigJson()
        {
            Wallet.CreateWalletAsync("default", "createWalletWorks", null, "{\"freshness_time\":1000}", null).Wait();
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

            Wallet.CreateWalletAsync("default", "createWalletWorks", "default", null, null).Wait();

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Wallet.CreateWalletAsync("default", "createWalletWorks", "default", null, null)
            );

            Assert.AreEqual(ErrorCode.WalletAlreadyExistsError, ex.ErrorCode);
        }
    }
}

using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.WalletTests
{
    [TestClass]
    public class DeleteWalletTest : IndyIntegrationTest
    {
        [TestMethod]
        public void TestDeleteWalletWorks()
        {

            Wallet.CreateWalletAsync("default", "DeleteWalletAsyncWorks", "default", null, null).Wait();
            Wallet.DeleteWalletAsync("DeleteWalletAsyncWorks", null).Wait();
            Wallet.CreateWalletAsync("default", "DeleteWalletAsyncWorks", "default", null, null).Wait();
        }

        [TestMethod]
        public void TestDeleteWalletWorksForClosed()
        {
            Wallet.CreateWalletAsync("default", "DeleteWalletAsyncWorksForClosed", null, null, null).Wait();

            var wallet = Wallet.OpenWalletAsync("DeleteWalletAsyncWorksForClosed", null, null).Result;
            Assert.IsNotNull(wallet);

            wallet.CloseAsync().Wait();
            Wallet.DeleteWalletAsync("DeleteWalletAsyncWorksForClosed", null).Wait();
            Wallet.CreateWalletAsync("default", "DeleteWalletAsyncWorksForClosed", null, null, null).Wait();
        }

        [TestMethod]
        [Ignore] //Bug in Indy
        public async Task TestDeleteWalletWorksForOpened()
        {
            Wallet.CreateWalletAsync("default", "DeleteWalletAsyncWorksForOpened", null, null, null).Wait();
            var wallet = Wallet.OpenWalletAsync("DeleteWalletAsyncWorksForOpened", null, null).Result;

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Wallet.DeleteWalletAsync("DeleteWalletAsyncWorksForOpened", null)
            );

            Assert.AreEqual(ex.ErrorCode, ErrorCode.CommonIOError);            
        }

        [TestMethod]
        public async Task TestDeleteWalletWorksForTwice()
        {
            Wallet.CreateWalletAsync("default", "DeleteWalletAsyncWorksForTwice", null, null, null).Wait();

            var wallet = Wallet.OpenWalletAsync("DeleteWalletAsyncWorksForTwice", null, null).Result;

            Assert.IsNotNull(wallet);

            wallet.CloseAsync().Wait();

            Wallet.DeleteWalletAsync("DeleteWalletAsyncWorksForTwice", null).Wait();

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                 Wallet.DeleteWalletAsync("DeleteWalletAsyncWorksForTwice", null)
            );

            Assert.AreEqual(ex.ErrorCode, ErrorCode.CommonIOError);
        
        }

        [TestMethod]
        public async Task TestDeleteWalletWorksForNotCreated()
        {
            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Wallet.DeleteWalletAsync("DeleteWalletAsyncWorksForTwice", null)
            );

            Assert.AreEqual(ex.ErrorCode, ErrorCode.CommonIOError);
        }
    }
}

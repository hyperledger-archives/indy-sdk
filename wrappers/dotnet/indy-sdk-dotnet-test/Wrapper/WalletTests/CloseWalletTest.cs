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
    public class CloseWalletTest : IndyIntegrationTest
    {
        [TestMethod]
        public void testCloseWalletWorks()
        {
            Wallet.CreateWalletAsync("default", "CloseAsyncWorks", "default", null, null).Wait();

            var wallet = Wallet.OpenWalletAsync("CloseAsyncWorks", null, null).Result;

            Assert.IsNotNull(wallet);

            wallet.CloseAsync().Wait();
        }

        [TestMethod]
        public void testCloseWalletWorksForTwice()
        {
            Wallet.CreateWalletAsync("default", "CloseAsyncWorksForTwice", "default", null, null).Wait();

            var wallet = Wallet.OpenWalletAsync("CloseAsyncWorksForTwice", null, null).Result;

            Assert.IsNotNull(wallet);

            wallet.CloseAsync().Wait();

            try
            {
                wallet.CloseAsync().Wait();
            }
            catch (IndyException e)
            {
                Assert.AreEqual(e.ErrorCode, ErrorCode.WalletInvalidHandle);
            }
        }
    }
}

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
    public class OpenWalletTest : IndyIntegrationTest
    {
        [TestMethod]
        public void TestOpenWalletWorks()
        {
            Wallet.CreateWalletAsync("default", "openWalletWorks", "default", null, null).Wait();
            Wallet wallet = Wallet.OpenWalletAsync("openWalletWorks", null, null).Result;

            Assert.IsNotNull(wallet);
        }

        [TestMethod]
        public void TestOpenWalletWorksForConfig()
        {
            Wallet.CreateWalletAsync("default", "openWalletWorksForConfig", "default", null, null).Wait();
            Wallet wallet = Wallet.OpenWalletAsync("openWalletWorksForConfig", "{\"freshness_time\":1000}", null).Result;

            Assert.IsNotNull(wallet);
        }

        [TestMethod]
        public void TestOpenWalletWorksForNotCreatedWallet()
        {
            try
            {
                var wallet = Wallet.OpenWalletAsync("openWalletWorksForNotCreatedWallet", null, null).Result;
            }
            catch (IndyException e)
            {
                Assert.AreEqual(e.ErrorCode, ErrorCode.CommonIOError);
            }
        }

        [TestMethod]
        public void TestOpenWalletWorksForTwice()
        {
            Wallet.CreateWalletAsync("default", "openWalletWorksForTwice", "default", null, null).Wait();

            var wallet1 = Wallet.OpenWalletAsync("openWalletWorksForTwice", null, null).Result;

            try
            {
                var wallet2 = Wallet.OpenWalletAsync("openWalletWorksForTwice", null, null).Result;
            }
            catch (IndyException e)
            {
                Assert.AreEqual(e.ErrorCode, ErrorCode.WalletAlreadyOpenedError);
            }
        }

        [TestMethod]
        public void TestOpenWalletWorksForNotCreated()
        {
            try
            {
                var wallet = Wallet.OpenWalletAsync("testOpenWalletWorksForNotCreated", null, null).Result;
            }
            catch (IndyException e)
            {
                Assert.AreEqual(e.ErrorCode, ErrorCode.CommonIOError);
            }
        }
    }
}

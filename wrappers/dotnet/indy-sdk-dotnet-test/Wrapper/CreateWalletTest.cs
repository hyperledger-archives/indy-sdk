using System;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Indy.Sdk.Dotnet.Wrapper;

namespace Indy.Sdk.Dotnet.Test.Wrapper
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
        public void TestCreateWalletWorksForUnknownType()
        {
            Wallet.CreateWalletAsync("default", "createWalletWorks", "unknown_type", null, null).Wait();
        }

        [TestMethod]
        public void TestCreateWalletWorksForEmptyName()
        {
            Wallet.CreateWalletAsync(string.Empty, "createWalletWorks", "default", null, null).Wait();
        }

        [TestMethod]
        [ExpectedException(typeof(IndyException))]
        public void TestCreateWalletWorksForDuplicateName()
        {
            Wallet.CreateWalletAsync("default", "createWalletWorks", "default", null, null).Wait();
            Wallet.CreateWalletAsync("default", "createWalletWorks", "default", null, null).Wait();
        }
    }
}

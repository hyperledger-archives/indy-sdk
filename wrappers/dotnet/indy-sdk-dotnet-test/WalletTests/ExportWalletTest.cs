using Hyperledger.Indy.DidApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.IO;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.WalletTests
{
    [TestClass]
    public class ExportWalletTest : IndyIntegrationTestWithSingleWallet
    {
        [TestMethod]
        public async Task TestExportWalletWorks()
        {
            await Did.CreateAndStoreMyDidAsync(wallet, "{}");
            await wallet.ExportAsync(EXPORT_CONFIG_JSON);
           
            Assert.IsTrue(Directory.Exists(EXPORT_PATH));
        }

        [TestMethod]
        public async Task TestExportWalletWorksForExistsPath()
        {
            Directory.CreateDirectory(EXPORT_PATH);

            var ex = await Assert.ThrowsExceptionAsync<InvalidWalletException>(() =>
                wallet.ExportAsync(EXPORT_CONFIG_JSON)
            );
        }
    }
}

using Hyperledger.Indy.DidApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.WalletTests
{
    [TestClass]
    public class ImportWalletTest : IndyIntegrationTestWithSingleWallet
    {
        [TestMethod]
        public async Task TestImportWalletWorks()
        {
            var result = await Did.CreateAndStoreMyDidAsync(wallet, "{}");
            var did = result.Did;

            await Did.SetDidMetadataAsync(wallet, did, METADATA);

            var didWithMetaBefore = await Did.GetDidMetadataAsync(wallet, did);

            await wallet.ExportAsync(EXPORT_CONFIG_JSON);

            await wallet.CloseAsync();
            await Wallet.DeleteWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS);

            await Wallet.ImportAsync(WALLET_CONFIG, WALLET_CREDENTIALS, EXPORT_CONFIG_JSON);

            wallet = await Wallet.OpenWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS);

            var didWithMetaAfter = await Did.GetDidMetadataAsync(wallet, did);

            Assert.AreEqual(didWithMetaBefore, didWithMetaAfter);
        }

        [TestMethod]
        public async Task TestImportWalletWorksForNotExists()
        {
            var ex = await Assert.ThrowsExceptionAsync<IOException>(() =>
                Wallet.ImportAsync(WALLET_CONFIG, WALLET_CREDENTIALS, EXPORT_CONFIG_JSON)
            );
        }
    }
}

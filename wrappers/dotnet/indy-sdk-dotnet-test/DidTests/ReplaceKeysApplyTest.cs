using Hyperledger.Indy.DidApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.DidTests
{
    [TestClass]
    public class ReplaceKeysApplyTest : IndyIntegrationTestWithSingleWallet
    {
        private string _did;

        [TestInitialize]
        public async Task CreateWalletWithDid()
        {
            var result = await Did.CreateAndStoreMyDidAsync(wallet, "{}");
            _did = result.Did;
        }        

        [TestMethod]
        public async Task TestReplaceKeysApplyWorks()
        {
            await Did.ReplaceKeysStartAsync(wallet, _did, "{}");
            await Did.ReplaceKeysApplyAsync(wallet, _did);
        }

        [TestMethod]
        public async Task TestReplaceKeysApplyWorksWithoutCallingReplaceStart()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletItemNotFoundException>(() =>
                Did.ReplaceKeysApplyAsync(wallet, _did)
            );
        }

        [TestMethod]
        public async Task TestReplaceKeysApplyWorksForNotFoundDid()
        {
            await Did.ReplaceKeysStartAsync(wallet, _did, "{}");

            var ex = await Assert.ThrowsExceptionAsync<WalletItemNotFoundException>(() =>
                Did.ReplaceKeysApplyAsync(wallet, DID)
            );
        }
    }
}

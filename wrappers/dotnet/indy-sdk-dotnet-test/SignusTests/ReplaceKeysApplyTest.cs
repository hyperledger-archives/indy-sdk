using Hyperledger.Indy.SignusApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.SignusTests
{
    [TestClass]
    public class ReplaceKeysApplyTest : IndyIntegrationTestWithSingleWallet
    {
        private string _did;

        [TestInitialize]
        public async Task CreateWalletWithDid()
        {
            var result = await Signus.CreateAndStoreMyDidAsync(wallet, "{}");
            _did = result.Did;
        }        

        [TestMethod]
        public async Task TestReplaceKeysApplyWorks()
        {
            await Signus.ReplaceKeysStartAsync(wallet, _did, "{}");
            await Signus.ReplaceKeysApplyAsync(wallet, _did);
        }

        [TestMethod]
        public async Task TestReplaceKeysApplyWorksWithoutCallingReplaceStart()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletValueNotFoundException>(() =>
                Signus.ReplaceKeysApplyAsync(wallet, _did)
            );
        }

        [TestMethod]
        public async Task TestReplaceKeysApplyWorksForNotFoundDid()
        {
            await Signus.ReplaceKeysStartAsync(wallet, _did, "{}");

            var ex = await Assert.ThrowsExceptionAsync<WalletValueNotFoundException>(() =>
                Signus.ReplaceKeysApplyAsync(wallet, DID1)
            );
        }
    }
}

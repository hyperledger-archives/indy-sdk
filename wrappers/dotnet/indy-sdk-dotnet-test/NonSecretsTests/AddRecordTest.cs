using Hyperledger.Indy.NonSecretsApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.NonSecretsTests
{
    [TestClass]
    public class AddRecordTest : NonSecretsIntegrationTestBase
    {
        [TestMethod]
        public async Task TestAddRecordWorks()
        {
            await NonSecrets.AddRecordAsync(wallet, type, id, value, tagsEmpty);
        }

        [TestMethod]
        public async Task TestAddRecordWorksForDifferentIds()
        {
            await NonSecrets.AddRecordAsync(wallet, type, id, value, tagsEmpty);
            await NonSecrets.AddRecordAsync(wallet, type, id2, value, tagsEmpty);
        }

        [TestMethod]
        public async Task TestAddRecordWorksForDuplicate()
        {
            await NonSecrets.AddRecordAsync(wallet, type, id, value, tagsEmpty);

            var ex = await Assert.ThrowsExceptionAsync<WalletItemAlreadyExistsException>(() =>
                NonSecrets.AddRecordAsync(wallet, type, id, value, tagsEmpty)
            );

        }
    }
}

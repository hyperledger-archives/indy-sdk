using Hyperledger.Indy.NonSecretsApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.NonSecretsTests
{
    [TestClass]
    public class DeleteRecordTest : NonSecretsIntegrationTestBase
    {
        [TestMethod]
        public async Task TestDeleteRecordWorks()
        {
            await NonSecrets.AddRecordAsync(wallet, type, id, value, tags);
            await NonSecrets.DeleteRecordAsync(wallet, type, id);

            var ex = await Assert.ThrowsExceptionAsync<WalletItemNotFoundException>(() =>
                NonSecrets.GetRecordAsync(wallet, type, id, optionsEmpty)
            );
        }

        [TestMethod]
        public async Task TestDeleteRecordWorksForTwice()
        {
            await NonSecrets.AddRecordAsync(wallet, type, id, value, tags);
            await NonSecrets.DeleteRecordAsync(wallet, type, id);

            var ex = await Assert.ThrowsExceptionAsync<WalletItemNotFoundException>(() =>
                NonSecrets.DeleteRecordAsync(wallet, type, id)
            );
        }

        [TestMethod]
        public async Task TestDeleteRecordWorksForNotFoundRecord()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletItemNotFoundException>(() =>
               NonSecrets.DeleteRecordAsync(wallet, type, id)
           );
        }
    }
}

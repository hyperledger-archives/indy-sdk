using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.DidApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;
using Hyperledger.Indy.NonSecretsApi;
using Hyperledger.Indy.WalletApi;

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

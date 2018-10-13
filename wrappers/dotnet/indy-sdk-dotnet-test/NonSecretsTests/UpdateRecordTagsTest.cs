using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.DidApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;
using Hyperledger.Indy.NonSecretsApi;
using Hyperledger.Indy.WalletApi;
using Newtonsoft.Json;

namespace Hyperledger.Indy.Test.NonSecretsTests
{
    [TestClass]
    public class UpdateRecordTagsTest : NonSecretsIntegrationTestBase
    {
        [TestMethod]
        public async Task TestUpdateRecordTagsWorks()
        {
            await NonSecrets.AddRecordAsync(wallet, type, id, value, tagsEmpty);

            await CheckRecordFieldAsync(wallet, type, id, "tags", tagsEmpty);

            await NonSecrets.UpdateRecordTagsAsync(wallet, type, id, tags);

            await CheckRecordFieldAsync(wallet, type, id, "tags", tags);

        }

        [TestMethod]
        public async Task TestUpdateRecordTagsWorksForTwice()
        {
            await NonSecrets.AddRecordAsync(wallet, type, id, value, tagsEmpty);

            await CheckRecordFieldAsync(wallet, type, id, "tags", tagsEmpty);

            await NonSecrets.UpdateRecordTagsAsync(wallet, type, id, tags);

            await CheckRecordFieldAsync(wallet, type, id, "tags", tags);

            await NonSecrets.UpdateRecordTagsAsync(wallet, type, id, tags2);

            await CheckRecordFieldAsync(wallet, type, id, "tags", tags2);

        }

        [TestMethod]
        public async Task testUpdateRecordTagsWorksForNotFoundRecord()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletItemNotFoundException>(() =>
                NonSecrets.UpdateRecordTagsAsync(wallet, type, id, tags)
            );
        }
    }
}

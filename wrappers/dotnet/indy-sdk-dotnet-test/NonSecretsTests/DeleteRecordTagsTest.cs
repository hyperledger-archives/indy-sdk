using Hyperledger.Indy.NonSecretsApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.NonSecretsTests
{
    [TestClass]
    public class DeleteRecordTagsTest : NonSecretsIntegrationTestBase
    {
        [TestMethod]
        public async Task TestDeleteRecordTagsWorks()
        {
            await NonSecrets.AddRecordAsync(wallet, type, id, value, tags);
            await CheckRecordFieldAsync(wallet, type, id, "tags", tags);

            await NonSecrets.DeleteRecordTagsAsync(wallet, type, id, "[\"tagName1\"]");

            var expectedTags = "{\"tagName2\": \"5\", \"tagName3\": \"12\"}";
            await CheckRecordFieldAsync(wallet, type, id, "tags", expectedTags);
        }

        [TestMethod]
        public async Task TestDeleteRecordTagsWorksForDeleteAll()
        {
            await NonSecrets.AddRecordAsync(wallet, type, id, value, tags);
            await CheckRecordFieldAsync(wallet, type, id, "tags", tags);

            await NonSecrets.DeleteRecordTagsAsync(wallet, type, id, "[\"tagName1\", \"tagName2\", \"tagName3\"]");
            await CheckRecordFieldAsync(wallet, type, id, "tags", tagsEmpty);
        }

        [TestMethod]
        public async Task TestDeleteRecordTagsWorksForNotFoundRecord()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletItemNotFoundException>(() =>
                NonSecrets.DeleteRecordTagsAsync(wallet, type, id, "[\"tagName1\"]")
            );

        }
    }
}

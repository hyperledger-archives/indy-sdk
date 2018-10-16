using Hyperledger.Indy.NonSecretsApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.NonSecretsTests
{
    [TestClass]
    public class AddRecordTagsTest : NonSecretsIntegrationTestBase
    {
        [TestMethod]
        public async Task TestBuildAttribRequestWorksForRawData()
        {
            await NonSecrets.AddRecordAsync(wallet, type, id, value, tagsEmpty);

            await CheckRecordFieldAsync(wallet, type, id, "tags", tagsEmpty);

            await NonSecrets.AddRecordTagsAsync(wallet, type, id, tags);

            await CheckRecordFieldAsync(wallet, type, id, "tags", tags);
        }

        [TestMethod]
        public async Task TestAddRecordTagsWorksForTwice()
        {
            await NonSecrets.AddRecordAsync(wallet, type, id, value, tagsEmpty);
            await CheckRecordFieldAsync(wallet, type, id, "tags", tagsEmpty);

            var tags1 = "{\"tagName1\": \"str1\"}";
            await NonSecrets.AddRecordTagsAsync(wallet, type, id, tags1);

            await CheckRecordFieldAsync(wallet, type, id, "tags", tags1);

            var tags2 = "{\"tagName2\": \"str2\"}";
            await NonSecrets.AddRecordTagsAsync(wallet, type, id, tags2);

            var expectedTags = "{\"tagName1\":\"str1\",\"tagName2\":\"str2\"}";
            await CheckRecordFieldAsync(wallet, type, id, "tags", expectedTags);
        }

        [TestMethod]
        public async Task TestAddRecordTagsWorksForNotFoundRecord()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletItemNotFoundException>(() =>
                NonSecrets.AddRecordTagsAsync(wallet, type, id, tags)
            );

        }
    }
}

using Hyperledger.Indy.NonSecretsApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.NonSecretsTests
{
    [TestClass]
    public class UpdateRecordValueTest : NonSecretsIntegrationTestBase
    {
        [TestMethod]
        public async Task TestUpdateRecordValueWorks()
        {
            await NonSecrets.AddRecordAsync(wallet, type, id, value, tagsEmpty);

            await CheckRecordFieldAsync(wallet, type, id, "value", value);

            await NonSecrets.UpdateRecordValueAsync(wallet, type, id, value2);

            await CheckRecordFieldAsync(wallet, type, id, "value", value2);

        }

        [TestMethod]
        public async Task TestUpdateRecordValueWorksForNotFoundRecord()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletItemNotFoundException>(() =>
                NonSecrets.UpdateRecordValueAsync(wallet, type, id, value)
            );
        }
    }
}

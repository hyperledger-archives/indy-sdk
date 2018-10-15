using Hyperledger.Indy.NonSecretsApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.NonSecretsTests
{
    [TestClass]
    public class GetRecordTest : NonSecretsIntegrationTestBase
    {
        [TestMethod]
        public async Task TestGetRecordWorksForDefaultOptions()
        {
            await NonSecrets.AddRecordAsync(wallet, type, id, value, tags);

            var recordJson = await NonSecrets.GetRecordAsync(wallet, type, id, optionsEmpty);            
            var actual = JObject.Parse(recordJson);

            var expected = JObject.FromObject(new
            {
                id = id,
                type = (object)null,
                value = value,
                tags = (object)null
            });

            Assert.IsTrue(JValue.DeepEquals(expected, actual));
        }

        [TestMethod]
        public async Task TestGetRecordWorksForFullData()
        {
            await NonSecrets.AddRecordAsync(wallet, type, id, value, tags);

            var optionsJson = JsonConvert.SerializeObject(
                new
                {
                    retrieveType = true,
                    retrieveValue = true,
                    retrieveTags = true
                }
            );

            var recordJson = await NonSecrets.GetRecordAsync(wallet, type, id, optionsJson);
            var record = JObject.Parse(recordJson);

            Assert.AreEqual(id, record["id"]);
            Assert.AreEqual(type, record["type"]);
            Assert.AreEqual(value, record["value"]);
            Assert.IsTrue(JValue.DeepEquals(JObject.Parse(tags), JObject.Parse(record["tags"].ToString())));
        }

        [TestMethod]
        public async Task TestGetRecordWorksForNotFoundRecord()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletItemNotFoundException>(() =>
               NonSecrets.GetRecordAsync(wallet, type, id, optionsEmpty)
           );
        }
    }
}

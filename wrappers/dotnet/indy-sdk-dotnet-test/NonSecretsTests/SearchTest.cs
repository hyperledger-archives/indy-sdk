using Hyperledger.Indy.NonSecretsApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.NonSecretsTests
{
    [TestClass]
    public class SearchTest : NonSecretsIntegrationTestBase
    {
        [TestMethod]
        public async Task TestWalletSearchWorks()
        {
            await NonSecrets.AddRecordAsync(wallet, type, id, value, tags);

            using (var search = await NonSecrets.OpenSearchAsync(wallet, type, queryEmpty, optionsEmpty))
            {
                var searchRecordsJson = await search.NextAsync(wallet, 1);
                var searchRecords = JObject.Parse(searchRecordsJson);

                var records = (JArray)searchRecords["records"];

                Assert.AreEqual(1, records.Count);

                var expected = JObject.FromObject(new
                {
                    id = id,
                    type = (object)null,
                    value = value,
                    tags = (object)null
                });

                Assert.IsTrue(JValue.DeepEquals(expected, records[0]));
            }
        }

        [TestMethod]
        public async Task TestWalletSearchWorksForOptions()
        {
            await NonSecrets.AddRecordAsync(wallet, type, id, value, tags);

            var options = JsonConvert.SerializeObject(
                new
                {
                    retrieveRecords = true,
                    retrieveTotalCount = false,
                    retrieveType = false,
                    retrieveValue = false,
                    retrieveTags = false
                }
            );
            using (var search = await NonSecrets.OpenSearchAsync(wallet, type, queryEmpty, options))
            {
                var searchRecordsJson = await search.NextAsync(wallet, 1);
                var searchRecords = JObject.Parse(searchRecordsJson);

                var records = (JArray)searchRecords["records"];

                Assert.AreEqual(1, records.Count);

                var expected = JObject.FromObject(
                    new
                    {
                        id = id,
                        type = (object)null,
                        value = (object)null,
                        tags = (object)null
                    }
                );

                Assert.IsTrue(JValue.DeepEquals(expected, records[0]));
            }
        }

        [TestMethod]
        public async Task TestWalletSearchWorksForQuery()
        {
            await NonSecrets.AddRecordAsync(wallet, type, id, value, tags);
            await NonSecrets.AddRecordAsync(wallet, type, id2, value2, tags2);
            await NonSecrets.AddRecordAsync(wallet, type, id3, value2, tags3);

            var query = "{\"tagName1\":\"str2\"}";

            using (var search = await NonSecrets.OpenSearchAsync(wallet, type, query, optionsEmpty))
            {
                var searchRecordsJson = await search.NextAsync(wallet, 3);
                var searchRecords = JObject.Parse(searchRecordsJson);

                var records = (JArray)searchRecords["records"];

                Assert.AreEqual(1, records.Count);

                var expected = JObject.FromObject(new
                {
                    id = id2,
                    type = (object)null,
                    value = value2,
                    tags = (object)null
                });

                Assert.IsTrue(JValue.DeepEquals(expected, records[0]));
            }
        }
    }
}

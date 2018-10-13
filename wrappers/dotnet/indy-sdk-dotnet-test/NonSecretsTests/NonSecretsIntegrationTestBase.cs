using Hyperledger.Indy.NonSecretsApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.NonSecretsTests
{
    public class NonSecretsIntegrationTestBase : IndyIntegrationTestWithPoolAndSingleWallet
    {
        protected string type = "TestType";
        protected string id = "RecordId";
        protected string id2 = "RecordId2";
        protected string id3 = "RecordId3";
        protected string value = "RecordValue";
        protected string value2 = "RecordValue2";
        protected string value3 = "RecordValue3";
        protected string tagsEmpty = "{}";
        protected string queryEmpty = "{}";
        protected string optionsEmpty = "{}";
        protected string tags = "{\"tagName1\":\"str1\",\"tagName2\":\"5\",\"tagName3\":\"12\"}";
        protected string tags2 = "{\"tagName1\":\"str2\",\"tagName2\":\"pre_str3\",\"tagName3\":\"2\"}";
        protected string tags3 = "{\"tagName1\":\"str1\",\"tagName2\":\"str2\",\"tagName3\":\"str3\"}";

        protected async Task CheckRecordFieldAsync(Wallet wallet, string type, string id, string field, string expectedValue)
        {
            string optionsFull = "{\"retrieveType\":true, \"retrieveValue\":true, \"retrieveTags\":true}";
            string recordJson = await NonSecrets.GetRecordAsync(wallet, type, id, optionsFull);
            var record = JObject.Parse(recordJson);

            switch (field)
            {
                case "value":
                    Assert.AreEqual(expectedValue, record["value"].ToString());
                    break;
                case "tags":
                    var expected = JObject.Parse(expectedValue);
                    Assert.IsTrue(JValue.DeepEquals(expected, JObject.Parse(record["tags"].ToString())));
                    break;
                default:
                    Assert.IsTrue(false);
                    break;
            }
        }
    }
}

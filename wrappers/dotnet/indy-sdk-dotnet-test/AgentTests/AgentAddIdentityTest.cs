using Hyperledger.Indy.Sdk.AgentApi;
using Hyperledger.Indy.Sdk.SignUsApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Sdk.Test.AgentTests
{
    [TestClass]
    public class AgentAddIdentityTest : AgentIntegrationTestBase
    {
        [TestMethod]
        public async Task TestAgentAddIdentityWorks()
        {
            var endpoint = "127.0.0.1:9601";

            var myDidResult = await SignUs.CreateAndStoreMyDidAsync(_wallet, "{}");

            var activeListener = await AgentListener.ListenAsync(endpoint);

            await activeListener.AddIdentityAsync(_pool, _wallet, myDidResult.Did);
        }

        [TestMethod]
        public async Task TestAgentAddIdentityWorksForMultiplyKeys()
        {
            var endpoint = "127.0.0.1:9602";

            var myDid1 = await SignUs.CreateAndStoreMyDidAsync(_wallet, "{}");
            var myDid2 = await SignUs.CreateAndStoreMyDidAsync(_wallet, "{}");

            CreateAndStoreMyDidResult[] didResults = { myDid1, myDid2 };

            var activeListener = await AgentListener.ListenAsync(endpoint);

            foreach (var didResult in didResults)
            {
                await activeListener.AddIdentityAsync(_pool, _wallet, didResult.Did);
            }
        }
    }
}

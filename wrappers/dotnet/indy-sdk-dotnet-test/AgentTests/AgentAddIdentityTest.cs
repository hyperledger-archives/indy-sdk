using Hyperledger.Indy.AgentApi;
using Hyperledger.Indy.SignusApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AgentTests
{
    [TestClass]
    public class AgentAddIdentityTest : AgentIntegrationTestBase
    {
        [TestMethod]
        public async Task TestAgentAddIdentityWorks()
        {
            var endpoint = "127.0.0.1:9601";

            var myDidResult = await Signus.CreateAndStoreMyDidAsync(wallet, "{}");

            var activeListener = await AgentListener.ListenAsync(endpoint);

            await activeListener.AddIdentityAsync(pool, wallet, myDidResult.Did);
        }

        [TestMethod]
        public async Task TestAgentAddIdentityWorksForMultiplyKeys()
        {
            var endpoint = "127.0.0.1:9602";

            var myDid1 = await Signus.CreateAndStoreMyDidAsync(wallet, "{}");
            var myDid2 = await Signus.CreateAndStoreMyDidAsync(wallet, "{}");

            CreateAndStoreMyDidResult[] didResults = { myDid1, myDid2 };

            var activeListener = await AgentListener.ListenAsync(endpoint);

            foreach (var didResult in didResults)
            {
                await activeListener.AddIdentityAsync(pool, wallet, didResult.Did);
            }
        }
    }
}

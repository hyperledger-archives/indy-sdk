using Hyperledger.Indy.Sdk.AgentApi;
using Hyperledger.Indy.Sdk.SignUsApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Sdk.Test.AgentTests
{
    [TestClass]
    public class AgentRemoveIdentityTest : AgentIntegrationTestBase
    {
        [TestMethod]
        public async Task TestAgentRemoveIdentityWorks()
        {
            var endpoint = "127.0.0.1:9608";

            var myDidResult = await SignUs.CreateAndStoreMyDidAsync(_wallet, "{}");

            var activeListener = await AgentListener.ListenAsync(endpoint);

            await activeListener.AddIdentityAsync(_pool, _wallet, myDidResult.Did);

            await activeListener.RemoveIdentityAsync(_wallet, myDidResult.Did);
        }
    }
}

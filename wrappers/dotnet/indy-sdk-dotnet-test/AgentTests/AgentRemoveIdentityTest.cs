using Hyperledger.Indy.AgentApi;
using Hyperledger.Indy.SignusApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AgentTests
{
    [TestClass]
    public class AgentRemoveIdentityTest : AgentIntegrationTestBase
    {
        [TestMethod]
        public async Task TestAgentRemoveIdentityWorks()
        {
            var endpoint = "127.0.0.1:9608";

            var myDidResult = await Signus.CreateAndStoreMyDidAsync(wallet, "{}");

            var activeListener = await AgentListener.ListenAsync(endpoint);
            await activeListener.AddIdentityAsync(pool, wallet, myDidResult.Did);

            await activeListener.RemoveIdentityAsync(wallet, myDidResult.Did);
        }
    }
}

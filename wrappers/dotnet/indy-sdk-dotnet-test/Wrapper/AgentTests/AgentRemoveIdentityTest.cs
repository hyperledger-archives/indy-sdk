using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.AgentTests
{
    [TestClass]
    public class AgentRemoveIdentityTest : AgentIntegrationTestBase
    {
        [TestMethod]
        public async Task TestAgentRemoveIdentityWorks()
        {
            var endpoint = "127.0.0.1:9608";

            var myDidResult = await Signus.CreateAndStoreMyDidAsync(_wallet, "{}");

            var activeListener = await AgentListener.ListenAsync(endpoint);

            await activeListener.AddIdentityAsync(_pool, _wallet, myDidResult.Did);

            await activeListener.RemoveIdentityAsync(_wallet, myDidResult.Did);
        }
    }
}

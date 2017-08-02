using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace Indy.Sdk.Dotnet.Test.Wrapper.AgentTests
{
    [TestClass]
    public class AgentRemoveIdentityTest : AgentIntegrationTestBase
    {
        [TestMethod]
        public void TestAgentRemoveIdentityWorks()
        {
            var endpoint = "127.0.0.1:9908";

            var myDidResult = Signus.CreateAndStoreMyDidAsync(_wallet, "{}").Result;

            var activeListener = Agent.AgentListenAsync(endpoint, _incomingConnectionObserver).Result;

            activeListener.AddIdentityAsync(_pool, _wallet, myDidResult.Did).Wait();

            activeListener.RemoveIdentityAsync(_wallet, myDidResult.Did).Wait();
        }
    }
}

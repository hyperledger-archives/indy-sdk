using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace Indy.Sdk.Dotnet.Test.Wrapper.AgentTests
{
    [TestClass]
    public class AgentListenTest : AgentIntegrationTestBase
    {
        [TestMethod]
        public void TestAgentListenWorks()
        {
            var endpoint = "127.0.0.1:9607";

            var didJson = "{\"seed\":\"sovrin_agent_connect_works_for_a\"}";
       
            var myDidResult = Signus.CreateAndStoreMyDidAsync(_wallet, didJson).Result;

            var identityJson = string.Format("{{\"did\":\"{0}\", \"pk\":\"{1}\", \"verkey\":\"{2}\", \"endpoint\":\"{3}\"}}",
                    myDidResult.Did, myDidResult.Pk, myDidResult.VerKey, endpoint);
            Signus.StoreTheirDidAsync(_wallet, identityJson).Wait();

            Agent.AgentListenAsync(endpoint, _incomingConnectionObserver).Wait();
        }
    }
}

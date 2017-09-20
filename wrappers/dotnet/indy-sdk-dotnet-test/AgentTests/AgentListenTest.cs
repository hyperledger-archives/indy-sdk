using Hyperledger.Indy.AgentApi;
using Hyperledger.Indy.SignusApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AgentTests
{
    [TestClass]
    public class AgentListenTest : AgentIntegrationTestBase
    {
        [TestMethod]
        public async Task TestAgentListenWorks()
        {
            var endpoint = "127.0.0.1:9607";

            var didJson = "{\"seed\":\"indy_agent_connect_works_for_aaa\"}";
       
            var myDidResult = await Signus.CreateAndStoreMyDidAsync(_wallet, didJson);

            var identityJson = string.Format("{{\"did\":\"{0}\", \"pk\":\"{1}\", \"verkey\":\"{2}\", \"endpoint\":\"{3}\"}}",
                    myDidResult.Did, myDidResult.Pk, myDidResult.VerKey, endpoint);
            await Signus.StoreTheirDidAsync(_wallet, identityJson);

            await AgentListener.ListenAsync(endpoint);
        }
    }
}

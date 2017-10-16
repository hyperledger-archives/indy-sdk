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
       
            var myDidResult = await Signus.CreateAndStoreMyDidAsync(wallet, "{}");

            var identityJson = string.Format(AGENT_IDENTITY_JSON_TEMPLATE, myDidResult.Did, myDidResult.Pk, myDidResult.VerKey, endpoint);
            await Signus.StoreTheirDidAsync(wallet, identityJson);

            await AgentListener.ListenAsync(endpoint);
        }
    }
}

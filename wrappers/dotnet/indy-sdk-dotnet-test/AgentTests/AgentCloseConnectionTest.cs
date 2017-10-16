using Hyperledger.Indy.AgentApi;
using Hyperledger.Indy.SignusApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AgentTests
{
    [TestClass]
    public class AgentCloseConnectionTest : AgentIntegrationTestBase
    {
        [TestMethod]
        public async Task TestAgentCloseConnectionWorksForOutgoing()
        {
            var endpoint = "127.0.0.1:9603";

            var myDid = await Signus.CreateAndStoreMyDidAsync(wallet, "{}");

            var identityJson = string.Format(AGENT_IDENTITY_JSON_TEMPLATE, myDid.Did, myDid.Pk, myDid.VerKey, endpoint);
            await Signus.StoreTheirDidAsync(wallet, identityJson);

            var activeListener = await AgentListener.ListenAsync(endpoint);
            await activeListener.AddIdentityAsync(pool, wallet, myDid.Did);

            var connection = await AgentConnection.ConnectAsync(pool, wallet, myDid.Did, myDid.Did);

            await connection.CloseAsync();

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                connection.SendAsync("msg")
            );
        }

        [TestMethod]
        public async Task TestAgentCloseConnectionWorksForIncoming()
        {
            var endpoint = "127.0.0.1:9613";

            var myDid = await Signus.CreateAndStoreMyDidAsync(wallet, "{}");

            var identityJson = string.Format(AGENT_IDENTITY_JSON_TEMPLATE, myDid.Did, myDid.Pk, myDid.VerKey, endpoint);

            await Signus.StoreTheirDidAsync(wallet, identityJson);
            var activeListener = await AgentListener.ListenAsync(endpoint);

            await activeListener.AddIdentityAsync(pool, wallet, myDid.Did);
            var connection = await AgentConnection.ConnectAsync(pool, wallet, myDid.Did, myDid.Did);

            var connectionEvent = await activeListener.WaitForConnectionAsync();
            var serverToClientConnection = connectionEvent.Connection;

            await serverToClientConnection.CloseAsync();

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                serverToClientConnection.SendAsync("msg")
            );
        }

    }
}

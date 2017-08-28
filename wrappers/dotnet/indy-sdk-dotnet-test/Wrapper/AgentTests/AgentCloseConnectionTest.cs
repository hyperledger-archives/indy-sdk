using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.AgentTests
{
    [TestClass]
    public class AgentCloseConnectionTest : AgentIntegrationTestBase
    {
        [TestMethod]
        public async Task TestAgentCloseConnectionWorksForOutgoing()
        {
            var endpoint = "127.0.0.1:9603";

            var myDid = await Signus.CreateAndStoreMyDidAsync(_wallet, "{}");

            var identityJson = string.Format("{{\"did\":\"{0}\", \"pk\":\"{1}\", \"verkey\":\"{2}\", \"endpoint\":\"{3}\"}}",
                    myDid.Did, myDid.Pk, myDid.VerKey, endpoint);

            await Signus.StoreTheirDidAsync(_wallet, identityJson);

            var activeListener = await AgentListener.ListenAsync(endpoint);

            await activeListener.AddIdentityAsync(_pool, _wallet, myDid.Did);

            var connection = await AgentConnection.ConnectAsync(_pool, _wallet, myDid.Did, myDid.Did);

            await connection.CloseAsync();

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                connection.SendAsync("msg")
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestAgentCloseConnectionWorksForIncoming()
        {
            var endpoint = "127.0.0.1:9613";

            var myDid = await Signus.CreateAndStoreMyDidAsync(_wallet, "{}");

            var identityJson = string.Format("{{\"did\":\"{0}\", \"pk\":\"{1}\", \"verkey\":\"{2}\", \"endpoint\":\"{3}\"}}",
                    myDid.Did, myDid.Pk, myDid.VerKey, endpoint);

            await Signus.StoreTheirDidAsync(_wallet, identityJson);

            var activeListener = await AgentListener.ListenAsync(endpoint);

            await activeListener.AddIdentityAsync(_pool, _wallet, myDid.Did);

            var connection = await AgentConnection.ConnectAsync(_pool, _wallet, myDid.Did, myDid.Did);

            var connectionEvent = await activeListener.WaitForConnection();
            var serverToClientConnection = connectionEvent.Connection;

            await serverToClientConnection.CloseAsync();

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                serverToClientConnection.SendAsync("msg")
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

    }
}

using Hyperledger.Indy.AgentApi;
using Hyperledger.Indy.SignusApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AgentTests
{
    [TestClass]
    public class DisposeAgentConnectionTest : AgentIntegrationTestBase
    {
        private string _myDid;
        private AgentListener _activeListener;

        public async Task PrepareForConnection(string endpoint)
        {
            var myDidResult = await Signus.CreateAndStoreMyDidAsync(wallet, "{}");
            _myDid = myDidResult.Did;

            var identityJson = string.Format(AGENT_IDENTITY_JSON_TEMPLATE, _myDid, myDidResult.Pk, myDidResult.VerKey, endpoint);
            await Signus.StoreTheirDidAsync(wallet, identityJson);

            _activeListener = await AgentListener.ListenAsync(endpoint);
            await _activeListener.AddIdentityAsync(pool, wallet, _myDid);
        }

        [TestCleanup]
        public async Task Cleanup()
        {
            if (_activeListener != null)
                await _activeListener.CloseAsync();
        }

        [TestMethod]
        public async Task CanDisposeClosedConnection()
        {
            await PrepareForConnection("127.0.0.1:9610");

            using (var connection = await AgentConnection.ConnectAsync(pool, wallet, _myDid, _myDid))
            {
                await connection.CloseAsync();
            }
        }

        [TestMethod]
        public async Task DisposeCanBeCalledRepeatedly()
        {
            await PrepareForConnection("127.0.0.1:9611");

            var connection = await AgentConnection.ConnectAsync(pool, wallet, _myDid, _myDid);
            connection.Dispose();
            connection.Dispose();
        }

        [TestMethod]
        [Ignore] //Appears endpoint cannot be re-connected to.  Requires further testing.
        public async Task EndpointCanBeReUsedAfterDispose()
        {
            await PrepareForConnection("127.0.0.1:9612");

            var connection = await AgentConnection.ConnectAsync(pool, wallet, _myDid, _myDid);
            await connection.CloseAsync();
            connection.Dispose();

            using (var newConnection = await AgentConnection.ConnectAsync(pool, wallet, _myDid, _myDid))
            {
            }
        }

        [TestMethod]
        [Ignore] //Wait until proper error is implemented in SDK and handle.
        public async Task CanCloseAfterDispose()
        {
            await PrepareForConnection("127.0.0.1:9618");

            var connection = await AgentConnection.ConnectAsync(pool, wallet, _myDid, _myDid);
            connection.Dispose();
            await connection.CloseAsync();
        }
    }
}
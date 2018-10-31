using Hyperledger.Indy.PaymentsApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.PairwiseTests
{
    [TestClass]
    public class ListPaymentAddressesTest : PaymentIntegrationTestBase
    {
        [TestMethod]
        public async Task TestListPaymentAddressesWorks()
        {
            var paymentAddressJson = await Payments.ListPaymentAddressesAsync(wallet);
            var paymentAddresses = JArray.Parse(paymentAddressJson);
            Assert.AreEqual(0, paymentAddresses.Count);
        }
    }
}

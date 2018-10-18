using Hyperledger.Indy.PaymentsApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.PairwiseTests
{
    [TestClass]
    public class ParsePaymentResponseTest : PaymentIntegrationTestBase
    {
        [TestMethod]
        public async Task TestParsePaymentResponseWorksForUnknownPaymentMethod()
        {
            var ex = await Assert.ThrowsExceptionAsync<UnknownPaymentMethodException>(() =>
                 Payments.ParsePaymentResponseAsync(paymentMethod, emptyObject)
             );
        }
    }
}

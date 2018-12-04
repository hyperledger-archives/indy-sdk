using Hyperledger.Indy.PaymentsApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.PairwiseTests
{
    [TestClass]
    public class ParseResponseWithFeesTest : PaymentIntegrationTestBase
    {
        [TestMethod]
        public async Task TestParseResponseWithFeesWorksForUnknownPaymentMethod()
        {
            var ex = await Assert.ThrowsExceptionAsync<UnknownPaymentMethodException>(() =>
                 Payments.ParseResponseWithFeesAsync(paymentMethod, emptyObject)
             );
        }
    }
}

using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace Hyperledger.Indy.Samples.Utils
{
    public static class Extensions
    {
        public static int MaxOrDefault(this IEnumerable<int> enumerable)
        {
            return enumerable.Any() ? enumerable.Max() : 0;
        }
    }
}

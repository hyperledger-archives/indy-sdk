// these are packages from .NET Core and other Nuget packages
using System;
using Newtonsoft.Json.Linq;

/*
Example demonstrating how to add DID with the role of Trust Anchor as Steward.

Uses seed to obtain Steward's DID which already exists on the ledger.
Then it generates new DID/Verkey pair for Trust Anchor.
Using Steward's DID, NYM transaction request is built to add Trust Anchor's DID and Verkey
on the ledger with the role of Trust Anchor.
Once the NYM is successfully written on the ledger, it generates new DID/Verkey pair that represents
a client, which are used to create GET_NYM request to query the ledger and confirm Trust Anchor's Verkey.

For the sake of simplicity, a single wallet is used. In the real world scenario, three different wallets
would be used and DIDs would be exchanged using some channel of communication
*/

// These packages are from the Hyperledger Nuget package
using Hyperledger.Indy.DidApi;
using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.WalletApi;

public class WriteDIDAndQueryVerkey
{
    public static void Demo()
    {

        Console.WriteLine("Step 1 -- set up some constants");

        string walletName = "myWallet";
        string poolName = "pool";
        string stewardSeed = "000000000000000000000000Steward1";
        string poolConfig = "{\"genesis_txn\": \"/home/vagrant/code/evernym/indy-sdk/cli/docker_pool_transactions_genesis\"}";

        // Step 2 code goes here.

        // Step 3 code goes here.

        // Step 4 code goes here.

        // Step 5 code goes here.
   
    }
}

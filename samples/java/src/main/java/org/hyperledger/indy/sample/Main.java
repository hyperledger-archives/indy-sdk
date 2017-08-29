package org.hyperledger.indy.sample;

import org.hyperledger.indy.sample.Tests.Agent;
import org.hyperledger.indy.sample.Tests.Anoncreds;
import org.hyperledger.indy.sample.Tests.Ledger;
import org.hyperledger.indy.sample.Tests.Signus;

public class Main {

	public static void main(String[] args) throws Exception {
		Agent.run();
		Anoncreds.run();
		Ledger.run();
		Signus.run();
	}
}

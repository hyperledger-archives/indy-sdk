		System.out.println("\n9. Build the SCHEMA request to add new schema to the ledger as a Steward\n");
		String name = "gvt";
		String version = "1.0";
		String attributes = "[\"age\", \"sex\", \"height\", \"name\"]";
		String schemaDataJSON = "{\"name\":\"" + name + "\",\"version\":\"" + version + "\",\"attr_names\":" + attributes + "}";
		System.out.println("Schema: " + schemaDataJSON);
		String schemaRequest = buildSchemaRequest(defaultStewardDid, schemaDataJSON).get();
		System.out.println("Schema request:\n" + schemaRequest);

		System.out.println("\n10. Sending the SCHEMA request to the ledger\n");
		String schemaResponse = signAndSubmitRequest(pool, walletHandle, defaultStewardDid, schemaRequest).get();
		System.out.println("Schema response:\n" + schemaResponse);

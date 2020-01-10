package com.evernym.sdk.vcx;

import com.evernym.sdk.vcx.schema.InvalidSchemahandleException;
import com.evernym.sdk.vcx.schema.SchemaApi;
import com.evernym.sdk.vcx.schema.SchemaPrepareForEndorserResult;
import com.evernym.sdk.vcx.vcx.VcxApi;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

import java.util.concurrent.ExecutionException;

public class SchemaApiTest {
    private String sourceId = "123";
    private String schemaName = "schema name";
    private String schemaVersion = "1.1.1";
    private String schemaData = "['attr1', 'attr2', 'height', 'weight']";
    private String endorser = "V4SGRU86Z58d6TV7PBUe6f";

    @BeforeEach
    void setup() throws Exception {
        System.setProperty(org.slf4j.impl.SimpleLogger.DEFAULT_LOG_LEVEL_KEY, "DEBUG");
        if (!TestHelper.vcxInitialized) {
            TestHelper.getResultFromFuture(VcxApi.vcxInit(TestHelper.VCX_CONFIG_TEST_MODE));
            TestHelper.vcxInitialized = true;
        }
    }

    @Test
    @DisplayName("create a schema")
    void createSchema() throws VcxException, ExecutionException, InterruptedException {
        int schemaHandle = TestHelper.getResultFromFuture(SchemaApi.schemaCreate(sourceId, schemaName, schemaVersion, TestHelper.convertToValidJson(schemaData), 0));
        assert (schemaHandle != 0);
    }

    @Test
    @DisplayName("serialise a schema")
    void serialiseSchema() throws VcxException, ExecutionException, InterruptedException {
        int schemaHandle = TestHelper.getResultFromFuture(SchemaApi.schemaCreate(sourceId, schemaName, schemaVersion, TestHelper.convertToValidJson(schemaData), 0));
        String serialisedSchema = TestHelper.getResultFromFuture(SchemaApi.schemaSerialize(schemaHandle));
        assert (serialisedSchema.contains(schemaName));
    }


    @Test
    @DisplayName("serialise a bad schema throws InvalidSchemahandleException")
    void serialiseBadSchema() {
        Assertions.assertThrows(InvalidSchemahandleException.class, () -> {
            TestHelper.getResultFromFuture(SchemaApi.schemaSerialize(242));

        });
    }

    @Test
    @DisplayName("deserialise a schema")
    void deserialiseSchema() throws VcxException, ExecutionException, InterruptedException {
        int schemaHandle = TestHelper.getResultFromFuture(SchemaApi.schemaCreate(sourceId, schemaName, schemaVersion, TestHelper.convertToValidJson(schemaData), 0));
        String serialisedSchema = TestHelper.getResultFromFuture(SchemaApi.schemaSerialize(schemaHandle));
        int deserilaisedSchemaHandle = TestHelper.getResultFromFuture(SchemaApi.schemaDeserialize(serialisedSchema));
        assert (deserilaisedSchemaHandle != 0);
    }

    @Test
    @DisplayName("get id from schema")
    void getId() throws VcxException, ExecutionException, InterruptedException {
        int schemaHandle = TestHelper.getResultFromFuture(SchemaApi.schemaCreate(sourceId, schemaName, schemaVersion, TestHelper.convertToValidJson(schemaData), 0));
        String schemaId = TestHelper.getResultFromFuture(SchemaApi.schemaGetSchemaId(schemaHandle));
        assert (!schemaId.equals(""));
    }

    @Test
    @DisplayName("get id from bad schema throws InvalidSchemaHandleException")
    void getIdWithError() {
        Assertions.assertThrows(InvalidSchemahandleException.class, () -> {
            TestHelper.getResultFromFuture(SchemaApi.schemaGetSchemaId(234));

        });
    }

    @Test
    @DisplayName("get attr from schema")
    void getAttributes() throws VcxException, ExecutionException, InterruptedException {
        int schemaHandle = TestHelper.getResultFromFuture(SchemaApi.schemaCreate(sourceId, schemaName, schemaVersion, TestHelper.convertToValidJson(schemaData), 0));
        String schemaId = TestHelper.getResultFromFuture(SchemaApi.schemaGetSchemaId(schemaHandle));
        String attr = TestHelper.getResultFromFuture(SchemaApi.schemaGetAttributes(sourceId, schemaId));
        assert (attr.contains("height"));
    }

    @Test
    @DisplayName("release schema")
    void releaseSchema() throws VcxException, ExecutionException, InterruptedException {
        int schemaHandle = TestHelper.getResultFromFuture(SchemaApi.schemaCreate(sourceId, schemaName, schemaVersion, TestHelper.convertToValidJson(schemaData), 0));
        int releaseHandle = SchemaApi.schemaRelease(schemaHandle);
        assert (releaseHandle == 0);
    }

    @Test
    @DisplayName("prepare a schema for endorser")
    void prepareSchemaForEndorser() throws VcxException, ExecutionException, InterruptedException {
        SchemaPrepareForEndorserResult schemaForEndorser = TestHelper.getResultFromFuture(SchemaApi.schemaPrepareForEndorser(sourceId, schemaName, schemaVersion, TestHelper.convertToValidJson(schemaData), endorser));
        assert (schemaForEndorser.getSchemaHandle() != 0);
        assert (!schemaForEndorser.getTransaction().isEmpty());
    }

    @Test
    @DisplayName("update schema state")
    void updateSchemaState() throws VcxException, ExecutionException, InterruptedException {
        SchemaPrepareForEndorserResult schemaForEndorser = TestHelper.getResultFromFuture(SchemaApi.schemaPrepareForEndorser(sourceId, schemaName, schemaVersion, TestHelper.convertToValidJson(schemaData), endorser));
        assert (schemaForEndorser.getSchemaHandle() != 0);
        assert (TestHelper.getResultFromFuture(SchemaApi.schemaGetState(schemaForEndorser.getSchemaHandle())) == 0);
        assert (TestHelper.getResultFromFuture(SchemaApi.schemaUpdateState(schemaForEndorser.getSchemaHandle())) == 1);
        assert (TestHelper.getResultFromFuture(SchemaApi.schemaGetState(schemaForEndorser.getSchemaHandle())) == 1);
    }
}

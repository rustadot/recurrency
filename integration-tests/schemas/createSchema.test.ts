import "@frequency-chain/api-augment";

import assert from "assert";

import { AVRO_GRAPH_CHANGE } from "./fixtures/avroGraphChangeSchemaType";
import { KeyringPair } from "@polkadot/keyring/types";
import { ExtrinsicHelper } from "../scaffolding/extrinsicHelpers";
import { createKeys, createAndFundKeypair, devAccounts } from "../scaffolding/helpers";

describe("#createSchema", function () {
    let keys: KeyringPair;
    let sudoKey: KeyringPair;
    let accountWithNoFunds: KeyringPair;

    before(async function () {
        // Using Alice as sudo key
        sudoKey = devAccounts[0].keys;
        keys = await createAndFundKeypair();
        accountWithNoFunds = createKeys();
    });

    it("should fail if account does not have enough tokens", async function () {

        await assert.rejects(ExtrinsicHelper.createSchema(accountWithNoFunds, AVRO_GRAPH_CHANGE, "AvroBinary", "OnChain").signAndSend(), {
            name: 'RpcError',
            message: /Inability to pay some fees/,
        });
    });

    it("should fail to create invalid schema", async function () {
        const f = ExtrinsicHelper.createSchema(keys, new Array(1000, 3), "AvroBinary", "OnChain");

        await assert.rejects(f.fundAndSend(), {
            name: 'InvalidSchema',
        });
    });

    it("should fail to create schema less than minimum size", async function () {
        const f = ExtrinsicHelper.createSchema(keys, {}, "AvroBinary", "OnChain");
        await assert.rejects(f.fundAndSend(), {
            name: 'LessThanMinSchemaModelBytes',
        });
    });

    it("should fail to create schema greater than maximum size", async function () {
        const maxBytes = (await ExtrinsicHelper.getSchemaMaxBytes()).toNumber();

        // Create a schema whose JSON representation is exactly 1 byte larger than the max allowed
        const hugeSchema = {
            type: "record",
            fields: [],
        }
        const hugeSize = JSON.stringify(hugeSchema).length;
        const sizeToFill = maxBytes - hugeSize - ',"name":""'.length + 1;
        hugeSchema["name"] = Array.from(Array(sizeToFill).keys()).map(i => 'a').join('');

        const f = ExtrinsicHelper.createSchema(keys, hugeSchema, "AvroBinary", "OnChain");
        await assert.rejects(f.fundAndSend(), {
            name: 'ExceedsMaxSchemaModelBytes',
        });
    });

    it("should successfully create an Avro GraphChange schema", async function () {
        const f = ExtrinsicHelper.createSchema(keys, AVRO_GRAPH_CHANGE, "AvroBinary", "OnChain");
        const [createSchemaEvent, eventMap] = await f.fundAndSend();

        assert.notEqual(eventMap["system.ExtrinsicSuccess"], undefined);
        assert.notEqual(createSchemaEvent, undefined);
    });

    it("should fail to create non itemized schema with AppendOnly settings", async function () {
      const ex = ExtrinsicHelper.createSchemaWithSettingsGov(keys, sudoKey, AVRO_GRAPH_CHANGE, "AvroBinary", "Paginated", "AppendOnly");
      await assert.rejects(ex.sudoSignAndSend(), {
        name: 'InvalidSetting'
      });
    });
})

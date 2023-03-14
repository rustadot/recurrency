// Integration tests for pallets/stateful-pallet-storage/handleItemized.ts
import "@frequency-chain/api-augment";
import assert from "assert";
import {createDelegatorAndDelegation, createProviderKeysAndId, getCurrentItemizedHash} from "../scaffolding/helpers";
import { KeyringPair } from "@polkadot/keyring/types";
import { ExtrinsicHelper } from "../scaffolding/extrinsicHelpers";
import { AVRO_CHAT_MESSAGE } from "../stateful-pallet-storage/fixtures/itemizedSchemaType";
import { MessageSourceId, PageHash, SchemaId } from "@frequency-chain/api-augment/interfaces";
import { Bytes, u16, u64 } from "@polkadot/types";

describe("📗 Stateful Pallet Storage", () => {
    let schemaId_deletable: SchemaId;
    let schemaId_unsupported: SchemaId;
    let msa_id: MessageSourceId;
    let providerId: MessageSourceId;
    let providerKeys: KeyringPair;

    before(async function () {

        // Create a provider for the MSA, the provider will be used to grant delegation
        [providerKeys, providerId] = await createProviderKeysAndId();
        assert.notEqual(providerId, undefined, "setup should populate providerId");
        assert.notEqual(providerKeys, undefined, "setup should populate providerKeys");

        // Create a schema to allow delete actions
        const createSchemaDeletable = ExtrinsicHelper.createSchema(providerKeys, AVRO_CHAT_MESSAGE, "AvroBinary", "Itemized");
        const [eventDeletable] = await createSchemaDeletable.fundAndSend();
        if (eventDeletable && createSchemaDeletable.api.events.schemas.SchemaCreated.is(eventDeletable)) {
            schemaId_deletable = eventDeletable.data.schemaId;
        }
        assert.notEqual(schemaId_deletable, undefined, "setup should populate schemaId");

        // Create non supported schema
        const createSchema2 = ExtrinsicHelper.createSchema(providerKeys, AVRO_CHAT_MESSAGE, "AvroBinary", "OnChain");
        const [event2] = await createSchema2.fundAndSend();
        assert.notEqual(event2, undefined, "setup should return a SchemaCreated event");
        if (event2 && createSchema2.api.events.schemas.SchemaCreated.is(event2)) {
            schemaId_unsupported = event2.data.schemaId;
            assert.notEqual(schemaId_unsupported, undefined, "setup should populate schemaId_unsupported");
        }

        // Create a MSA for the delegator and delegate to the provider
        [, msa_id] = await createDelegatorAndDelegation(schemaId_deletable, providerId, providerKeys);
        assert.notEqual(msa_id, undefined, "setup should populate msa_id");
    });

    describe("Itemized Storage Tests 😊/😥", () => {

        it("✅ should be able to call applyItemizedAction and apply actions", async function () {

            // Add and update actions
            let payload_1 = new Bytes(ExtrinsicHelper.api.registry, "Hello World From Frequency");

            const add_action = {
                "Add": payload_1
            }

            let payload_2 = new Bytes(ExtrinsicHelper.api.registry, "Hello World Again From Frequency");

            const update_action = {
                "Add": payload_2
            }

            const target_hash = await getCurrentItemizedHash(msa_id, schemaId_deletable);

            let add_actions = [add_action, update_action];
            let itemized_add_result_1 = ExtrinsicHelper.applyItemActions(providerKeys, schemaId_deletable, msa_id, add_actions, target_hash);
            const [pageUpdateEvent1, chainEvents] = await itemized_add_result_1.fundAndSend();
            assert.notEqual(chainEvents["system.ExtrinsicSuccess"], undefined, "should have returned an ExtrinsicSuccess event");
            assert.notEqual(chainEvents["transactionPayment.TransactionFeePaid"], undefined, "should have returned a TransactionFeePaid event");
            assert.notEqual(pageUpdateEvent1, undefined, "should have returned a PalletStatefulStorageItemizedActionApplied event");
        }).timeout(10000);

        it("🛑 should fail call to applyItemizedAction with invalid schemaId", async function () {

            let payload_1 = new Bytes(ExtrinsicHelper.api.registry, "Hello World From Frequency");
            const add_action = {
                "Add": payload_1
            }
            let add_actions = [add_action];
            let fake_schema_id = new u16(ExtrinsicHelper.api.registry, 999);
            let itemized_add_result_1 = ExtrinsicHelper.applyItemActions(providerKeys, fake_schema_id, msa_id, add_actions, 0);
            await assert.rejects(async () => {
                await itemized_add_result_1.fundAndSend();
            }, {
                name: 'InvalidSchemaId',
                section: 'statefulStorage',
            });
        }).timeout(10000);

        it("🛑 should fail call to applyItemizedAction with invalid schema location", async function () {

            let payload_1 = new Bytes(ExtrinsicHelper.api.registry, "Hello World From Frequency");
            const add_action = {
                "Add": payload_1
            }
            let add_actions = [add_action];
            let itemized_add_result_1 = ExtrinsicHelper.applyItemActions(providerKeys, schemaId_unsupported, msa_id, add_actions, 0);
            await assert.rejects(async () => {
                await itemized_add_result_1.fundAndSend();
            }, {
                name: 'SchemaPayloadLocationMismatch',
                section: 'statefulStorage',
            });
        }).timeout(10000);

        it("🛑 should fail call to applyItemizedAction with for un-delegated attempts", async function () {

            let payload_1 = new Bytes(ExtrinsicHelper.api.registry, "Hello World From Frequency");
            const add_action = {
                "Add": payload_1
            }
            let add_actions = [add_action];
            let bad_msa_id = new u64(ExtrinsicHelper.api.registry, 999)

            let itemized_add_result_1 = ExtrinsicHelper.applyItemActions(providerKeys, schemaId_deletable, bad_msa_id, add_actions, 0);
            await assert.rejects(async () => {
                await itemized_add_result_1.fundAndSend();
            }, {
                name: 'UnauthorizedDelegate',
                section: 'statefulStorage',
            });
        }).timeout(10000);

        it("🛑 should fail call to applyItemizedAction for target hash mismatch", async function () {

            // Add and update actions
            let payload_1 = new Bytes(ExtrinsicHelper.api.registry, "Hello World From Frequency");

            const add_action = {
                "Add": payload_1
            }

            let payload_2 = new Bytes(ExtrinsicHelper.api.registry, "Hello World Again From Frequency");

            const update_action = {
                "Add": payload_2
            }

            let add_actions = [add_action, update_action];
            let itemized_add_result_1 = ExtrinsicHelper.applyItemActions(providerKeys, schemaId_deletable, msa_id, add_actions, 0);
            await assert.rejects(itemized_add_result_1.fundAndSend(), { name: 'StalePageState' });
        }).timeout(10000);
    });

    describe("Itemized Storage Remove Action Tests", () => {

        it("✅ should be able to call applyItemizedAction and apply remove actions", async function () {
            let target_hash = await getCurrentItemizedHash(msa_id, schemaId_deletable);

            // Delete action
            const idx_1: u16 = new u16(ExtrinsicHelper.api.registry, 1)
            const remove_action_1 = {
                "Delete": idx_1,
            }

            target_hash = await getCurrentItemizedHash(msa_id, schemaId_deletable);

            let remove_actions = [remove_action_1];
            let itemized_remove_result_1 = ExtrinsicHelper.applyItemActions(providerKeys, schemaId_deletable, msa_id, remove_actions, target_hash);
            const [pageUpdateEvent2, chainEvents2] = await itemized_remove_result_1.fundAndSend();
            assert.notEqual(chainEvents2["system.ExtrinsicSuccess"], undefined, "should have returned an ExtrinsicSuccess event");
            assert.notEqual(chainEvents2["transactionPayment.TransactionFeePaid"], undefined, "should have returned a TransactionFeePaid event");
            assert.notEqual(pageUpdateEvent2, undefined, "should have returned a event");
        }).timeout(10000);

        it("🛑 should fail call to remove action with invalid schemaId", async function () {
            const idx_1: u16 = new u16(ExtrinsicHelper.api.registry, 1)
            const remove_action_1 = {
                "Delete": idx_1,
            }
            let remove_actions = [remove_action_1];
            let fake_schema_id = new u16(ExtrinsicHelper.api.registry, 999);
            let itemized_remove_result_1 = ExtrinsicHelper.applyItemActions(providerKeys, fake_schema_id, msa_id, remove_actions, 0);
            await assert.rejects(async () => {
                await itemized_remove_result_1.fundAndSend();
            }, {
                name: 'InvalidSchemaId',
                section: 'statefulStorage',
            });
        }).timeout(10000);

        it("🛑 should fail call to remove action with invalid schema location", async function () {
            const idx_1: u16 = new u16(ExtrinsicHelper.api.registry, 1)
            const remove_action_1 = {
                "Delete": idx_1,
            }
            let remove_actions = [remove_action_1];
            let itemized_remove_result_1 = ExtrinsicHelper.applyItemActions(providerKeys, schemaId_unsupported, msa_id, remove_actions, 0);
            await assert.rejects(async () => {
                await itemized_remove_result_1.fundAndSend();
            }, {
                name: 'SchemaPayloadLocationMismatch',
                section: 'statefulStorage',
            });
        }).timeout(10000);

        it("🛑 should fail call to remove action with invalid msa_id", async function () {
            const idx_1: u16 = new u16(ExtrinsicHelper.api.registry, 1)
            const remove_action_1 = {
                "Delete": idx_1,
            }
            let remove_actions = [remove_action_1];
            let bad_msa_id = new u64(ExtrinsicHelper.api.registry, 999)
            let itemized_remove_result_1 = ExtrinsicHelper.applyItemActions(providerKeys, schemaId_deletable, bad_msa_id, remove_actions, 0);
            await assert.rejects(async () => {
                await itemized_remove_result_1.fundAndSend();
            }, {
                name: 'UnauthorizedDelegate',
                section: 'statefulStorage',
            });
        }).timeout(10000);

        it("🛑 should fail call to remove action with stale state hash", async function () {
            const idx_1: u16 = new u16(ExtrinsicHelper.api.registry, 1);
            const remove_action = {
                "Delete": idx_1,
            }
            let remove_actions = [remove_action];
            let op = ExtrinsicHelper.applyItemActions(providerKeys, schemaId_deletable, msa_id, remove_actions, 0);
            await assert.rejects(op.fundAndSend(), { name: 'StalePageState' })
        }).timeout(10000);
    });

    describe("Itemized Storage RPC Tests", () => {
        it("✅ should be able to call getItemizedStorage and get data for itemized schema", async function () {
            const result = await ExtrinsicHelper.getItemizedStorage(msa_id, schemaId_deletable);
            assert.notEqual(result.hash, undefined, "should have returned a hash");
            assert.notEqual(result.size, undefined, "should have returned a itemized responses");
        }).timeout(10000);
    });
});

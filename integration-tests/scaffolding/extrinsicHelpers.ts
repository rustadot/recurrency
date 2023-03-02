import { ApiPromise, ApiRx } from "@polkadot/api";
import { ApiTypes, AugmentedEvent, SubmittableExtrinsic } from "@polkadot/api/types";
import { KeyringPair } from "@polkadot/keyring/types";
import { Compact, u128, u16, u64 } from "@polkadot/types";
import { FrameSystemAccountInfo } from "@polkadot/types/lookup";
import { AnyNumber, AnyTuple, Codec, IEvent, ISubmittableResult } from "@polkadot/types/types";
import { firstValueFrom, filter, map, pipe, tap } from "rxjs";
import { devAccounts, log, Sr25519Signature } from "./helpers";
import { connect, connectPromise } from "./apiConnection";
import { DispatchError, Event, SignedBlock } from "@polkadot/types/interfaces";
import { IsEvent } from "@polkadot/types/metadata/decorate/types";

export type AddKeyData = { msaId?: u64; expiration?: any; newPublicKey?: any; }
export type AddProviderPayload = { authorizedMsaId?: u64; schemaIds?: u16[], expiration?: any; }

export class EventError extends Error {
    name: string = '';
    message: string = '';
    stack?: string = '';
    section?: string = '';
    rawError: DispatchError;

    constructor(source: DispatchError) {
        super();

        if (source.isModule) {
            const decoded = source.registry.findMetaError(source.asModule);
            this.name = decoded.name;
            this.message = decoded.docs.join(' ');
            this.section = decoded.section;
        } else {
            this.name = source.type;
            this.message = source.type;
            this.section = '';
        }
        this.rawError = source;
    }

    public toString() {
        return `${this.section}.${this.name}: ${this.message}`;
    }
}

export type EventMap = { [key: string]: Event }

function eventKey(event: Event): string {
    return `${event.section}.${event.method}`;
}

/**
 * These helpers return a map of events, some of which contain useful data, some of which don't.
 * Extrinsics that "create" records typically contain an ID of the entity they created, and this
 * would be a useful value to return. However, this data seems to be nested inside an array of arrays.
 *
 * Ex: schemaId = events["schemas.SchemaCreated"][<arbitrary_index>]
 *
 * To get the value associated with an event key, we would need to query inside that nested array with
 * a set of arbitrary indices. Should an object at any level of that querying be undefined, the helper
 * will throw an unchecked exception.
 *
 * To get type checking and cast a returned event as a specific event type, you can utilize TypeScripts
 * type guard functionality like so:
 *
 *      const msaCreatedEvent = events.defaultEvent;
 *      if (ExtrinsicHelper.api.events.msa.MsaCreated.is(msaCreatedEvent)) {
 *          msaId = msaCreatedEvent.data.msaId;
 *      }
 *
 * Normally, I'd say the best experience is for the helper to return both the ID of the created entity
 * along with a map of emitted events. But in this case, returning that value will increase the complexity
 * of each helper, since each would have to check for undefined values at every lookup. So, this may be
 * a rare case when it is best to simply return the map of emitted events and trust the user to look them
 * up in the test.
 */

type ParsedEvent<C extends Codec[] = Codec[], N = unknown> = IEvent<C, N>;
export type ParsedEventResult<C extends Codec[] = Codec[], N = unknown> = [ParsedEvent<C, N> | undefined, EventMap];


export class Extrinsic<T extends ISubmittableResult = ISubmittableResult, C extends Codec[] = Codec[], N = unknown> {

    private event?: IsEvent<C, N>;
    private extrinsic: () => SubmittableExtrinsic<"rxjs", T>;
    private keys: KeyringPair;
    public api: ApiRx;

    constructor(extrinsic: () => SubmittableExtrinsic<"rxjs", T>, keys: KeyringPair, targetEvent?: IsEvent<C, N>) {
        this.extrinsic = extrinsic;
        this.keys = keys;
        this.event = targetEvent;
        this.api = ExtrinsicHelper.api;
    }

    public signAndSend(nonce?: number): Promise<ParsedEventResult> {
        return firstValueFrom(this.extrinsic().signAndSend(this.keys, {nonce: nonce}).pipe(
            filter(({ status }) => status.isInBlock || status.isFinalized),
            this.parseResult(this.event),
        ))
    }

    public sudoSignAndSend(): Promise<[ParsedEvent<C, N> | undefined, EventMap]> {
        return firstValueFrom(this.api.tx.sudo.sudo(this.extrinsic()).signAndSend(this.keys).pipe(
            filter(({ status }) => status.isInBlock || status.isFinalized),
            this.parseResult(this.event),
        ))
    }

    public getEstimatedTxFee(): Promise<bigint> {
        return firstValueFrom(this.extrinsic().paymentInfo(this.keys).pipe(
            map((info) => info.partialFee.toBigInt())
        ));
    }

    public async fundOperation(source?: KeyringPair, nonce?: number): Promise<void> {
        const amount = await this.getEstimatedTxFee();
        await ExtrinsicHelper.transferFunds(source || devAccounts[0].keys, this.keys, amount).signAndSend(nonce);
    }

    public async fundAndSend(source?: KeyringPair, nonce?: number): Promise<ParsedEventResult> {
        await this.fundOperation(source);
        return this.signAndSend(nonce);
    }

    private parseResult<ApiType extends ApiTypes = "rxjs", T extends AnyTuple = AnyTuple, N = unknown>(targetEvent?: AugmentedEvent<ApiType, T, N>) {
        return pipe(
            tap((result: ISubmittableResult) => {
                if (result.dispatchError) {
                    let err = new EventError(result.dispatchError);
                    log(err.toString());
                    throw err;
                }
            }),
            map((result: ISubmittableResult) => result.events.reduce((acc, { event }) => {
                acc[eventKey(event)] = event;
                if (targetEvent && targetEvent.is(event)) {
                    acc["defaultEvent"] = event;
                }
                return acc;
            }, {} as EventMap)),
            map((em) => {
                let result: ParsedEventResult<T, N> = [undefined, {}];
                if (targetEvent && targetEvent.is(em?.defaultEvent)) {
                    result[0] = em.defaultEvent;
                }
                result[1] = em;
                return result;
            }),
            tap((events) => log(events)),
        );
    }

}

export class ExtrinsicHelper {
    public static api: ApiRx;
    public static apiPromise: ApiPromise;

    constructor() { }

    public static async initialize(providerUrl?: string | string[] | undefined) {
        ExtrinsicHelper.api = await connect(providerUrl);
        // For single state queries (api.query), ApiPromise is better
        ExtrinsicHelper.apiPromise = await connectPromise(providerUrl);
    }

    public static getLastBlock(): Promise<SignedBlock> {
        return firstValueFrom(ExtrinsicHelper.api.rpc.chain.getBlock());
    }

    /** Query Extrinsics */
    public static getAccountInfo(address: string): Promise<FrameSystemAccountInfo> {
        return ExtrinsicHelper.apiPromise.query.system.account(address);
    }

    public static getSchemaMaxBytes() {
        return ExtrinsicHelper.apiPromise.query.schemas.governanceSchemaModelMaxBytes();
    }

    /** Balance Extrinsics */
    public static transferFunds(keys: KeyringPair, dest: KeyringPair, amount: Compact<u128> | AnyNumber): Extrinsic {
        return new Extrinsic(() => ExtrinsicHelper.api.tx.balances.transfer(dest.address, amount), keys, ExtrinsicHelper.api.events.balances.Transfer);
    }

    /** Schema Extrinsics */
    public static createSchema(keys: KeyringPair, model: any, modelType: "AvroBinary" | "Parquet", payloadLocation: "OnChain" | "IPFS"): Extrinsic {
        return new Extrinsic(() => ExtrinsicHelper.api.tx.schemas.createSchema(JSON.stringify(model), modelType, payloadLocation), keys, ExtrinsicHelper.api.events.schemas.SchemaCreated);
    }

    /** MSA Extrinsics */
    public static createMsa(keys: KeyringPair): Extrinsic {
        return new Extrinsic(() => ExtrinsicHelper.api.tx.msa.create(), keys, ExtrinsicHelper.api.events.msa.MsaCreated);
    }

    public static addPublicKeyToMsa(keys: KeyringPair, ownerSignature: Sr25519Signature, newSignature: Sr25519Signature, payload: AddKeyData): Extrinsic {
        return new Extrinsic(() => ExtrinsicHelper.api.tx.msa.addPublicKeyToMsa(keys.publicKey, ownerSignature, newSignature, payload), keys, ExtrinsicHelper.api.events.msa.PublicKeyAdded);
    }

    public static deletePublicKey(keys: KeyringPair, publicKey: Uint8Array): Extrinsic {
        ExtrinsicHelper.api.query.msa
        return new Extrinsic(() => ExtrinsicHelper.api.tx.msa.deleteMsaPublicKey(publicKey), keys, ExtrinsicHelper.api.events.msa.PublicKeyDeleted);
    }

    public static retireMsa(keys: KeyringPair): Extrinsic {
        return new Extrinsic(() => ExtrinsicHelper.api.tx.msa.retireMsa(), keys, ExtrinsicHelper.api.events.msa.MsaRetired);
    }

    public static createProvider(keys: KeyringPair, providerName: string): Extrinsic {
        return new Extrinsic(() => ExtrinsicHelper.api.tx.msa.createProvider(providerName), keys, ExtrinsicHelper.api.events.msa.ProviderCreated);
    }

    public static createSponsoredAccountWithDelegation(delegatorKeys: KeyringPair, providerKeys: KeyringPair, signature: Sr25519Signature, payload: AddProviderPayload): Extrinsic {
        return new Extrinsic(() => ExtrinsicHelper.api.tx.msa.createSponsoredAccountWithDelegation(delegatorKeys.publicKey, signature, payload), providerKeys, ExtrinsicHelper.api.events.msa.MsaCreated);
    }

    public static grantDelegation(delegatorKeys: KeyringPair, providerKeys: KeyringPair, signature: Sr25519Signature, payload: AddProviderPayload): Extrinsic {
        return new Extrinsic(() => ExtrinsicHelper.api.tx.msa.grantDelegation(delegatorKeys.publicKey, signature, payload), providerKeys, ExtrinsicHelper.api.events.msa.DelegationGranted);
    }

    public static grantSchemaPermissions(delegatorKeys: KeyringPair, providerMsaId: any, schemaIds: any): Extrinsic {
        return new Extrinsic(() => ExtrinsicHelper.api.tx.msa.grantSchemaPermissions(providerMsaId, schemaIds), delegatorKeys, ExtrinsicHelper.api.events.msa.DelegationUpdated);
    }

    public static revokeSchemaPermissions(delegatorKeys: KeyringPair, providerMsaId: any, schemaIds: any): Extrinsic {
        return new Extrinsic(() => ExtrinsicHelper.api.tx.msa.revokeSchemaPermissions(providerMsaId, schemaIds), delegatorKeys, ExtrinsicHelper.api.events.msa.DelegationUpdated);
    }

    public static revokeDelegationByDelegator(keys: KeyringPair, providerMsaId: any): Extrinsic {
        return new Extrinsic(() => ExtrinsicHelper.api.tx.msa.revokeDelegationByDelegator(providerMsaId), keys, ExtrinsicHelper.api.events.msa.DelegationRevoked);
    }

    public static revokeDelegationByProvider(delegatorMsaId: u64, providerKeys: KeyringPair): Extrinsic {
        return new Extrinsic(() => ExtrinsicHelper.api.tx.msa.revokeDelegationByProvider(delegatorMsaId), providerKeys, ExtrinsicHelper.api.events.msa.DelegationRevoked);
    }

    /** Messages Extrinsics */
    public static addIPFSMessage(keys: KeyringPair, schemaId: any, cid: string, payload_length: number): Extrinsic {
        return new Extrinsic(() => ExtrinsicHelper.api.tx.messages.addIpfsMessage(schemaId, cid, payload_length), keys, ExtrinsicHelper.api.events.messages.MessagesStored);
    }
}

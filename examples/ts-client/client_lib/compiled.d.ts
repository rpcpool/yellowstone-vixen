import * as $protobuf from "protobufjs";
import Long = require("long");
export namespace vixen {

    namespace stream {

        class ProgramStreams extends $protobuf.rpc.Service {
            constructor(rpcImpl: $protobuf.RPCImpl, requestDelimited?: boolean, responseDelimited?: boolean);
            public static create(rpcImpl: $protobuf.RPCImpl, requestDelimited?: boolean, responseDelimited?: boolean): ProgramStreams;
            public subscribe(request: vixen.stream.ISubscribeRequest, callback: vixen.stream.ProgramStreams.SubscribeCallback): void;
            public subscribe(request: vixen.stream.ISubscribeRequest): Promise<vixen.stream.SubscribeUpdate>;
        }

        namespace ProgramStreams {

            type SubscribeCallback = (error: (Error|null), response?: vixen.stream.SubscribeUpdate) => void;
        }

        interface ISubscribeRequest {
            program?: (string|null);
        }

        class SubscribeRequest implements ISubscribeRequest {
            constructor(properties?: vixen.stream.ISubscribeRequest);
            public program: string;
            public static create(properties?: vixen.stream.ISubscribeRequest): vixen.stream.SubscribeRequest;
            public static encode(message: vixen.stream.ISubscribeRequest, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.stream.ISubscribeRequest, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.stream.SubscribeRequest;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.stream.SubscribeRequest;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.stream.SubscribeRequest;
            public static toObject(message: vixen.stream.SubscribeRequest, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ISubscribeUpdate {
            parsed?: (google.protobuf.IAny|null);
        }

        class SubscribeUpdate implements ISubscribeUpdate {
            constructor(properties?: vixen.stream.ISubscribeUpdate);
            public parsed?: (google.protobuf.IAny|null);
            public static create(properties?: vixen.stream.ISubscribeUpdate): vixen.stream.SubscribeUpdate;
            public static encode(message: vixen.stream.ISubscribeUpdate, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.stream.ISubscribeUpdate, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.stream.SubscribeUpdate;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.stream.SubscribeUpdate;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.stream.SubscribeUpdate;
            public static toObject(message: vixen.stream.SubscribeUpdate, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }
    }

    namespace parser {

        interface ITokenProgramStateProto {
            tokenAccount?: (vixen.parser.ITokenAccountProto|null);
            mint?: (vixen.parser.IMintProto|null);
            multisig?: (vixen.parser.IMultisigProto|null);
        }

        class TokenProgramStateProto implements ITokenProgramStateProto {
            constructor(properties?: vixen.parser.ITokenProgramStateProto);
            public tokenAccount?: (vixen.parser.ITokenAccountProto|null);
            public mint?: (vixen.parser.IMintProto|null);
            public multisig?: (vixen.parser.IMultisigProto|null);
            public stateOneof?: ("tokenAccount"|"mint"|"multisig");
            public static create(properties?: vixen.parser.ITokenProgramStateProto): vixen.parser.TokenProgramStateProto;
            public static encode(message: vixen.parser.ITokenProgramStateProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ITokenProgramStateProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.TokenProgramStateProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.TokenProgramStateProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.TokenProgramStateProto;
            public static toObject(message: vixen.parser.TokenProgramStateProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ITokenProgramIxProto {
            transfer?: (vixen.parser.ITransferIxProto|null);
            initializeMint?: (vixen.parser.IInitializeMintIxProto|null);
            initializeAccount?: (vixen.parser.IInitializeAccountIxProto|null);
            initializeAccount2?: (vixen.parser.IInitializeAccount2IxProto|null);
            initializeAccount3?: (vixen.parser.IInitializeAccount3IxProto|null);
            initializeMultisig?: (vixen.parser.IInitializeMultisigIxProto|null);
            approve?: (vixen.parser.IApproveIxProto|null);
            revoke?: (vixen.parser.IRevokeIxProto|null);
            setAuthority?: (vixen.parser.ISetAuthorityIxProto|null);
            mintTo?: (vixen.parser.IMintToIxProto|null);
            burn?: (vixen.parser.IBurnIxProto|null);
            closeAccount?: (vixen.parser.ICloseAccountIxProto|null);
            freezeAccount?: (vixen.parser.IFreezeAccountIxProto|null);
            thawAccount?: (vixen.parser.IThawAccountIxProto|null);
            transferChecked?: (vixen.parser.ITransferCheckedIxProto|null);
            approveChecked?: (vixen.parser.IApproveCheckedIxProto|null);
            mintToChecked?: (vixen.parser.IMintToCheckedIxProto|null);
            burnChecked?: (vixen.parser.IBurnCheckedIxProto|null);
            syncNative?: (vixen.parser.ISyncNativeIxProto|null);
            getAccountDataSize?: (vixen.parser.IGetAccountDataSizeIxProto|null);
            initializeImmutableOwner?: (vixen.parser.IInitializeImmutableOwnerIxProto|null);
            amountToUiAmount?: (vixen.parser.IAmountToUiAmountIxProto|null);
            uiAmountToAmount?: (vixen.parser.IUiAmountToAmountIxProto|null);
        }

        class TokenProgramIxProto implements ITokenProgramIxProto {
            constructor(properties?: vixen.parser.ITokenProgramIxProto);
            public transfer?: (vixen.parser.ITransferIxProto|null);
            public initializeMint?: (vixen.parser.IInitializeMintIxProto|null);
            public initializeAccount?: (vixen.parser.IInitializeAccountIxProto|null);
            public initializeAccount2?: (vixen.parser.IInitializeAccount2IxProto|null);
            public initializeAccount3?: (vixen.parser.IInitializeAccount3IxProto|null);
            public initializeMultisig?: (vixen.parser.IInitializeMultisigIxProto|null);
            public approve?: (vixen.parser.IApproveIxProto|null);
            public revoke?: (vixen.parser.IRevokeIxProto|null);
            public setAuthority?: (vixen.parser.ISetAuthorityIxProto|null);
            public mintTo?: (vixen.parser.IMintToIxProto|null);
            public burn?: (vixen.parser.IBurnIxProto|null);
            public closeAccount?: (vixen.parser.ICloseAccountIxProto|null);
            public freezeAccount?: (vixen.parser.IFreezeAccountIxProto|null);
            public thawAccount?: (vixen.parser.IThawAccountIxProto|null);
            public transferChecked?: (vixen.parser.ITransferCheckedIxProto|null);
            public approveChecked?: (vixen.parser.IApproveCheckedIxProto|null);
            public mintToChecked?: (vixen.parser.IMintToCheckedIxProto|null);
            public burnChecked?: (vixen.parser.IBurnCheckedIxProto|null);
            public syncNative?: (vixen.parser.ISyncNativeIxProto|null);
            public getAccountDataSize?: (vixen.parser.IGetAccountDataSizeIxProto|null);
            public initializeImmutableOwner?: (vixen.parser.IInitializeImmutableOwnerIxProto|null);
            public amountToUiAmount?: (vixen.parser.IAmountToUiAmountIxProto|null);
            public uiAmountToAmount?: (vixen.parser.IUiAmountToAmountIxProto|null);
            public ixOneof?: ("transfer"|"initializeMint"|"initializeAccount"|"initializeAccount2"|"initializeAccount3"|"initializeMultisig"|"approve"|"revoke"|"setAuthority"|"mintTo"|"burn"|"closeAccount"|"freezeAccount"|"thawAccount"|"transferChecked"|"approveChecked"|"mintToChecked"|"burnChecked"|"syncNative"|"getAccountDataSize"|"initializeImmutableOwner"|"amountToUiAmount"|"uiAmountToAmount");
            public static create(properties?: vixen.parser.ITokenProgramIxProto): vixen.parser.TokenProgramIxProto;
            public static encode(message: vixen.parser.ITokenProgramIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ITokenProgramIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.TokenProgramIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.TokenProgramIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.TokenProgramIxProto;
            public static toObject(message: vixen.parser.TokenProgramIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ITokenExtensionStateProto {
            extendedTokenAccount?: (vixen.parser.IExtendedTokenAccountProto|null);
            extendedMintAccount?: (vixen.parser.IExtendedMintProto|null);
            multisig?: (vixen.parser.IMultisigProto|null);
        }

        class TokenExtensionStateProto implements ITokenExtensionStateProto {
            constructor(properties?: vixen.parser.ITokenExtensionStateProto);
            public extendedTokenAccount?: (vixen.parser.IExtendedTokenAccountProto|null);
            public extendedMintAccount?: (vixen.parser.IExtendedMintProto|null);
            public multisig?: (vixen.parser.IMultisigProto|null);
            public stateOneof?: ("extendedTokenAccount"|"extendedMintAccount"|"multisig");
            public static create(properties?: vixen.parser.ITokenExtensionStateProto): vixen.parser.TokenExtensionStateProto;
            public static encode(message: vixen.parser.ITokenExtensionStateProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ITokenExtensionStateProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.TokenExtensionStateProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.TokenExtensionStateProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.TokenExtensionStateProto;
            public static toObject(message: vixen.parser.TokenExtensionStateProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ITokenExtensionProgramIxProto {
            tokenProgramIx?: (vixen.parser.ITokenProgramIxProto|null);
            transferFeeIx?: (vixen.parser.ITransferFeeIxProto|null);
            tokenMetadataIx?: (vixen.parser.ITokenMetadataIxProto|null);
            tokenGroupIx?: (vixen.parser.ITokenGroupIxProto|null);
            confidentialTransferIx?: (vixen.parser.IConfidentialTransferExtIxProto|null);
            confidentialTransferFeeIx?: (vixen.parser.IConfidentialTransferFeeIxProto|null);
            cpiGuardIx?: (vixen.parser.ICpiGuardIxProto|null);
            transferHookIx?: (vixen.parser.ITransferHookIxProto|null);
            metadataPointerIx?: (vixen.parser.IMetadataPointerIxProto|null);
            memoTransferIx?: (vixen.parser.IMemoTransferIxProto|null);
            interestBearingMintIx?: (vixen.parser.IInterestBearingMintIxProto|null);
            defaultAccountStateIx?: (vixen.parser.IDefaultAccountStateIxProto|null);
            groupMemberPointerIx?: (vixen.parser.IGroupMemberPointerIxProto|null);
            groupPointerIx?: (vixen.parser.IGroupPointerIxProto|null);
            withdrawExcessLamportsIx?: (vixen.parser.IWithdrawExcessLamportsIxProto|null);
            initializePermanentDelegateIx?: (vixen.parser.IInitializePermanentDelegateIxProto|null);
            reallocateIx?: (vixen.parser.IReallocateIxProto|null);
            initializeNonTransferableMintIx?: (vixen.parser.IInitializeNonTransferableMintIxProto|null);
            initializeMintCloseAuthorityIx?: (vixen.parser.IInitializeMintCloseAuthorityIxProto|null);
            createNativeMintIx?: (vixen.parser.ICreateNativeMintIxProto|null);
            setAuthority?: (vixen.parser.ISetAuthorityIxProto|null);
        }

        class TokenExtensionProgramIxProto implements ITokenExtensionProgramIxProto {
            constructor(properties?: vixen.parser.ITokenExtensionProgramIxProto);
            public tokenProgramIx?: (vixen.parser.ITokenProgramIxProto|null);
            public transferFeeIx?: (vixen.parser.ITransferFeeIxProto|null);
            public tokenMetadataIx?: (vixen.parser.ITokenMetadataIxProto|null);
            public tokenGroupIx?: (vixen.parser.ITokenGroupIxProto|null);
            public confidentialTransferIx?: (vixen.parser.IConfidentialTransferExtIxProto|null);
            public confidentialTransferFeeIx?: (vixen.parser.IConfidentialTransferFeeIxProto|null);
            public cpiGuardIx?: (vixen.parser.ICpiGuardIxProto|null);
            public transferHookIx?: (vixen.parser.ITransferHookIxProto|null);
            public metadataPointerIx?: (vixen.parser.IMetadataPointerIxProto|null);
            public memoTransferIx?: (vixen.parser.IMemoTransferIxProto|null);
            public interestBearingMintIx?: (vixen.parser.IInterestBearingMintIxProto|null);
            public defaultAccountStateIx?: (vixen.parser.IDefaultAccountStateIxProto|null);
            public groupMemberPointerIx?: (vixen.parser.IGroupMemberPointerIxProto|null);
            public groupPointerIx?: (vixen.parser.IGroupPointerIxProto|null);
            public withdrawExcessLamportsIx?: (vixen.parser.IWithdrawExcessLamportsIxProto|null);
            public initializePermanentDelegateIx?: (vixen.parser.IInitializePermanentDelegateIxProto|null);
            public reallocateIx?: (vixen.parser.IReallocateIxProto|null);
            public initializeNonTransferableMintIx?: (vixen.parser.IInitializeNonTransferableMintIxProto|null);
            public initializeMintCloseAuthorityIx?: (vixen.parser.IInitializeMintCloseAuthorityIxProto|null);
            public createNativeMintIx?: (vixen.parser.ICreateNativeMintIxProto|null);
            public setAuthority?: (vixen.parser.ISetAuthorityIxProto|null);
            public ixOneof?: ("tokenProgramIx"|"transferFeeIx"|"tokenMetadataIx"|"tokenGroupIx"|"confidentialTransferIx"|"confidentialTransferFeeIx"|"cpiGuardIx"|"transferHookIx"|"metadataPointerIx"|"memoTransferIx"|"interestBearingMintIx"|"defaultAccountStateIx"|"groupMemberPointerIx"|"groupPointerIx"|"withdrawExcessLamportsIx"|"initializePermanentDelegateIx"|"reallocateIx"|"initializeNonTransferableMintIx"|"initializeMintCloseAuthorityIx"|"createNativeMintIx"|"setAuthority");
            public static create(properties?: vixen.parser.ITokenExtensionProgramIxProto): vixen.parser.TokenExtensionProgramIxProto;
            public static encode(message: vixen.parser.ITokenExtensionProgramIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ITokenExtensionProgramIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.TokenExtensionProgramIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.TokenExtensionProgramIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.TokenExtensionProgramIxProto;
            public static toObject(message: vixen.parser.TokenExtensionProgramIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IOrcaProgramStateProto {
            whirlpool?: (vixen.parser.IWhirlpoolProto|null);
            whirlpoolsConfig?: (vixen.parser.IWhirlpoolsConfigProto|null);
            feeTier?: (vixen.parser.IFeeTierProto|null);
            position?: (vixen.parser.IPositionProto|null);
            tickArray?: (vixen.parser.IOrcaTickArrayProto|null);
        }

        class OrcaProgramStateProto implements IOrcaProgramStateProto {
            constructor(properties?: vixen.parser.IOrcaProgramStateProto);
            public whirlpool?: (vixen.parser.IWhirlpoolProto|null);
            public whirlpoolsConfig?: (vixen.parser.IWhirlpoolsConfigProto|null);
            public feeTier?: (vixen.parser.IFeeTierProto|null);
            public position?: (vixen.parser.IPositionProto|null);
            public tickArray?: (vixen.parser.IOrcaTickArrayProto|null);
            public stateOneof?: ("whirlpool"|"whirlpoolsConfig"|"feeTier"|"position"|"tickArray");
            public static create(properties?: vixen.parser.IOrcaProgramStateProto): vixen.parser.OrcaProgramStateProto;
            public static encode(message: vixen.parser.IOrcaProgramStateProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IOrcaProgramStateProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.OrcaProgramStateProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.OrcaProgramStateProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.OrcaProgramStateProto;
            public static toObject(message: vixen.parser.OrcaProgramStateProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IOrcaProgramIxProto {
            swap?: (vixen.parser.IOrcaSwapInstructionProto|null);
            swapV2?: (vixen.parser.IOrcaSwapV2InstructionProto|null);
        }

        class OrcaProgramIxProto implements IOrcaProgramIxProto {
            constructor(properties?: vixen.parser.IOrcaProgramIxProto);
            public swap?: (vixen.parser.IOrcaSwapInstructionProto|null);
            public swapV2?: (vixen.parser.IOrcaSwapV2InstructionProto|null);
            public ixOneof?: ("swap"|"swapV2");
            public static create(properties?: vixen.parser.IOrcaProgramIxProto): vixen.parser.OrcaProgramIxProto;
            public static encode(message: vixen.parser.IOrcaProgramIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IOrcaProgramIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.OrcaProgramIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.OrcaProgramIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.OrcaProgramIxProto;
            public static toObject(message: vixen.parser.OrcaProgramIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IRaydiumProgramStateProto {
            ammConfig?: (vixen.parser.IAmmConfigProto|null);
            operationState?: (vixen.parser.IOperationStateProto|null);
            observationState?: (vixen.parser.IObservationStateProto|null);
            personalPositionState?: (vixen.parser.IPersonalPositionStateProto|null);
            poolState?: (vixen.parser.IPoolStateProto|null);
            protocolPositionState?: (vixen.parser.IProtocolPositionStateProto|null);
            tickArrayState?: (vixen.parser.IRaydiumTickArrayStateProto|null);
            tickArrayBitmapExtension?: (vixen.parser.ITickArrayBitmapExtensionProto|null);
        }

        class RaydiumProgramStateProto implements IRaydiumProgramStateProto {
            constructor(properties?: vixen.parser.IRaydiumProgramStateProto);
            public ammConfig?: (vixen.parser.IAmmConfigProto|null);
            public operationState?: (vixen.parser.IOperationStateProto|null);
            public observationState?: (vixen.parser.IObservationStateProto|null);
            public personalPositionState?: (vixen.parser.IPersonalPositionStateProto|null);
            public poolState?: (vixen.parser.IPoolStateProto|null);
            public protocolPositionState?: (vixen.parser.IProtocolPositionStateProto|null);
            public tickArrayState?: (vixen.parser.IRaydiumTickArrayStateProto|null);
            public tickArrayBitmapExtension?: (vixen.parser.ITickArrayBitmapExtensionProto|null);
            public stateOneof?: ("ammConfig"|"operationState"|"observationState"|"personalPositionState"|"poolState"|"protocolPositionState"|"tickArrayState"|"tickArrayBitmapExtension");
            public static create(properties?: vixen.parser.IRaydiumProgramStateProto): vixen.parser.RaydiumProgramStateProto;
            public static encode(message: vixen.parser.IRaydiumProgramStateProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IRaydiumProgramStateProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.RaydiumProgramStateProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.RaydiumProgramStateProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.RaydiumProgramStateProto;
            public static toObject(message: vixen.parser.RaydiumProgramStateProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IRaydiumProgramIxProto {
            swap?: (vixen.parser.IRaydiumSwapInstructionProto|null);
            swapV2?: (vixen.parser.IRaydiumSwapV2InstructionProto|null);
        }

        class RaydiumProgramIxProto implements IRaydiumProgramIxProto {
            constructor(properties?: vixen.parser.IRaydiumProgramIxProto);
            public swap?: (vixen.parser.IRaydiumSwapInstructionProto|null);
            public swapV2?: (vixen.parser.IRaydiumSwapV2InstructionProto|null);
            public ixOneof?: ("swap"|"swapV2");
            public static create(properties?: vixen.parser.IRaydiumProgramIxProto): vixen.parser.RaydiumProgramIxProto;
            public static encode(message: vixen.parser.IRaydiumProgramIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IRaydiumProgramIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.RaydiumProgramIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.RaydiumProgramIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.RaydiumProgramIxProto;
            public static toObject(message: vixen.parser.RaydiumProgramIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        enum AccountStateProto {
            Uninitialized = 0,
            Initialized = 1,
            Frozen = 2
        }

        interface ITokenAccountProto {
            mint?: (string|null);
            owner?: (string|null);
            amount?: (number|Long|null);
            delegate?: (string|null);
            state?: (vixen.parser.AccountStateProto|null);
            isNative?: (number|Long|null);
            delegatedAmount?: (number|Long|null);
            closeAuthority?: (string|null);
        }

        class TokenAccountProto implements ITokenAccountProto {
            constructor(properties?: vixen.parser.ITokenAccountProto);
            public mint: string;
            public owner: string;
            public amount: (number|Long);
            public delegate?: (string|null);
            public state: vixen.parser.AccountStateProto;
            public isNative?: (number|Long|null);
            public delegatedAmount: (number|Long);
            public closeAuthority?: (string|null);
            public static create(properties?: vixen.parser.ITokenAccountProto): vixen.parser.TokenAccountProto;
            public static encode(message: vixen.parser.ITokenAccountProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ITokenAccountProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.TokenAccountProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.TokenAccountProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.TokenAccountProto;
            public static toObject(message: vixen.parser.TokenAccountProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IMintProto {
            mintAuthority?: (string|null);
            supply?: (number|Long|null);
            decimals?: (number|Long|null);
            isInitialized?: (boolean|null);
            freezeAuthority?: (string|null);
        }

        class MintProto implements IMintProto {
            constructor(properties?: vixen.parser.IMintProto);
            public mintAuthority?: (string|null);
            public supply: (number|Long);
            public decimals: (number|Long);
            public isInitialized: boolean;
            public freezeAuthority?: (string|null);
            public static create(properties?: vixen.parser.IMintProto): vixen.parser.MintProto;
            public static encode(message: vixen.parser.IMintProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IMintProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.MintProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.MintProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.MintProto;
            public static toObject(message: vixen.parser.MintProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IMultisigProto {
            m?: (number|Long|null);
            n?: (number|Long|null);
            isInitialized?: (boolean|null);
            signers?: (string[]|null);
        }

        class MultisigProto implements IMultisigProto {
            constructor(properties?: vixen.parser.IMultisigProto);
            public m: (number|Long);
            public n: (number|Long);
            public isInitialized: boolean;
            public signers: string[];
            public static create(properties?: vixen.parser.IMultisigProto): vixen.parser.MultisigProto;
            public static encode(message: vixen.parser.IMultisigProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IMultisigProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.MultisigProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.MultisigProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.MultisigProto;
            public static toObject(message: vixen.parser.MultisigProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IImmutableOwnerProto {
        }

        class ImmutableOwnerProto implements IImmutableOwnerProto {
            constructor(properties?: vixen.parser.IImmutableOwnerProto);
            public static create(properties?: vixen.parser.IImmutableOwnerProto): vixen.parser.ImmutableOwnerProto;
            public static encode(message: vixen.parser.IImmutableOwnerProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IImmutableOwnerProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ImmutableOwnerProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ImmutableOwnerProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ImmutableOwnerProto;
            public static toObject(message: vixen.parser.ImmutableOwnerProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ITransferFeeAmountProto {
            withheldAmount?: (number|Long|null);
        }

        class TransferFeeAmountProto implements ITransferFeeAmountProto {
            constructor(properties?: vixen.parser.ITransferFeeAmountProto);
            public withheldAmount: (number|Long);
            public static create(properties?: vixen.parser.ITransferFeeAmountProto): vixen.parser.TransferFeeAmountProto;
            public static encode(message: vixen.parser.ITransferFeeAmountProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ITransferFeeAmountProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.TransferFeeAmountProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.TransferFeeAmountProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.TransferFeeAmountProto;
            public static toObject(message: vixen.parser.TransferFeeAmountProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IConfidentialTransferAccountProto {
            approved?: (boolean|null);
            elgamalPubkey?: (string|null);
            pendingBalanceLo?: (string|null);
            pendingBalanceHi?: (string|null);
            pendingBalance?: (string|null);
            availableBalance?: (string|null);
            decryptableAvailableBalance?: (string|null);
            allowConfidentialCredits?: (boolean|null);
            pendingBalanceCreditCounter?: (number|Long|null);
            maximumPendingBalanceCreditCounter?: (number|Long|null);
            expectedPendingBalanceCreditCounter?: (number|Long|null);
            actualPendingBalanceCreditCounter?: (number|Long|null);
        }

        class ConfidentialTransferAccountProto implements IConfidentialTransferAccountProto {
            constructor(properties?: vixen.parser.IConfidentialTransferAccountProto);
            public approved: boolean;
            public elgamalPubkey: string;
            public pendingBalanceLo: string;
            public pendingBalanceHi: string;
            public pendingBalance: string;
            public availableBalance: string;
            public decryptableAvailableBalance: string;
            public allowConfidentialCredits: boolean;
            public pendingBalanceCreditCounter: (number|Long);
            public maximumPendingBalanceCreditCounter: (number|Long);
            public expectedPendingBalanceCreditCounter: (number|Long);
            public actualPendingBalanceCreditCounter: (number|Long);
            public static create(properties?: vixen.parser.IConfidentialTransferAccountProto): vixen.parser.ConfidentialTransferAccountProto;
            public static encode(message: vixen.parser.IConfidentialTransferAccountProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IConfidentialTransferAccountProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ConfidentialTransferAccountProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ConfidentialTransferAccountProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ConfidentialTransferAccountProto;
            public static toObject(message: vixen.parser.ConfidentialTransferAccountProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IMemoTransferProto {
            requireIncomingTransferMemos?: (boolean|null);
        }

        class MemoTransferProto implements IMemoTransferProto {
            constructor(properties?: vixen.parser.IMemoTransferProto);
            public requireIncomingTransferMemos: boolean;
            public static create(properties?: vixen.parser.IMemoTransferProto): vixen.parser.MemoTransferProto;
            public static encode(message: vixen.parser.IMemoTransferProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IMemoTransferProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.MemoTransferProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.MemoTransferProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.MemoTransferProto;
            public static toObject(message: vixen.parser.MemoTransferProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface INonTransferableAccountProto {
        }

        class NonTransferableAccountProto implements INonTransferableAccountProto {
            constructor(properties?: vixen.parser.INonTransferableAccountProto);
            public static create(properties?: vixen.parser.INonTransferableAccountProto): vixen.parser.NonTransferableAccountProto;
            public static encode(message: vixen.parser.INonTransferableAccountProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.INonTransferableAccountProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.NonTransferableAccountProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.NonTransferableAccountProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.NonTransferableAccountProto;
            public static toObject(message: vixen.parser.NonTransferableAccountProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ITransferHookAccountProto {
            transferring?: (boolean|null);
        }

        class TransferHookAccountProto implements ITransferHookAccountProto {
            constructor(properties?: vixen.parser.ITransferHookAccountProto);
            public transferring: boolean;
            public static create(properties?: vixen.parser.ITransferHookAccountProto): vixen.parser.TransferHookAccountProto;
            public static encode(message: vixen.parser.ITransferHookAccountProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ITransferHookAccountProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.TransferHookAccountProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.TransferHookAccountProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.TransferHookAccountProto;
            public static toObject(message: vixen.parser.TransferHookAccountProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ICpiGuardProto {
            lockCpi?: (boolean|null);
        }

        class CpiGuardProto implements ICpiGuardProto {
            constructor(properties?: vixen.parser.ICpiGuardProto);
            public lockCpi: boolean;
            public static create(properties?: vixen.parser.ICpiGuardProto): vixen.parser.CpiGuardProto;
            public static encode(message: vixen.parser.ICpiGuardProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ICpiGuardProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.CpiGuardProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.CpiGuardProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.CpiGuardProto;
            public static toObject(message: vixen.parser.CpiGuardProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IConfidentialTransferFeeAmountProto {
            withheldAmount?: (string|null);
        }

        class ConfidentialTransferFeeAmountProto implements IConfidentialTransferFeeAmountProto {
            constructor(properties?: vixen.parser.IConfidentialTransferFeeAmountProto);
            public withheldAmount: string;
            public static create(properties?: vixen.parser.IConfidentialTransferFeeAmountProto): vixen.parser.ConfidentialTransferFeeAmountProto;
            public static encode(message: vixen.parser.IConfidentialTransferFeeAmountProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IConfidentialTransferFeeAmountProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ConfidentialTransferFeeAmountProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ConfidentialTransferFeeAmountProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ConfidentialTransferFeeAmountProto;
            public static toObject(message: vixen.parser.ConfidentialTransferFeeAmountProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ITransferFeeProto {
            epoch?: (number|Long|null);
            maximumFee?: (number|Long|null);
            transferFeeBasisPoints?: (number|Long|null);
        }

        class TransferFeeProto implements ITransferFeeProto {
            constructor(properties?: vixen.parser.ITransferFeeProto);
            public epoch: (number|Long);
            public maximumFee: (number|Long);
            public transferFeeBasisPoints: (number|Long);
            public static create(properties?: vixen.parser.ITransferFeeProto): vixen.parser.TransferFeeProto;
            public static encode(message: vixen.parser.ITransferFeeProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ITransferFeeProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.TransferFeeProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.TransferFeeProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.TransferFeeProto;
            public static toObject(message: vixen.parser.TransferFeeProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ITransferFeeConfigProto {
            transferFeeConfigAuthority?: (string|null);
            withdrawWithheldAuthority?: (string|null);
            withheldAmount?: (number|Long|null);
            olderTransferFee?: (vixen.parser.ITransferFeeProto|null);
            newerTransferFee?: (vixen.parser.ITransferFeeProto|null);
        }

        class TransferFeeConfigProto implements ITransferFeeConfigProto {
            constructor(properties?: vixen.parser.ITransferFeeConfigProto);
            public transferFeeConfigAuthority: string;
            public withdrawWithheldAuthority: string;
            public withheldAmount: (number|Long);
            public olderTransferFee?: (vixen.parser.ITransferFeeProto|null);
            public newerTransferFee?: (vixen.parser.ITransferFeeProto|null);
            public static create(properties?: vixen.parser.ITransferFeeConfigProto): vixen.parser.TransferFeeConfigProto;
            public static encode(message: vixen.parser.ITransferFeeConfigProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ITransferFeeConfigProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.TransferFeeConfigProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.TransferFeeConfigProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.TransferFeeConfigProto;
            public static toObject(message: vixen.parser.TransferFeeConfigProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IMintCloseAuthorityProto {
            closeAuthority?: (string|null);
        }

        class MintCloseAuthorityProto implements IMintCloseAuthorityProto {
            constructor(properties?: vixen.parser.IMintCloseAuthorityProto);
            public closeAuthority: string;
            public static create(properties?: vixen.parser.IMintCloseAuthorityProto): vixen.parser.MintCloseAuthorityProto;
            public static encode(message: vixen.parser.IMintCloseAuthorityProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IMintCloseAuthorityProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.MintCloseAuthorityProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.MintCloseAuthorityProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.MintCloseAuthorityProto;
            public static toObject(message: vixen.parser.MintCloseAuthorityProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IConfidentialTransferMintProto {
            authority?: (string|null);
            autoApproveNewAccounts?: (boolean|null);
            auditorElgamalPubkey?: (string|null);
        }

        class ConfidentialTransferMintProto implements IConfidentialTransferMintProto {
            constructor(properties?: vixen.parser.IConfidentialTransferMintProto);
            public authority: string;
            public autoApproveNewAccounts: boolean;
            public auditorElgamalPubkey?: (string|null);
            public static create(properties?: vixen.parser.IConfidentialTransferMintProto): vixen.parser.ConfidentialTransferMintProto;
            public static encode(message: vixen.parser.IConfidentialTransferMintProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IConfidentialTransferMintProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ConfidentialTransferMintProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ConfidentialTransferMintProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ConfidentialTransferMintProto;
            public static toObject(message: vixen.parser.ConfidentialTransferMintProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IDefaultAccountStateProto {
            state?: (number|Long|null);
        }

        class DefaultAccountStateProto implements IDefaultAccountStateProto {
            constructor(properties?: vixen.parser.IDefaultAccountStateProto);
            public state: (number|Long);
            public static create(properties?: vixen.parser.IDefaultAccountStateProto): vixen.parser.DefaultAccountStateProto;
            public static encode(message: vixen.parser.IDefaultAccountStateProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IDefaultAccountStateProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.DefaultAccountStateProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.DefaultAccountStateProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.DefaultAccountStateProto;
            public static toObject(message: vixen.parser.DefaultAccountStateProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface INonTransferableProto {
        }

        class NonTransferableProto implements INonTransferableProto {
            constructor(properties?: vixen.parser.INonTransferableProto);
            public static create(properties?: vixen.parser.INonTransferableProto): vixen.parser.NonTransferableProto;
            public static encode(message: vixen.parser.INonTransferableProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.INonTransferableProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.NonTransferableProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.NonTransferableProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.NonTransferableProto;
            public static toObject(message: vixen.parser.NonTransferableProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInterestBearingConfigProto {
            rateAuthority?: (string|null);
            initializationTimestamp?: (number|Long|null);
            preUpdateAverageRate?: (number|Long|null);
            lastUpdateTimestamp?: (number|Long|null);
            currentRate?: (number|Long|null);
        }

        class InterestBearingConfigProto implements IInterestBearingConfigProto {
            constructor(properties?: vixen.parser.IInterestBearingConfigProto);
            public rateAuthority: string;
            public initializationTimestamp: (number|Long);
            public preUpdateAverageRate: (number|Long);
            public lastUpdateTimestamp: (number|Long);
            public currentRate: (number|Long);
            public static create(properties?: vixen.parser.IInterestBearingConfigProto): vixen.parser.InterestBearingConfigProto;
            public static encode(message: vixen.parser.IInterestBearingConfigProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInterestBearingConfigProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InterestBearingConfigProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InterestBearingConfigProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InterestBearingConfigProto;
            public static toObject(message: vixen.parser.InterestBearingConfigProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IPermanentDelegateProto {
            delegate?: (string|null);
        }

        class PermanentDelegateProto implements IPermanentDelegateProto {
            constructor(properties?: vixen.parser.IPermanentDelegateProto);
            public delegate: string;
            public static create(properties?: vixen.parser.IPermanentDelegateProto): vixen.parser.PermanentDelegateProto;
            public static encode(message: vixen.parser.IPermanentDelegateProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IPermanentDelegateProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.PermanentDelegateProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.PermanentDelegateProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.PermanentDelegateProto;
            public static toObject(message: vixen.parser.PermanentDelegateProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ITransferHookProto {
            authority?: (string|null);
            programId?: (string|null);
        }

        class TransferHookProto implements ITransferHookProto {
            constructor(properties?: vixen.parser.ITransferHookProto);
            public authority: string;
            public programId: string;
            public static create(properties?: vixen.parser.ITransferHookProto): vixen.parser.TransferHookProto;
            public static encode(message: vixen.parser.ITransferHookProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ITransferHookProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.TransferHookProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.TransferHookProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.TransferHookProto;
            public static toObject(message: vixen.parser.TransferHookProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IConfidentialTransferFeeConfigProto {
            authority?: (string|null);
            withdrawWithheldAuthorityElgamalPubkey?: (string|null);
            harvestToMintEnabled?: (boolean|null);
            withheldAmount?: (string|null);
        }

        class ConfidentialTransferFeeConfigProto implements IConfidentialTransferFeeConfigProto {
            constructor(properties?: vixen.parser.IConfidentialTransferFeeConfigProto);
            public authority: string;
            public withdrawWithheldAuthorityElgamalPubkey: string;
            public harvestToMintEnabled: boolean;
            public withheldAmount: string;
            public static create(properties?: vixen.parser.IConfidentialTransferFeeConfigProto): vixen.parser.ConfidentialTransferFeeConfigProto;
            public static encode(message: vixen.parser.IConfidentialTransferFeeConfigProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IConfidentialTransferFeeConfigProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ConfidentialTransferFeeConfigProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ConfidentialTransferFeeConfigProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ConfidentialTransferFeeConfigProto;
            public static toObject(message: vixen.parser.ConfidentialTransferFeeConfigProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IMetadataPointerProto {
            authority?: (string|null);
            metadataAddress?: (string|null);
        }

        class MetadataPointerProto implements IMetadataPointerProto {
            constructor(properties?: vixen.parser.IMetadataPointerProto);
            public authority: string;
            public metadataAddress: string;
            public static create(properties?: vixen.parser.IMetadataPointerProto): vixen.parser.MetadataPointerProto;
            public static encode(message: vixen.parser.IMetadataPointerProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IMetadataPointerProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.MetadataPointerProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.MetadataPointerProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.MetadataPointerProto;
            public static toObject(message: vixen.parser.MetadataPointerProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IKeyValue {
            key?: (string|null);
            value?: (string|null);
        }

        class KeyValue implements IKeyValue {
            constructor(properties?: vixen.parser.IKeyValue);
            public key: string;
            public value: string;
            public static create(properties?: vixen.parser.IKeyValue): vixen.parser.KeyValue;
            public static encode(message: vixen.parser.IKeyValue, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IKeyValue, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.KeyValue;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.KeyValue;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.KeyValue;
            public static toObject(message: vixen.parser.KeyValue, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ITokenMetadataProto {
            updateAuthority?: (string|null);
            mint?: (string|null);
            name?: (string|null);
            symbol?: (string|null);
            uri?: (string|null);
            additionalMetadata?: (vixen.parser.IKeyValue[]|null);
        }

        class TokenMetadataProto implements ITokenMetadataProto {
            constructor(properties?: vixen.parser.ITokenMetadataProto);
            public updateAuthority: string;
            public mint: string;
            public name: string;
            public symbol: string;
            public uri: string;
            public additionalMetadata: vixen.parser.IKeyValue[];
            public static create(properties?: vixen.parser.ITokenMetadataProto): vixen.parser.TokenMetadataProto;
            public static encode(message: vixen.parser.ITokenMetadataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ITokenMetadataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.TokenMetadataProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.TokenMetadataProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.TokenMetadataProto;
            public static toObject(message: vixen.parser.TokenMetadataProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IGroupPointerProto {
            authority?: (string|null);
            groupAddress?: (string|null);
        }

        class GroupPointerProto implements IGroupPointerProto {
            constructor(properties?: vixen.parser.IGroupPointerProto);
            public authority: string;
            public groupAddress: string;
            public static create(properties?: vixen.parser.IGroupPointerProto): vixen.parser.GroupPointerProto;
            public static encode(message: vixen.parser.IGroupPointerProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IGroupPointerProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.GroupPointerProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.GroupPointerProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.GroupPointerProto;
            public static toObject(message: vixen.parser.GroupPointerProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ITokenGroupProto {
            updateAuthority?: (string|null);
            mint?: (string|null);
            size?: (number|null);
            maxSize?: (number|null);
        }

        class TokenGroupProto implements ITokenGroupProto {
            constructor(properties?: vixen.parser.ITokenGroupProto);
            public updateAuthority: string;
            public mint: string;
            public size: number;
            public maxSize: number;
            public static create(properties?: vixen.parser.ITokenGroupProto): vixen.parser.TokenGroupProto;
            public static encode(message: vixen.parser.ITokenGroupProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ITokenGroupProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.TokenGroupProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.TokenGroupProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.TokenGroupProto;
            public static toObject(message: vixen.parser.TokenGroupProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IGroupMemberPointerProto {
            authority?: (string|null);
            memberAddress?: (string|null);
        }

        class GroupMemberPointerProto implements IGroupMemberPointerProto {
            constructor(properties?: vixen.parser.IGroupMemberPointerProto);
            public authority: string;
            public memberAddress: string;
            public static create(properties?: vixen.parser.IGroupMemberPointerProto): vixen.parser.GroupMemberPointerProto;
            public static encode(message: vixen.parser.IGroupMemberPointerProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IGroupMemberPointerProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.GroupMemberPointerProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.GroupMemberPointerProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.GroupMemberPointerProto;
            public static toObject(message: vixen.parser.GroupMemberPointerProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ITokenGroupMemberProto {
            mint?: (string|null);
            group?: (string|null);
            memberNumber?: (number|Long|null);
        }

        class TokenGroupMemberProto implements ITokenGroupMemberProto {
            constructor(properties?: vixen.parser.ITokenGroupMemberProto);
            public mint: string;
            public group: string;
            public memberNumber: (number|Long);
            public static create(properties?: vixen.parser.ITokenGroupMemberProto): vixen.parser.TokenGroupMemberProto;
            public static encode(message: vixen.parser.ITokenGroupMemberProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ITokenGroupMemberProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.TokenGroupMemberProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.TokenGroupMemberProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.TokenGroupMemberProto;
            public static toObject(message: vixen.parser.TokenGroupMemberProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IExtensionDataProto {
            immutableOwner?: (vixen.parser.IImmutableOwnerProto|null);
            transferFeeAmount?: (vixen.parser.ITransferFeeAmountProto|null);
            confidentialTransferAccount?: (vixen.parser.IConfidentialTransferAccountProto|null);
            memoTransfer?: (vixen.parser.IMemoTransferProto|null);
            nonTransferableAccount?: (vixen.parser.INonTransferableAccountProto|null);
            transferHookAccount?: (vixen.parser.ITransferHookAccountProto|null);
            cpiGuard?: (vixen.parser.ICpiGuardProto|null);
            confidentialTransferFeeAmount?: (vixen.parser.IConfidentialTransferFeeAmountProto|null);
            transferFeeConfig?: (vixen.parser.ITransferFeeConfigProto|null);
            mintCloseAuthority?: (vixen.parser.IMintCloseAuthorityProto|null);
            confidentialTransferMint?: (vixen.parser.IConfidentialTransferMintProto|null);
            defaultAccountState?: (vixen.parser.IDefaultAccountStateProto|null);
            nonTransferable?: (vixen.parser.INonTransferableProto|null);
            interestBearingConfig?: (vixen.parser.IInterestBearingConfigProto|null);
            permanentDelegate?: (vixen.parser.IPermanentDelegateProto|null);
            transferHook?: (vixen.parser.ITransferHookProto|null);
            confidentialTransferFeeConfig?: (vixen.parser.IConfidentialTransferFeeConfigProto|null);
            metadataPointer?: (vixen.parser.IMetadataPointerProto|null);
            tokenMetadata?: (vixen.parser.ITokenMetadataProto|null);
            groupPointer?: (vixen.parser.IGroupPointerProto|null);
            tokenGroup?: (vixen.parser.ITokenGroupProto|null);
            groupMemberPointer?: (vixen.parser.IGroupMemberPointerProto|null);
            tokenGroupMember?: (vixen.parser.ITokenGroupMemberProto|null);
        }

        class ExtensionDataProto implements IExtensionDataProto {
            constructor(properties?: vixen.parser.IExtensionDataProto);
            public immutableOwner?: (vixen.parser.IImmutableOwnerProto|null);
            public transferFeeAmount?: (vixen.parser.ITransferFeeAmountProto|null);
            public confidentialTransferAccount?: (vixen.parser.IConfidentialTransferAccountProto|null);
            public memoTransfer?: (vixen.parser.IMemoTransferProto|null);
            public nonTransferableAccount?: (vixen.parser.INonTransferableAccountProto|null);
            public transferHookAccount?: (vixen.parser.ITransferHookAccountProto|null);
            public cpiGuard?: (vixen.parser.ICpiGuardProto|null);
            public confidentialTransferFeeAmount?: (vixen.parser.IConfidentialTransferFeeAmountProto|null);
            public transferFeeConfig?: (vixen.parser.ITransferFeeConfigProto|null);
            public mintCloseAuthority?: (vixen.parser.IMintCloseAuthorityProto|null);
            public confidentialTransferMint?: (vixen.parser.IConfidentialTransferMintProto|null);
            public defaultAccountState?: (vixen.parser.IDefaultAccountStateProto|null);
            public nonTransferable?: (vixen.parser.INonTransferableProto|null);
            public interestBearingConfig?: (vixen.parser.IInterestBearingConfigProto|null);
            public permanentDelegate?: (vixen.parser.IPermanentDelegateProto|null);
            public transferHook?: (vixen.parser.ITransferHookProto|null);
            public confidentialTransferFeeConfig?: (vixen.parser.IConfidentialTransferFeeConfigProto|null);
            public metadataPointer?: (vixen.parser.IMetadataPointerProto|null);
            public tokenMetadata?: (vixen.parser.ITokenMetadataProto|null);
            public groupPointer?: (vixen.parser.IGroupPointerProto|null);
            public tokenGroup?: (vixen.parser.ITokenGroupProto|null);
            public groupMemberPointer?: (vixen.parser.IGroupMemberPointerProto|null);
            public tokenGroupMember?: (vixen.parser.ITokenGroupMemberProto|null);
            public data?: ("immutableOwner"|"transferFeeAmount"|"confidentialTransferAccount"|"memoTransfer"|"nonTransferableAccount"|"transferHookAccount"|"cpiGuard"|"confidentialTransferFeeAmount"|"transferFeeConfig"|"mintCloseAuthority"|"confidentialTransferMint"|"defaultAccountState"|"nonTransferable"|"interestBearingConfig"|"permanentDelegate"|"transferHook"|"confidentialTransferFeeConfig"|"metadataPointer"|"tokenMetadata"|"groupPointer"|"tokenGroup"|"groupMemberPointer"|"tokenGroupMember");
            public static create(properties?: vixen.parser.IExtensionDataProto): vixen.parser.ExtensionDataProto;
            public static encode(message: vixen.parser.IExtensionDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IExtensionDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ExtensionDataProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ExtensionDataProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ExtensionDataProto;
            public static toObject(message: vixen.parser.ExtensionDataProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IExtendedTokenAccountProto {
            baseAccount?: (vixen.parser.ITokenAccountProto|null);
            extensionDataVec?: (vixen.parser.IExtensionDataProto[]|null);
        }

        class ExtendedTokenAccountProto implements IExtendedTokenAccountProto {
            constructor(properties?: vixen.parser.IExtendedTokenAccountProto);
            public baseAccount?: (vixen.parser.ITokenAccountProto|null);
            public extensionDataVec: vixen.parser.IExtensionDataProto[];
            public static create(properties?: vixen.parser.IExtendedTokenAccountProto): vixen.parser.ExtendedTokenAccountProto;
            public static encode(message: vixen.parser.IExtendedTokenAccountProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IExtendedTokenAccountProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ExtendedTokenAccountProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ExtendedTokenAccountProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ExtendedTokenAccountProto;
            public static toObject(message: vixen.parser.ExtendedTokenAccountProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IExtendedMintProto {
            baseMint?: (vixen.parser.IMintProto|null);
            extensionDataVec?: (vixen.parser.IExtensionDataProto[]|null);
        }

        class ExtendedMintProto implements IExtendedMintProto {
            constructor(properties?: vixen.parser.IExtendedMintProto);
            public baseMint?: (vixen.parser.IMintProto|null);
            public extensionDataVec: vixen.parser.IExtensionDataProto[];
            public static create(properties?: vixen.parser.IExtendedMintProto): vixen.parser.ExtendedMintProto;
            public static encode(message: vixen.parser.IExtendedMintProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IExtendedMintProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ExtendedMintProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ExtendedMintProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ExtendedMintProto;
            public static toObject(message: vixen.parser.ExtendedMintProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        enum AuthorityType {
            MINT_TOKEN = 0,
            FREEZE_ACCOUNT = 1,
            ACCOUNT_OWNER = 2,
            CLOSE_ACCOUNT = 3
        }

        interface ITransferAccountsProto {
            source?: (string|null);
            destination?: (string|null);
            owner?: (string|null);
            multisigSigners?: (string[]|null);
        }

        class TransferAccountsProto implements ITransferAccountsProto {
            constructor(properties?: vixen.parser.ITransferAccountsProto);
            public source: string;
            public destination: string;
            public owner: string;
            public multisigSigners: string[];
            public static create(properties?: vixen.parser.ITransferAccountsProto): vixen.parser.TransferAccountsProto;
            public static encode(message: vixen.parser.ITransferAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ITransferAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.TransferAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.TransferAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.TransferAccountsProto;
            public static toObject(message: vixen.parser.TransferAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ITransferDataProto {
            amount?: (number|Long|null);
        }

        class TransferDataProto implements ITransferDataProto {
            constructor(properties?: vixen.parser.ITransferDataProto);
            public amount: (number|Long);
            public static create(properties?: vixen.parser.ITransferDataProto): vixen.parser.TransferDataProto;
            public static encode(message: vixen.parser.ITransferDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ITransferDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.TransferDataProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.TransferDataProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.TransferDataProto;
            public static toObject(message: vixen.parser.TransferDataProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ITransferIxProto {
            accounts?: (vixen.parser.ITransferAccountsProto|null);
            data?: (vixen.parser.ITransferDataProto|null);
        }

        class TransferIxProto implements ITransferIxProto {
            constructor(properties?: vixen.parser.ITransferIxProto);
            public accounts?: (vixen.parser.ITransferAccountsProto|null);
            public data?: (vixen.parser.ITransferDataProto|null);
            public static create(properties?: vixen.parser.ITransferIxProto): vixen.parser.TransferIxProto;
            public static encode(message: vixen.parser.ITransferIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ITransferIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.TransferIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.TransferIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.TransferIxProto;
            public static toObject(message: vixen.parser.TransferIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializeMintAccountsProto {
            mint?: (string|null);
        }

        class InitializeMintAccountsProto implements IInitializeMintAccountsProto {
            constructor(properties?: vixen.parser.IInitializeMintAccountsProto);
            public mint: string;
            public static create(properties?: vixen.parser.IInitializeMintAccountsProto): vixen.parser.InitializeMintAccountsProto;
            public static encode(message: vixen.parser.IInitializeMintAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializeMintAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializeMintAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializeMintAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializeMintAccountsProto;
            public static toObject(message: vixen.parser.InitializeMintAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializeMintDataProto {
            decimals?: (number|Long|null);
            mintAuthority?: (string|null);
            freezeAuthority?: (string|null);
        }

        class InitializeMintDataProto implements IInitializeMintDataProto {
            constructor(properties?: vixen.parser.IInitializeMintDataProto);
            public decimals: (number|Long);
            public mintAuthority?: (string|null);
            public freezeAuthority?: (string|null);
            public static create(properties?: vixen.parser.IInitializeMintDataProto): vixen.parser.InitializeMintDataProto;
            public static encode(message: vixen.parser.IInitializeMintDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializeMintDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializeMintDataProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializeMintDataProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializeMintDataProto;
            public static toObject(message: vixen.parser.InitializeMintDataProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializeMintIxProto {
            accounts?: (vixen.parser.IInitializeMintAccountsProto|null);
            data?: (vixen.parser.IInitializeMintDataProto|null);
        }

        class InitializeMintIxProto implements IInitializeMintIxProto {
            constructor(properties?: vixen.parser.IInitializeMintIxProto);
            public accounts?: (vixen.parser.IInitializeMintAccountsProto|null);
            public data?: (vixen.parser.IInitializeMintDataProto|null);
            public static create(properties?: vixen.parser.IInitializeMintIxProto): vixen.parser.InitializeMintIxProto;
            public static encode(message: vixen.parser.IInitializeMintIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializeMintIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializeMintIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializeMintIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializeMintIxProto;
            public static toObject(message: vixen.parser.InitializeMintIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializeAccountAccountsProto {
            account?: (string|null);
            mint?: (string|null);
            owner?: (string|null);
        }

        class InitializeAccountAccountsProto implements IInitializeAccountAccountsProto {
            constructor(properties?: vixen.parser.IInitializeAccountAccountsProto);
            public account: string;
            public mint: string;
            public owner: string;
            public static create(properties?: vixen.parser.IInitializeAccountAccountsProto): vixen.parser.InitializeAccountAccountsProto;
            public static encode(message: vixen.parser.IInitializeAccountAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializeAccountAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializeAccountAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializeAccountAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializeAccountAccountsProto;
            public static toObject(message: vixen.parser.InitializeAccountAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializeAccountDataProto {
            amount?: (number|Long|null);
        }

        class InitializeAccountDataProto implements IInitializeAccountDataProto {
            constructor(properties?: vixen.parser.IInitializeAccountDataProto);
            public amount: (number|Long);
            public static create(properties?: vixen.parser.IInitializeAccountDataProto): vixen.parser.InitializeAccountDataProto;
            public static encode(message: vixen.parser.IInitializeAccountDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializeAccountDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializeAccountDataProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializeAccountDataProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializeAccountDataProto;
            public static toObject(message: vixen.parser.InitializeAccountDataProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializeAccountIxProto {
            accounts?: (vixen.parser.IInitializeAccountAccountsProto|null);
            data?: (vixen.parser.IInitializeAccountDataProto|null);
        }

        class InitializeAccountIxProto implements IInitializeAccountIxProto {
            constructor(properties?: vixen.parser.IInitializeAccountIxProto);
            public accounts?: (vixen.parser.IInitializeAccountAccountsProto|null);
            public data?: (vixen.parser.IInitializeAccountDataProto|null);
            public static create(properties?: vixen.parser.IInitializeAccountIxProto): vixen.parser.InitializeAccountIxProto;
            public static encode(message: vixen.parser.IInitializeAccountIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializeAccountIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializeAccountIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializeAccountIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializeAccountIxProto;
            public static toObject(message: vixen.parser.InitializeAccountIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializeAccount2AccountsProto {
            account?: (string|null);
            mint?: (string|null);
        }

        class InitializeAccount2AccountsProto implements IInitializeAccount2AccountsProto {
            constructor(properties?: vixen.parser.IInitializeAccount2AccountsProto);
            public account: string;
            public mint: string;
            public static create(properties?: vixen.parser.IInitializeAccount2AccountsProto): vixen.parser.InitializeAccount2AccountsProto;
            public static encode(message: vixen.parser.IInitializeAccount2AccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializeAccount2AccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializeAccount2AccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializeAccount2AccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializeAccount2AccountsProto;
            public static toObject(message: vixen.parser.InitializeAccount2AccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializeAccountData2Proto {
            owner?: (string|null);
        }

        class InitializeAccountData2Proto implements IInitializeAccountData2Proto {
            constructor(properties?: vixen.parser.IInitializeAccountData2Proto);
            public owner: string;
            public static create(properties?: vixen.parser.IInitializeAccountData2Proto): vixen.parser.InitializeAccountData2Proto;
            public static encode(message: vixen.parser.IInitializeAccountData2Proto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializeAccountData2Proto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializeAccountData2Proto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializeAccountData2Proto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializeAccountData2Proto;
            public static toObject(message: vixen.parser.InitializeAccountData2Proto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializeAccount2IxProto {
            accounts?: (vixen.parser.IInitializeAccount2AccountsProto|null);
            data?: (vixen.parser.IInitializeAccountData2Proto|null);
        }

        class InitializeAccount2IxProto implements IInitializeAccount2IxProto {
            constructor(properties?: vixen.parser.IInitializeAccount2IxProto);
            public accounts?: (vixen.parser.IInitializeAccount2AccountsProto|null);
            public data?: (vixen.parser.IInitializeAccountData2Proto|null);
            public static create(properties?: vixen.parser.IInitializeAccount2IxProto): vixen.parser.InitializeAccount2IxProto;
            public static encode(message: vixen.parser.IInitializeAccount2IxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializeAccount2IxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializeAccount2IxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializeAccount2IxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializeAccount2IxProto;
            public static toObject(message: vixen.parser.InitializeAccount2IxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializeAccount3IxProto {
            accounts?: (vixen.parser.IInitializeAccount2AccountsProto|null);
            data?: (vixen.parser.IInitializeAccountData2Proto|null);
        }

        class InitializeAccount3IxProto implements IInitializeAccount3IxProto {
            constructor(properties?: vixen.parser.IInitializeAccount3IxProto);
            public accounts?: (vixen.parser.IInitializeAccount2AccountsProto|null);
            public data?: (vixen.parser.IInitializeAccountData2Proto|null);
            public static create(properties?: vixen.parser.IInitializeAccount3IxProto): vixen.parser.InitializeAccount3IxProto;
            public static encode(message: vixen.parser.IInitializeAccount3IxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializeAccount3IxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializeAccount3IxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializeAccount3IxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializeAccount3IxProto;
            public static toObject(message: vixen.parser.InitializeAccount3IxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializeMultisigAccountsProto {
            multisig?: (string|null);
            signers?: (string[]|null);
        }

        class InitializeMultisigAccountsProto implements IInitializeMultisigAccountsProto {
            constructor(properties?: vixen.parser.IInitializeMultisigAccountsProto);
            public multisig: string;
            public signers: string[];
            public static create(properties?: vixen.parser.IInitializeMultisigAccountsProto): vixen.parser.InitializeMultisigAccountsProto;
            public static encode(message: vixen.parser.IInitializeMultisigAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializeMultisigAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializeMultisigAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializeMultisigAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializeMultisigAccountsProto;
            public static toObject(message: vixen.parser.InitializeMultisigAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializeMultisigDataProto {
            m?: (number|Long|null);
        }

        class InitializeMultisigDataProto implements IInitializeMultisigDataProto {
            constructor(properties?: vixen.parser.IInitializeMultisigDataProto);
            public m: (number|Long);
            public static create(properties?: vixen.parser.IInitializeMultisigDataProto): vixen.parser.InitializeMultisigDataProto;
            public static encode(message: vixen.parser.IInitializeMultisigDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializeMultisigDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializeMultisigDataProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializeMultisigDataProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializeMultisigDataProto;
            public static toObject(message: vixen.parser.InitializeMultisigDataProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializeMultisigIxProto {
            accounts?: (vixen.parser.IInitializeMultisigAccountsProto|null);
            data?: (vixen.parser.IInitializeMultisigDataProto|null);
        }

        class InitializeMultisigIxProto implements IInitializeMultisigIxProto {
            constructor(properties?: vixen.parser.IInitializeMultisigIxProto);
            public accounts?: (vixen.parser.IInitializeMultisigAccountsProto|null);
            public data?: (vixen.parser.IInitializeMultisigDataProto|null);
            public static create(properties?: vixen.parser.IInitializeMultisigIxProto): vixen.parser.InitializeMultisigIxProto;
            public static encode(message: vixen.parser.IInitializeMultisigIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializeMultisigIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializeMultisigIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializeMultisigIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializeMultisigIxProto;
            public static toObject(message: vixen.parser.InitializeMultisigIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IApproveAccountsProto {
            source?: (string|null);
            delegate?: (string|null);
            owner?: (string|null);
            multisigSigners?: (string[]|null);
        }

        class ApproveAccountsProto implements IApproveAccountsProto {
            constructor(properties?: vixen.parser.IApproveAccountsProto);
            public source: string;
            public delegate: string;
            public owner: string;
            public multisigSigners: string[];
            public static create(properties?: vixen.parser.IApproveAccountsProto): vixen.parser.ApproveAccountsProto;
            public static encode(message: vixen.parser.IApproveAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IApproveAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ApproveAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ApproveAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ApproveAccountsProto;
            public static toObject(message: vixen.parser.ApproveAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IApproveDataProto {
            amount?: (number|Long|null);
        }

        class ApproveDataProto implements IApproveDataProto {
            constructor(properties?: vixen.parser.IApproveDataProto);
            public amount: (number|Long);
            public static create(properties?: vixen.parser.IApproveDataProto): vixen.parser.ApproveDataProto;
            public static encode(message: vixen.parser.IApproveDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IApproveDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ApproveDataProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ApproveDataProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ApproveDataProto;
            public static toObject(message: vixen.parser.ApproveDataProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IApproveIxProto {
            accounts?: (vixen.parser.IApproveAccountsProto|null);
            data?: (vixen.parser.IApproveDataProto|null);
        }

        class ApproveIxProto implements IApproveIxProto {
            constructor(properties?: vixen.parser.IApproveIxProto);
            public accounts?: (vixen.parser.IApproveAccountsProto|null);
            public data?: (vixen.parser.IApproveDataProto|null);
            public static create(properties?: vixen.parser.IApproveIxProto): vixen.parser.ApproveIxProto;
            public static encode(message: vixen.parser.IApproveIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IApproveIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ApproveIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ApproveIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ApproveIxProto;
            public static toObject(message: vixen.parser.ApproveIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IRevokeAccountsProto {
            source?: (string|null);
            owner?: (string|null);
            multisigSigners?: (string[]|null);
        }

        class RevokeAccountsProto implements IRevokeAccountsProto {
            constructor(properties?: vixen.parser.IRevokeAccountsProto);
            public source: string;
            public owner: string;
            public multisigSigners: string[];
            public static create(properties?: vixen.parser.IRevokeAccountsProto): vixen.parser.RevokeAccountsProto;
            public static encode(message: vixen.parser.IRevokeAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IRevokeAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.RevokeAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.RevokeAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.RevokeAccountsProto;
            public static toObject(message: vixen.parser.RevokeAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IRevokeIxProto {
            accounts?: (vixen.parser.IRevokeAccountsProto|null);
        }

        class RevokeIxProto implements IRevokeIxProto {
            constructor(properties?: vixen.parser.IRevokeIxProto);
            public accounts?: (vixen.parser.IRevokeAccountsProto|null);
            public static create(properties?: vixen.parser.IRevokeIxProto): vixen.parser.RevokeIxProto;
            public static encode(message: vixen.parser.IRevokeIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IRevokeIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.RevokeIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.RevokeIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.RevokeIxProto;
            public static toObject(message: vixen.parser.RevokeIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ISetAuthorityAccountsProto {
            currentAuthority?: (string|null);
            account?: (string|null);
            multisigSigners?: (string[]|null);
        }

        class SetAuthorityAccountsProto implements ISetAuthorityAccountsProto {
            constructor(properties?: vixen.parser.ISetAuthorityAccountsProto);
            public currentAuthority: string;
            public account: string;
            public multisigSigners: string[];
            public static create(properties?: vixen.parser.ISetAuthorityAccountsProto): vixen.parser.SetAuthorityAccountsProto;
            public static encode(message: vixen.parser.ISetAuthorityAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ISetAuthorityAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.SetAuthorityAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.SetAuthorityAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.SetAuthorityAccountsProto;
            public static toObject(message: vixen.parser.SetAuthorityAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ISetAuthorityDataProto {
            authorityType?: (vixen.parser.AuthorityType|null);
            newAuthority?: (string|null);
        }

        class SetAuthorityDataProto implements ISetAuthorityDataProto {
            constructor(properties?: vixen.parser.ISetAuthorityDataProto);
            public authorityType: vixen.parser.AuthorityType;
            public newAuthority?: (string|null);
            public static create(properties?: vixen.parser.ISetAuthorityDataProto): vixen.parser.SetAuthorityDataProto;
            public static encode(message: vixen.parser.ISetAuthorityDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ISetAuthorityDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.SetAuthorityDataProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.SetAuthorityDataProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.SetAuthorityDataProto;
            public static toObject(message: vixen.parser.SetAuthorityDataProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ISetAuthorityIxProto {
            accounts?: (vixen.parser.ISetAuthorityAccountsProto|null);
            data?: (vixen.parser.ISetAuthorityDataProto|null);
        }

        class SetAuthorityIxProto implements ISetAuthorityIxProto {
            constructor(properties?: vixen.parser.ISetAuthorityIxProto);
            public accounts?: (vixen.parser.ISetAuthorityAccountsProto|null);
            public data?: (vixen.parser.ISetAuthorityDataProto|null);
            public static create(properties?: vixen.parser.ISetAuthorityIxProto): vixen.parser.SetAuthorityIxProto;
            public static encode(message: vixen.parser.ISetAuthorityIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ISetAuthorityIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.SetAuthorityIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.SetAuthorityIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.SetAuthorityIxProto;
            public static toObject(message: vixen.parser.SetAuthorityIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IMintToAccountsProto {
            mint?: (string|null);
            account?: (string|null);
            mintAuthority?: (string|null);
            multisigSigners?: (string[]|null);
        }

        class MintToAccountsProto implements IMintToAccountsProto {
            constructor(properties?: vixen.parser.IMintToAccountsProto);
            public mint: string;
            public account: string;
            public mintAuthority: string;
            public multisigSigners: string[];
            public static create(properties?: vixen.parser.IMintToAccountsProto): vixen.parser.MintToAccountsProto;
            public static encode(message: vixen.parser.IMintToAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IMintToAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.MintToAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.MintToAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.MintToAccountsProto;
            public static toObject(message: vixen.parser.MintToAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IMintToDataProto {
            amount?: (number|Long|null);
        }

        class MintToDataProto implements IMintToDataProto {
            constructor(properties?: vixen.parser.IMintToDataProto);
            public amount: (number|Long);
            public static create(properties?: vixen.parser.IMintToDataProto): vixen.parser.MintToDataProto;
            public static encode(message: vixen.parser.IMintToDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IMintToDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.MintToDataProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.MintToDataProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.MintToDataProto;
            public static toObject(message: vixen.parser.MintToDataProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IMintToIxProto {
            accounts?: (vixen.parser.IMintToAccountsProto|null);
            data?: (vixen.parser.IMintToDataProto|null);
        }

        class MintToIxProto implements IMintToIxProto {
            constructor(properties?: vixen.parser.IMintToIxProto);
            public accounts?: (vixen.parser.IMintToAccountsProto|null);
            public data?: (vixen.parser.IMintToDataProto|null);
            public static create(properties?: vixen.parser.IMintToIxProto): vixen.parser.MintToIxProto;
            public static encode(message: vixen.parser.IMintToIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IMintToIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.MintToIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.MintToIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.MintToIxProto;
            public static toObject(message: vixen.parser.MintToIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IBurnAccountsProto {
            account?: (string|null);
            mint?: (string|null);
            owner?: (string|null);
            multisigSigners?: (string[]|null);
        }

        class BurnAccountsProto implements IBurnAccountsProto {
            constructor(properties?: vixen.parser.IBurnAccountsProto);
            public account: string;
            public mint: string;
            public owner: string;
            public multisigSigners: string[];
            public static create(properties?: vixen.parser.IBurnAccountsProto): vixen.parser.BurnAccountsProto;
            public static encode(message: vixen.parser.IBurnAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IBurnAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.BurnAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.BurnAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.BurnAccountsProto;
            public static toObject(message: vixen.parser.BurnAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IBurnDataProto {
            amount?: (number|Long|null);
        }

        class BurnDataProto implements IBurnDataProto {
            constructor(properties?: vixen.parser.IBurnDataProto);
            public amount: (number|Long);
            public static create(properties?: vixen.parser.IBurnDataProto): vixen.parser.BurnDataProto;
            public static encode(message: vixen.parser.IBurnDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IBurnDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.BurnDataProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.BurnDataProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.BurnDataProto;
            public static toObject(message: vixen.parser.BurnDataProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IBurnIxProto {
            accounts?: (vixen.parser.IBurnAccountsProto|null);
            data?: (vixen.parser.IBurnDataProto|null);
        }

        class BurnIxProto implements IBurnIxProto {
            constructor(properties?: vixen.parser.IBurnIxProto);
            public accounts?: (vixen.parser.IBurnAccountsProto|null);
            public data?: (vixen.parser.IBurnDataProto|null);
            public static create(properties?: vixen.parser.IBurnIxProto): vixen.parser.BurnIxProto;
            public static encode(message: vixen.parser.IBurnIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IBurnIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.BurnIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.BurnIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.BurnIxProto;
            public static toObject(message: vixen.parser.BurnIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ICloseAccountAccountsProto {
            account?: (string|null);
            destination?: (string|null);
            owner?: (string|null);
            multisigSigners?: (string[]|null);
        }

        class CloseAccountAccountsProto implements ICloseAccountAccountsProto {
            constructor(properties?: vixen.parser.ICloseAccountAccountsProto);
            public account: string;
            public destination: string;
            public owner: string;
            public multisigSigners: string[];
            public static create(properties?: vixen.parser.ICloseAccountAccountsProto): vixen.parser.CloseAccountAccountsProto;
            public static encode(message: vixen.parser.ICloseAccountAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ICloseAccountAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.CloseAccountAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.CloseAccountAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.CloseAccountAccountsProto;
            public static toObject(message: vixen.parser.CloseAccountAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ICloseAccountIxProto {
            accounts?: (vixen.parser.ICloseAccountAccountsProto|null);
        }

        class CloseAccountIxProto implements ICloseAccountIxProto {
            constructor(properties?: vixen.parser.ICloseAccountIxProto);
            public accounts?: (vixen.parser.ICloseAccountAccountsProto|null);
            public static create(properties?: vixen.parser.ICloseAccountIxProto): vixen.parser.CloseAccountIxProto;
            public static encode(message: vixen.parser.ICloseAccountIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ICloseAccountIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.CloseAccountIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.CloseAccountIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.CloseAccountIxProto;
            public static toObject(message: vixen.parser.CloseAccountIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IFreezeAccountAccountsProto {
            account?: (string|null);
            mint?: (string|null);
            mintFreezeAuthority?: (string|null);
            multisigSigners?: (string[]|null);
        }

        class FreezeAccountAccountsProto implements IFreezeAccountAccountsProto {
            constructor(properties?: vixen.parser.IFreezeAccountAccountsProto);
            public account: string;
            public mint: string;
            public mintFreezeAuthority: string;
            public multisigSigners: string[];
            public static create(properties?: vixen.parser.IFreezeAccountAccountsProto): vixen.parser.FreezeAccountAccountsProto;
            public static encode(message: vixen.parser.IFreezeAccountAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IFreezeAccountAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.FreezeAccountAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.FreezeAccountAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.FreezeAccountAccountsProto;
            public static toObject(message: vixen.parser.FreezeAccountAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IFreezeAccountIxProto {
            accounts?: (vixen.parser.IFreezeAccountAccountsProto|null);
        }

        class FreezeAccountIxProto implements IFreezeAccountIxProto {
            constructor(properties?: vixen.parser.IFreezeAccountIxProto);
            public accounts?: (vixen.parser.IFreezeAccountAccountsProto|null);
            public static create(properties?: vixen.parser.IFreezeAccountIxProto): vixen.parser.FreezeAccountIxProto;
            public static encode(message: vixen.parser.IFreezeAccountIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IFreezeAccountIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.FreezeAccountIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.FreezeAccountIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.FreezeAccountIxProto;
            public static toObject(message: vixen.parser.FreezeAccountIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IThawAccountAccountsProto {
            account?: (string|null);
            mint?: (string|null);
            mintFreezeAuthority?: (string|null);
            multisigSigners?: (string[]|null);
        }

        class ThawAccountAccountsProto implements IThawAccountAccountsProto {
            constructor(properties?: vixen.parser.IThawAccountAccountsProto);
            public account: string;
            public mint: string;
            public mintFreezeAuthority: string;
            public multisigSigners: string[];
            public static create(properties?: vixen.parser.IThawAccountAccountsProto): vixen.parser.ThawAccountAccountsProto;
            public static encode(message: vixen.parser.IThawAccountAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IThawAccountAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ThawAccountAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ThawAccountAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ThawAccountAccountsProto;
            public static toObject(message: vixen.parser.ThawAccountAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IThawAccountIxProto {
            accounts?: (vixen.parser.IThawAccountAccountsProto|null);
        }

        class ThawAccountIxProto implements IThawAccountIxProto {
            constructor(properties?: vixen.parser.IThawAccountIxProto);
            public accounts?: (vixen.parser.IThawAccountAccountsProto|null);
            public static create(properties?: vixen.parser.IThawAccountIxProto): vixen.parser.ThawAccountIxProto;
            public static encode(message: vixen.parser.IThawAccountIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IThawAccountIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ThawAccountIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ThawAccountIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ThawAccountIxProto;
            public static toObject(message: vixen.parser.ThawAccountIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ITransferCheckedAccountsProto {
            source?: (string|null);
            mint?: (string|null);
            destination?: (string|null);
            owner?: (string|null);
            multisigSigners?: (string[]|null);
        }

        class TransferCheckedAccountsProto implements ITransferCheckedAccountsProto {
            constructor(properties?: vixen.parser.ITransferCheckedAccountsProto);
            public source: string;
            public mint: string;
            public destination: string;
            public owner: string;
            public multisigSigners: string[];
            public static create(properties?: vixen.parser.ITransferCheckedAccountsProto): vixen.parser.TransferCheckedAccountsProto;
            public static encode(message: vixen.parser.ITransferCheckedAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ITransferCheckedAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.TransferCheckedAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.TransferCheckedAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.TransferCheckedAccountsProto;
            public static toObject(message: vixen.parser.TransferCheckedAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ITransferCheckedDataProto {
            amount?: (number|Long|null);
            decimals?: (number|Long|null);
        }

        class TransferCheckedDataProto implements ITransferCheckedDataProto {
            constructor(properties?: vixen.parser.ITransferCheckedDataProto);
            public amount: (number|Long);
            public decimals: (number|Long);
            public static create(properties?: vixen.parser.ITransferCheckedDataProto): vixen.parser.TransferCheckedDataProto;
            public static encode(message: vixen.parser.ITransferCheckedDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ITransferCheckedDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.TransferCheckedDataProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.TransferCheckedDataProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.TransferCheckedDataProto;
            public static toObject(message: vixen.parser.TransferCheckedDataProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ITransferCheckedIxProto {
            accounts?: (vixen.parser.ITransferCheckedAccountsProto|null);
            data?: (vixen.parser.ITransferCheckedDataProto|null);
        }

        class TransferCheckedIxProto implements ITransferCheckedIxProto {
            constructor(properties?: vixen.parser.ITransferCheckedIxProto);
            public accounts?: (vixen.parser.ITransferCheckedAccountsProto|null);
            public data?: (vixen.parser.ITransferCheckedDataProto|null);
            public static create(properties?: vixen.parser.ITransferCheckedIxProto): vixen.parser.TransferCheckedIxProto;
            public static encode(message: vixen.parser.ITransferCheckedIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ITransferCheckedIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.TransferCheckedIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.TransferCheckedIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.TransferCheckedIxProto;
            public static toObject(message: vixen.parser.TransferCheckedIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IApproveCheckedAccountsProto {
            source?: (string|null);
            mint?: (string|null);
            delegate?: (string|null);
            owner?: (string|null);
            multisigSigners?: (string[]|null);
        }

        class ApproveCheckedAccountsProto implements IApproveCheckedAccountsProto {
            constructor(properties?: vixen.parser.IApproveCheckedAccountsProto);
            public source: string;
            public mint: string;
            public delegate: string;
            public owner: string;
            public multisigSigners: string[];
            public static create(properties?: vixen.parser.IApproveCheckedAccountsProto): vixen.parser.ApproveCheckedAccountsProto;
            public static encode(message: vixen.parser.IApproveCheckedAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IApproveCheckedAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ApproveCheckedAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ApproveCheckedAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ApproveCheckedAccountsProto;
            public static toObject(message: vixen.parser.ApproveCheckedAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IApproveCheckedDataProto {
            amount?: (number|Long|null);
            decimals?: (number|Long|null);
        }

        class ApproveCheckedDataProto implements IApproveCheckedDataProto {
            constructor(properties?: vixen.parser.IApproveCheckedDataProto);
            public amount: (number|Long);
            public decimals: (number|Long);
            public static create(properties?: vixen.parser.IApproveCheckedDataProto): vixen.parser.ApproveCheckedDataProto;
            public static encode(message: vixen.parser.IApproveCheckedDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IApproveCheckedDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ApproveCheckedDataProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ApproveCheckedDataProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ApproveCheckedDataProto;
            public static toObject(message: vixen.parser.ApproveCheckedDataProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IApproveCheckedIxProto {
            accounts?: (vixen.parser.IApproveCheckedAccountsProto|null);
            data?: (vixen.parser.IApproveCheckedDataProto|null);
        }

        class ApproveCheckedIxProto implements IApproveCheckedIxProto {
            constructor(properties?: vixen.parser.IApproveCheckedIxProto);
            public accounts?: (vixen.parser.IApproveCheckedAccountsProto|null);
            public data?: (vixen.parser.IApproveCheckedDataProto|null);
            public static create(properties?: vixen.parser.IApproveCheckedIxProto): vixen.parser.ApproveCheckedIxProto;
            public static encode(message: vixen.parser.IApproveCheckedIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IApproveCheckedIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ApproveCheckedIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ApproveCheckedIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ApproveCheckedIxProto;
            public static toObject(message: vixen.parser.ApproveCheckedIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IMintToCheckedAccountsProto {
            mint?: (string|null);
            account?: (string|null);
            mintAuthority?: (string|null);
            multisigSigners?: (string[]|null);
        }

        class MintToCheckedAccountsProto implements IMintToCheckedAccountsProto {
            constructor(properties?: vixen.parser.IMintToCheckedAccountsProto);
            public mint: string;
            public account: string;
            public mintAuthority: string;
            public multisigSigners: string[];
            public static create(properties?: vixen.parser.IMintToCheckedAccountsProto): vixen.parser.MintToCheckedAccountsProto;
            public static encode(message: vixen.parser.IMintToCheckedAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IMintToCheckedAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.MintToCheckedAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.MintToCheckedAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.MintToCheckedAccountsProto;
            public static toObject(message: vixen.parser.MintToCheckedAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IMintToCheckedDataProto {
            amount?: (number|Long|null);
            decimals?: (number|Long|null);
        }

        class MintToCheckedDataProto implements IMintToCheckedDataProto {
            constructor(properties?: vixen.parser.IMintToCheckedDataProto);
            public amount: (number|Long);
            public decimals: (number|Long);
            public static create(properties?: vixen.parser.IMintToCheckedDataProto): vixen.parser.MintToCheckedDataProto;
            public static encode(message: vixen.parser.IMintToCheckedDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IMintToCheckedDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.MintToCheckedDataProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.MintToCheckedDataProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.MintToCheckedDataProto;
            public static toObject(message: vixen.parser.MintToCheckedDataProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IMintToCheckedIxProto {
            accounts?: (vixen.parser.IMintToCheckedAccountsProto|null);
            data?: (vixen.parser.IMintToCheckedDataProto|null);
        }

        class MintToCheckedIxProto implements IMintToCheckedIxProto {
            constructor(properties?: vixen.parser.IMintToCheckedIxProto);
            public accounts?: (vixen.parser.IMintToCheckedAccountsProto|null);
            public data?: (vixen.parser.IMintToCheckedDataProto|null);
            public static create(properties?: vixen.parser.IMintToCheckedIxProto): vixen.parser.MintToCheckedIxProto;
            public static encode(message: vixen.parser.IMintToCheckedIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IMintToCheckedIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.MintToCheckedIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.MintToCheckedIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.MintToCheckedIxProto;
            public static toObject(message: vixen.parser.MintToCheckedIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IBurnCheckedAccountsProto {
            account?: (string|null);
            mint?: (string|null);
            owner?: (string|null);
            multisigSigners?: (string[]|null);
        }

        class BurnCheckedAccountsProto implements IBurnCheckedAccountsProto {
            constructor(properties?: vixen.parser.IBurnCheckedAccountsProto);
            public account: string;
            public mint: string;
            public owner: string;
            public multisigSigners: string[];
            public static create(properties?: vixen.parser.IBurnCheckedAccountsProto): vixen.parser.BurnCheckedAccountsProto;
            public static encode(message: vixen.parser.IBurnCheckedAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IBurnCheckedAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.BurnCheckedAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.BurnCheckedAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.BurnCheckedAccountsProto;
            public static toObject(message: vixen.parser.BurnCheckedAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IBurnCheckedDataProto {
            amount?: (number|Long|null);
            decimals?: (number|Long|null);
        }

        class BurnCheckedDataProto implements IBurnCheckedDataProto {
            constructor(properties?: vixen.parser.IBurnCheckedDataProto);
            public amount: (number|Long);
            public decimals: (number|Long);
            public static create(properties?: vixen.parser.IBurnCheckedDataProto): vixen.parser.BurnCheckedDataProto;
            public static encode(message: vixen.parser.IBurnCheckedDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IBurnCheckedDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.BurnCheckedDataProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.BurnCheckedDataProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.BurnCheckedDataProto;
            public static toObject(message: vixen.parser.BurnCheckedDataProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IBurnCheckedIxProto {
            accounts?: (vixen.parser.IBurnCheckedAccountsProto|null);
            data?: (vixen.parser.IBurnCheckedDataProto|null);
        }

        class BurnCheckedIxProto implements IBurnCheckedIxProto {
            constructor(properties?: vixen.parser.IBurnCheckedIxProto);
            public accounts?: (vixen.parser.IBurnCheckedAccountsProto|null);
            public data?: (vixen.parser.IBurnCheckedDataProto|null);
            public static create(properties?: vixen.parser.IBurnCheckedIxProto): vixen.parser.BurnCheckedIxProto;
            public static encode(message: vixen.parser.IBurnCheckedIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IBurnCheckedIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.BurnCheckedIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.BurnCheckedIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.BurnCheckedIxProto;
            public static toObject(message: vixen.parser.BurnCheckedIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ISyncNativeAccountsProto {
            account?: (string|null);
        }

        class SyncNativeAccountsProto implements ISyncNativeAccountsProto {
            constructor(properties?: vixen.parser.ISyncNativeAccountsProto);
            public account: string;
            public static create(properties?: vixen.parser.ISyncNativeAccountsProto): vixen.parser.SyncNativeAccountsProto;
            public static encode(message: vixen.parser.ISyncNativeAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ISyncNativeAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.SyncNativeAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.SyncNativeAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.SyncNativeAccountsProto;
            public static toObject(message: vixen.parser.SyncNativeAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ISyncNativeIxProto {
            accounts?: (vixen.parser.ISyncNativeAccountsProto|null);
        }

        class SyncNativeIxProto implements ISyncNativeIxProto {
            constructor(properties?: vixen.parser.ISyncNativeIxProto);
            public accounts?: (vixen.parser.ISyncNativeAccountsProto|null);
            public static create(properties?: vixen.parser.ISyncNativeIxProto): vixen.parser.SyncNativeIxProto;
            public static encode(message: vixen.parser.ISyncNativeIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ISyncNativeIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.SyncNativeIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.SyncNativeIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.SyncNativeIxProto;
            public static toObject(message: vixen.parser.SyncNativeIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IGetAccountDataSizeAccountsProto {
            mint?: (string|null);
        }

        class GetAccountDataSizeAccountsProto implements IGetAccountDataSizeAccountsProto {
            constructor(properties?: vixen.parser.IGetAccountDataSizeAccountsProto);
            public mint: string;
            public static create(properties?: vixen.parser.IGetAccountDataSizeAccountsProto): vixen.parser.GetAccountDataSizeAccountsProto;
            public static encode(message: vixen.parser.IGetAccountDataSizeAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IGetAccountDataSizeAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.GetAccountDataSizeAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.GetAccountDataSizeAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.GetAccountDataSizeAccountsProto;
            public static toObject(message: vixen.parser.GetAccountDataSizeAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IGetAccountDataSizeIxProto {
            accounts?: (vixen.parser.IGetAccountDataSizeAccountsProto|null);
        }

        class GetAccountDataSizeIxProto implements IGetAccountDataSizeIxProto {
            constructor(properties?: vixen.parser.IGetAccountDataSizeIxProto);
            public accounts?: (vixen.parser.IGetAccountDataSizeAccountsProto|null);
            public static create(properties?: vixen.parser.IGetAccountDataSizeIxProto): vixen.parser.GetAccountDataSizeIxProto;
            public static encode(message: vixen.parser.IGetAccountDataSizeIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IGetAccountDataSizeIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.GetAccountDataSizeIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.GetAccountDataSizeIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.GetAccountDataSizeIxProto;
            public static toObject(message: vixen.parser.GetAccountDataSizeIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializeImmutableOwnerAccountsProto {
            account?: (string|null);
        }

        class InitializeImmutableOwnerAccountsProto implements IInitializeImmutableOwnerAccountsProto {
            constructor(properties?: vixen.parser.IInitializeImmutableOwnerAccountsProto);
            public account: string;
            public static create(properties?: vixen.parser.IInitializeImmutableOwnerAccountsProto): vixen.parser.InitializeImmutableOwnerAccountsProto;
            public static encode(message: vixen.parser.IInitializeImmutableOwnerAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializeImmutableOwnerAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializeImmutableOwnerAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializeImmutableOwnerAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializeImmutableOwnerAccountsProto;
            public static toObject(message: vixen.parser.InitializeImmutableOwnerAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializeImmutableOwnerIxProto {
            accounts?: (vixen.parser.IInitializeImmutableOwnerAccountsProto|null);
        }

        class InitializeImmutableOwnerIxProto implements IInitializeImmutableOwnerIxProto {
            constructor(properties?: vixen.parser.IInitializeImmutableOwnerIxProto);
            public accounts?: (vixen.parser.IInitializeImmutableOwnerAccountsProto|null);
            public static create(properties?: vixen.parser.IInitializeImmutableOwnerIxProto): vixen.parser.InitializeImmutableOwnerIxProto;
            public static encode(message: vixen.parser.IInitializeImmutableOwnerIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializeImmutableOwnerIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializeImmutableOwnerIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializeImmutableOwnerIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializeImmutableOwnerIxProto;
            public static toObject(message: vixen.parser.InitializeImmutableOwnerIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IAmountToUiAmountAccountsProto {
            mint?: (string|null);
        }

        class AmountToUiAmountAccountsProto implements IAmountToUiAmountAccountsProto {
            constructor(properties?: vixen.parser.IAmountToUiAmountAccountsProto);
            public mint: string;
            public static create(properties?: vixen.parser.IAmountToUiAmountAccountsProto): vixen.parser.AmountToUiAmountAccountsProto;
            public static encode(message: vixen.parser.IAmountToUiAmountAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IAmountToUiAmountAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.AmountToUiAmountAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.AmountToUiAmountAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.AmountToUiAmountAccountsProto;
            public static toObject(message: vixen.parser.AmountToUiAmountAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IAmountToUiAmountDataProto {
            amount?: (number|Long|null);
        }

        class AmountToUiAmountDataProto implements IAmountToUiAmountDataProto {
            constructor(properties?: vixen.parser.IAmountToUiAmountDataProto);
            public amount: (number|Long);
            public static create(properties?: vixen.parser.IAmountToUiAmountDataProto): vixen.parser.AmountToUiAmountDataProto;
            public static encode(message: vixen.parser.IAmountToUiAmountDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IAmountToUiAmountDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.AmountToUiAmountDataProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.AmountToUiAmountDataProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.AmountToUiAmountDataProto;
            public static toObject(message: vixen.parser.AmountToUiAmountDataProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IAmountToUiAmountIxProto {
            accounts?: (vixen.parser.IAmountToUiAmountAccountsProto|null);
            data?: (vixen.parser.IAmountToUiAmountDataProto|null);
        }

        class AmountToUiAmountIxProto implements IAmountToUiAmountIxProto {
            constructor(properties?: vixen.parser.IAmountToUiAmountIxProto);
            public accounts?: (vixen.parser.IAmountToUiAmountAccountsProto|null);
            public data?: (vixen.parser.IAmountToUiAmountDataProto|null);
            public static create(properties?: vixen.parser.IAmountToUiAmountIxProto): vixen.parser.AmountToUiAmountIxProto;
            public static encode(message: vixen.parser.IAmountToUiAmountIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IAmountToUiAmountIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.AmountToUiAmountIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.AmountToUiAmountIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.AmountToUiAmountIxProto;
            public static toObject(message: vixen.parser.AmountToUiAmountIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IUiAmountToAmountAccountsProto {
            mint?: (string|null);
        }

        class UiAmountToAmountAccountsProto implements IUiAmountToAmountAccountsProto {
            constructor(properties?: vixen.parser.IUiAmountToAmountAccountsProto);
            public mint: string;
            public static create(properties?: vixen.parser.IUiAmountToAmountAccountsProto): vixen.parser.UiAmountToAmountAccountsProto;
            public static encode(message: vixen.parser.IUiAmountToAmountAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IUiAmountToAmountAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.UiAmountToAmountAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.UiAmountToAmountAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.UiAmountToAmountAccountsProto;
            public static toObject(message: vixen.parser.UiAmountToAmountAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IUiAmountToAmountDataProto {
            uiAmount?: (string|null);
        }

        class UiAmountToAmountDataProto implements IUiAmountToAmountDataProto {
            constructor(properties?: vixen.parser.IUiAmountToAmountDataProto);
            public uiAmount: string;
            public static create(properties?: vixen.parser.IUiAmountToAmountDataProto): vixen.parser.UiAmountToAmountDataProto;
            public static encode(message: vixen.parser.IUiAmountToAmountDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IUiAmountToAmountDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.UiAmountToAmountDataProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.UiAmountToAmountDataProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.UiAmountToAmountDataProto;
            public static toObject(message: vixen.parser.UiAmountToAmountDataProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IUiAmountToAmountIxProto {
            accounts?: (vixen.parser.IUiAmountToAmountAccountsProto|null);
            data?: (vixen.parser.IUiAmountToAmountDataProto|null);
        }

        class UiAmountToAmountIxProto implements IUiAmountToAmountIxProto {
            constructor(properties?: vixen.parser.IUiAmountToAmountIxProto);
            public accounts?: (vixen.parser.IUiAmountToAmountAccountsProto|null);
            public data?: (vixen.parser.IUiAmountToAmountDataProto|null);
            public static create(properties?: vixen.parser.IUiAmountToAmountIxProto): vixen.parser.UiAmountToAmountIxProto;
            public static encode(message: vixen.parser.IUiAmountToAmountIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IUiAmountToAmountIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.UiAmountToAmountIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.UiAmountToAmountIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.UiAmountToAmountIxProto;
            public static toObject(message: vixen.parser.UiAmountToAmountIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ITransferCheckedWithFeeAccountsProto {
            source?: (string|null);
            mint?: (string|null);
            destination?: (string|null);
            owner?: (string|null);
            multisigSigners?: (string[]|null);
        }

        class TransferCheckedWithFeeAccountsProto implements ITransferCheckedWithFeeAccountsProto {
            constructor(properties?: vixen.parser.ITransferCheckedWithFeeAccountsProto);
            public source: string;
            public mint: string;
            public destination: string;
            public owner: string;
            public multisigSigners: string[];
            public static create(properties?: vixen.parser.ITransferCheckedWithFeeAccountsProto): vixen.parser.TransferCheckedWithFeeAccountsProto;
            public static encode(message: vixen.parser.ITransferCheckedWithFeeAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ITransferCheckedWithFeeAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.TransferCheckedWithFeeAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.TransferCheckedWithFeeAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.TransferCheckedWithFeeAccountsProto;
            public static toObject(message: vixen.parser.TransferCheckedWithFeeAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ITransferCheckedWithFeeDataProto {
            amount?: (number|Long|null);
            feeAmount?: (number|Long|null);
            decimals?: (number|Long|null);
        }

        class TransferCheckedWithFeeDataProto implements ITransferCheckedWithFeeDataProto {
            constructor(properties?: vixen.parser.ITransferCheckedWithFeeDataProto);
            public amount: (number|Long);
            public feeAmount: (number|Long);
            public decimals: (number|Long);
            public static create(properties?: vixen.parser.ITransferCheckedWithFeeDataProto): vixen.parser.TransferCheckedWithFeeDataProto;
            public static encode(message: vixen.parser.ITransferCheckedWithFeeDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ITransferCheckedWithFeeDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.TransferCheckedWithFeeDataProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.TransferCheckedWithFeeDataProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.TransferCheckedWithFeeDataProto;
            public static toObject(message: vixen.parser.TransferCheckedWithFeeDataProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ITransferCheckedWithFeeIxProto {
            accounts?: (vixen.parser.ITransferCheckedWithFeeAccountsProto|null);
            data?: (vixen.parser.ITransferCheckedWithFeeDataProto|null);
        }

        class TransferCheckedWithFeeIxProto implements ITransferCheckedWithFeeIxProto {
            constructor(properties?: vixen.parser.ITransferCheckedWithFeeIxProto);
            public accounts?: (vixen.parser.ITransferCheckedWithFeeAccountsProto|null);
            public data?: (vixen.parser.ITransferCheckedWithFeeDataProto|null);
            public static create(properties?: vixen.parser.ITransferCheckedWithFeeIxProto): vixen.parser.TransferCheckedWithFeeIxProto;
            public static encode(message: vixen.parser.ITransferCheckedWithFeeIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ITransferCheckedWithFeeIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.TransferCheckedWithFeeIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.TransferCheckedWithFeeIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.TransferCheckedWithFeeIxProto;
            public static toObject(message: vixen.parser.TransferCheckedWithFeeIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializeTransferFeeConfigAccountsProto {
            mint?: (string|null);
        }

        class InitializeTransferFeeConfigAccountsProto implements IInitializeTransferFeeConfigAccountsProto {
            constructor(properties?: vixen.parser.IInitializeTransferFeeConfigAccountsProto);
            public mint: string;
            public static create(properties?: vixen.parser.IInitializeTransferFeeConfigAccountsProto): vixen.parser.InitializeTransferFeeConfigAccountsProto;
            public static encode(message: vixen.parser.IInitializeTransferFeeConfigAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializeTransferFeeConfigAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializeTransferFeeConfigAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializeTransferFeeConfigAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializeTransferFeeConfigAccountsProto;
            public static toObject(message: vixen.parser.InitializeTransferFeeConfigAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializeTransferFeeConfigDataProto {
            transferFeeConfigAuthority?: (string|null);
            withdrawWithheldAuthority?: (string|null);
            transferFeeBasisPoints?: (number|Long|null);
            maximumFee?: (number|Long|null);
        }

        class InitializeTransferFeeConfigDataProto implements IInitializeTransferFeeConfigDataProto {
            constructor(properties?: vixen.parser.IInitializeTransferFeeConfigDataProto);
            public transferFeeConfigAuthority?: (string|null);
            public withdrawWithheldAuthority?: (string|null);
            public transferFeeBasisPoints: (number|Long);
            public maximumFee: (number|Long);
            public static create(properties?: vixen.parser.IInitializeTransferFeeConfigDataProto): vixen.parser.InitializeTransferFeeConfigDataProto;
            public static encode(message: vixen.parser.IInitializeTransferFeeConfigDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializeTransferFeeConfigDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializeTransferFeeConfigDataProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializeTransferFeeConfigDataProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializeTransferFeeConfigDataProto;
            public static toObject(message: vixen.parser.InitializeTransferFeeConfigDataProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializeTransferFeeConfigIxProto {
            accounts?: (vixen.parser.IInitializeTransferFeeConfigAccountsProto|null);
            data?: (vixen.parser.IInitializeTransferFeeConfigDataProto|null);
        }

        class InitializeTransferFeeConfigIxProto implements IInitializeTransferFeeConfigIxProto {
            constructor(properties?: vixen.parser.IInitializeTransferFeeConfigIxProto);
            public accounts?: (vixen.parser.IInitializeTransferFeeConfigAccountsProto|null);
            public data?: (vixen.parser.IInitializeTransferFeeConfigDataProto|null);
            public static create(properties?: vixen.parser.IInitializeTransferFeeConfigIxProto): vixen.parser.InitializeTransferFeeConfigIxProto;
            public static encode(message: vixen.parser.IInitializeTransferFeeConfigIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializeTransferFeeConfigIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializeTransferFeeConfigIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializeTransferFeeConfigIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializeTransferFeeConfigIxProto;
            public static toObject(message: vixen.parser.InitializeTransferFeeConfigIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IWithdrawWithheldTokensFromMintAccountsProto {
            mint?: (string|null);
            feeRecipient?: (string|null);
            withdrawWithheldAuthority?: (string|null);
            multisigSigners?: (string[]|null);
        }

        class WithdrawWithheldTokensFromMintAccountsProto implements IWithdrawWithheldTokensFromMintAccountsProto {
            constructor(properties?: vixen.parser.IWithdrawWithheldTokensFromMintAccountsProto);
            public mint: string;
            public feeRecipient: string;
            public withdrawWithheldAuthority: string;
            public multisigSigners: string[];
            public static create(properties?: vixen.parser.IWithdrawWithheldTokensFromMintAccountsProto): vixen.parser.WithdrawWithheldTokensFromMintAccountsProto;
            public static encode(message: vixen.parser.IWithdrawWithheldTokensFromMintAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IWithdrawWithheldTokensFromMintAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.WithdrawWithheldTokensFromMintAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.WithdrawWithheldTokensFromMintAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.WithdrawWithheldTokensFromMintAccountsProto;
            public static toObject(message: vixen.parser.WithdrawWithheldTokensFromMintAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IWithdrawWithheldTokensFromMintIxProto {
            accounts?: (vixen.parser.IWithdrawWithheldTokensFromMintAccountsProto|null);
        }

        class WithdrawWithheldTokensFromMintIxProto implements IWithdrawWithheldTokensFromMintIxProto {
            constructor(properties?: vixen.parser.IWithdrawWithheldTokensFromMintIxProto);
            public accounts?: (vixen.parser.IWithdrawWithheldTokensFromMintAccountsProto|null);
            public static create(properties?: vixen.parser.IWithdrawWithheldTokensFromMintIxProto): vixen.parser.WithdrawWithheldTokensFromMintIxProto;
            public static encode(message: vixen.parser.IWithdrawWithheldTokensFromMintIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IWithdrawWithheldTokensFromMintIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.WithdrawWithheldTokensFromMintIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.WithdrawWithheldTokensFromMintIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.WithdrawWithheldTokensFromMintIxProto;
            public static toObject(message: vixen.parser.WithdrawWithheldTokensFromMintIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IWithdrawWithheldTokensFromAccountsAccountsProto {
            mint?: (string|null);
            feeRecipient?: (string|null);
            withdrawWithheldAuthority?: (string|null);
            sourceAccounts?: (string[]|null);
            multisigSigners?: (string[]|null);
        }

        class WithdrawWithheldTokensFromAccountsAccountsProto implements IWithdrawWithheldTokensFromAccountsAccountsProto {
            constructor(properties?: vixen.parser.IWithdrawWithheldTokensFromAccountsAccountsProto);
            public mint: string;
            public feeRecipient: string;
            public withdrawWithheldAuthority: string;
            public sourceAccounts: string[];
            public multisigSigners: string[];
            public static create(properties?: vixen.parser.IWithdrawWithheldTokensFromAccountsAccountsProto): vixen.parser.WithdrawWithheldTokensFromAccountsAccountsProto;
            public static encode(message: vixen.parser.IWithdrawWithheldTokensFromAccountsAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IWithdrawWithheldTokensFromAccountsAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.WithdrawWithheldTokensFromAccountsAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.WithdrawWithheldTokensFromAccountsAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.WithdrawWithheldTokensFromAccountsAccountsProto;
            public static toObject(message: vixen.parser.WithdrawWithheldTokensFromAccountsAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IWithdrawWithheldTokensFromAccountsDataProto {
            numTokenAccounts?: (number|Long|null);
        }

        class WithdrawWithheldTokensFromAccountsDataProto implements IWithdrawWithheldTokensFromAccountsDataProto {
            constructor(properties?: vixen.parser.IWithdrawWithheldTokensFromAccountsDataProto);
            public numTokenAccounts: (number|Long);
            public static create(properties?: vixen.parser.IWithdrawWithheldTokensFromAccountsDataProto): vixen.parser.WithdrawWithheldTokensFromAccountsDataProto;
            public static encode(message: vixen.parser.IWithdrawWithheldTokensFromAccountsDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IWithdrawWithheldTokensFromAccountsDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.WithdrawWithheldTokensFromAccountsDataProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.WithdrawWithheldTokensFromAccountsDataProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.WithdrawWithheldTokensFromAccountsDataProto;
            public static toObject(message: vixen.parser.WithdrawWithheldTokensFromAccountsDataProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IWithdrawWithheldTokensFromAccountsIxProto {
            accounts?: (vixen.parser.IWithdrawWithheldTokensFromAccountsAccountsProto|null);
            data?: (vixen.parser.IWithdrawWithheldTokensFromAccountsDataProto|null);
        }

        class WithdrawWithheldTokensFromAccountsIxProto implements IWithdrawWithheldTokensFromAccountsIxProto {
            constructor(properties?: vixen.parser.IWithdrawWithheldTokensFromAccountsIxProto);
            public accounts?: (vixen.parser.IWithdrawWithheldTokensFromAccountsAccountsProto|null);
            public data?: (vixen.parser.IWithdrawWithheldTokensFromAccountsDataProto|null);
            public static create(properties?: vixen.parser.IWithdrawWithheldTokensFromAccountsIxProto): vixen.parser.WithdrawWithheldTokensFromAccountsIxProto;
            public static encode(message: vixen.parser.IWithdrawWithheldTokensFromAccountsIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IWithdrawWithheldTokensFromAccountsIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.WithdrawWithheldTokensFromAccountsIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.WithdrawWithheldTokensFromAccountsIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.WithdrawWithheldTokensFromAccountsIxProto;
            public static toObject(message: vixen.parser.WithdrawWithheldTokensFromAccountsIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IHarvestWithheldTokensToMintAccountsProto {
            mint?: (string|null);
            mintFeeOwner?: (string|null);
        }

        class HarvestWithheldTokensToMintAccountsProto implements IHarvestWithheldTokensToMintAccountsProto {
            constructor(properties?: vixen.parser.IHarvestWithheldTokensToMintAccountsProto);
            public mint: string;
            public mintFeeOwner: string;
            public static create(properties?: vixen.parser.IHarvestWithheldTokensToMintAccountsProto): vixen.parser.HarvestWithheldTokensToMintAccountsProto;
            public static encode(message: vixen.parser.IHarvestWithheldTokensToMintAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IHarvestWithheldTokensToMintAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.HarvestWithheldTokensToMintAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.HarvestWithheldTokensToMintAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.HarvestWithheldTokensToMintAccountsProto;
            public static toObject(message: vixen.parser.HarvestWithheldTokensToMintAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IHarvestWithheldTokensToMintIxProto {
            accounts?: (vixen.parser.IHarvestWithheldTokensToMintAccountsProto|null);
        }

        class HarvestWithheldTokensToMintIxProto implements IHarvestWithheldTokensToMintIxProto {
            constructor(properties?: vixen.parser.IHarvestWithheldTokensToMintIxProto);
            public accounts?: (vixen.parser.IHarvestWithheldTokensToMintAccountsProto|null);
            public static create(properties?: vixen.parser.IHarvestWithheldTokensToMintIxProto): vixen.parser.HarvestWithheldTokensToMintIxProto;
            public static encode(message: vixen.parser.IHarvestWithheldTokensToMintIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IHarvestWithheldTokensToMintIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.HarvestWithheldTokensToMintIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.HarvestWithheldTokensToMintIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.HarvestWithheldTokensToMintIxProto;
            public static toObject(message: vixen.parser.HarvestWithheldTokensToMintIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ISetTransferFeeAccountsProto {
            mint?: (string|null);
            mintFeeAccOwner?: (string|null);
            multisigSigners?: (string[]|null);
        }

        class SetTransferFeeAccountsProto implements ISetTransferFeeAccountsProto {
            constructor(properties?: vixen.parser.ISetTransferFeeAccountsProto);
            public mint: string;
            public mintFeeAccOwner: string;
            public multisigSigners: string[];
            public static create(properties?: vixen.parser.ISetTransferFeeAccountsProto): vixen.parser.SetTransferFeeAccountsProto;
            public static encode(message: vixen.parser.ISetTransferFeeAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ISetTransferFeeAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.SetTransferFeeAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.SetTransferFeeAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.SetTransferFeeAccountsProto;
            public static toObject(message: vixen.parser.SetTransferFeeAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ISetTransferFeeDataProto {
            transferFeeBasisPoints?: (number|Long|null);
            maximumFee?: (number|Long|null);
        }

        class SetTransferFeeDataProto implements ISetTransferFeeDataProto {
            constructor(properties?: vixen.parser.ISetTransferFeeDataProto);
            public transferFeeBasisPoints: (number|Long);
            public maximumFee: (number|Long);
            public static create(properties?: vixen.parser.ISetTransferFeeDataProto): vixen.parser.SetTransferFeeDataProto;
            public static encode(message: vixen.parser.ISetTransferFeeDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ISetTransferFeeDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.SetTransferFeeDataProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.SetTransferFeeDataProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.SetTransferFeeDataProto;
            public static toObject(message: vixen.parser.SetTransferFeeDataProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ISetTransferFeeIxProto {
            accounts?: (vixen.parser.ISetTransferFeeAccountsProto|null);
            data?: (vixen.parser.ISetTransferFeeDataProto|null);
        }

        class SetTransferFeeIxProto implements ISetTransferFeeIxProto {
            constructor(properties?: vixen.parser.ISetTransferFeeIxProto);
            public accounts?: (vixen.parser.ISetTransferFeeAccountsProto|null);
            public data?: (vixen.parser.ISetTransferFeeDataProto|null);
            public static create(properties?: vixen.parser.ISetTransferFeeIxProto): vixen.parser.SetTransferFeeIxProto;
            public static encode(message: vixen.parser.ISetTransferFeeIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ISetTransferFeeIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.SetTransferFeeIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.SetTransferFeeIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.SetTransferFeeIxProto;
            public static toObject(message: vixen.parser.SetTransferFeeIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ITransferFeeIxProto {
            transferCheckedWithFeeIx?: (vixen.parser.ITransferCheckedWithFeeIxProto|null);
            initializeTransferFeeConfigIx?: (vixen.parser.IInitializeTransferFeeConfigIxProto|null);
            withdrawWithheldTokensFromMintIx?: (vixen.parser.IWithdrawWithheldTokensFromMintIxProto|null);
            withdrawWithheldTokensFromAccountsIx?: (vixen.parser.IWithdrawWithheldTokensFromAccountsIxProto|null);
            harvestWithheldTokensToMintIx?: (vixen.parser.IHarvestWithheldTokensToMintIxProto|null);
            setTransferFeeIx?: (vixen.parser.ISetTransferFeeIxProto|null);
        }

        class TransferFeeIxProto implements ITransferFeeIxProto {
            constructor(properties?: vixen.parser.ITransferFeeIxProto);
            public transferCheckedWithFeeIx?: (vixen.parser.ITransferCheckedWithFeeIxProto|null);
            public initializeTransferFeeConfigIx?: (vixen.parser.IInitializeTransferFeeConfigIxProto|null);
            public withdrawWithheldTokensFromMintIx?: (vixen.parser.IWithdrawWithheldTokensFromMintIxProto|null);
            public withdrawWithheldTokensFromAccountsIx?: (vixen.parser.IWithdrawWithheldTokensFromAccountsIxProto|null);
            public harvestWithheldTokensToMintIx?: (vixen.parser.IHarvestWithheldTokensToMintIxProto|null);
            public setTransferFeeIx?: (vixen.parser.ISetTransferFeeIxProto|null);
            public ixOneof?: ("transferCheckedWithFeeIx"|"initializeTransferFeeConfigIx"|"withdrawWithheldTokensFromMintIx"|"withdrawWithheldTokensFromAccountsIx"|"harvestWithheldTokensToMintIx"|"setTransferFeeIx");
            public static create(properties?: vixen.parser.ITransferFeeIxProto): vixen.parser.TransferFeeIxProto;
            public static encode(message: vixen.parser.ITransferFeeIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ITransferFeeIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.TransferFeeIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.TransferFeeIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.TransferFeeIxProto;
            public static toObject(message: vixen.parser.TransferFeeIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializeAccountsProto {
            metadata?: (string|null);
            updateAuthority?: (string|null);
            mint?: (string|null);
            mintAuthority?: (string|null);
        }

        class InitializeAccountsProto implements IInitializeAccountsProto {
            constructor(properties?: vixen.parser.IInitializeAccountsProto);
            public metadata: string;
            public updateAuthority: string;
            public mint: string;
            public mintAuthority: string;
            public static create(properties?: vixen.parser.IInitializeAccountsProto): vixen.parser.InitializeAccountsProto;
            public static encode(message: vixen.parser.IInitializeAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializeAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializeAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializeAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializeAccountsProto;
            public static toObject(message: vixen.parser.InitializeAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializeDataProto {
            name?: (string|null);
            symbol?: (string|null);
            uri?: (string|null);
        }

        class InitializeDataProto implements IInitializeDataProto {
            constructor(properties?: vixen.parser.IInitializeDataProto);
            public name: string;
            public symbol: string;
            public uri: string;
            public static create(properties?: vixen.parser.IInitializeDataProto): vixen.parser.InitializeDataProto;
            public static encode(message: vixen.parser.IInitializeDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializeDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializeDataProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializeDataProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializeDataProto;
            public static toObject(message: vixen.parser.InitializeDataProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializeIxProto {
            accounts?: (vixen.parser.IInitializeAccountsProto|null);
            data?: (vixen.parser.IInitializeDataProto|null);
        }

        class InitializeIxProto implements IInitializeIxProto {
            constructor(properties?: vixen.parser.IInitializeIxProto);
            public accounts?: (vixen.parser.IInitializeAccountsProto|null);
            public data?: (vixen.parser.IInitializeDataProto|null);
            public static create(properties?: vixen.parser.IInitializeIxProto): vixen.parser.InitializeIxProto;
            public static encode(message: vixen.parser.IInitializeIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializeIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializeIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializeIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializeIxProto;
            public static toObject(message: vixen.parser.InitializeIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IUpdateFieldAccountsProto {
            metadata?: (string|null);
            updateAuthority?: (string|null);
        }

        class UpdateFieldAccountsProto implements IUpdateFieldAccountsProto {
            constructor(properties?: vixen.parser.IUpdateFieldAccountsProto);
            public metadata: string;
            public updateAuthority: string;
            public static create(properties?: vixen.parser.IUpdateFieldAccountsProto): vixen.parser.UpdateFieldAccountsProto;
            public static encode(message: vixen.parser.IUpdateFieldAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IUpdateFieldAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.UpdateFieldAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.UpdateFieldAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.UpdateFieldAccountsProto;
            public static toObject(message: vixen.parser.UpdateFieldAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IUpdateFieldDataProto {
            key?: (string|null);
            value?: (string|null);
        }

        class UpdateFieldDataProto implements IUpdateFieldDataProto {
            constructor(properties?: vixen.parser.IUpdateFieldDataProto);
            public key: string;
            public value: string;
            public static create(properties?: vixen.parser.IUpdateFieldDataProto): vixen.parser.UpdateFieldDataProto;
            public static encode(message: vixen.parser.IUpdateFieldDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IUpdateFieldDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.UpdateFieldDataProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.UpdateFieldDataProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.UpdateFieldDataProto;
            public static toObject(message: vixen.parser.UpdateFieldDataProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IUpdateFieldIxProto {
            accounts?: (vixen.parser.IUpdateFieldAccountsProto|null);
            data?: (vixen.parser.IUpdateFieldDataProto|null);
        }

        class UpdateFieldIxProto implements IUpdateFieldIxProto {
            constructor(properties?: vixen.parser.IUpdateFieldIxProto);
            public accounts?: (vixen.parser.IUpdateFieldAccountsProto|null);
            public data?: (vixen.parser.IUpdateFieldDataProto|null);
            public static create(properties?: vixen.parser.IUpdateFieldIxProto): vixen.parser.UpdateFieldIxProto;
            public static encode(message: vixen.parser.IUpdateFieldIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IUpdateFieldIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.UpdateFieldIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.UpdateFieldIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.UpdateFieldIxProto;
            public static toObject(message: vixen.parser.UpdateFieldIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IRmoveKeyAccountsProto {
            metadata?: (string|null);
            updateAuthority?: (string|null);
        }

        class RmoveKeyAccountsProto implements IRmoveKeyAccountsProto {
            constructor(properties?: vixen.parser.IRmoveKeyAccountsProto);
            public metadata: string;
            public updateAuthority: string;
            public static create(properties?: vixen.parser.IRmoveKeyAccountsProto): vixen.parser.RmoveKeyAccountsProto;
            public static encode(message: vixen.parser.IRmoveKeyAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IRmoveKeyAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.RmoveKeyAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.RmoveKeyAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.RmoveKeyAccountsProto;
            public static toObject(message: vixen.parser.RmoveKeyAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IRemoveKeyDataProto {
            idempotent?: (boolean|null);
            key?: (string|null);
        }

        class RemoveKeyDataProto implements IRemoveKeyDataProto {
            constructor(properties?: vixen.parser.IRemoveKeyDataProto);
            public idempotent: boolean;
            public key: string;
            public static create(properties?: vixen.parser.IRemoveKeyDataProto): vixen.parser.RemoveKeyDataProto;
            public static encode(message: vixen.parser.IRemoveKeyDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IRemoveKeyDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.RemoveKeyDataProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.RemoveKeyDataProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.RemoveKeyDataProto;
            public static toObject(message: vixen.parser.RemoveKeyDataProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IRemoveKeyIxProto {
            accounts?: (vixen.parser.IRmoveKeyAccountsProto|null);
            data?: (vixen.parser.IRemoveKeyDataProto|null);
        }

        class RemoveKeyIxProto implements IRemoveKeyIxProto {
            constructor(properties?: vixen.parser.IRemoveKeyIxProto);
            public accounts?: (vixen.parser.IRmoveKeyAccountsProto|null);
            public data?: (vixen.parser.IRemoveKeyDataProto|null);
            public static create(properties?: vixen.parser.IRemoveKeyIxProto): vixen.parser.RemoveKeyIxProto;
            public static encode(message: vixen.parser.IRemoveKeyIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IRemoveKeyIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.RemoveKeyIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.RemoveKeyIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.RemoveKeyIxProto;
            public static toObject(message: vixen.parser.RemoveKeyIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IUpdateAuthorityAccountsProto {
            metadata?: (string|null);
            currentUpdateAuthority?: (string|null);
        }

        class UpdateAuthorityAccountsProto implements IUpdateAuthorityAccountsProto {
            constructor(properties?: vixen.parser.IUpdateAuthorityAccountsProto);
            public metadata: string;
            public currentUpdateAuthority: string;
            public static create(properties?: vixen.parser.IUpdateAuthorityAccountsProto): vixen.parser.UpdateAuthorityAccountsProto;
            public static encode(message: vixen.parser.IUpdateAuthorityAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IUpdateAuthorityAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.UpdateAuthorityAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.UpdateAuthorityAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.UpdateAuthorityAccountsProto;
            public static toObject(message: vixen.parser.UpdateAuthorityAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IUpdateAuthorityDataProto {
            newAuthority?: (string|null);
        }

        class UpdateAuthorityDataProto implements IUpdateAuthorityDataProto {
            constructor(properties?: vixen.parser.IUpdateAuthorityDataProto);
            public newAuthority: string;
            public static create(properties?: vixen.parser.IUpdateAuthorityDataProto): vixen.parser.UpdateAuthorityDataProto;
            public static encode(message: vixen.parser.IUpdateAuthorityDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IUpdateAuthorityDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.UpdateAuthorityDataProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.UpdateAuthorityDataProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.UpdateAuthorityDataProto;
            public static toObject(message: vixen.parser.UpdateAuthorityDataProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IUpdateAuthorityIxProto {
            accounts?: (vixen.parser.IUpdateAuthorityAccountsProto|null);
            data?: (vixen.parser.IUpdateAuthorityDataProto|null);
        }

        class UpdateAuthorityIxProto implements IUpdateAuthorityIxProto {
            constructor(properties?: vixen.parser.IUpdateAuthorityIxProto);
            public accounts?: (vixen.parser.IUpdateAuthorityAccountsProto|null);
            public data?: (vixen.parser.IUpdateAuthorityDataProto|null);
            public static create(properties?: vixen.parser.IUpdateAuthorityIxProto): vixen.parser.UpdateAuthorityIxProto;
            public static encode(message: vixen.parser.IUpdateAuthorityIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IUpdateAuthorityIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.UpdateAuthorityIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.UpdateAuthorityIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.UpdateAuthorityIxProto;
            public static toObject(message: vixen.parser.UpdateAuthorityIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IEmitAccountsProto {
            metadata?: (string|null);
        }

        class EmitAccountsProto implements IEmitAccountsProto {
            constructor(properties?: vixen.parser.IEmitAccountsProto);
            public metadata: string;
            public static create(properties?: vixen.parser.IEmitAccountsProto): vixen.parser.EmitAccountsProto;
            public static encode(message: vixen.parser.IEmitAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IEmitAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.EmitAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.EmitAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.EmitAccountsProto;
            public static toObject(message: vixen.parser.EmitAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IEmitDataProto {
            start?: (number|Long|null);
            end?: (number|Long|null);
        }

        class EmitDataProto implements IEmitDataProto {
            constructor(properties?: vixen.parser.IEmitDataProto);
            public start?: (number|Long|null);
            public end?: (number|Long|null);
            public static create(properties?: vixen.parser.IEmitDataProto): vixen.parser.EmitDataProto;
            public static encode(message: vixen.parser.IEmitDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IEmitDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.EmitDataProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.EmitDataProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.EmitDataProto;
            public static toObject(message: vixen.parser.EmitDataProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IEmitIxProto {
            accounts?: (vixen.parser.IEmitAccountsProto|null);
            data?: (vixen.parser.IEmitDataProto|null);
        }

        class EmitIxProto implements IEmitIxProto {
            constructor(properties?: vixen.parser.IEmitIxProto);
            public accounts?: (vixen.parser.IEmitAccountsProto|null);
            public data?: (vixen.parser.IEmitDataProto|null);
            public static create(properties?: vixen.parser.IEmitIxProto): vixen.parser.EmitIxProto;
            public static encode(message: vixen.parser.IEmitIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IEmitIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.EmitIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.EmitIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.EmitIxProto;
            public static toObject(message: vixen.parser.EmitIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ITokenMetadataIxProto {
            initializeIx?: (vixen.parser.IInitializeIxProto|null);
            updateFieldsIx?: (vixen.parser.IUpdateFieldIxProto|null);
            removeKeyIx?: (vixen.parser.IRemoveKeyIxProto|null);
            updateAuthorityIx?: (vixen.parser.IUpdateAuthorityIxProto|null);
            emitIx?: (vixen.parser.IEmitIxProto|null);
        }

        class TokenMetadataIxProto implements ITokenMetadataIxProto {
            constructor(properties?: vixen.parser.ITokenMetadataIxProto);
            public initializeIx?: (vixen.parser.IInitializeIxProto|null);
            public updateFieldsIx?: (vixen.parser.IUpdateFieldIxProto|null);
            public removeKeyIx?: (vixen.parser.IRemoveKeyIxProto|null);
            public updateAuthorityIx?: (vixen.parser.IUpdateAuthorityIxProto|null);
            public emitIx?: (vixen.parser.IEmitIxProto|null);
            public ixOneof?: ("initializeIx"|"updateFieldsIx"|"removeKeyIx"|"updateAuthorityIx"|"emitIx");
            public static create(properties?: vixen.parser.ITokenMetadataIxProto): vixen.parser.TokenMetadataIxProto;
            public static encode(message: vixen.parser.ITokenMetadataIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ITokenMetadataIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.TokenMetadataIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.TokenMetadataIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.TokenMetadataIxProto;
            public static toObject(message: vixen.parser.TokenMetadataIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializeGroupAccountsProto {
            group?: (string|null);
            mint?: (string|null);
            mintAuthority?: (string|null);
        }

        class InitializeGroupAccountsProto implements IInitializeGroupAccountsProto {
            constructor(properties?: vixen.parser.IInitializeGroupAccountsProto);
            public group: string;
            public mint: string;
            public mintAuthority: string;
            public static create(properties?: vixen.parser.IInitializeGroupAccountsProto): vixen.parser.InitializeGroupAccountsProto;
            public static encode(message: vixen.parser.IInitializeGroupAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializeGroupAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializeGroupAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializeGroupAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializeGroupAccountsProto;
            public static toObject(message: vixen.parser.InitializeGroupAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializeGroupDataProto {
            updateAuthority?: (string|null);
            maxSize?: (number|Long|null);
        }

        class InitializeGroupDataProto implements IInitializeGroupDataProto {
            constructor(properties?: vixen.parser.IInitializeGroupDataProto);
            public updateAuthority?: (string|null);
            public maxSize: (number|Long);
            public static create(properties?: vixen.parser.IInitializeGroupDataProto): vixen.parser.InitializeGroupDataProto;
            public static encode(message: vixen.parser.IInitializeGroupDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializeGroupDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializeGroupDataProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializeGroupDataProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializeGroupDataProto;
            public static toObject(message: vixen.parser.InitializeGroupDataProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializeGroupIxProto {
            accounts?: (vixen.parser.IInitializeGroupAccountsProto|null);
            data?: (vixen.parser.IInitializeGroupDataProto|null);
        }

        class InitializeGroupIxProto implements IInitializeGroupIxProto {
            constructor(properties?: vixen.parser.IInitializeGroupIxProto);
            public accounts?: (vixen.parser.IInitializeGroupAccountsProto|null);
            public data?: (vixen.parser.IInitializeGroupDataProto|null);
            public static create(properties?: vixen.parser.IInitializeGroupIxProto): vixen.parser.InitializeGroupIxProto;
            public static encode(message: vixen.parser.IInitializeGroupIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializeGroupIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializeGroupIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializeGroupIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializeGroupIxProto;
            public static toObject(message: vixen.parser.InitializeGroupIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IUpdateGroupMaxSizeAccountsProto {
            group?: (string|null);
            updateAuthority?: (string|null);
        }

        class UpdateGroupMaxSizeAccountsProto implements IUpdateGroupMaxSizeAccountsProto {
            constructor(properties?: vixen.parser.IUpdateGroupMaxSizeAccountsProto);
            public group: string;
            public updateAuthority: string;
            public static create(properties?: vixen.parser.IUpdateGroupMaxSizeAccountsProto): vixen.parser.UpdateGroupMaxSizeAccountsProto;
            public static encode(message: vixen.parser.IUpdateGroupMaxSizeAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IUpdateGroupMaxSizeAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.UpdateGroupMaxSizeAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.UpdateGroupMaxSizeAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.UpdateGroupMaxSizeAccountsProto;
            public static toObject(message: vixen.parser.UpdateGroupMaxSizeAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IUpdateGroupMaxSizeDataProto {
            maxSize?: (number|Long|null);
        }

        class UpdateGroupMaxSizeDataProto implements IUpdateGroupMaxSizeDataProto {
            constructor(properties?: vixen.parser.IUpdateGroupMaxSizeDataProto);
            public maxSize: (number|Long);
            public static create(properties?: vixen.parser.IUpdateGroupMaxSizeDataProto): vixen.parser.UpdateGroupMaxSizeDataProto;
            public static encode(message: vixen.parser.IUpdateGroupMaxSizeDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IUpdateGroupMaxSizeDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.UpdateGroupMaxSizeDataProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.UpdateGroupMaxSizeDataProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.UpdateGroupMaxSizeDataProto;
            public static toObject(message: vixen.parser.UpdateGroupMaxSizeDataProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IUpdateGroupMaxSizeIxProto {
            accounts?: (vixen.parser.IUpdateGroupMaxSizeAccountsProto|null);
            data?: (vixen.parser.IUpdateGroupMaxSizeDataProto|null);
        }

        class UpdateGroupMaxSizeIxProto implements IUpdateGroupMaxSizeIxProto {
            constructor(properties?: vixen.parser.IUpdateGroupMaxSizeIxProto);
            public accounts?: (vixen.parser.IUpdateGroupMaxSizeAccountsProto|null);
            public data?: (vixen.parser.IUpdateGroupMaxSizeDataProto|null);
            public static create(properties?: vixen.parser.IUpdateGroupMaxSizeIxProto): vixen.parser.UpdateGroupMaxSizeIxProto;
            public static encode(message: vixen.parser.IUpdateGroupMaxSizeIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IUpdateGroupMaxSizeIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.UpdateGroupMaxSizeIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.UpdateGroupMaxSizeIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.UpdateGroupMaxSizeIxProto;
            public static toObject(message: vixen.parser.UpdateGroupMaxSizeIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IUpdateGroupAuthorityAccountsProto {
            group?: (string|null);
            currentAuthority?: (string|null);
        }

        class UpdateGroupAuthorityAccountsProto implements IUpdateGroupAuthorityAccountsProto {
            constructor(properties?: vixen.parser.IUpdateGroupAuthorityAccountsProto);
            public group: string;
            public currentAuthority: string;
            public static create(properties?: vixen.parser.IUpdateGroupAuthorityAccountsProto): vixen.parser.UpdateGroupAuthorityAccountsProto;
            public static encode(message: vixen.parser.IUpdateGroupAuthorityAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IUpdateGroupAuthorityAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.UpdateGroupAuthorityAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.UpdateGroupAuthorityAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.UpdateGroupAuthorityAccountsProto;
            public static toObject(message: vixen.parser.UpdateGroupAuthorityAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IUpdateGroupAuthorityDataProto {
            newAuthority?: (string|null);
        }

        class UpdateGroupAuthorityDataProto implements IUpdateGroupAuthorityDataProto {
            constructor(properties?: vixen.parser.IUpdateGroupAuthorityDataProto);
            public newAuthority: string;
            public static create(properties?: vixen.parser.IUpdateGroupAuthorityDataProto): vixen.parser.UpdateGroupAuthorityDataProto;
            public static encode(message: vixen.parser.IUpdateGroupAuthorityDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IUpdateGroupAuthorityDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.UpdateGroupAuthorityDataProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.UpdateGroupAuthorityDataProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.UpdateGroupAuthorityDataProto;
            public static toObject(message: vixen.parser.UpdateGroupAuthorityDataProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IUpdateGroupAuthorityIxProto {
            accounts?: (vixen.parser.IUpdateGroupAuthorityAccountsProto|null);
            data?: (vixen.parser.IUpdateGroupAuthorityDataProto|null);
        }

        class UpdateGroupAuthorityIxProto implements IUpdateGroupAuthorityIxProto {
            constructor(properties?: vixen.parser.IUpdateGroupAuthorityIxProto);
            public accounts?: (vixen.parser.IUpdateGroupAuthorityAccountsProto|null);
            public data?: (vixen.parser.IUpdateGroupAuthorityDataProto|null);
            public static create(properties?: vixen.parser.IUpdateGroupAuthorityIxProto): vixen.parser.UpdateGroupAuthorityIxProto;
            public static encode(message: vixen.parser.IUpdateGroupAuthorityIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IUpdateGroupAuthorityIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.UpdateGroupAuthorityIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.UpdateGroupAuthorityIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.UpdateGroupAuthorityIxProto;
            public static toObject(message: vixen.parser.UpdateGroupAuthorityIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializeMemberAccountsProto {
            member?: (string|null);
            memberMint?: (string|null);
            memberMintAuthority?: (string|null);
            group?: (string|null);
            groupUpdateAuthority?: (string|null);
        }

        class InitializeMemberAccountsProto implements IInitializeMemberAccountsProto {
            constructor(properties?: vixen.parser.IInitializeMemberAccountsProto);
            public member: string;
            public memberMint: string;
            public memberMintAuthority: string;
            public group: string;
            public groupUpdateAuthority: string;
            public static create(properties?: vixen.parser.IInitializeMemberAccountsProto): vixen.parser.InitializeMemberAccountsProto;
            public static encode(message: vixen.parser.IInitializeMemberAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializeMemberAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializeMemberAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializeMemberAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializeMemberAccountsProto;
            public static toObject(message: vixen.parser.InitializeMemberAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializeMemberIxProto {
            accounts?: (vixen.parser.IInitializeMemberAccountsProto|null);
        }

        class InitializeMemberIxProto implements IInitializeMemberIxProto {
            constructor(properties?: vixen.parser.IInitializeMemberIxProto);
            public accounts?: (vixen.parser.IInitializeMemberAccountsProto|null);
            public static create(properties?: vixen.parser.IInitializeMemberIxProto): vixen.parser.InitializeMemberIxProto;
            public static encode(message: vixen.parser.IInitializeMemberIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializeMemberIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializeMemberIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializeMemberIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializeMemberIxProto;
            public static toObject(message: vixen.parser.InitializeMemberIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ITokenGroupIxProto {
            initializeGroupIx?: (vixen.parser.IInitializeGroupIxProto|null);
            updateGroupMaxSizeIx?: (vixen.parser.IUpdateGroupMaxSizeIxProto|null);
            updateGroupAuthorityIx?: (vixen.parser.IUpdateGroupAuthorityIxProto|null);
            initializeMemberIx?: (vixen.parser.IInitializeMemberIxProto|null);
        }

        class TokenGroupIxProto implements ITokenGroupIxProto {
            constructor(properties?: vixen.parser.ITokenGroupIxProto);
            public initializeGroupIx?: (vixen.parser.IInitializeGroupIxProto|null);
            public updateGroupMaxSizeIx?: (vixen.parser.IUpdateGroupMaxSizeIxProto|null);
            public updateGroupAuthorityIx?: (vixen.parser.IUpdateGroupAuthorityIxProto|null);
            public initializeMemberIx?: (vixen.parser.IInitializeMemberIxProto|null);
            public ixOneof?: ("initializeGroupIx"|"updateGroupMaxSizeIx"|"updateGroupAuthorityIx"|"initializeMemberIx");
            public static create(properties?: vixen.parser.ITokenGroupIxProto): vixen.parser.TokenGroupIxProto;
            public static encode(message: vixen.parser.ITokenGroupIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ITokenGroupIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.TokenGroupIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.TokenGroupIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.TokenGroupIxProto;
            public static toObject(message: vixen.parser.TokenGroupIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializeConfidentialMintAccountsProto {
            mint?: (string|null);
        }

        class InitializeConfidentialMintAccountsProto implements IInitializeConfidentialMintAccountsProto {
            constructor(properties?: vixen.parser.IInitializeConfidentialMintAccountsProto);
            public mint: string;
            public static create(properties?: vixen.parser.IInitializeConfidentialMintAccountsProto): vixen.parser.InitializeConfidentialMintAccountsProto;
            public static encode(message: vixen.parser.IInitializeConfidentialMintAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializeConfidentialMintAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializeConfidentialMintAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializeConfidentialMintAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializeConfidentialMintAccountsProto;
            public static toObject(message: vixen.parser.InitializeConfidentialMintAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializeConfidentialMintIxProto {
            accounts?: (vixen.parser.IInitializeConfidentialMintAccountsProto|null);
        }

        class InitializeConfidentialMintIxProto implements IInitializeConfidentialMintIxProto {
            constructor(properties?: vixen.parser.IInitializeConfidentialMintIxProto);
            public accounts?: (vixen.parser.IInitializeConfidentialMintAccountsProto|null);
            public static create(properties?: vixen.parser.IInitializeConfidentialMintIxProto): vixen.parser.InitializeConfidentialMintIxProto;
            public static encode(message: vixen.parser.IInitializeConfidentialMintIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializeConfidentialMintIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializeConfidentialMintIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializeConfidentialMintIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializeConfidentialMintIxProto;
            public static toObject(message: vixen.parser.InitializeConfidentialMintIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IUpdateMintAccountsProto {
            mint?: (string|null);
            authority?: (string|null);
        }

        class UpdateMintAccountsProto implements IUpdateMintAccountsProto {
            constructor(properties?: vixen.parser.IUpdateMintAccountsProto);
            public mint: string;
            public authority: string;
            public static create(properties?: vixen.parser.IUpdateMintAccountsProto): vixen.parser.UpdateMintAccountsProto;
            public static encode(message: vixen.parser.IUpdateMintAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IUpdateMintAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.UpdateMintAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.UpdateMintAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.UpdateMintAccountsProto;
            public static toObject(message: vixen.parser.UpdateMintAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IUpdateMintIxProto {
            accounts?: (vixen.parser.IUpdateMintAccountsProto|null);
        }

        class UpdateMintIxProto implements IUpdateMintIxProto {
            constructor(properties?: vixen.parser.IUpdateMintIxProto);
            public accounts?: (vixen.parser.IUpdateMintAccountsProto|null);
            public static create(properties?: vixen.parser.IUpdateMintIxProto): vixen.parser.UpdateMintIxProto;
            public static encode(message: vixen.parser.IUpdateMintIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IUpdateMintIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.UpdateMintIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.UpdateMintIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.UpdateMintIxProto;
            public static toObject(message: vixen.parser.UpdateMintIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IConfigureAccountAccountsProto {
            account?: (string|null);
            mint?: (string|null);
            sysvar?: (string|null);
            owner?: (string|null);
            multisigSigners?: (string[]|null);
        }

        class ConfigureAccountAccountsProto implements IConfigureAccountAccountsProto {
            constructor(properties?: vixen.parser.IConfigureAccountAccountsProto);
            public account: string;
            public mint: string;
            public sysvar: string;
            public owner: string;
            public multisigSigners: string[];
            public static create(properties?: vixen.parser.IConfigureAccountAccountsProto): vixen.parser.ConfigureAccountAccountsProto;
            public static encode(message: vixen.parser.IConfigureAccountAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IConfigureAccountAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ConfigureAccountAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ConfigureAccountAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ConfigureAccountAccountsProto;
            public static toObject(message: vixen.parser.ConfigureAccountAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IConfigureAccountIxProto {
            accounts?: (vixen.parser.IConfigureAccountAccountsProto|null);
        }

        class ConfigureAccountIxProto implements IConfigureAccountIxProto {
            constructor(properties?: vixen.parser.IConfigureAccountIxProto);
            public accounts?: (vixen.parser.IConfigureAccountAccountsProto|null);
            public static create(properties?: vixen.parser.IConfigureAccountIxProto): vixen.parser.ConfigureAccountIxProto;
            public static encode(message: vixen.parser.IConfigureAccountIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IConfigureAccountIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ConfigureAccountIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ConfigureAccountIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ConfigureAccountIxProto;
            public static toObject(message: vixen.parser.ConfigureAccountIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IApproveAccountAccountsProto {
            account?: (string|null);
            mint?: (string|null);
            authority?: (string|null);
        }

        class ApproveAccountAccountsProto implements IApproveAccountAccountsProto {
            constructor(properties?: vixen.parser.IApproveAccountAccountsProto);
            public account: string;
            public mint: string;
            public authority: string;
            public static create(properties?: vixen.parser.IApproveAccountAccountsProto): vixen.parser.ApproveAccountAccountsProto;
            public static encode(message: vixen.parser.IApproveAccountAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IApproveAccountAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ApproveAccountAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ApproveAccountAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ApproveAccountAccountsProto;
            public static toObject(message: vixen.parser.ApproveAccountAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IApproveAccountIxProto {
            accounts?: (vixen.parser.IApproveAccountAccountsProto|null);
        }

        class ApproveAccountIxProto implements IApproveAccountIxProto {
            constructor(properties?: vixen.parser.IApproveAccountIxProto);
            public accounts?: (vixen.parser.IApproveAccountAccountsProto|null);
            public static create(properties?: vixen.parser.IApproveAccountIxProto): vixen.parser.ApproveAccountIxProto;
            public static encode(message: vixen.parser.IApproveAccountIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IApproveAccountIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ApproveAccountIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ApproveAccountIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ApproveAccountIxProto;
            public static toObject(message: vixen.parser.ApproveAccountIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IEmptyAccountAccountsProto {
            account?: (string|null);
            sysvar?: (string|null);
            owner?: (string|null);
            multisigSigners?: (string[]|null);
        }

        class EmptyAccountAccountsProto implements IEmptyAccountAccountsProto {
            constructor(properties?: vixen.parser.IEmptyAccountAccountsProto);
            public account: string;
            public sysvar: string;
            public owner: string;
            public multisigSigners: string[];
            public static create(properties?: vixen.parser.IEmptyAccountAccountsProto): vixen.parser.EmptyAccountAccountsProto;
            public static encode(message: vixen.parser.IEmptyAccountAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IEmptyAccountAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.EmptyAccountAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.EmptyAccountAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.EmptyAccountAccountsProto;
            public static toObject(message: vixen.parser.EmptyAccountAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IEmptyAccountIxProto {
            accounts?: (vixen.parser.IEmptyAccountAccountsProto|null);
        }

        class EmptyAccountIxProto implements IEmptyAccountIxProto {
            constructor(properties?: vixen.parser.IEmptyAccountIxProto);
            public accounts?: (vixen.parser.IEmptyAccountAccountsProto|null);
            public static create(properties?: vixen.parser.IEmptyAccountIxProto): vixen.parser.EmptyAccountIxProto;
            public static encode(message: vixen.parser.IEmptyAccountIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IEmptyAccountIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.EmptyAccountIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.EmptyAccountIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.EmptyAccountIxProto;
            public static toObject(message: vixen.parser.EmptyAccountIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IDepositAccountsProto {
            account?: (string|null);
            mint?: (string|null);
            owner?: (string|null);
            multisigSigners?: (string[]|null);
        }

        class DepositAccountsProto implements IDepositAccountsProto {
            constructor(properties?: vixen.parser.IDepositAccountsProto);
            public account: string;
            public mint: string;
            public owner: string;
            public multisigSigners: string[];
            public static create(properties?: vixen.parser.IDepositAccountsProto): vixen.parser.DepositAccountsProto;
            public static encode(message: vixen.parser.IDepositAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IDepositAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.DepositAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.DepositAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.DepositAccountsProto;
            public static toObject(message: vixen.parser.DepositAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IDepositIxProto {
            accounts?: (vixen.parser.IDepositAccountsProto|null);
        }

        class DepositIxProto implements IDepositIxProto {
            constructor(properties?: vixen.parser.IDepositIxProto);
            public accounts?: (vixen.parser.IDepositAccountsProto|null);
            public static create(properties?: vixen.parser.IDepositIxProto): vixen.parser.DepositIxProto;
            public static encode(message: vixen.parser.IDepositIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IDepositIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.DepositIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.DepositIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.DepositIxProto;
            public static toObject(message: vixen.parser.DepositIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IWithdrawAccountsProto {
            sourceAccount?: (string|null);
            mint?: (string|null);
            destination?: (string|null);
            owner?: (string|null);
            multisigSigners?: (string[]|null);
        }

        class WithdrawAccountsProto implements IWithdrawAccountsProto {
            constructor(properties?: vixen.parser.IWithdrawAccountsProto);
            public sourceAccount: string;
            public mint: string;
            public destination: string;
            public owner: string;
            public multisigSigners: string[];
            public static create(properties?: vixen.parser.IWithdrawAccountsProto): vixen.parser.WithdrawAccountsProto;
            public static encode(message: vixen.parser.IWithdrawAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IWithdrawAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.WithdrawAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.WithdrawAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.WithdrawAccountsProto;
            public static toObject(message: vixen.parser.WithdrawAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IWithdrawIxProto {
            accounts?: (vixen.parser.IWithdrawAccountsProto|null);
        }

        class WithdrawIxProto implements IWithdrawIxProto {
            constructor(properties?: vixen.parser.IWithdrawIxProto);
            public accounts?: (vixen.parser.IWithdrawAccountsProto|null);
            public static create(properties?: vixen.parser.IWithdrawIxProto): vixen.parser.WithdrawIxProto;
            public static encode(message: vixen.parser.IWithdrawIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IWithdrawIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.WithdrawIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.WithdrawIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.WithdrawIxProto;
            public static toObject(message: vixen.parser.WithdrawIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IConfidentialTransferAccountsProto {
            sourceAccount?: (string|null);
            mint?: (string|null);
            destination?: (string|null);
            owner?: (string|null);
            contextAccount?: (string|null);
            multisigSigners?: (string[]|null);
        }

        class ConfidentialTransferAccountsProto implements IConfidentialTransferAccountsProto {
            constructor(properties?: vixen.parser.IConfidentialTransferAccountsProto);
            public sourceAccount: string;
            public mint: string;
            public destination: string;
            public owner: string;
            public contextAccount: string;
            public multisigSigners: string[];
            public static create(properties?: vixen.parser.IConfidentialTransferAccountsProto): vixen.parser.ConfidentialTransferAccountsProto;
            public static encode(message: vixen.parser.IConfidentialTransferAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IConfidentialTransferAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ConfidentialTransferAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ConfidentialTransferAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ConfidentialTransferAccountsProto;
            public static toObject(message: vixen.parser.ConfidentialTransferAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IConfidentialTransferIxProto {
            accounts?: (vixen.parser.IConfidentialTransferAccountsProto|null);
        }

        class ConfidentialTransferIxProto implements IConfidentialTransferIxProto {
            constructor(properties?: vixen.parser.IConfidentialTransferIxProto);
            public accounts?: (vixen.parser.IConfidentialTransferAccountsProto|null);
            public static create(properties?: vixen.parser.IConfidentialTransferIxProto): vixen.parser.ConfidentialTransferIxProto;
            public static encode(message: vixen.parser.IConfidentialTransferIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IConfidentialTransferIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ConfidentialTransferIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ConfidentialTransferIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ConfidentialTransferIxProto;
            public static toObject(message: vixen.parser.ConfidentialTransferIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IApplyPendingBalanceAccountsProto {
            account?: (string|null);
            owner?: (string|null);
            multisigSigners?: (string[]|null);
        }

        class ApplyPendingBalanceAccountsProto implements IApplyPendingBalanceAccountsProto {
            constructor(properties?: vixen.parser.IApplyPendingBalanceAccountsProto);
            public account: string;
            public owner: string;
            public multisigSigners: string[];
            public static create(properties?: vixen.parser.IApplyPendingBalanceAccountsProto): vixen.parser.ApplyPendingBalanceAccountsProto;
            public static encode(message: vixen.parser.IApplyPendingBalanceAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IApplyPendingBalanceAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ApplyPendingBalanceAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ApplyPendingBalanceAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ApplyPendingBalanceAccountsProto;
            public static toObject(message: vixen.parser.ApplyPendingBalanceAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IApplyPendingBalanceIxProto {
            accounts?: (vixen.parser.IApplyPendingBalanceAccountsProto|null);
        }

        class ApplyPendingBalanceIxProto implements IApplyPendingBalanceIxProto {
            constructor(properties?: vixen.parser.IApplyPendingBalanceIxProto);
            public accounts?: (vixen.parser.IApplyPendingBalanceAccountsProto|null);
            public static create(properties?: vixen.parser.IApplyPendingBalanceIxProto): vixen.parser.ApplyPendingBalanceIxProto;
            public static encode(message: vixen.parser.IApplyPendingBalanceIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IApplyPendingBalanceIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ApplyPendingBalanceIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ApplyPendingBalanceIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ApplyPendingBalanceIxProto;
            public static toObject(message: vixen.parser.ApplyPendingBalanceIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ICreditsAccountsProto {
            account?: (string|null);
            owner?: (string|null);
            multisigSigners?: (string[]|null);
        }

        class CreditsAccountsProto implements ICreditsAccountsProto {
            constructor(properties?: vixen.parser.ICreditsAccountsProto);
            public account: string;
            public owner: string;
            public multisigSigners: string[];
            public static create(properties?: vixen.parser.ICreditsAccountsProto): vixen.parser.CreditsAccountsProto;
            public static encode(message: vixen.parser.ICreditsAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ICreditsAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.CreditsAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.CreditsAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.CreditsAccountsProto;
            public static toObject(message: vixen.parser.CreditsAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IEnableConfidentialCreditsIxProto {
            accounts?: (vixen.parser.ICreditsAccountsProto|null);
        }

        class EnableConfidentialCreditsIxProto implements IEnableConfidentialCreditsIxProto {
            constructor(properties?: vixen.parser.IEnableConfidentialCreditsIxProto);
            public accounts?: (vixen.parser.ICreditsAccountsProto|null);
            public static create(properties?: vixen.parser.IEnableConfidentialCreditsIxProto): vixen.parser.EnableConfidentialCreditsIxProto;
            public static encode(message: vixen.parser.IEnableConfidentialCreditsIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IEnableConfidentialCreditsIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.EnableConfidentialCreditsIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.EnableConfidentialCreditsIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.EnableConfidentialCreditsIxProto;
            public static toObject(message: vixen.parser.EnableConfidentialCreditsIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IDisableConfidentialCreditsIxProto {
            accounts?: (vixen.parser.ICreditsAccountsProto|null);
        }

        class DisableConfidentialCreditsIxProto implements IDisableConfidentialCreditsIxProto {
            constructor(properties?: vixen.parser.IDisableConfidentialCreditsIxProto);
            public accounts?: (vixen.parser.ICreditsAccountsProto|null);
            public static create(properties?: vixen.parser.IDisableConfidentialCreditsIxProto): vixen.parser.DisableConfidentialCreditsIxProto;
            public static encode(message: vixen.parser.IDisableConfidentialCreditsIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IDisableConfidentialCreditsIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.DisableConfidentialCreditsIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.DisableConfidentialCreditsIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.DisableConfidentialCreditsIxProto;
            public static toObject(message: vixen.parser.DisableConfidentialCreditsIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IEnableNonConfidentialCreditsIxProto {
            accounts?: (vixen.parser.ICreditsAccountsProto|null);
        }

        class EnableNonConfidentialCreditsIxProto implements IEnableNonConfidentialCreditsIxProto {
            constructor(properties?: vixen.parser.IEnableNonConfidentialCreditsIxProto);
            public accounts?: (vixen.parser.ICreditsAccountsProto|null);
            public static create(properties?: vixen.parser.IEnableNonConfidentialCreditsIxProto): vixen.parser.EnableNonConfidentialCreditsIxProto;
            public static encode(message: vixen.parser.IEnableNonConfidentialCreditsIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IEnableNonConfidentialCreditsIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.EnableNonConfidentialCreditsIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.EnableNonConfidentialCreditsIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.EnableNonConfidentialCreditsIxProto;
            public static toObject(message: vixen.parser.EnableNonConfidentialCreditsIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IDisableNonConfidentialCreditsIxProto {
            accounts?: (vixen.parser.ICreditsAccountsProto|null);
        }

        class DisableNonConfidentialCreditsIxProto implements IDisableNonConfidentialCreditsIxProto {
            constructor(properties?: vixen.parser.IDisableNonConfidentialCreditsIxProto);
            public accounts?: (vixen.parser.ICreditsAccountsProto|null);
            public static create(properties?: vixen.parser.IDisableNonConfidentialCreditsIxProto): vixen.parser.DisableNonConfidentialCreditsIxProto;
            public static encode(message: vixen.parser.IDisableNonConfidentialCreditsIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IDisableNonConfidentialCreditsIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.DisableNonConfidentialCreditsIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.DisableNonConfidentialCreditsIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.DisableNonConfidentialCreditsIxProto;
            public static toObject(message: vixen.parser.DisableNonConfidentialCreditsIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ITransferWithSplitProofsAccountsProto {
            sourceAccount?: (string|null);
            mint?: (string|null);
            destination?: (string|null);
            verifyCiphertextCommitmentEqualityProof?: (string|null);
            verifyBatchedGroupedCipherText_2HandlesValidityProof?: (string|null);
            verifyBatchedRangeProofU128?: (string|null);
            verifyBatchedRangeProofU256?: (string|null);
            verifyBatchedGroupedCipherText_2HandlesValidityProofNext?: (string|null);
            verifyFeeSigmaProof?: (string|null);
            destinationAccountForLamports?: (string|null);
            contextStateAccountOwner?: (string|null);
            zkTokenProofProgram?: (string|null);
            owner?: (string|null);
        }

        class TransferWithSplitProofsAccountsProto implements ITransferWithSplitProofsAccountsProto {
            constructor(properties?: vixen.parser.ITransferWithSplitProofsAccountsProto);
            public sourceAccount: string;
            public mint: string;
            public destination: string;
            public verifyCiphertextCommitmentEqualityProof: string;
            public verifyBatchedGroupedCipherText_2HandlesValidityProof: string;
            public verifyBatchedRangeProofU128?: (string|null);
            public verifyBatchedRangeProofU256?: (string|null);
            public verifyBatchedGroupedCipherText_2HandlesValidityProofNext?: (string|null);
            public verifyFeeSigmaProof?: (string|null);
            public destinationAccountForLamports?: (string|null);
            public contextStateAccountOwner?: (string|null);
            public zkTokenProofProgram?: (string|null);
            public owner?: (string|null);
            public static create(properties?: vixen.parser.ITransferWithSplitProofsAccountsProto): vixen.parser.TransferWithSplitProofsAccountsProto;
            public static encode(message: vixen.parser.ITransferWithSplitProofsAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ITransferWithSplitProofsAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.TransferWithSplitProofsAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.TransferWithSplitProofsAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.TransferWithSplitProofsAccountsProto;
            public static toObject(message: vixen.parser.TransferWithSplitProofsAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ITransferWithSplitProofsIxProto {
            accounts?: (vixen.parser.ITransferWithSplitProofsAccountsProto|null);
        }

        class TransferWithSplitProofsIxProto implements ITransferWithSplitProofsIxProto {
            constructor(properties?: vixen.parser.ITransferWithSplitProofsIxProto);
            public accounts?: (vixen.parser.ITransferWithSplitProofsAccountsProto|null);
            public static create(properties?: vixen.parser.ITransferWithSplitProofsIxProto): vixen.parser.TransferWithSplitProofsIxProto;
            public static encode(message: vixen.parser.ITransferWithSplitProofsIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ITransferWithSplitProofsIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.TransferWithSplitProofsIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.TransferWithSplitProofsIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.TransferWithSplitProofsIxProto;
            public static toObject(message: vixen.parser.TransferWithSplitProofsIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IConfidentialTransferExtIxProto {
            initializeMintIx?: (vixen.parser.IInitializeConfidentialMintIxProto|null);
            updateMintIx?: (vixen.parser.IUpdateMintIxProto|null);
            configureAccountIx?: (vixen.parser.IConfigureAccountIxProto|null);
            approveAccountIx?: (vixen.parser.IApproveAccountIxProto|null);
            emptyAccountIx?: (vixen.parser.IEmptyAccountIxProto|null);
            depositIx?: (vixen.parser.IDepositIxProto|null);
            withdrawIx?: (vixen.parser.IWithdrawIxProto|null);
            transferIx?: (vixen.parser.IConfidentialTransferIxProto|null);
            applyPendingBalanceIx?: (vixen.parser.IApplyPendingBalanceIxProto|null);
            enableConfidentialCreditsIx?: (vixen.parser.IEnableConfidentialCreditsIxProto|null);
            disableConfidentialCreditsIx?: (vixen.parser.IDisableConfidentialCreditsIxProto|null);
            enableNonConfidentialCreditsIx?: (vixen.parser.IEnableNonConfidentialCreditsIxProto|null);
            disableNonConfidentialCreditsIx?: (vixen.parser.IDisableNonConfidentialCreditsIxProto|null);
            transferWithSplitProofsIx?: (vixen.parser.ITransferWithSplitProofsIxProto|null);
        }

        class ConfidentialTransferExtIxProto implements IConfidentialTransferExtIxProto {
            constructor(properties?: vixen.parser.IConfidentialTransferExtIxProto);
            public initializeMintIx?: (vixen.parser.IInitializeConfidentialMintIxProto|null);
            public updateMintIx?: (vixen.parser.IUpdateMintIxProto|null);
            public configureAccountIx?: (vixen.parser.IConfigureAccountIxProto|null);
            public approveAccountIx?: (vixen.parser.IApproveAccountIxProto|null);
            public emptyAccountIx?: (vixen.parser.IEmptyAccountIxProto|null);
            public depositIx?: (vixen.parser.IDepositIxProto|null);
            public withdrawIx?: (vixen.parser.IWithdrawIxProto|null);
            public transferIx?: (vixen.parser.IConfidentialTransferIxProto|null);
            public applyPendingBalanceIx?: (vixen.parser.IApplyPendingBalanceIxProto|null);
            public enableConfidentialCreditsIx?: (vixen.parser.IEnableConfidentialCreditsIxProto|null);
            public disableConfidentialCreditsIx?: (vixen.parser.IDisableConfidentialCreditsIxProto|null);
            public enableNonConfidentialCreditsIx?: (vixen.parser.IEnableNonConfidentialCreditsIxProto|null);
            public disableNonConfidentialCreditsIx?: (vixen.parser.IDisableNonConfidentialCreditsIxProto|null);
            public transferWithSplitProofsIx?: (vixen.parser.ITransferWithSplitProofsIxProto|null);
            public ixOneof?: ("initializeMintIx"|"updateMintIx"|"configureAccountIx"|"approveAccountIx"|"emptyAccountIx"|"depositIx"|"withdrawIx"|"transferIx"|"applyPendingBalanceIx"|"enableConfidentialCreditsIx"|"disableConfidentialCreditsIx"|"enableNonConfidentialCreditsIx"|"disableNonConfidentialCreditsIx"|"transferWithSplitProofsIx");
            public static create(properties?: vixen.parser.IConfidentialTransferExtIxProto): vixen.parser.ConfidentialTransferExtIxProto;
            public static encode(message: vixen.parser.IConfidentialTransferExtIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IConfidentialTransferExtIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ConfidentialTransferExtIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ConfidentialTransferExtIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ConfidentialTransferExtIxProto;
            public static toObject(message: vixen.parser.ConfidentialTransferExtIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializeConfidentialTransferFeeConfigAccountsProto {
            mint?: (string|null);
        }

        class InitializeConfidentialTransferFeeConfigAccountsProto implements IInitializeConfidentialTransferFeeConfigAccountsProto {
            constructor(properties?: vixen.parser.IInitializeConfidentialTransferFeeConfigAccountsProto);
            public mint: string;
            public static create(properties?: vixen.parser.IInitializeConfidentialTransferFeeConfigAccountsProto): vixen.parser.InitializeConfidentialTransferFeeConfigAccountsProto;
            public static encode(message: vixen.parser.IInitializeConfidentialTransferFeeConfigAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializeConfidentialTransferFeeConfigAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializeConfidentialTransferFeeConfigAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializeConfidentialTransferFeeConfigAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializeConfidentialTransferFeeConfigAccountsProto;
            public static toObject(message: vixen.parser.InitializeConfidentialTransferFeeConfigAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializeConfidentialTransferFeeConfigIxProto {
            accounts?: (vixen.parser.IInitializeConfidentialTransferFeeConfigAccountsProto|null);
        }

        class InitializeConfidentialTransferFeeConfigIxProto implements IInitializeConfidentialTransferFeeConfigIxProto {
            constructor(properties?: vixen.parser.IInitializeConfidentialTransferFeeConfigIxProto);
            public accounts?: (vixen.parser.IInitializeConfidentialTransferFeeConfigAccountsProto|null);
            public static create(properties?: vixen.parser.IInitializeConfidentialTransferFeeConfigIxProto): vixen.parser.InitializeConfidentialTransferFeeConfigIxProto;
            public static encode(message: vixen.parser.IInitializeConfidentialTransferFeeConfigIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializeConfidentialTransferFeeConfigIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializeConfidentialTransferFeeConfigIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializeConfidentialTransferFeeConfigIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializeConfidentialTransferFeeConfigIxProto;
            public static toObject(message: vixen.parser.InitializeConfidentialTransferFeeConfigIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IConfidentialWithdrawWithheldTokensFromMintAccountsProto {
            mint?: (string|null);
            feeRecipient?: (string|null);
            sysvar?: (string|null);
            withdrawWithheldAuthority?: (string|null);
            multisigSigners?: (string[]|null);
        }

        class ConfidentialWithdrawWithheldTokensFromMintAccountsProto implements IConfidentialWithdrawWithheldTokensFromMintAccountsProto {
            constructor(properties?: vixen.parser.IConfidentialWithdrawWithheldTokensFromMintAccountsProto);
            public mint: string;
            public feeRecipient: string;
            public sysvar: string;
            public withdrawWithheldAuthority: string;
            public multisigSigners: string[];
            public static create(properties?: vixen.parser.IConfidentialWithdrawWithheldTokensFromMintAccountsProto): vixen.parser.ConfidentialWithdrawWithheldTokensFromMintAccountsProto;
            public static encode(message: vixen.parser.IConfidentialWithdrawWithheldTokensFromMintAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IConfidentialWithdrawWithheldTokensFromMintAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ConfidentialWithdrawWithheldTokensFromMintAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ConfidentialWithdrawWithheldTokensFromMintAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ConfidentialWithdrawWithheldTokensFromMintAccountsProto;
            public static toObject(message: vixen.parser.ConfidentialWithdrawWithheldTokensFromMintAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IConfidentialWithdrawWithheldTokensFromMintIxProto {
            accounts?: (vixen.parser.IConfidentialWithdrawWithheldTokensFromMintAccountsProto|null);
        }

        class ConfidentialWithdrawWithheldTokensFromMintIxProto implements IConfidentialWithdrawWithheldTokensFromMintIxProto {
            constructor(properties?: vixen.parser.IConfidentialWithdrawWithheldTokensFromMintIxProto);
            public accounts?: (vixen.parser.IConfidentialWithdrawWithheldTokensFromMintAccountsProto|null);
            public static create(properties?: vixen.parser.IConfidentialWithdrawWithheldTokensFromMintIxProto): vixen.parser.ConfidentialWithdrawWithheldTokensFromMintIxProto;
            public static encode(message: vixen.parser.IConfidentialWithdrawWithheldTokensFromMintIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IConfidentialWithdrawWithheldTokensFromMintIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ConfidentialWithdrawWithheldTokensFromMintIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ConfidentialWithdrawWithheldTokensFromMintIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ConfidentialWithdrawWithheldTokensFromMintIxProto;
            public static toObject(message: vixen.parser.ConfidentialWithdrawWithheldTokensFromMintIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IConfidentialWithdrawWithheldTokensFromAccountsAccountsProto {
            mint?: (string|null);
            feeRecipient?: (string|null);
            withdrawWithheldAuthority?: (string|null);
            sourceAccounts?: (string[]|null);
            multisigSigners?: (string[]|null);
        }

        class ConfidentialWithdrawWithheldTokensFromAccountsAccountsProto implements IConfidentialWithdrawWithheldTokensFromAccountsAccountsProto {
            constructor(properties?: vixen.parser.IConfidentialWithdrawWithheldTokensFromAccountsAccountsProto);
            public mint: string;
            public feeRecipient: string;
            public withdrawWithheldAuthority: string;
            public sourceAccounts: string[];
            public multisigSigners: string[];
            public static create(properties?: vixen.parser.IConfidentialWithdrawWithheldTokensFromAccountsAccountsProto): vixen.parser.ConfidentialWithdrawWithheldTokensFromAccountsAccountsProto;
            public static encode(message: vixen.parser.IConfidentialWithdrawWithheldTokensFromAccountsAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IConfidentialWithdrawWithheldTokensFromAccountsAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ConfidentialWithdrawWithheldTokensFromAccountsAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ConfidentialWithdrawWithheldTokensFromAccountsAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ConfidentialWithdrawWithheldTokensFromAccountsAccountsProto;
            public static toObject(message: vixen.parser.ConfidentialWithdrawWithheldTokensFromAccountsAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IConfidentialWithdrawWithheldTokensFromAccountsIxProto {
            accounts?: (vixen.parser.IConfidentialWithdrawWithheldTokensFromAccountsAccountsProto|null);
        }

        class ConfidentialWithdrawWithheldTokensFromAccountsIxProto implements IConfidentialWithdrawWithheldTokensFromAccountsIxProto {
            constructor(properties?: vixen.parser.IConfidentialWithdrawWithheldTokensFromAccountsIxProto);
            public accounts?: (vixen.parser.IConfidentialWithdrawWithheldTokensFromAccountsAccountsProto|null);
            public static create(properties?: vixen.parser.IConfidentialWithdrawWithheldTokensFromAccountsIxProto): vixen.parser.ConfidentialWithdrawWithheldTokensFromAccountsIxProto;
            public static encode(message: vixen.parser.IConfidentialWithdrawWithheldTokensFromAccountsIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IConfidentialWithdrawWithheldTokensFromAccountsIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ConfidentialWithdrawWithheldTokensFromAccountsIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ConfidentialWithdrawWithheldTokensFromAccountsIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ConfidentialWithdrawWithheldTokensFromAccountsIxProto;
            public static toObject(message: vixen.parser.ConfidentialWithdrawWithheldTokensFromAccountsIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IConfidentialHarvestWithheldTokensToMintAccountsProto {
            mint?: (string|null);
            sourceAccounts?: (string[]|null);
        }

        class ConfidentialHarvestWithheldTokensToMintAccountsProto implements IConfidentialHarvestWithheldTokensToMintAccountsProto {
            constructor(properties?: vixen.parser.IConfidentialHarvestWithheldTokensToMintAccountsProto);
            public mint: string;
            public sourceAccounts: string[];
            public static create(properties?: vixen.parser.IConfidentialHarvestWithheldTokensToMintAccountsProto): vixen.parser.ConfidentialHarvestWithheldTokensToMintAccountsProto;
            public static encode(message: vixen.parser.IConfidentialHarvestWithheldTokensToMintAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IConfidentialHarvestWithheldTokensToMintAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ConfidentialHarvestWithheldTokensToMintAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ConfidentialHarvestWithheldTokensToMintAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ConfidentialHarvestWithheldTokensToMintAccountsProto;
            public static toObject(message: vixen.parser.ConfidentialHarvestWithheldTokensToMintAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IConfidentialHarvestWithheldTokensToMintIxProto {
            accounts?: (vixen.parser.IConfidentialHarvestWithheldTokensToMintAccountsProto|null);
        }

        class ConfidentialHarvestWithheldTokensToMintIxProto implements IConfidentialHarvestWithheldTokensToMintIxProto {
            constructor(properties?: vixen.parser.IConfidentialHarvestWithheldTokensToMintIxProto);
            public accounts?: (vixen.parser.IConfidentialHarvestWithheldTokensToMintAccountsProto|null);
            public static create(properties?: vixen.parser.IConfidentialHarvestWithheldTokensToMintIxProto): vixen.parser.ConfidentialHarvestWithheldTokensToMintIxProto;
            public static encode(message: vixen.parser.IConfidentialHarvestWithheldTokensToMintIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IConfidentialHarvestWithheldTokensToMintIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ConfidentialHarvestWithheldTokensToMintIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ConfidentialHarvestWithheldTokensToMintIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ConfidentialHarvestWithheldTokensToMintIxProto;
            public static toObject(message: vixen.parser.ConfidentialHarvestWithheldTokensToMintIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IEnableHarvestToMintAccountsProto {
            mint?: (string|null);
            confidentialTransferFeeAuthority?: (string|null);
            multisigSigners?: (string[]|null);
        }

        class EnableHarvestToMintAccountsProto implements IEnableHarvestToMintAccountsProto {
            constructor(properties?: vixen.parser.IEnableHarvestToMintAccountsProto);
            public mint: string;
            public confidentialTransferFeeAuthority: string;
            public multisigSigners: string[];
            public static create(properties?: vixen.parser.IEnableHarvestToMintAccountsProto): vixen.parser.EnableHarvestToMintAccountsProto;
            public static encode(message: vixen.parser.IEnableHarvestToMintAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IEnableHarvestToMintAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.EnableHarvestToMintAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.EnableHarvestToMintAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.EnableHarvestToMintAccountsProto;
            public static toObject(message: vixen.parser.EnableHarvestToMintAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IDisableHarvestToMintAccountsProto {
            account?: (string|null);
            confidentialTransferFeeAuthority?: (string|null);
            multisigSigners?: (string[]|null);
        }

        class DisableHarvestToMintAccountsProto implements IDisableHarvestToMintAccountsProto {
            constructor(properties?: vixen.parser.IDisableHarvestToMintAccountsProto);
            public account: string;
            public confidentialTransferFeeAuthority: string;
            public multisigSigners: string[];
            public static create(properties?: vixen.parser.IDisableHarvestToMintAccountsProto): vixen.parser.DisableHarvestToMintAccountsProto;
            public static encode(message: vixen.parser.IDisableHarvestToMintAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IDisableHarvestToMintAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.DisableHarvestToMintAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.DisableHarvestToMintAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.DisableHarvestToMintAccountsProto;
            public static toObject(message: vixen.parser.DisableHarvestToMintAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IEnableHarvestToMintIxProto {
            accounts?: (vixen.parser.IEnableHarvestToMintAccountsProto|null);
        }

        class EnableHarvestToMintIxProto implements IEnableHarvestToMintIxProto {
            constructor(properties?: vixen.parser.IEnableHarvestToMintIxProto);
            public accounts?: (vixen.parser.IEnableHarvestToMintAccountsProto|null);
            public static create(properties?: vixen.parser.IEnableHarvestToMintIxProto): vixen.parser.EnableHarvestToMintIxProto;
            public static encode(message: vixen.parser.IEnableHarvestToMintIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IEnableHarvestToMintIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.EnableHarvestToMintIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.EnableHarvestToMintIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.EnableHarvestToMintIxProto;
            public static toObject(message: vixen.parser.EnableHarvestToMintIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IDisableHarvestToMintIxProto {
            accounts?: (vixen.parser.IDisableHarvestToMintAccountsProto|null);
        }

        class DisableHarvestToMintIxProto implements IDisableHarvestToMintIxProto {
            constructor(properties?: vixen.parser.IDisableHarvestToMintIxProto);
            public accounts?: (vixen.parser.IDisableHarvestToMintAccountsProto|null);
            public static create(properties?: vixen.parser.IDisableHarvestToMintIxProto): vixen.parser.DisableHarvestToMintIxProto;
            public static encode(message: vixen.parser.IDisableHarvestToMintIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IDisableHarvestToMintIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.DisableHarvestToMintIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.DisableHarvestToMintIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.DisableHarvestToMintIxProto;
            public static toObject(message: vixen.parser.DisableHarvestToMintIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IConfidentialTransferFeeIxProto {
            initializeConfidentialTransferFeeConfigIx?: (vixen.parser.IInitializeConfidentialTransferFeeConfigIxProto|null);
            withdrawWithheldTokensFromMintIx?: (vixen.parser.IConfidentialWithdrawWithheldTokensFromMintIxProto|null);
            withdrawWithheldTokensFromAccountsIx?: (vixen.parser.IConfidentialWithdrawWithheldTokensFromAccountsIxProto|null);
            harvestWithheldTokensToMintIx?: (vixen.parser.IConfidentialHarvestWithheldTokensToMintIxProto|null);
            enableHarvestToMintIx?: (vixen.parser.IEnableHarvestToMintIxProto|null);
            disableHarvestToMintIx?: (vixen.parser.IDisableHarvestToMintIxProto|null);
        }

        class ConfidentialTransferFeeIxProto implements IConfidentialTransferFeeIxProto {
            constructor(properties?: vixen.parser.IConfidentialTransferFeeIxProto);
            public initializeConfidentialTransferFeeConfigIx?: (vixen.parser.IInitializeConfidentialTransferFeeConfigIxProto|null);
            public withdrawWithheldTokensFromMintIx?: (vixen.parser.IConfidentialWithdrawWithheldTokensFromMintIxProto|null);
            public withdrawWithheldTokensFromAccountsIx?: (vixen.parser.IConfidentialWithdrawWithheldTokensFromAccountsIxProto|null);
            public harvestWithheldTokensToMintIx?: (vixen.parser.IConfidentialHarvestWithheldTokensToMintIxProto|null);
            public enableHarvestToMintIx?: (vixen.parser.IEnableHarvestToMintIxProto|null);
            public disableHarvestToMintIx?: (vixen.parser.IDisableHarvestToMintIxProto|null);
            public ixOneof?: ("initializeConfidentialTransferFeeConfigIx"|"withdrawWithheldTokensFromMintIx"|"withdrawWithheldTokensFromAccountsIx"|"harvestWithheldTokensToMintIx"|"enableHarvestToMintIx"|"disableHarvestToMintIx");
            public static create(properties?: vixen.parser.IConfidentialTransferFeeIxProto): vixen.parser.ConfidentialTransferFeeIxProto;
            public static encode(message: vixen.parser.IConfidentialTransferFeeIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IConfidentialTransferFeeIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ConfidentialTransferFeeIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ConfidentialTransferFeeIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ConfidentialTransferFeeIxProto;
            public static toObject(message: vixen.parser.ConfidentialTransferFeeIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IExtInitializeAccountsProto {
            mint?: (string|null);
        }

        class ExtInitializeAccountsProto implements IExtInitializeAccountsProto {
            constructor(properties?: vixen.parser.IExtInitializeAccountsProto);
            public mint: string;
            public static create(properties?: vixen.parser.IExtInitializeAccountsProto): vixen.parser.ExtInitializeAccountsProto;
            public static encode(message: vixen.parser.IExtInitializeAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IExtInitializeAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ExtInitializeAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ExtInitializeAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ExtInitializeAccountsProto;
            public static toObject(message: vixen.parser.ExtInitializeAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IExtInitializeIxProto {
            accounts?: (vixen.parser.IExtInitializeAccountsProto|null);
        }

        class ExtInitializeIxProto implements IExtInitializeIxProto {
            constructor(properties?: vixen.parser.IExtInitializeIxProto);
            public accounts?: (vixen.parser.IExtInitializeAccountsProto|null);
            public static create(properties?: vixen.parser.IExtInitializeIxProto): vixen.parser.ExtInitializeIxProto;
            public static encode(message: vixen.parser.IExtInitializeIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IExtInitializeIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ExtInitializeIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ExtInitializeIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ExtInitializeIxProto;
            public static toObject(message: vixen.parser.ExtInitializeIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IUpdateAccountsProto {
            mint?: (string|null);
            extensionAuthority?: (string|null);
            multisigSigners?: (string[]|null);
        }

        class UpdateAccountsProto implements IUpdateAccountsProto {
            constructor(properties?: vixen.parser.IUpdateAccountsProto);
            public mint: string;
            public extensionAuthority: string;
            public multisigSigners: string[];
            public static create(properties?: vixen.parser.IUpdateAccountsProto): vixen.parser.UpdateAccountsProto;
            public static encode(message: vixen.parser.IUpdateAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IUpdateAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.UpdateAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.UpdateAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.UpdateAccountsProto;
            public static toObject(message: vixen.parser.UpdateAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IUpdateIxProto {
            accounts?: (vixen.parser.IUpdateAccountsProto|null);
        }

        class UpdateIxProto implements IUpdateIxProto {
            constructor(properties?: vixen.parser.IUpdateIxProto);
            public accounts?: (vixen.parser.IUpdateAccountsProto|null);
            public static create(properties?: vixen.parser.IUpdateIxProto): vixen.parser.UpdateIxProto;
            public static encode(message: vixen.parser.IUpdateIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IUpdateIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.UpdateIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.UpdateIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.UpdateIxProto;
            public static toObject(message: vixen.parser.UpdateIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IEnableAccountsProto {
            account?: (string|null);
            owner?: (string|null);
            multisigSigners?: (string[]|null);
        }

        class EnableAccountsProto implements IEnableAccountsProto {
            constructor(properties?: vixen.parser.IEnableAccountsProto);
            public account: string;
            public owner: string;
            public multisigSigners: string[];
            public static create(properties?: vixen.parser.IEnableAccountsProto): vixen.parser.EnableAccountsProto;
            public static encode(message: vixen.parser.IEnableAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IEnableAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.EnableAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.EnableAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.EnableAccountsProto;
            public static toObject(message: vixen.parser.EnableAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IEnableIxProto {
            accounts?: (vixen.parser.IEnableAccountsProto|null);
        }

        class EnableIxProto implements IEnableIxProto {
            constructor(properties?: vixen.parser.IEnableIxProto);
            public accounts?: (vixen.parser.IEnableAccountsProto|null);
            public static create(properties?: vixen.parser.IEnableIxProto): vixen.parser.EnableIxProto;
            public static encode(message: vixen.parser.IEnableIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IEnableIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.EnableIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.EnableIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.EnableIxProto;
            public static toObject(message: vixen.parser.EnableIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IDisableAccountsProto {
            account?: (string|null);
            owner?: (string|null);
            multisigSigners?: (string[]|null);
        }

        class DisableAccountsProto implements IDisableAccountsProto {
            constructor(properties?: vixen.parser.IDisableAccountsProto);
            public account: string;
            public owner: string;
            public multisigSigners: string[];
            public static create(properties?: vixen.parser.IDisableAccountsProto): vixen.parser.DisableAccountsProto;
            public static encode(message: vixen.parser.IDisableAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IDisableAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.DisableAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.DisableAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.DisableAccountsProto;
            public static toObject(message: vixen.parser.DisableAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IDisableIxProto {
            accounts?: (vixen.parser.IDisableAccountsProto|null);
        }

        class DisableIxProto implements IDisableIxProto {
            constructor(properties?: vixen.parser.IDisableIxProto);
            public accounts?: (vixen.parser.IDisableAccountsProto|null);
            public static create(properties?: vixen.parser.IDisableIxProto): vixen.parser.DisableIxProto;
            public static encode(message: vixen.parser.IDisableIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IDisableIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.DisableIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.DisableIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.DisableIxProto;
            public static toObject(message: vixen.parser.DisableIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ICommonExtensionIxProto {
            extInitializeIx?: (vixen.parser.IExtInitializeIxProto|null);
            updateIx?: (vixen.parser.IUpdateIxProto|null);
            enableIx?: (vixen.parser.IEnableIxProto|null);
            disableIx?: (vixen.parser.IDisableIxProto|null);
        }

        class CommonExtensionIxProto implements ICommonExtensionIxProto {
            constructor(properties?: vixen.parser.ICommonExtensionIxProto);
            public extInitializeIx?: (vixen.parser.IExtInitializeIxProto|null);
            public updateIx?: (vixen.parser.IUpdateIxProto|null);
            public enableIx?: (vixen.parser.IEnableIxProto|null);
            public disableIx?: (vixen.parser.IDisableIxProto|null);
            public ixOneof?: ("extInitializeIx"|"updateIx"|"enableIx"|"disableIx");
            public static create(properties?: vixen.parser.ICommonExtensionIxProto): vixen.parser.CommonExtensionIxProto;
            public static encode(message: vixen.parser.ICommonExtensionIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ICommonExtensionIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.CommonExtensionIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.CommonExtensionIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.CommonExtensionIxProto;
            public static toObject(message: vixen.parser.CommonExtensionIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        enum ExtensionWithCommonIxsProto {
            CPI_GUARD = 0,
            DEFAULT_ACCOUNT_STATE = 1,
            INTEREST_BEARING_MINT = 2,
            MEMO_TRANSFER = 3,
            GROUP_MEMBER_POINTER = 4,
            GROUP_POINTER = 5,
            METADATA_POINTER = 6,
            TRANSFER_HOOK = 7
        }

        interface ICommonExtensionIxsProto {
            extension?: (vixen.parser.ExtensionWithCommonIxsProto|null);
            ix?: (vixen.parser.ICommonExtensionIxProto|null);
        }

        class CommonExtensionIxsProto implements ICommonExtensionIxsProto {
            constructor(properties?: vixen.parser.ICommonExtensionIxsProto);
            public extension: vixen.parser.ExtensionWithCommonIxsProto;
            public ix?: (vixen.parser.ICommonExtensionIxProto|null);
            public static create(properties?: vixen.parser.ICommonExtensionIxsProto): vixen.parser.CommonExtensionIxsProto;
            public static encode(message: vixen.parser.ICommonExtensionIxsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ICommonExtensionIxsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.CommonExtensionIxsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.CommonExtensionIxsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.CommonExtensionIxsProto;
            public static toObject(message: vixen.parser.CommonExtensionIxsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ICpiGuardIxProto {
            ix?: (vixen.parser.ICommonExtensionIxProto|null);
        }

        class CpiGuardIxProto implements ICpiGuardIxProto {
            constructor(properties?: vixen.parser.ICpiGuardIxProto);
            public ix?: (vixen.parser.ICommonExtensionIxProto|null);
            public static create(properties?: vixen.parser.ICpiGuardIxProto): vixen.parser.CpiGuardIxProto;
            public static encode(message: vixen.parser.ICpiGuardIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ICpiGuardIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.CpiGuardIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.CpiGuardIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.CpiGuardIxProto;
            public static toObject(message: vixen.parser.CpiGuardIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IDefaultAccountStateIxProto {
            ix?: (vixen.parser.ICommonExtensionIxProto|null);
        }

        class DefaultAccountStateIxProto implements IDefaultAccountStateIxProto {
            constructor(properties?: vixen.parser.IDefaultAccountStateIxProto);
            public ix?: (vixen.parser.ICommonExtensionIxProto|null);
            public static create(properties?: vixen.parser.IDefaultAccountStateIxProto): vixen.parser.DefaultAccountStateIxProto;
            public static encode(message: vixen.parser.IDefaultAccountStateIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IDefaultAccountStateIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.DefaultAccountStateIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.DefaultAccountStateIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.DefaultAccountStateIxProto;
            public static toObject(message: vixen.parser.DefaultAccountStateIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInterestBearingMintIxProto {
            ix?: (vixen.parser.ICommonExtensionIxProto|null);
        }

        class InterestBearingMintIxProto implements IInterestBearingMintIxProto {
            constructor(properties?: vixen.parser.IInterestBearingMintIxProto);
            public ix?: (vixen.parser.ICommonExtensionIxProto|null);
            public static create(properties?: vixen.parser.IInterestBearingMintIxProto): vixen.parser.InterestBearingMintIxProto;
            public static encode(message: vixen.parser.IInterestBearingMintIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInterestBearingMintIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InterestBearingMintIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InterestBearingMintIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InterestBearingMintIxProto;
            public static toObject(message: vixen.parser.InterestBearingMintIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IMemoTransferIxProto {
            ix?: (vixen.parser.ICommonExtensionIxProto|null);
        }

        class MemoTransferIxProto implements IMemoTransferIxProto {
            constructor(properties?: vixen.parser.IMemoTransferIxProto);
            public ix?: (vixen.parser.ICommonExtensionIxProto|null);
            public static create(properties?: vixen.parser.IMemoTransferIxProto): vixen.parser.MemoTransferIxProto;
            public static encode(message: vixen.parser.IMemoTransferIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IMemoTransferIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.MemoTransferIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.MemoTransferIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.MemoTransferIxProto;
            public static toObject(message: vixen.parser.MemoTransferIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IGroupMemberPointerIxProto {
            ix?: (vixen.parser.ICommonExtensionIxProto|null);
        }

        class GroupMemberPointerIxProto implements IGroupMemberPointerIxProto {
            constructor(properties?: vixen.parser.IGroupMemberPointerIxProto);
            public ix?: (vixen.parser.ICommonExtensionIxProto|null);
            public static create(properties?: vixen.parser.IGroupMemberPointerIxProto): vixen.parser.GroupMemberPointerIxProto;
            public static encode(message: vixen.parser.IGroupMemberPointerIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IGroupMemberPointerIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.GroupMemberPointerIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.GroupMemberPointerIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.GroupMemberPointerIxProto;
            public static toObject(message: vixen.parser.GroupMemberPointerIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IGroupPointerIxProto {
            ix?: (vixen.parser.ICommonExtensionIxProto|null);
        }

        class GroupPointerIxProto implements IGroupPointerIxProto {
            constructor(properties?: vixen.parser.IGroupPointerIxProto);
            public ix?: (vixen.parser.ICommonExtensionIxProto|null);
            public static create(properties?: vixen.parser.IGroupPointerIxProto): vixen.parser.GroupPointerIxProto;
            public static encode(message: vixen.parser.IGroupPointerIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IGroupPointerIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.GroupPointerIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.GroupPointerIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.GroupPointerIxProto;
            public static toObject(message: vixen.parser.GroupPointerIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IMetadataPointerIxProto {
            ix?: (vixen.parser.ICommonExtensionIxProto|null);
        }

        class MetadataPointerIxProto implements IMetadataPointerIxProto {
            constructor(properties?: vixen.parser.IMetadataPointerIxProto);
            public ix?: (vixen.parser.ICommonExtensionIxProto|null);
            public static create(properties?: vixen.parser.IMetadataPointerIxProto): vixen.parser.MetadataPointerIxProto;
            public static encode(message: vixen.parser.IMetadataPointerIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IMetadataPointerIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.MetadataPointerIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.MetadataPointerIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.MetadataPointerIxProto;
            public static toObject(message: vixen.parser.MetadataPointerIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ITransferHookIxProto {
            ix?: (vixen.parser.ICommonExtensionIxProto|null);
        }

        class TransferHookIxProto implements ITransferHookIxProto {
            constructor(properties?: vixen.parser.ITransferHookIxProto);
            public ix?: (vixen.parser.ICommonExtensionIxProto|null);
            public static create(properties?: vixen.parser.ITransferHookIxProto): vixen.parser.TransferHookIxProto;
            public static encode(message: vixen.parser.ITransferHookIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ITransferHookIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.TransferHookIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.TransferHookIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.TransferHookIxProto;
            public static toObject(message: vixen.parser.TransferHookIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IWithdrawExcessLamportsAccountsProto {
            sourceAccount?: (string|null);
            destinationAccount?: (string|null);
            authority?: (string|null);
            multisigSigners?: (string[]|null);
        }

        class WithdrawExcessLamportsAccountsProto implements IWithdrawExcessLamportsAccountsProto {
            constructor(properties?: vixen.parser.IWithdrawExcessLamportsAccountsProto);
            public sourceAccount: string;
            public destinationAccount: string;
            public authority: string;
            public multisigSigners: string[];
            public static create(properties?: vixen.parser.IWithdrawExcessLamportsAccountsProto): vixen.parser.WithdrawExcessLamportsAccountsProto;
            public static encode(message: vixen.parser.IWithdrawExcessLamportsAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IWithdrawExcessLamportsAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.WithdrawExcessLamportsAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.WithdrawExcessLamportsAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.WithdrawExcessLamportsAccountsProto;
            public static toObject(message: vixen.parser.WithdrawExcessLamportsAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IWithdrawExcessLamportsIxProto {
            accounts?: (vixen.parser.IWithdrawExcessLamportsAccountsProto|null);
        }

        class WithdrawExcessLamportsIxProto implements IWithdrawExcessLamportsIxProto {
            constructor(properties?: vixen.parser.IWithdrawExcessLamportsIxProto);
            public accounts?: (vixen.parser.IWithdrawExcessLamportsAccountsProto|null);
            public static create(properties?: vixen.parser.IWithdrawExcessLamportsIxProto): vixen.parser.WithdrawExcessLamportsIxProto;
            public static encode(message: vixen.parser.IWithdrawExcessLamportsIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IWithdrawExcessLamportsIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.WithdrawExcessLamportsIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.WithdrawExcessLamportsIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.WithdrawExcessLamportsIxProto;
            public static toObject(message: vixen.parser.WithdrawExcessLamportsIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializePermanentDelegateAccountsProto {
            account?: (string|null);
        }

        class InitializePermanentDelegateAccountsProto implements IInitializePermanentDelegateAccountsProto {
            constructor(properties?: vixen.parser.IInitializePermanentDelegateAccountsProto);
            public account: string;
            public static create(properties?: vixen.parser.IInitializePermanentDelegateAccountsProto): vixen.parser.InitializePermanentDelegateAccountsProto;
            public static encode(message: vixen.parser.IInitializePermanentDelegateAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializePermanentDelegateAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializePermanentDelegateAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializePermanentDelegateAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializePermanentDelegateAccountsProto;
            public static toObject(message: vixen.parser.InitializePermanentDelegateAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializePermanentDelegateDataProto {
            delegate?: (string|null);
        }

        class InitializePermanentDelegateDataProto implements IInitializePermanentDelegateDataProto {
            constructor(properties?: vixen.parser.IInitializePermanentDelegateDataProto);
            public delegate: string;
            public static create(properties?: vixen.parser.IInitializePermanentDelegateDataProto): vixen.parser.InitializePermanentDelegateDataProto;
            public static encode(message: vixen.parser.IInitializePermanentDelegateDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializePermanentDelegateDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializePermanentDelegateDataProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializePermanentDelegateDataProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializePermanentDelegateDataProto;
            public static toObject(message: vixen.parser.InitializePermanentDelegateDataProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializePermanentDelegateIxProto {
            accounts?: (vixen.parser.IInitializePermanentDelegateAccountsProto|null);
            data?: (vixen.parser.IInitializePermanentDelegateDataProto|null);
        }

        class InitializePermanentDelegateIxProto implements IInitializePermanentDelegateIxProto {
            constructor(properties?: vixen.parser.IInitializePermanentDelegateIxProto);
            public accounts?: (vixen.parser.IInitializePermanentDelegateAccountsProto|null);
            public data?: (vixen.parser.IInitializePermanentDelegateDataProto|null);
            public static create(properties?: vixen.parser.IInitializePermanentDelegateIxProto): vixen.parser.InitializePermanentDelegateIxProto;
            public static encode(message: vixen.parser.IInitializePermanentDelegateIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializePermanentDelegateIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializePermanentDelegateIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializePermanentDelegateIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializePermanentDelegateIxProto;
            public static toObject(message: vixen.parser.InitializePermanentDelegateIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IReallocateAccountsProto {
            account?: (string|null);
            payerAccount?: (string|null);
            owner?: (string|null);
            multisigSigners?: (string[]|null);
        }

        class ReallocateAccountsProto implements IReallocateAccountsProto {
            constructor(properties?: vixen.parser.IReallocateAccountsProto);
            public account: string;
            public payerAccount: string;
            public owner: string;
            public multisigSigners: string[];
            public static create(properties?: vixen.parser.IReallocateAccountsProto): vixen.parser.ReallocateAccountsProto;
            public static encode(message: vixen.parser.IReallocateAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IReallocateAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ReallocateAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ReallocateAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ReallocateAccountsProto;
            public static toObject(message: vixen.parser.ReallocateAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        enum ExtensionType {
            UNINITIALIZED_EXT = 0,
            TRANSFER_FEE_CONFIG_EXT = 1,
            TRANSFER_FEE_AMOUNT_EXT = 2,
            MINT_CLOSE_AUTHORITY_EXT = 3,
            CONFIDENTIAL_TRANSFER_MINT_EXT = 4,
            CONFIDENTIAL_TRANSFER_ACCOUNT_EXT = 5,
            DEFAULT_ACCOUNT_STATE_EXT = 6,
            IMMUTABLE_OWNER_EXT = 7,
            MEMO_TRANSFER_EXT = 8,
            NON_TRANSFERABLE_EXT = 9,
            INTEREST_BEARING_CONFIG_EXT = 10,
            CPI_GUARD_EXT = 11,
            PERMANENT_DELEGATE_EXT = 12,
            NON_TRANSFERABLE_ACCOUNT_EXT = 13,
            TRANSFER_HOOK_EXT = 14,
            TRANSFER_HOOK_ACCOUNT_EXT = 15,
            CONFIDENTIAL_TRANSFER_FEE_CONFIG_EXT = 16,
            CONFIDENTIAL_TRANSFER_FEE_AMOUNT_EXT = 17,
            METADATA_POINTER_EXT = 18,
            TOKEN_METADATA_EXT = 19,
            GROUP_POINTER_EXT = 20,
            TOKEN_GROUP_EXT = 21,
            GROUP_MEMBER_POINTER_EXT = 22,
            TOKEN_GROUP_MEMBER_EXT = 23
        }

        interface IReallocateDataProto {
            extensionsTypes?: (vixen.parser.ExtensionType[]|null);
        }

        class ReallocateDataProto implements IReallocateDataProto {
            constructor(properties?: vixen.parser.IReallocateDataProto);
            public extensionsTypes: vixen.parser.ExtensionType[];
            public static create(properties?: vixen.parser.IReallocateDataProto): vixen.parser.ReallocateDataProto;
            public static encode(message: vixen.parser.IReallocateDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IReallocateDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ReallocateDataProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ReallocateDataProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ReallocateDataProto;
            public static toObject(message: vixen.parser.ReallocateDataProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IReallocateIxProto {
            accounts?: (vixen.parser.IReallocateAccountsProto|null);
            data?: (vixen.parser.IReallocateDataProto|null);
        }

        class ReallocateIxProto implements IReallocateIxProto {
            constructor(properties?: vixen.parser.IReallocateIxProto);
            public accounts?: (vixen.parser.IReallocateAccountsProto|null);
            public data?: (vixen.parser.IReallocateDataProto|null);
            public static create(properties?: vixen.parser.IReallocateIxProto): vixen.parser.ReallocateIxProto;
            public static encode(message: vixen.parser.IReallocateIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IReallocateIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ReallocateIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ReallocateIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ReallocateIxProto;
            public static toObject(message: vixen.parser.ReallocateIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializeNonTransferableMintAccountsProto {
            mint?: (string|null);
        }

        class InitializeNonTransferableMintAccountsProto implements IInitializeNonTransferableMintAccountsProto {
            constructor(properties?: vixen.parser.IInitializeNonTransferableMintAccountsProto);
            public mint: string;
            public static create(properties?: vixen.parser.IInitializeNonTransferableMintAccountsProto): vixen.parser.InitializeNonTransferableMintAccountsProto;
            public static encode(message: vixen.parser.IInitializeNonTransferableMintAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializeNonTransferableMintAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializeNonTransferableMintAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializeNonTransferableMintAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializeNonTransferableMintAccountsProto;
            public static toObject(message: vixen.parser.InitializeNonTransferableMintAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializeNonTransferableMintIxProto {
            accounts?: (vixen.parser.IInitializeNonTransferableMintAccountsProto|null);
        }

        class InitializeNonTransferableMintIxProto implements IInitializeNonTransferableMintIxProto {
            constructor(properties?: vixen.parser.IInitializeNonTransferableMintIxProto);
            public accounts?: (vixen.parser.IInitializeNonTransferableMintAccountsProto|null);
            public static create(properties?: vixen.parser.IInitializeNonTransferableMintIxProto): vixen.parser.InitializeNonTransferableMintIxProto;
            public static encode(message: vixen.parser.IInitializeNonTransferableMintIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializeNonTransferableMintIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializeNonTransferableMintIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializeNonTransferableMintIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializeNonTransferableMintIxProto;
            public static toObject(message: vixen.parser.InitializeNonTransferableMintIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializeMintCloseAuthorityAccountsProto {
            mint?: (string|null);
        }

        class InitializeMintCloseAuthorityAccountsProto implements IInitializeMintCloseAuthorityAccountsProto {
            constructor(properties?: vixen.parser.IInitializeMintCloseAuthorityAccountsProto);
            public mint: string;
            public static create(properties?: vixen.parser.IInitializeMintCloseAuthorityAccountsProto): vixen.parser.InitializeMintCloseAuthorityAccountsProto;
            public static encode(message: vixen.parser.IInitializeMintCloseAuthorityAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializeMintCloseAuthorityAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializeMintCloseAuthorityAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializeMintCloseAuthorityAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializeMintCloseAuthorityAccountsProto;
            public static toObject(message: vixen.parser.InitializeMintCloseAuthorityAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializeMintCloseAuthorityDataProto {
            closeAuthority?: (string|null);
        }

        class InitializeMintCloseAuthorityDataProto implements IInitializeMintCloseAuthorityDataProto {
            constructor(properties?: vixen.parser.IInitializeMintCloseAuthorityDataProto);
            public closeAuthority?: (string|null);
            public static create(properties?: vixen.parser.IInitializeMintCloseAuthorityDataProto): vixen.parser.InitializeMintCloseAuthorityDataProto;
            public static encode(message: vixen.parser.IInitializeMintCloseAuthorityDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializeMintCloseAuthorityDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializeMintCloseAuthorityDataProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializeMintCloseAuthorityDataProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializeMintCloseAuthorityDataProto;
            public static toObject(message: vixen.parser.InitializeMintCloseAuthorityDataProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IInitializeMintCloseAuthorityIxProto {
            accounts?: (vixen.parser.IInitializeMintCloseAuthorityAccountsProto|null);
            data?: (vixen.parser.IInitializeMintCloseAuthorityDataProto|null);
        }

        class InitializeMintCloseAuthorityIxProto implements IInitializeMintCloseAuthorityIxProto {
            constructor(properties?: vixen.parser.IInitializeMintCloseAuthorityIxProto);
            public accounts?: (vixen.parser.IInitializeMintCloseAuthorityAccountsProto|null);
            public data?: (vixen.parser.IInitializeMintCloseAuthorityDataProto|null);
            public static create(properties?: vixen.parser.IInitializeMintCloseAuthorityIxProto): vixen.parser.InitializeMintCloseAuthorityIxProto;
            public static encode(message: vixen.parser.IInitializeMintCloseAuthorityIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IInitializeMintCloseAuthorityIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.InitializeMintCloseAuthorityIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.InitializeMintCloseAuthorityIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.InitializeMintCloseAuthorityIxProto;
            public static toObject(message: vixen.parser.InitializeMintCloseAuthorityIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ICreateNativeMintAccountsProto {
            mint?: (string|null);
            fundingAccount?: (string|null);
        }

        class CreateNativeMintAccountsProto implements ICreateNativeMintAccountsProto {
            constructor(properties?: vixen.parser.ICreateNativeMintAccountsProto);
            public mint: string;
            public fundingAccount: string;
            public static create(properties?: vixen.parser.ICreateNativeMintAccountsProto): vixen.parser.CreateNativeMintAccountsProto;
            public static encode(message: vixen.parser.ICreateNativeMintAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ICreateNativeMintAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.CreateNativeMintAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.CreateNativeMintAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.CreateNativeMintAccountsProto;
            public static toObject(message: vixen.parser.CreateNativeMintAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ICreateNativeMintIxProto {
            accounts?: (vixen.parser.ICreateNativeMintAccountsProto|null);
        }

        class CreateNativeMintIxProto implements ICreateNativeMintIxProto {
            constructor(properties?: vixen.parser.ICreateNativeMintIxProto);
            public accounts?: (vixen.parser.ICreateNativeMintAccountsProto|null);
            public static create(properties?: vixen.parser.ICreateNativeMintIxProto): vixen.parser.CreateNativeMintIxProto;
            public static encode(message: vixen.parser.ICreateNativeMintIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ICreateNativeMintIxProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.CreateNativeMintIxProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.CreateNativeMintIxProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.CreateNativeMintIxProto;
            public static toObject(message: vixen.parser.CreateNativeMintIxProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IWhirlpoolRewardInfoProto {
            mint?: (string|null);
            vault?: (string|null);
            authority?: (string|null);
            emissionsPerSecondX64?: (string|null);
            growthGlobalX64?: (string|null);
        }

        class WhirlpoolRewardInfoProto implements IWhirlpoolRewardInfoProto {
            constructor(properties?: vixen.parser.IWhirlpoolRewardInfoProto);
            public mint: string;
            public vault: string;
            public authority: string;
            public emissionsPerSecondX64: string;
            public growthGlobalX64: string;
            public static create(properties?: vixen.parser.IWhirlpoolRewardInfoProto): vixen.parser.WhirlpoolRewardInfoProto;
            public static encode(message: vixen.parser.IWhirlpoolRewardInfoProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IWhirlpoolRewardInfoProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.WhirlpoolRewardInfoProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.WhirlpoolRewardInfoProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.WhirlpoolRewardInfoProto;
            public static toObject(message: vixen.parser.WhirlpoolRewardInfoProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IWhirlpoolProto {
            discriminator?: (Uint8Array|null);
            whirlpoolsConfig?: (string|null);
            whirlpoolBump?: (number|null);
            tickSpacing?: (number|null);
            tickSpacingSeed?: (Uint8Array|null);
            feeRate?: (number|null);
            protocolFeeRate?: (number|null);
            liquidity?: (string|null);
            sqrtPrice?: (string|null);
            tickCurrentIndex?: (number|null);
            protocolFeeOwedA?: (number|Long|null);
            protocolFeeOwedB?: (number|Long|null);
            tokenMintA?: (string|null);
            tokenVaultA?: (string|null);
            feeGrowthGlobalA?: (string|null);
            tokenMintB?: (string|null);
            tokenVaultB?: (string|null);
            feeGrowthGlobalB?: (string|null);
            rewardLastUpdatedTimestamp?: (number|Long|null);
            rewardInfos?: (vixen.parser.IWhirlpoolRewardInfoProto[]|null);
        }

        class WhirlpoolProto implements IWhirlpoolProto {
            constructor(properties?: vixen.parser.IWhirlpoolProto);
            public discriminator: Uint8Array;
            public whirlpoolsConfig: string;
            public whirlpoolBump: number;
            public tickSpacing: number;
            public tickSpacingSeed: Uint8Array;
            public feeRate: number;
            public protocolFeeRate: number;
            public liquidity: string;
            public sqrtPrice: string;
            public tickCurrentIndex: number;
            public protocolFeeOwedA: (number|Long);
            public protocolFeeOwedB: (number|Long);
            public tokenMintA: string;
            public tokenVaultA: string;
            public feeGrowthGlobalA: string;
            public tokenMintB: string;
            public tokenVaultB: string;
            public feeGrowthGlobalB: string;
            public rewardLastUpdatedTimestamp: (number|Long);
            public rewardInfos: vixen.parser.IWhirlpoolRewardInfoProto[];
            public static create(properties?: vixen.parser.IWhirlpoolProto): vixen.parser.WhirlpoolProto;
            public static encode(message: vixen.parser.IWhirlpoolProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IWhirlpoolProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.WhirlpoolProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.WhirlpoolProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.WhirlpoolProto;
            public static toObject(message: vixen.parser.WhirlpoolProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IWhirlpoolsConfigProto {
            discriminator?: (Uint8Array|null);
            feeAuthority?: (string|null);
            collectProtocolFeesAuthority?: (string|null);
            rewardEmissionsSuperAuthority?: (string|null);
            defaultProtocolFeeRate?: (number|null);
        }

        class WhirlpoolsConfigProto implements IWhirlpoolsConfigProto {
            constructor(properties?: vixen.parser.IWhirlpoolsConfigProto);
            public discriminator: Uint8Array;
            public feeAuthority: string;
            public collectProtocolFeesAuthority: string;
            public rewardEmissionsSuperAuthority: string;
            public defaultProtocolFeeRate: number;
            public static create(properties?: vixen.parser.IWhirlpoolsConfigProto): vixen.parser.WhirlpoolsConfigProto;
            public static encode(message: vixen.parser.IWhirlpoolsConfigProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IWhirlpoolsConfigProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.WhirlpoolsConfigProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.WhirlpoolsConfigProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.WhirlpoolsConfigProto;
            public static toObject(message: vixen.parser.WhirlpoolsConfigProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IFeeTierProto {
            discriminator?: (Uint8Array|null);
            whirlpoolsConfig?: (string|null);
            tickSpacing?: (number|null);
            defaultFeeRate?: (number|null);
        }

        class FeeTierProto implements IFeeTierProto {
            constructor(properties?: vixen.parser.IFeeTierProto);
            public discriminator: Uint8Array;
            public whirlpoolsConfig: string;
            public tickSpacing: number;
            public defaultFeeRate: number;
            public static create(properties?: vixen.parser.IFeeTierProto): vixen.parser.FeeTierProto;
            public static encode(message: vixen.parser.IFeeTierProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IFeeTierProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.FeeTierProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.FeeTierProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.FeeTierProto;
            public static toObject(message: vixen.parser.FeeTierProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IPositionProto {
            discriminator?: (Uint8Array|null);
            whirlpool?: (string|null);
            positionMint?: (string|null);
            liquidity?: (string|null);
            tickLowerIndex?: (number|null);
            tickUpperIndex?: (number|null);
            feeGrowthCheckpointA?: (string|null);
            feeOwedA?: (number|Long|null);
            feeGrowthCheckpointB?: (string|null);
            feeOwedB?: (number|Long|null);
            rewardInfos?: (vixen.parser.IOrcaPositionRewardInfoProto[]|null);
        }

        class PositionProto implements IPositionProto {
            constructor(properties?: vixen.parser.IPositionProto);
            public discriminator: Uint8Array;
            public whirlpool: string;
            public positionMint: string;
            public liquidity: string;
            public tickLowerIndex: number;
            public tickUpperIndex: number;
            public feeGrowthCheckpointA: string;
            public feeOwedA: (number|Long);
            public feeGrowthCheckpointB: string;
            public feeOwedB: (number|Long);
            public rewardInfos: vixen.parser.IOrcaPositionRewardInfoProto[];
            public static create(properties?: vixen.parser.IPositionProto): vixen.parser.PositionProto;
            public static encode(message: vixen.parser.IPositionProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IPositionProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.PositionProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.PositionProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.PositionProto;
            public static toObject(message: vixen.parser.PositionProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IOrcaPositionRewardInfoProto {
            growthInsideCheckpoint?: (string|null);
            amountOwed?: (number|Long|null);
        }

        class OrcaPositionRewardInfoProto implements IOrcaPositionRewardInfoProto {
            constructor(properties?: vixen.parser.IOrcaPositionRewardInfoProto);
            public growthInsideCheckpoint: string;
            public amountOwed: (number|Long);
            public static create(properties?: vixen.parser.IOrcaPositionRewardInfoProto): vixen.parser.OrcaPositionRewardInfoProto;
            public static encode(message: vixen.parser.IOrcaPositionRewardInfoProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IOrcaPositionRewardInfoProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.OrcaPositionRewardInfoProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.OrcaPositionRewardInfoProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.OrcaPositionRewardInfoProto;
            public static toObject(message: vixen.parser.OrcaPositionRewardInfoProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IOrcaTickProto {
            initialized?: (boolean|null);
            liquidityNet?: (string|null);
            liquidityGross?: (string|null);
            feeGrowthOutsideA?: (string|null);
            feeGrowthOutsideB?: (string|null);
            rewardGrowthsOutside?: (string[]|null);
        }

        class OrcaTickProto implements IOrcaTickProto {
            constructor(properties?: vixen.parser.IOrcaTickProto);
            public initialized: boolean;
            public liquidityNet: string;
            public liquidityGross: string;
            public feeGrowthOutsideA: string;
            public feeGrowthOutsideB: string;
            public rewardGrowthsOutside: string[];
            public static create(properties?: vixen.parser.IOrcaTickProto): vixen.parser.OrcaTickProto;
            public static encode(message: vixen.parser.IOrcaTickProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IOrcaTickProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.OrcaTickProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.OrcaTickProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.OrcaTickProto;
            public static toObject(message: vixen.parser.OrcaTickProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IOrcaTickArrayProto {
            discriminator?: (Uint8Array|null);
            startTickIndex?: (number|null);
            ticks?: (vixen.parser.IOrcaTickProto[]|null);
            whirlpool?: (string|null);
        }

        class OrcaTickArrayProto implements IOrcaTickArrayProto {
            constructor(properties?: vixen.parser.IOrcaTickArrayProto);
            public discriminator: Uint8Array;
            public startTickIndex: number;
            public ticks: vixen.parser.IOrcaTickProto[];
            public whirlpool: string;
            public static create(properties?: vixen.parser.IOrcaTickArrayProto): vixen.parser.OrcaTickArrayProto;
            public static encode(message: vixen.parser.IOrcaTickArrayProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IOrcaTickArrayProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.OrcaTickArrayProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.OrcaTickArrayProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.OrcaTickArrayProto;
            public static toObject(message: vixen.parser.OrcaTickArrayProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IOrcaSwapAccountsProto {
            tokenProgram?: (string|null);
            tokenAuthority?: (string|null);
            whirlpool?: (string|null);
            tokenOwnerAccountA?: (string|null);
            tokenVaultA?: (string|null);
            tokenOwnerAccountB?: (string|null);
            tokenVaultB?: (string|null);
            tickArray0?: (string|null);
            tickArray1?: (string|null);
            tickArray2?: (string|null);
            oracle?: (string|null);
        }

        class OrcaSwapAccountsProto implements IOrcaSwapAccountsProto {
            constructor(properties?: vixen.parser.IOrcaSwapAccountsProto);
            public tokenProgram: string;
            public tokenAuthority: string;
            public whirlpool: string;
            public tokenOwnerAccountA: string;
            public tokenVaultA: string;
            public tokenOwnerAccountB: string;
            public tokenVaultB: string;
            public tickArray0: string;
            public tickArray1: string;
            public tickArray2: string;
            public oracle: string;
            public static create(properties?: vixen.parser.IOrcaSwapAccountsProto): vixen.parser.OrcaSwapAccountsProto;
            public static encode(message: vixen.parser.IOrcaSwapAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IOrcaSwapAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.OrcaSwapAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.OrcaSwapAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.OrcaSwapAccountsProto;
            public static toObject(message: vixen.parser.OrcaSwapAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IOrcaSwapIxDataProto {
            amount?: (number|Long|null);
            otherAmountThreshold?: (number|Long|null);
            sqrtPriceLimit?: (string|null);
            amountSpecifiedIsInput?: (boolean|null);
            aToB?: (boolean|null);
        }

        class OrcaSwapIxDataProto implements IOrcaSwapIxDataProto {
            constructor(properties?: vixen.parser.IOrcaSwapIxDataProto);
            public amount: (number|Long);
            public otherAmountThreshold: (number|Long);
            public sqrtPriceLimit: string;
            public amountSpecifiedIsInput: boolean;
            public aToB: boolean;
            public static create(properties?: vixen.parser.IOrcaSwapIxDataProto): vixen.parser.OrcaSwapIxDataProto;
            public static encode(message: vixen.parser.IOrcaSwapIxDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IOrcaSwapIxDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.OrcaSwapIxDataProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.OrcaSwapIxDataProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.OrcaSwapIxDataProto;
            public static toObject(message: vixen.parser.OrcaSwapIxDataProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IOrcaSwapV2AccountsProto {
            tokenProgramA?: (string|null);
            tokenProgramB?: (string|null);
            memoProgram?: (string|null);
            tokenAuthority?: (string|null);
            whirlpool?: (string|null);
            tokenMintA?: (string|null);
            tokenMintB?: (string|null);
            tokenOwnerAccountA?: (string|null);
            tokenVaultA?: (string|null);
            tokenOwnerAccountB?: (string|null);
            tokenVaultB?: (string|null);
            tickArray0?: (string|null);
            tickArray1?: (string|null);
            tickArray2?: (string|null);
            oracle?: (string|null);
        }

        class OrcaSwapV2AccountsProto implements IOrcaSwapV2AccountsProto {
            constructor(properties?: vixen.parser.IOrcaSwapV2AccountsProto);
            public tokenProgramA: string;
            public tokenProgramB: string;
            public memoProgram: string;
            public tokenAuthority: string;
            public whirlpool: string;
            public tokenMintA: string;
            public tokenMintB: string;
            public tokenOwnerAccountA: string;
            public tokenVaultA: string;
            public tokenOwnerAccountB: string;
            public tokenVaultB: string;
            public tickArray0: string;
            public tickArray1: string;
            public tickArray2: string;
            public oracle: string;
            public static create(properties?: vixen.parser.IOrcaSwapV2AccountsProto): vixen.parser.OrcaSwapV2AccountsProto;
            public static encode(message: vixen.parser.IOrcaSwapV2AccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IOrcaSwapV2AccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.OrcaSwapV2AccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.OrcaSwapV2AccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.OrcaSwapV2AccountsProto;
            public static toObject(message: vixen.parser.OrcaSwapV2AccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IOrcaSwapV2IxDataProto {
            amount?: (number|Long|null);
            otherAmountThreshold?: (number|Long|null);
            sqrtPriceLimit?: (string|null);
            amountSpecifiedIsInput?: (boolean|null);
            aToB?: (boolean|null);
        }

        class OrcaSwapV2IxDataProto implements IOrcaSwapV2IxDataProto {
            constructor(properties?: vixen.parser.IOrcaSwapV2IxDataProto);
            public amount: (number|Long);
            public otherAmountThreshold: (number|Long);
            public sqrtPriceLimit: string;
            public amountSpecifiedIsInput: boolean;
            public aToB: boolean;
            public static create(properties?: vixen.parser.IOrcaSwapV2IxDataProto): vixen.parser.OrcaSwapV2IxDataProto;
            public static encode(message: vixen.parser.IOrcaSwapV2IxDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IOrcaSwapV2IxDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.OrcaSwapV2IxDataProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.OrcaSwapV2IxDataProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.OrcaSwapV2IxDataProto;
            public static toObject(message: vixen.parser.OrcaSwapV2IxDataProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IOrcaSwapInstructionProto {
            accounts?: (vixen.parser.IOrcaSwapAccountsProto|null);
            data?: (vixen.parser.IOrcaSwapIxDataProto|null);
        }

        class OrcaSwapInstructionProto implements IOrcaSwapInstructionProto {
            constructor(properties?: vixen.parser.IOrcaSwapInstructionProto);
            public accounts?: (vixen.parser.IOrcaSwapAccountsProto|null);
            public data?: (vixen.parser.IOrcaSwapIxDataProto|null);
            public static create(properties?: vixen.parser.IOrcaSwapInstructionProto): vixen.parser.OrcaSwapInstructionProto;
            public static encode(message: vixen.parser.IOrcaSwapInstructionProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IOrcaSwapInstructionProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.OrcaSwapInstructionProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.OrcaSwapInstructionProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.OrcaSwapInstructionProto;
            public static toObject(message: vixen.parser.OrcaSwapInstructionProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IOrcaSwapV2InstructionProto {
            accounts?: (vixen.parser.IOrcaSwapV2AccountsProto|null);
            data?: (vixen.parser.IOrcaSwapV2IxDataProto|null);
        }

        class OrcaSwapV2InstructionProto implements IOrcaSwapV2InstructionProto {
            constructor(properties?: vixen.parser.IOrcaSwapV2InstructionProto);
            public accounts?: (vixen.parser.IOrcaSwapV2AccountsProto|null);
            public data?: (vixen.parser.IOrcaSwapV2IxDataProto|null);
            public static create(properties?: vixen.parser.IOrcaSwapV2InstructionProto): vixen.parser.OrcaSwapV2InstructionProto;
            public static encode(message: vixen.parser.IOrcaSwapV2InstructionProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IOrcaSwapV2InstructionProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.OrcaSwapV2InstructionProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.OrcaSwapV2InstructionProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.OrcaSwapV2InstructionProto;
            public static toObject(message: vixen.parser.OrcaSwapV2InstructionProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IAmmConfigProto {
            bump?: (number|null);
            index?: (number|null);
            owner?: (string|null);
            protocolFeeRate?: (number|null);
            tradeFeeRate?: (number|null);
            tickSpacing?: (number|null);
            fundFeeRate?: (number|null);
            paddingU32?: (number|null);
            fundOwner?: (string|null);
            padding?: ((number|Long)[]|null);
        }

        class AmmConfigProto implements IAmmConfigProto {
            constructor(properties?: vixen.parser.IAmmConfigProto);
            public bump: number;
            public index: number;
            public owner: string;
            public protocolFeeRate: number;
            public tradeFeeRate: number;
            public tickSpacing: number;
            public fundFeeRate: number;
            public paddingU32: number;
            public fundOwner: string;
            public padding: (number|Long)[];
            public static create(properties?: vixen.parser.IAmmConfigProto): vixen.parser.AmmConfigProto;
            public static encode(message: vixen.parser.IAmmConfigProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IAmmConfigProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.AmmConfigProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.AmmConfigProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.AmmConfigProto;
            public static toObject(message: vixen.parser.AmmConfigProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IOperationStateProto {
            bump?: (number|null);
            operationOwners?: (string[]|null);
            whitelistMints?: (string[]|null);
        }

        class OperationStateProto implements IOperationStateProto {
            constructor(properties?: vixen.parser.IOperationStateProto);
            public bump: number;
            public operationOwners: string[];
            public whitelistMints: string[];
            public static create(properties?: vixen.parser.IOperationStateProto): vixen.parser.OperationStateProto;
            public static encode(message: vixen.parser.IOperationStateProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IOperationStateProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.OperationStateProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.OperationStateProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.OperationStateProto;
            public static toObject(message: vixen.parser.OperationStateProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IObservationProto {
            blockTimestamp?: (number|null);
            tickCumulative?: (number|Long|null);
            padding?: ((number|Long)[]|null);
        }

        class ObservationProto implements IObservationProto {
            constructor(properties?: vixen.parser.IObservationProto);
            public blockTimestamp: number;
            public tickCumulative: (number|Long);
            public padding: (number|Long)[];
            public static create(properties?: vixen.parser.IObservationProto): vixen.parser.ObservationProto;
            public static encode(message: vixen.parser.IObservationProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IObservationProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ObservationProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ObservationProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ObservationProto;
            public static toObject(message: vixen.parser.ObservationProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IObservationStateProto {
            initialized?: (boolean|null);
            recentEpoch?: (number|Long|null);
            observationIndex?: (number|null);
            poolId?: (string|null);
            observations?: (vixen.parser.IObservationProto[]|null);
            padding?: ((number|Long)[]|null);
        }

        class ObservationStateProto implements IObservationStateProto {
            constructor(properties?: vixen.parser.IObservationStateProto);
            public initialized: boolean;
            public recentEpoch: (number|Long);
            public observationIndex: number;
            public poolId: string;
            public observations: vixen.parser.IObservationProto[];
            public padding: (number|Long)[];
            public static create(properties?: vixen.parser.IObservationStateProto): vixen.parser.ObservationStateProto;
            public static encode(message: vixen.parser.IObservationStateProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IObservationStateProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ObservationStateProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ObservationStateProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ObservationStateProto;
            public static toObject(message: vixen.parser.ObservationStateProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IRaydiumPositionRewardInfoProto {
            growthInsideLastX64?: (string|null);
            rewardAmountOwed?: (number|Long|null);
        }

        class RaydiumPositionRewardInfoProto implements IRaydiumPositionRewardInfoProto {
            constructor(properties?: vixen.parser.IRaydiumPositionRewardInfoProto);
            public growthInsideLastX64: string;
            public rewardAmountOwed: (number|Long);
            public static create(properties?: vixen.parser.IRaydiumPositionRewardInfoProto): vixen.parser.RaydiumPositionRewardInfoProto;
            public static encode(message: vixen.parser.IRaydiumPositionRewardInfoProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IRaydiumPositionRewardInfoProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.RaydiumPositionRewardInfoProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.RaydiumPositionRewardInfoProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.RaydiumPositionRewardInfoProto;
            public static toObject(message: vixen.parser.RaydiumPositionRewardInfoProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IPersonalPositionStateProto {
            bump?: (number|null);
            nftMint?: (string|null);
            poolId?: (string|null);
            tickLowerIndex?: (number|null);
            tickUpperIndex?: (number|null);
            liquidity?: (string|null);
            feeGrowthInside_0LastX64?: (string|null);
            feeGrowthInside_1LastX64?: (string|null);
            tokenFeesOwed_0?: (number|Long|null);
            tokenFeesOwed_1?: (number|Long|null);
            rewardInfos?: (vixen.parser.IRaydiumPositionRewardInfoProto[]|null);
            recentEpoch?: (number|Long|null);
            padding?: ((number|Long)[]|null);
        }

        class PersonalPositionStateProto implements IPersonalPositionStateProto {
            constructor(properties?: vixen.parser.IPersonalPositionStateProto);
            public bump: number;
            public nftMint: string;
            public poolId: string;
            public tickLowerIndex: number;
            public tickUpperIndex: number;
            public liquidity: string;
            public feeGrowthInside_0LastX64: string;
            public feeGrowthInside_1LastX64: string;
            public tokenFeesOwed_0: (number|Long);
            public tokenFeesOwed_1: (number|Long);
            public rewardInfos: vixen.parser.IRaydiumPositionRewardInfoProto[];
            public recentEpoch: (number|Long);
            public padding: (number|Long)[];
            public static create(properties?: vixen.parser.IPersonalPositionStateProto): vixen.parser.PersonalPositionStateProto;
            public static encode(message: vixen.parser.IPersonalPositionStateProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IPersonalPositionStateProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.PersonalPositionStateProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.PersonalPositionStateProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.PersonalPositionStateProto;
            public static toObject(message: vixen.parser.PersonalPositionStateProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IRewardInfoProto {
            rewardState?: (number|null);
            openTime?: (number|Long|null);
            endTime?: (number|Long|null);
            lastUpdateTime?: (number|Long|null);
            emissionsPerSecondX64?: (string|null);
            rewardTotalEmissioned?: (number|Long|null);
            rewardClaimed?: (number|Long|null);
            tokenMint?: (string|null);
            tokenVault?: (string|null);
            authority?: (string|null);
            rewardGrowthGlobalX64?: (string|null);
        }

        class RewardInfoProto implements IRewardInfoProto {
            constructor(properties?: vixen.parser.IRewardInfoProto);
            public rewardState: number;
            public openTime: (number|Long);
            public endTime: (number|Long);
            public lastUpdateTime: (number|Long);
            public emissionsPerSecondX64: string;
            public rewardTotalEmissioned: (number|Long);
            public rewardClaimed: (number|Long);
            public tokenMint: string;
            public tokenVault: string;
            public authority: string;
            public rewardGrowthGlobalX64: string;
            public static create(properties?: vixen.parser.IRewardInfoProto): vixen.parser.RewardInfoProto;
            public static encode(message: vixen.parser.IRewardInfoProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IRewardInfoProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.RewardInfoProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.RewardInfoProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.RewardInfoProto;
            public static toObject(message: vixen.parser.RewardInfoProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IPoolStateProto {
            bump?: (number|null);
            ammConfig?: (string|null);
            owner?: (string|null);
            tokenMint_0?: (string|null);
            tokenMint_1?: (string|null);
            tokenVault_0?: (string|null);
            tokenVault_1?: (string|null);
            observationKey?: (string|null);
            mintDecimals_0?: (number|null);
            mintDecimals_1?: (number|null);
            tickSpacing?: (number|null);
            liquidity?: (string|null);
            sqrtPriceX64?: (string|null);
            tickCurrent?: (number|null);
            padding3?: (number|null);
            padding4?: (number|null);
            feeGrowthGlobal_0X64?: (string|null);
            feeGrowthGlobal_1X64?: (string|null);
            protocolFeesToken_0?: (number|Long|null);
            protocolFeesToken_1?: (number|Long|null);
            swapInAmountToken_0?: (string|null);
            swapOutAmountToken_1?: (string|null);
            swapInAmountToken_1?: (string|null);
            swapOutAmountToken_0?: (string|null);
            status?: (number|null);
            padding?: (Uint8Array|null);
            rewardInfos?: (vixen.parser.IRewardInfoProto[]|null);
            tickArrayBitmap?: ((number|Long)[]|null);
            totalFeesToken_0?: (number|Long|null);
            totalFeesClaimedToken_0?: (number|Long|null);
            totalFeesToken_1?: (number|Long|null);
            totalFeesClaimedToken_1?: (number|Long|null);
            fundFeesToken_0?: (number|Long|null);
            fundFeesToken_1?: (number|Long|null);
            openTime?: (number|Long|null);
            recentEpoch?: (number|Long|null);
            padding1?: ((number|Long)[]|null);
            padding2?: ((number|Long)[]|null);
        }

        class PoolStateProto implements IPoolStateProto {
            constructor(properties?: vixen.parser.IPoolStateProto);
            public bump: number;
            public ammConfig: string;
            public owner: string;
            public tokenMint_0: string;
            public tokenMint_1: string;
            public tokenVault_0: string;
            public tokenVault_1: string;
            public observationKey: string;
            public mintDecimals_0: number;
            public mintDecimals_1: number;
            public tickSpacing: number;
            public liquidity: string;
            public sqrtPriceX64: string;
            public tickCurrent: number;
            public padding3: number;
            public padding4: number;
            public feeGrowthGlobal_0X64: string;
            public feeGrowthGlobal_1X64: string;
            public protocolFeesToken_0: (number|Long);
            public protocolFeesToken_1: (number|Long);
            public swapInAmountToken_0: string;
            public swapOutAmountToken_1: string;
            public swapInAmountToken_1: string;
            public swapOutAmountToken_0: string;
            public status: number;
            public padding: Uint8Array;
            public rewardInfos: vixen.parser.IRewardInfoProto[];
            public tickArrayBitmap: (number|Long)[];
            public totalFeesToken_0: (number|Long);
            public totalFeesClaimedToken_0: (number|Long);
            public totalFeesToken_1: (number|Long);
            public totalFeesClaimedToken_1: (number|Long);
            public fundFeesToken_0: (number|Long);
            public fundFeesToken_1: (number|Long);
            public openTime: (number|Long);
            public recentEpoch: (number|Long);
            public padding1: (number|Long)[];
            public padding2: (number|Long)[];
            public static create(properties?: vixen.parser.IPoolStateProto): vixen.parser.PoolStateProto;
            public static encode(message: vixen.parser.IPoolStateProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IPoolStateProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.PoolStateProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.PoolStateProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.PoolStateProto;
            public static toObject(message: vixen.parser.PoolStateProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IProtocolPositionStateProto {
            bump?: (number|null);
            poolId?: (string|null);
            tickLowerIndex?: (number|null);
            tickUpperIndex?: (number|null);
            liquidity?: (string|null);
            feeGrowthInside_0LastX64?: (string|null);
            feeGrowthInside_1LastX64?: (string|null);
            tokenFeesOwed_0?: (number|Long|null);
            tokenFeesOwed_1?: (number|Long|null);
            rewardGrowthInside?: (string[]|null);
            recentEpoch?: (number|Long|null);
            padding?: ((number|Long)[]|null);
        }

        class ProtocolPositionStateProto implements IProtocolPositionStateProto {
            constructor(properties?: vixen.parser.IProtocolPositionStateProto);
            public bump: number;
            public poolId: string;
            public tickLowerIndex: number;
            public tickUpperIndex: number;
            public liquidity: string;
            public feeGrowthInside_0LastX64: string;
            public feeGrowthInside_1LastX64: string;
            public tokenFeesOwed_0: (number|Long);
            public tokenFeesOwed_1: (number|Long);
            public rewardGrowthInside: string[];
            public recentEpoch: (number|Long);
            public padding: (number|Long)[];
            public static create(properties?: vixen.parser.IProtocolPositionStateProto): vixen.parser.ProtocolPositionStateProto;
            public static encode(message: vixen.parser.IProtocolPositionStateProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IProtocolPositionStateProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.ProtocolPositionStateProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.ProtocolPositionStateProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.ProtocolPositionStateProto;
            public static toObject(message: vixen.parser.ProtocolPositionStateProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IRaydiumTickStateProto {
            tick?: (number|null);
            liquidityNet?: (string|null);
            liquidityGross?: (string|null);
            feeGrowthOutside_0X64?: (string|null);
            feeGrowthOutside_1X64?: (string|null);
            rewardGrowthsOutsideX64?: (string[]|null);
            padding?: (number[]|null);
        }

        class RaydiumTickStateProto implements IRaydiumTickStateProto {
            constructor(properties?: vixen.parser.IRaydiumTickStateProto);
            public tick: number;
            public liquidityNet: string;
            public liquidityGross: string;
            public feeGrowthOutside_0X64: string;
            public feeGrowthOutside_1X64: string;
            public rewardGrowthsOutsideX64: string[];
            public padding: number[];
            public static create(properties?: vixen.parser.IRaydiumTickStateProto): vixen.parser.RaydiumTickStateProto;
            public static encode(message: vixen.parser.IRaydiumTickStateProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IRaydiumTickStateProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.RaydiumTickStateProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.RaydiumTickStateProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.RaydiumTickStateProto;
            public static toObject(message: vixen.parser.RaydiumTickStateProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IRaydiumTickArrayStateProto {
            poolId?: (string|null);
            startTickIndex?: (number|null);
            ticks?: (vixen.parser.IRaydiumTickStateProto[]|null);
            initializedTickCount?: (number|null);
            recentEpoch?: (number|Long|null);
            padding?: (Uint8Array|null);
        }

        class RaydiumTickArrayStateProto implements IRaydiumTickArrayStateProto {
            constructor(properties?: vixen.parser.IRaydiumTickArrayStateProto);
            public poolId: string;
            public startTickIndex: number;
            public ticks: vixen.parser.IRaydiumTickStateProto[];
            public initializedTickCount: number;
            public recentEpoch: (number|Long);
            public padding: Uint8Array;
            public static create(properties?: vixen.parser.IRaydiumTickArrayStateProto): vixen.parser.RaydiumTickArrayStateProto;
            public static encode(message: vixen.parser.IRaydiumTickArrayStateProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IRaydiumTickArrayStateProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.RaydiumTickArrayStateProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.RaydiumTickArrayStateProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.RaydiumTickArrayStateProto;
            public static toObject(message: vixen.parser.RaydiumTickArrayStateProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ITickArrayBitmapProto {
            data?: ((number|Long)[]|null);
        }

        class TickArrayBitmapProto implements ITickArrayBitmapProto {
            constructor(properties?: vixen.parser.ITickArrayBitmapProto);
            public data: (number|Long)[];
            public static create(properties?: vixen.parser.ITickArrayBitmapProto): vixen.parser.TickArrayBitmapProto;
            public static encode(message: vixen.parser.ITickArrayBitmapProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ITickArrayBitmapProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.TickArrayBitmapProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.TickArrayBitmapProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.TickArrayBitmapProto;
            public static toObject(message: vixen.parser.TickArrayBitmapProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface ITickArrayBitmapExtensionProto {
            poolId?: (string|null);
            positiveTickArrayBitmap?: (vixen.parser.ITickArrayBitmapProto[]|null);
            negativeTickArrayBitmap?: (vixen.parser.ITickArrayBitmapProto[]|null);
        }

        class TickArrayBitmapExtensionProto implements ITickArrayBitmapExtensionProto {
            constructor(properties?: vixen.parser.ITickArrayBitmapExtensionProto);
            public poolId: string;
            public positiveTickArrayBitmap: vixen.parser.ITickArrayBitmapProto[];
            public negativeTickArrayBitmap: vixen.parser.ITickArrayBitmapProto[];
            public static create(properties?: vixen.parser.ITickArrayBitmapExtensionProto): vixen.parser.TickArrayBitmapExtensionProto;
            public static encode(message: vixen.parser.ITickArrayBitmapExtensionProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.ITickArrayBitmapExtensionProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.TickArrayBitmapExtensionProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.TickArrayBitmapExtensionProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.TickArrayBitmapExtensionProto;
            public static toObject(message: vixen.parser.TickArrayBitmapExtensionProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IRaydiumSwapAccountsProto {
            payer?: (string|null);
            ammConfig?: (string|null);
            poolState?: (string|null);
            inputTokenAccount?: (string|null);
            outputTokenAccount?: (string|null);
            inputVault?: (string|null);
            outputVault?: (string|null);
            observationState?: (string|null);
            tokenProgram?: (string|null);
            tickArray?: (string|null);
        }

        class RaydiumSwapAccountsProto implements IRaydiumSwapAccountsProto {
            constructor(properties?: vixen.parser.IRaydiumSwapAccountsProto);
            public payer: string;
            public ammConfig: string;
            public poolState: string;
            public inputTokenAccount: string;
            public outputTokenAccount: string;
            public inputVault: string;
            public outputVault: string;
            public observationState: string;
            public tokenProgram: string;
            public tickArray: string;
            public static create(properties?: vixen.parser.IRaydiumSwapAccountsProto): vixen.parser.RaydiumSwapAccountsProto;
            public static encode(message: vixen.parser.IRaydiumSwapAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IRaydiumSwapAccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.RaydiumSwapAccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.RaydiumSwapAccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.RaydiumSwapAccountsProto;
            public static toObject(message: vixen.parser.RaydiumSwapAccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IRaydiumSwapV2AccountsProto {
            payer?: (string|null);
            ammConfig?: (string|null);
            poolState?: (string|null);
            inputTokenAccount?: (string|null);
            outputTokenAccount?: (string|null);
            inputVault?: (string|null);
            outputVault?: (string|null);
            observationState?: (string|null);
            tokenProgram?: (string|null);
            token_2022Program?: (string|null);
            memoProgram?: (string|null);
            inputVaultMint?: (string|null);
            outputVaultMint?: (string|null);
            tickArray?: (string|null);
        }

        class RaydiumSwapV2AccountsProto implements IRaydiumSwapV2AccountsProto {
            constructor(properties?: vixen.parser.IRaydiumSwapV2AccountsProto);
            public payer: string;
            public ammConfig: string;
            public poolState: string;
            public inputTokenAccount: string;
            public outputTokenAccount: string;
            public inputVault: string;
            public outputVault: string;
            public observationState: string;
            public tokenProgram: string;
            public token_2022Program: string;
            public memoProgram: string;
            public inputVaultMint: string;
            public outputVaultMint: string;
            public tickArray: string;
            public static create(properties?: vixen.parser.IRaydiumSwapV2AccountsProto): vixen.parser.RaydiumSwapV2AccountsProto;
            public static encode(message: vixen.parser.IRaydiumSwapV2AccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IRaydiumSwapV2AccountsProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.RaydiumSwapV2AccountsProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.RaydiumSwapV2AccountsProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.RaydiumSwapV2AccountsProto;
            public static toObject(message: vixen.parser.RaydiumSwapV2AccountsProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IRaydiumSwapIxDataProto {
            amount?: (number|Long|null);
            otherAmountThreshold?: (number|Long|null);
            sqrtPriceLimitX64?: (string|null);
            isBaseInput?: (boolean|null);
        }

        class RaydiumSwapIxDataProto implements IRaydiumSwapIxDataProto {
            constructor(properties?: vixen.parser.IRaydiumSwapIxDataProto);
            public amount: (number|Long);
            public otherAmountThreshold: (number|Long);
            public sqrtPriceLimitX64: string;
            public isBaseInput: boolean;
            public static create(properties?: vixen.parser.IRaydiumSwapIxDataProto): vixen.parser.RaydiumSwapIxDataProto;
            public static encode(message: vixen.parser.IRaydiumSwapIxDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IRaydiumSwapIxDataProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.RaydiumSwapIxDataProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.RaydiumSwapIxDataProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.RaydiumSwapIxDataProto;
            public static toObject(message: vixen.parser.RaydiumSwapIxDataProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IRaydiumSwapInstructionProto {
            accounts?: (vixen.parser.IRaydiumSwapAccountsProto|null);
            data?: (vixen.parser.IRaydiumSwapIxDataProto|null);
        }

        class RaydiumSwapInstructionProto implements IRaydiumSwapInstructionProto {
            constructor(properties?: vixen.parser.IRaydiumSwapInstructionProto);
            public accounts?: (vixen.parser.IRaydiumSwapAccountsProto|null);
            public data?: (vixen.parser.IRaydiumSwapIxDataProto|null);
            public static create(properties?: vixen.parser.IRaydiumSwapInstructionProto): vixen.parser.RaydiumSwapInstructionProto;
            public static encode(message: vixen.parser.IRaydiumSwapInstructionProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IRaydiumSwapInstructionProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.RaydiumSwapInstructionProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.RaydiumSwapInstructionProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.RaydiumSwapInstructionProto;
            public static toObject(message: vixen.parser.RaydiumSwapInstructionProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IRaydiumSwapV2InstructionProto {
            accounts?: (vixen.parser.IRaydiumSwapV2AccountsProto|null);
            data?: (vixen.parser.IRaydiumSwapIxDataProto|null);
        }

        class RaydiumSwapV2InstructionProto implements IRaydiumSwapV2InstructionProto {
            constructor(properties?: vixen.parser.IRaydiumSwapV2InstructionProto);
            public accounts?: (vixen.parser.IRaydiumSwapV2AccountsProto|null);
            public data?: (vixen.parser.IRaydiumSwapIxDataProto|null);
            public static create(properties?: vixen.parser.IRaydiumSwapV2InstructionProto): vixen.parser.RaydiumSwapV2InstructionProto;
            public static encode(message: vixen.parser.IRaydiumSwapV2InstructionProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: vixen.parser.IRaydiumSwapV2InstructionProto, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): vixen.parser.RaydiumSwapV2InstructionProto;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): vixen.parser.RaydiumSwapV2InstructionProto;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): vixen.parser.RaydiumSwapV2InstructionProto;
            public static toObject(message: vixen.parser.RaydiumSwapV2InstructionProto, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }
    }
}

export namespace google {

    namespace protobuf {

        interface IAny {
            type_url?: (string|null);
            value?: (Uint8Array|null);
        }

        class Any implements IAny {
            constructor(properties?: google.protobuf.IAny);
            public type_url: string;
            public value: Uint8Array;
            public static create(properties?: google.protobuf.IAny): google.protobuf.Any;
            public static encode(message: google.protobuf.IAny, writer?: $protobuf.Writer): $protobuf.Writer;
            public static encodeDelimited(message: google.protobuf.IAny, writer?: $protobuf.Writer): $protobuf.Writer;
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): google.protobuf.Any;
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): google.protobuf.Any;
            public static verify(message: { [k: string]: any }): (string|null);
            public static fromObject(object: { [k: string]: any }): google.protobuf.Any;
            public static toObject(message: google.protobuf.Any, options?: $protobuf.IConversionOptions): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }
    }
}

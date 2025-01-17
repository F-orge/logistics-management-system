// @generated by protobuf-ts 2.9.4
// @generated from protobuf file "health_check.proto" (package "health_check", syntax proto3)
// tslint:disable
import { ServiceType } from "@protobuf-ts/runtime-rpc";
import { WireType } from "@protobuf-ts/runtime";
import type { BinaryWriteOptions } from "@protobuf-ts/runtime";
import type { IBinaryWriter } from "@protobuf-ts/runtime";
import { UnknownFieldHandler } from "@protobuf-ts/runtime";
import type { BinaryReadOptions } from "@protobuf-ts/runtime";
import type { IBinaryReader } from "@protobuf-ts/runtime";
import type { PartialMessage } from "@protobuf-ts/runtime";
import { reflectionMergePartial } from "@protobuf-ts/runtime";
import { MessageType } from "@protobuf-ts/runtime";
/**
 * @generated from protobuf message health_check.HealthCheckRequest
 */
export interface HealthCheckRequest {}
/**
 * @generated from protobuf message health_check.HealthCheckResponse
 */
export interface HealthCheckResponse {
	/**
	 * @generated from protobuf field: int64 status = 1;
	 */
	status: bigint;
	/**
	 * @generated from protobuf field: string message = 2;
	 */
	message: string;
}
// @generated message type with reflection information, may provide speed optimized methods
class HealthCheckRequest$Type extends MessageType<HealthCheckRequest> {
	constructor() {
		super("health_check.HealthCheckRequest", []);
	}
	create(value?: PartialMessage<HealthCheckRequest>): HealthCheckRequest {
		const message = globalThis.Object.create(this.messagePrototype!);
		if (value !== undefined)
			reflectionMergePartial<HealthCheckRequest>(this, message, value);
		return message;
	}
	internalBinaryRead(
		reader: IBinaryReader,
		length: number,
		options: BinaryReadOptions,
		target?: HealthCheckRequest,
	): HealthCheckRequest {
		return target ?? this.create();
	}
	internalBinaryWrite(
		message: HealthCheckRequest,
		writer: IBinaryWriter,
		options: BinaryWriteOptions,
	): IBinaryWriter {
		let u = options.writeUnknownFields;
		if (u !== false)
			(u == true ? UnknownFieldHandler.onWrite : u)(
				this.typeName,
				message,
				writer,
			);
		return writer;
	}
}
/**
 * @generated MessageType for protobuf message health_check.HealthCheckRequest
 */
export const HealthCheckRequest = new HealthCheckRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class HealthCheckResponse$Type extends MessageType<HealthCheckResponse> {
	constructor() {
		super("health_check.HealthCheckResponse", [
			{
				no: 1,
				name: "status",
				kind: "scalar",
				T: 3 /*ScalarType.INT64*/,
				L: 0 /*LongType.BIGINT*/,
			},
			{ no: 2, name: "message", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
		]);
	}
	create(value?: PartialMessage<HealthCheckResponse>): HealthCheckResponse {
		const message = globalThis.Object.create(this.messagePrototype!);
		message.status = 0n;
		message.message = "";
		if (value !== undefined)
			reflectionMergePartial<HealthCheckResponse>(this, message, value);
		return message;
	}
	internalBinaryRead(
		reader: IBinaryReader,
		length: number,
		options: BinaryReadOptions,
		target?: HealthCheckResponse,
	): HealthCheckResponse {
		let message = target ?? this.create(),
			end = reader.pos + length;
		while (reader.pos < end) {
			let [fieldNo, wireType] = reader.tag();
			switch (fieldNo) {
				case /* int64 status */ 1:
					message.status = reader.int64().toBigInt();
					break;
				case /* string message */ 2:
					message.message = reader.string();
					break;
				default:
					let u = options.readUnknownField;
					if (u === "throw")
						throw new globalThis.Error(
							`Unknown field ${fieldNo} (wire type ${wireType}) for ${this.typeName}`,
						);
					let d = reader.skip(wireType);
					if (u !== false)
						(u === true ? UnknownFieldHandler.onRead : u)(
							this.typeName,
							message,
							fieldNo,
							wireType,
							d,
						);
			}
		}
		return message;
	}
	internalBinaryWrite(
		message: HealthCheckResponse,
		writer: IBinaryWriter,
		options: BinaryWriteOptions,
	): IBinaryWriter {
		/* int64 status = 1; */
		if (message.status !== 0n)
			writer.tag(1, WireType.Varint).int64(message.status);
		/* string message = 2; */
		if (message.message !== "")
			writer.tag(2, WireType.LengthDelimited).string(message.message);
		let u = options.writeUnknownFields;
		if (u !== false)
			(u == true ? UnknownFieldHandler.onWrite : u)(
				this.typeName,
				message,
				writer,
			);
		return writer;
	}
}
/**
 * @generated MessageType for protobuf message health_check.HealthCheckResponse
 */
export const HealthCheckResponse = new HealthCheckResponse$Type();
/**
 * @generated ServiceType for protobuf service health_check.HealthCheck
 */
export const HealthCheck = new ServiceType("health_check.HealthCheck", [
	{ name: "Check", options: {}, I: HealthCheckRequest, O: HealthCheckResponse },
]);

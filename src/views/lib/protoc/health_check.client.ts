// @generated by protobuf-ts 2.9.4
// @generated from protobuf file "health_check.proto" (package "health_check", syntax proto3)
// tslint:disable
import type { RpcTransport } from "@protobuf-ts/runtime-rpc";
import type { ServiceInfo } from "@protobuf-ts/runtime-rpc";
import { HealthCheck } from "./health_check";
import { stackIntercept } from "@protobuf-ts/runtime-rpc";
import type { HealthCheckResponse } from "./health_check";
import type { HealthCheckRequest } from "./health_check";
import type { UnaryCall } from "@protobuf-ts/runtime-rpc";
import type { RpcOptions } from "@protobuf-ts/runtime-rpc";
/**
 * @generated from protobuf service health_check.HealthCheck
 */
export interface IHealthCheckClient {
    /**
     * @generated from protobuf rpc: Check(health_check.HealthCheckRequest) returns (health_check.HealthCheckResponse);
     */
    check(input: HealthCheckRequest, options?: RpcOptions): UnaryCall<HealthCheckRequest, HealthCheckResponse>;
}
/**
 * @generated from protobuf service health_check.HealthCheck
 */
export class HealthCheckClient implements IHealthCheckClient, ServiceInfo {
    typeName = HealthCheck.typeName;
    methods = HealthCheck.methods;
    options = HealthCheck.options;
    constructor(private readonly _transport: RpcTransport) {
    }
    /**
     * @generated from protobuf rpc: Check(health_check.HealthCheckRequest) returns (health_check.HealthCheckResponse);
     */
    check(input: HealthCheckRequest, options?: RpcOptions): UnaryCall<HealthCheckRequest, HealthCheckResponse> {
        const method = this.methods[0], opt = this._transport.mergeOptions(options);
        return stackIntercept<HealthCheckRequest, HealthCheckResponse>("unary", this._transport, method, opt, input);
    }
}
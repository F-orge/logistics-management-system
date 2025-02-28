import { GrpcTransport } from "@protobuf-ts/grpc-transport";
import { ChannelCredentials } from "@grpc/grpc-js";
import { AuthServiceClient } from "./proto/auth.client";
import { StorageServiceClient } from "./proto/storage.client";

export const grpcTransport = new GrpcTransport({
  host: process.env.NEXTJS_GRPC_HOST_URL || "",
  channelCredentials: ChannelCredentials.createInsecure(),
});

export const authClient = new AuthServiceClient(grpcTransport);
export const storageClient = new StorageServiceClient(grpcTransport);

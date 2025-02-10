// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/stream.proto

import type * as grpc from '@grpc/grpc-js'
import type { MethodDefinition } from '@grpc/proto-loader'
import type { SubscribeRequest as _vixen_stream_SubscribeRequest, SubscribeRequest__Output as _vixen_stream_SubscribeRequest__Output } from '../../vixen/stream/SubscribeRequest';
import type { SubscribeUpdate as _vixen_stream_SubscribeUpdate, SubscribeUpdate__Output as _vixen_stream_SubscribeUpdate__Output } from '../../vixen/stream/SubscribeUpdate';

export interface ProgramStreamsClient extends grpc.Client {
  Subscribe(argument: _vixen_stream_SubscribeRequest, metadata: grpc.Metadata, options?: grpc.CallOptions): grpc.ClientReadableStream<_vixen_stream_SubscribeUpdate__Output>;
  Subscribe(argument: _vixen_stream_SubscribeRequest, options?: grpc.CallOptions): grpc.ClientReadableStream<_vixen_stream_SubscribeUpdate__Output>;
  subscribe(argument: _vixen_stream_SubscribeRequest, metadata: grpc.Metadata, options?: grpc.CallOptions): grpc.ClientReadableStream<_vixen_stream_SubscribeUpdate__Output>;
  subscribe(argument: _vixen_stream_SubscribeRequest, options?: grpc.CallOptions): grpc.ClientReadableStream<_vixen_stream_SubscribeUpdate__Output>;
  
}

export interface ProgramStreamsHandlers extends grpc.UntypedServiceImplementation {
  Subscribe: grpc.handleServerStreamingCall<_vixen_stream_SubscribeRequest__Output, _vixen_stream_SubscribeUpdate>;
  
}

export interface ProgramStreamsDefinition extends grpc.ServiceDefinition {
  Subscribe: MethodDefinition<_vixen_stream_SubscribeRequest, _vixen_stream_SubscribeUpdate, _vixen_stream_SubscribeRequest__Output, _vixen_stream_SubscribeUpdate__Output>
}

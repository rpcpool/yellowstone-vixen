import type * as grpc from '@grpc/grpc-js';
import type { MessageTypeDefinition } from '@grpc/proto-loader';

import type { ProgramStreamsClient as _vixen_stream_ProgramStreamsClient, ProgramStreamsDefinition as _vixen_stream_ProgramStreamsDefinition } from './vixen/stream/ProgramStreams';

type SubtypeConstructor<Constructor extends new (...args: any) => any, Subtype> = {
  new(...args: ConstructorParameters<Constructor>): Subtype;
};

export interface ProtoGrpcType {
  google: {
    protobuf: {
      Any: MessageTypeDefinition
    }
  }
  vixen: {
    stream: {
      ProgramStreams: SubtypeConstructor<typeof grpc.Client, _vixen_stream_ProgramStreamsClient> & { service: _vixen_stream_ProgramStreamsDefinition }
      SubscribeRequest: MessageTypeDefinition
      SubscribeUpdate: MessageTypeDefinition
    }
  }
}


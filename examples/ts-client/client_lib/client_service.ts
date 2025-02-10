import { makeGenericClientConstructor } from "@grpc/grpc-js";
import { vixen, google } from "./compiled";
import { VixenParserTypesUnion, decodeInnerMap } from "./elem_types";

export { vixen };
export { VixenParserTypesUnion };

export const ProgramStreamsServiceClient = makeGenericClientConstructor(
  {
    Subscribe: {
      path: "/vixen.stream.ProgramStreams/Subscribe",
      requestStream: false,
      responseStream: true,
      requestSerialize: (arg) => {
        return Buffer.from(vixen.stream.SubscribeRequest.encode(arg).finish());
      },
      requestDeserialize: (arg) => arg,
      responseSerialize: (arg) => arg,
      responseDeserialize: (arg): VixenParserTypesUnion => {
        const any = vixen.stream.SubscribeUpdate.decode(arg).parsed;
        if (any instanceof google.protobuf.Any) {
          let splitedUrl = any.type_url.split(".");
          let anyInnerType = splitedUrl[splitedUrl.length - 1];
          const decodeFn =
            decodeInnerMap[anyInnerType as keyof typeof decodeInnerMap];

          return decodeFn(any.value);
        } else {
          throw new Error("received stream not google.protobuf.Any");
        }
      },
    },
  },
  "ProgramStreams"
);

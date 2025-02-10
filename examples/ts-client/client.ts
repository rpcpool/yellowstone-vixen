import { INamespace, Root } from "protobufjs";
import { loadPackageDefinition, credentials } from "@grpc/grpc-js";
import { fromJSON } from "@grpc/proto-loader";
import { Any__Output } from "./proto/google/protobuf/Any";
import type { ProtoGrpcType } from "./proto/stream";
import type { ProtoGrpcType as ProtoGrpcTypeParser } from "./proto/parser";
import { SubscribeUpdate__Output } from "./proto/vixen/stream/SubscribeUpdate";
import root from "./compiled.json";

function decodeVixenParserHelper(parsed: Any__Output) {
  let parser = Root.fromJSON(root as INamespace);
  let anyInnerType = parsed.type_url.slice(1);

  return parser.lookupType(anyInnerType).decode(parsed.value);
}

function main() {
  let proto = loadPackageDefinition(
    fromJSON(root as INamespace)
  ) as unknown as ProtoGrpcType & ProtoGrpcTypeParser;

  const client = new proto.vixen.stream.ProgramStreams(
    "127.0.0.1:3030",
    credentials.createInsecure()
  );

  let stream = client.Subscribe({
    program: "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
  });

  stream.on("data", function (update: SubscribeUpdate__Output) {
    let decoded = decodeVixenParserHelper(update.parsed!);
    console.log(`decoded = ${JSON.stringify(decoded)}`);
  });

  stream.on("end", () => console.log("end"));

  stream.on("error", function (e: any) {
    console.log("error: ", e);
  });
}

main();

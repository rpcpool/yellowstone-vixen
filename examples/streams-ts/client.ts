import { INamespace, Root } from "protobufjs";
import { loadPackageDefinition, credentials } from "@grpc/grpc-js";
import { fromJSON } from "@grpc/proto-loader";
import root from "./compiled.json";

function decodeVixenParserHelper(parsed: any) {
  let parser = Root.fromJSON(root as INamespace);
  let anyInnerType = parsed.type_url.slice(1);

  return parser.lookupType(anyInnerType).decode(parsed.value);
}

function main() {
  let proto = loadPackageDefinition(fromJSON(root as INamespace)) as any;

  const client = new proto.vixen.stream.ProgramStreams(
    "127.0.0.1:3030",
    credentials.createInsecure()
  );

  let stream = client.Subscribe({
    program: "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
  });

  stream.on("data", function (update: any) {
    let decoded = decodeVixenParserHelper(update.parsed!);
    console.log(`decoded = ${JSON.stringify(decoded)}`);
  });

  stream.on("end", () => console.log("end"));

  stream.on("error", function (e: any) {
    console.log("error: ", e);
  });
}

main();

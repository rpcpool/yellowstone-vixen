import { credentials } from "@grpc/grpc-js";
import {
  ProgramStreamsServiceClient,
  vixen,
  VixenParserTypesUnion,
} from "./client_lib/client_service";

function main() {
  const client = new ProgramStreamsServiceClient(
    "127.0.0.1:3030",
    credentials.createInsecure()
  );

  let stream = client.Subscribe({
    program: "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
  });

  stream.on("data", function (update: VixenParserTypesUnion) {
    if (update instanceof vixen.parser.TokenExtensionStateProto) {
      console.log(update);
    } else {
      console.log("# Other event received: ", update.constructor.name);
    }
  });

  stream.on("end", () => console.log("end"));
  stream.on("error", (e: Error) => console.log("error: ", e));
}

main();

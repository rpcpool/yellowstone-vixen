var protobuf = require("protobufjs");
var grpc = require("@grpc/grpc-js");
var protoLoader = require("@grpc/proto-loader");

var PROTO_DIR = __dirname + "../../../crates/proto/proto";

var packageDefinition = protoLoader.loadSync(PROTO_DIR + "/stream.proto", {
  keepCase: true,
  longs: String,
  enums: String,
  defaults: true,
  oneofs: true,
});
let vixenStream = grpc.loadPackageDefinition(packageDefinition).vixen.stream;
let parser = protobuf.loadSync(PROTO_DIR + "/parser.proto");

function decodeVixenParserHelper(parsed) {
  let protoType = parsed.type_url.slice(1);
  return parser.lookupType(protoType).decode(parsed.value);
}

function main() {
  let client = new vixenStream.ProgramStreams(
    "127.0.0.1:3030",
    grpc.credentials.createInsecure()
  );

  let stream = client.Subscribe({
    program: "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
  });

  stream.on("data", function (update) {
    let decoded = decodeVixenParserHelper(update.parsed);
    console.log(`decoded = ${JSON.stringify(decoded)}`);
  });

  stream.on("end", () => console.log("end"));

  stream.on("error", function (e) {
    console.log("error: ", e);
  });
}

main();

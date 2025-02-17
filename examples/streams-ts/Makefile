STREAM_PROTO="./proto/stream.proto"
PARSER_PROTO="./proto/parser.proto"

# Default value, can be overwriten as a cli arg to `make`
SERVER_URL="127.0.0.1:3030"

.PHONY: clean build protos json types

protos:
	grpcurl -proto-out-dir "./proto" -plaintext $(SERVER_URL) describe vixen.stream.ProgramStreams.Subscribe
	grpcurl -proto-out-dir "./proto" -plaintext $(SERVER_URL) describe vixen.parser.TokenProgramStateProto

json: protos
	npx pbjs -t json -w commonjs -o compiled.json $(STREAM_PROTO) $(PARSER_PROTO)

clean:
	rm -fR ./proto/

build: | protos json types clean
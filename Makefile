# OpenAPI Generator Makefile

.PHONY: generate-openapi generate-sdks clean help

# Generate OpenAPI YAML specification from Petstore API
generate-openapi:
	@echo "🔧 Generating OpenAPI specification..."
	cargo run --bin generate-openapi
	@echo "✅ OpenAPI specification generated: petstore-api.yaml"

# Generate SDKs from OpenAPI specification
generate-sdks: generate-openapi
	@echo "🚀 Generating SDKs from OpenAPI specification..."
	mkdir -p test-output
	cargo run --bin openapi-generator -- generate \
		--input ./petstore-api.yaml \
		--output ./test-output \
		--languages typescript \
		--verbose
	@echo "✅ SDKs generated in ./test-output/"

# Clean generated files
clean:
	@echo "🧹 Cleaning generated files..."
	rm -f petstore-api.yaml
	rm -rf test-output/
	@echo "✅ Cleaned generated files"

# Show help
help:
	@echo "OpenAPI Generator Commands:"
	@echo "  generate-openapi  - Generate OpenAPI YAML from Petstore API"
	@echo "  generate-sdks     - Generate SDKs from OpenAPI specification"
	@echo "  clean            - Clean generated files"
	@echo "  help             - Show this help message"

# Default target
all: generate-sdks

CARGO = cargo
WASM_PACK = wasm-pack

build:
	${WASM_PACK} build --release --target web

test:
	${CARGO} test

run:
	${CARGO} run

clean:
	${CARGO} clean



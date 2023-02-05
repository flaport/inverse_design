build:
	cargo build --release
	cp target/release/libinverse_design_rs.so inverse_design_rs.so

test: build
	python test.py

clean:
	rm -rf target
	rm *.so

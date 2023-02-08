build:
	cargo build --release
	cp target/release/libinverse_design_rs.so inverse_design_rs.so
	cp target/release/libinverse_design_rs.so ../inverse_design_rs.so

test: build
	python test.py

time: build
	python time.py

clean:
	rm -rf target
	rm -f *.so

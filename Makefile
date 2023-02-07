.ONESHELL:
SHELL := /bin/bash
SRC = $(wildcard nbs/*.ipynb)

all: lib docs

.SILENT: docker
docker:
	docker build . -t id

arm_env:
	CONDA_SUBDIR=osx-arm64 conda env create -f environment.yml -n y

.PHONY: rust
rust:
	cd rust && maturin develop --release && cd -

lib: rust
	nbdev_export

run:
	find . -name "*.ipynb" | grep -v ipynb_checkpoints | xargs -I {} papermill {} {}

dist: clean
	python -m build --sdist --wheel

clean:
	nbdev_clean
	find . -name "*.ipynb" | xargs nbstripout
	find . -name "dist" | xargs rm -rf
	find . -name "build" | xargs rm -rf
	find . -name "builds" | xargs rm -rf
	find . -name "__pycache__" | xargs rm -rf
	find . -name "*.so" | xargs rm -rf
	find . -name "*.egg-info" | xargs rm -rf
	find . -name ".ipynb_checkpoints" | xargs rm -rf
	find . -name ".pytest_cache" | xargs rm -rf
	rm -f *.so
	cd rust && make clean && cd -

reset:
	rm -rf inverse_design
	rm -rf docs
	git checkout -- docs
	nbdev_build_lib
	make clean

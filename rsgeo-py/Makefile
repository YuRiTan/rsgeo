test-build:
	maturin build -o rsgeo-py/wheels -i $$(which python)

test-install: test-build
	python -m pip install -U wheels/rsgeo*

release-build:
	maturin build --release -o wheels -i $$(which python)

release-install: release-build
	python -m pip install -U wheels/rsgeo*

run-tests: test-install
	pytest tests

run-tests-with-cov: test-install
	cd .. && python -m pytest --cov-report=xml --cov=rsgeo rsgeo-py/tests/ && cd rsgeo-py

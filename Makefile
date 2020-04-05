.PHONY: clean clean-test clean-pyc clean-build

clean: clean-build clean-pyc clean-test ## remove all build, test, coverage and Python artifacts

clean-build: ## remove build artifacts
	rm -fr build/
	rm -fr dist/
	rm -fr .eggs/
	find . -name '*.egg-info' -exec rm -fr {} +
	find . -name '*.egg' -exec rm -f {} +

clean-pyc: ## remove Python file artifacts
	find . -name '*.pyc' -exec rm -f {} +
	find . -name '*.pyo' -exec rm -f {} +
	find . -name '*~' -exec rm -f {} +
	find . -name '__pycache__' -exec rm -fr {} +

clean-test: ## remove test and coverage artifacts
	rm -fr .tox/
	rm -f .coverage
	rm -fr htmlcov/
	rm -fr .pytest_cache

lint: ## check style and typing
	pipenv run flake8 genart tests setup.py
	pipenv run black --check --diff genart tests setup.py
	pipenv run isort --recursive --check-only genart tests setup.py
	pipenv run mypy genart

test: ## run tests quickly with the default Python
	pipenv run pytest tests/unit tests/integration


benchmark:
	pipenv run pytest tests/benchmarks


# Makefile helpers
.PHONY: ci-local ci-local-container update-goldens

ci-local:
	bash scripts/ci-local.sh

ci-local-container:
	bash scripts/ci-local-container.sh

update-goldens:
	bash scripts/update-goldens.sh

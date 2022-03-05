CURR_SHA = $(shell git rev-parse HEAD)
CURR_BRANCH = $(shell git rev-parse --abbrev-ref HEAD)

release:
		@echo Currently on branch $(CURR_BRANCH)
		-git branch -D gh-pages
		git checkout -b gh-pages
		bash build.sh
		git add -f docs
		git commit -m "Auto-generated commit of site from sha $(CURR_SHA)"
		git push origin gh-pages --force
		git checkout $(CURR_BRANCH)


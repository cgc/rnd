# addref

A small tool to append bibtex references to a file from a URL. Queries [ZoteroBib](https://zbib.org/)'s [translation server](https://github.com/zotero/translation-server). Python 3 version is `addref`,  Bash version is `addref.sh`, and Node.js version is `node/`.

```bash
$ addref references.bib https://onlinelibrary.wiley.com/doi/abs/10.1080/03640210701802071
Adding @article{Goodman2008Rational,
	journal = {Cognitive Science},
	doi = {10.1080/03640210701802071},
	issn = {1551-6709},
	number = {1},
	language = {en},
	title = {A Rational Analysis of Rule-Based Concept Learning},
	url = {https://onlinelibrary.wiley.com/doi/abs/10.1080/03640210701802071},
	volume = {32},
	author = {Goodman, Noah D. and Tenenbaum, Joshua B. and Feldman, Jacob and Griffiths, Thomas L.},
	note = {[Online; accessed 2020-04-26]},
	pages = {108--154},
	date = {2008},
	year = {2008},
}
```

A way to simplify use is make an alias that hardcodes the references file in, making `addref $URL` work.
```bash
alias addref='~/code/addref/addref ~/notes/references.bib'
```

The latest version of this script can optionally make use of GROBID, excellent software for extracting citation information from PDFs.

## a few other utilities

I've written a few other small utilities
- `annotatepdf FILE.pdf` which runs GROBID on the file and augments the file metadata with the extracted metadata.
- `batch_annotate_rp1` scans PDF files on a Sony DPT-RP1, annotating them using `annotatepdf`.

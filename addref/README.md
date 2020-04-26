# addref

A small tool to append references to a file. Uses [Citation.js](https://citation.js.org/) to look up a reference by URL from [ZoteroBib](https://zbib.org/)'s [translation server](https://github.com/zotero/translation-server).

Set things up by installing dependencies

```bash
$ npm install
```

Then add some references!

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

#!/usr/bin/env python3
import contextlib
import sqlite3
import os
import zlib
import json
import urllib.parse

@contextlib.contextmanager
def with_sqlite3(path):
    con = sqlite3.connect(path)
    try:
        cur = con.cursor()
        yield cur
    finally:
        con.close()

def sorted_entries(path):
    with with_sqlite3(path) as cur:
        for title, url, p in sorted(
            list(cur.execute('select title,url,position from cloud_tabs;')),
            key=lambda things: json.loads(zlib.decompress(things[-1]))['sortValues'][0]['sortValue'],
        ):
            yield title, url

def pinboard(title, url):
    return 'https://pinboard.in/add?next=same&'+urllib.parse.urlencode(dict(title=title, url=url))

TEMPLATE = '''
<!doctype html>
<html xmlns="http://www.w3.org/1999/xhtml" lang="en">
  <head>
    <meta charset="utf-8" />
    <style>
/* from https://github.com/mdn/learning-area/blob/master/html/tables/basic/minimal-table.css */
    html {
  font-family: sans-serif;
}

table {
  border-collapse: collapse;
  border: 2px solid rgb(200,200,200);
  letter-spacing: 1px;
  font-size: 0.8rem;
}

td, th {
  border: 1px solid rgb(190,190,190);
  padding: 10px 20px;
}

th {
  background-color: rgb(235,235,235);
}

td {
  text-align: left;
}

tr:nth-child(even) td {
  background-color: rgb(250,250,250);
}

tr:nth-child(odd) td {
  background-color: rgb(245,245,245);
}

caption {
  padding: 10px;
}

a {
color: black;
}

a:visited {
color: grey;
}

    </style>
  </head>
  <body>
  <table>
  %s
  </table>
  </body>
</html>
'''

def html(path):
    def trunced(s):
        lim = 80
        if len(s) > lim:
            return s[:lim]+'...'
        return s
    rows = [
        #f'<tr><td>{title}</td><td><a href="{pinboard(title, url)}">pinboard</a></td><td><a href="{url}">{title} {url}</a></td></tr>'
        f'<tr><td><a href="{pinboard(title, url)}">pinboard</a></td><td><a href="{url}">{trunced(title)}<br />{trunced(url)}</a></td></tr>'
        for title, url in list(sorted_entries(path))
    ]
    return TEMPLATE % ('\n'.join(rows))

if __name__ == '__main__':
    path = os.getenv('HOME') + '/Library/Safari/CloudTabs.db'
    with open('target/BOOKMARK_CACHE.HTML', 'w') as f:
        f.write(html(path))

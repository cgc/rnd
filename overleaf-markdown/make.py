from __future__ import print_function
try:
    from urllib.parse import quote
except:
    from urllib import quote

with open('overleaf-markdown.js') as f:
    result = []
    for idx, line in enumerate(f.readlines()):
        comment = '//'
        if comment in line:
            line = line[:line.index(comment)]
        line = line.strip()
        if line and line[-1] not in '`,{};[':
            print('Line {} may have bad ending: {}'.format(idx, line))
        result.append(line)
    code = ''.join(result)
    code = quote(code)
    bookmarklet = 'javascript:(function(){'+code+'})();'

import json
with open('../_data/overleaf_markdown.json', 'w') as f:
    json.dump(dict(BOOKMARKLET=bookmarklet), f)

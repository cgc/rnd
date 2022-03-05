from __future__ import print_function
try:
    from urllib.parse import quote
except:
    from urllib import quote

def buildjs(fn):
    with open(fn) as f:
        result = []
        for idx, line in enumerate(f.readlines()):
            comment = '//'
            # HACK this will skip comments on lines with URLs
            if comment in line and 'https://' not in line:
                line = line[:line.index(comment)]
            line = line.strip()
            if line and line[-1] not in '`,{};[':
                print(fn, 'Line {} may have bad ending: {}'.format(idx, line))
            result.append(line)
        code = ''.join(result)
        return code

code = quote(buildjs('markdown-it-katex.js') + buildjs('overleaf-markdown.js'))
bookmarklet = 'javascript:(function(){'+code+'})();'

import json
with open('../_data/overleaf_markdown.json', 'w') as f:
    json.dump(dict(BOOKMARKLET=bookmarklet), f)

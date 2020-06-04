---
title: Overleaf-Markdown
---

# Overleaf-Markdown bookmarklet

This bookmarklet adds support to overleaf for a markdown-based project using real-time compilation of markdown and latex (via KaTeX).

## Installation

On a desktop computer, drag the bookmarklet link above to your bookmarks toolbar. To use it, go to a recipe and click on the bookmark.

[Overleaf-Markdown link]({{site.data.overleaf_markdown.BOOKMARKLET}})

On iOS,
1. Bookmark this page (tap share icon, then bookmark icon).
2. <a href="#" onclick="copy();return false;">Click here</a> to copy the bookmark or select and copy it below.
<code style="display: block;overflow: hidden;white-space: nowrap;">{{site.data.overleaf_markdown.BOOKMARKLET}}</code>
3. Go to bookmarks and tap "edit". Paste in the address you copied in the previous step.
4. Try it out: Go to a recipe and click on the bookmark!

<script>
function copy() {
  const p = document.querySelector('code');
  const r = document.createRange();
  r.setStart(p, 0);
  r.setEnd(p, 1);

  // Select text
  const s = window.getSelection();
  s.removeAllRanges();
  s.addRange(r);
  // Copy
  document.execCommand('copy');
  // Unselect text
  s.removeAllRanges();
}
</script>

---
title: Overleaf-Markdown
---

# Overleaf-Markdown bookmark

This bookmark adds support to Overleaf for real-time compilation of Markdown and LaTeX (via KaTeX). To use, open or create a project with Markdown files (make sure to use `.md` as a file extension) then click on this bookmark.

LaTeX expressions like `$x=3$` are compiled via [KaTeX](https://katex.org/). For inline math, use one dollar sign `$x=3$`. For display mode, use two dollar signs `$$x=3$$`.

## Installation

On a desktop computer, drag this bookmarklet link to your bookmarks toolbar. To use it, visit your Overleaf project with markdown files and click on the bookmark.

[Overleaf-Markdown link]({{site.data.overleaf_markdown.BOOKMARKLET}})

On iOS,
1. Bookmark this page (tap share icon, then bookmark icon).
2. <a href="#" onclick="copy();return false;">Click here</a> to copy the bookmark or select and copy it below.
<code style="display: block;overflow: hidden;white-space: nowrap;">{{site.data.overleaf_markdown.BOOKMARKLET}}</code>
3. Go to bookmarks and tap "edit". Paste in the address you copied in the previous step.
4. Try it out: Go to a recipe and click on the bookmark!

<script>
function copy() {
  const p = Array.from(document.querySelectorAll('code')).find(
    el => el.innerHTML.startsWith('javascript:'));
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

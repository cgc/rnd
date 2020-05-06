function parseNumber(numberString) {
    let rx = /^(\d+) (\d+)\/(\d+)$/;
    let match = rx.exec(numberString);
    if (match) {
        return parseFloat(match[1]) + parseFloat(match[2]) / parseFloat(match[3]);
    }

    rx = /^(\d+)\/(\d+)$/;
    match = rx.exec(numberString);
    if (match) {
        return parseFloat(match[1]) / parseFloat(match[2]);
    }

    return parseFloat(numberString);
}

function parseIngredient(ingredient) {
    const rx = /(\d+(?:\/\d+)?)( \d+\/\d+)?/g;
    const result = [];
    let lastIndex = 0;

    function addText(match) {
        const end = match ? match.index : ingredient.length;
        const text = ingredient.slice(lastIndex, end);
        if (text) {
            result.push({
                type: 'text',
                text: text,
            });
        }
        // Update lastIndex to ensure we only add text after this.
        lastIndex = end;
    }

    let matchCount = 0;
    while (true) {
        // We have at most 2 matches...
        if (matchCount >= 2) {
            break;
        }

        const match = rx.exec(ingredient);

        // Skip this loop if the match is inside an HTML tag
        if (match != null) {
            let open = ingredient.slice(0, match.index).match(/</g) || [];
            let closed = ingredient.slice(0, match.index).match(/>/g) || [];
            if (open.length != closed.length) {
                continue;
            }
        }

        // Add text from end of last match to start of this match.
        addText(match);

        if (match == null) {
            break;
        }

        const matchString = match[0];
        result.push({
            type: 'number',
            text: matchString,
            value: parseNumber(matchString),
        });

        // Save index of remaining text for use at start of loop
        lastIndex = match.index + matchString.length;

        matchCount++;
    }

    addText(null);

    return result;
}

const decimalCount = 2;

const renderSuffix = {};
renderSuffix[(0).toFixed(decimalCount).slice(1)] = '';

for (let denom = 0; denom < 10; denom++) {
    for (let numer = 0; numer < denom; numer++) {
        const suffix = (numer/denom).toFixed(2).slice(1); // slice at 1 to skip leading zero of 0.xx
        if (!renderSuffix.hasOwnProperty(suffix)) {
            renderSuffix[suffix] = ` ${numer}/${denom}`;
        }
    }
}

function renderNumber(n) {
    let v = (n).toFixed(decimalCount);
    const suffix = v.slice(v.length - (decimalCount + 1));
    if (renderSuffix.hasOwnProperty(suffix)) {
        v = v.slice(0, v.length-suffix.length) + renderSuffix[suffix];
    }
    const prefix = '0 ';
    if (v.startsWith(prefix)) {
        v = v.slice(prefix.length);
    }
    return v;
}

function renderIngredient(ingredient, ratio) {
    return ingredient.map(i => {
        if (i.type == 'text') {
            return i.text;
        } else if (i.type == 'number') {
            let v = renderNumber(ratio*i.value);
            return '<span contenteditable class="EditableNumber" data-value="'+i.value+'">'+v+'</span>';
        }
    }).join('');
}

function editNumber(el, setRatio) {
    const value = parseFloat(el.dataset.value);
    const newValue = parseNumber(el.textContent);
    const ratio = newValue / value;
    setRatio(ratio);
}

function init() {
    const defaultSelector = '[itemprop="recipeIngredient"]';
    const selector = {
        'cooking.nytimes.com': '.recipe-ingredients > li > span',
        'www.kingarthurflour.com': '.recipe .recipe__ingredients ul li',
    }[window.location.hostname] || defaultSelector;

    const ingredients = Array.from(document.querySelectorAll(selector));
    const parsed = ingredients.map(el => parseIngredient(el.innerHTML));

    const state = { };

    function setRatio(ratio) {
        state.ratio = ratio;
        ingredients.forEach((el, idx) => {
            el.innerHTML = renderIngredient(parsed[idx], state.ratio);
        });
    }

    setRatio(1.);

    let mostRecentFocus;

    document.addEventListener('focusin', function(e) {
        if (e.target.classList.contains('EditableNumber')) {
            const p = e.target;
            var s = window.getSelection();
            var r = document.createRange();
            r.setStart(p, 0);
            r.setEnd(p, 1);
            s.removeAllRanges();
            s.addRange(r);

            mostRecentFocus = el.textContent;
        }
    });

    document.addEventListener('focusout', function(e) {
        if (e.target.classList.contains('EditableNumber')) {
            if (el.textContent != mostRecentFocus) {
              editNumber(e.target, setRatio);
            }

            mostRecentFocus = null;
        }
    });

    document.addEventListener('keydown', function(e) {
        if (e.target.classList.contains('EditableNumber')) {
            if (e.keyCode == 13) {
                e.preventDefault();
                editNumber(e.target, setRatio);
            }
        }
    });
}

function addStyle(styleString) {
  const style = document.createElement('style');
  style.textContent = styleString;
  document.head.append(style);
}

addStyle(`
.EditableNumber {
    border: 1px solid black;
    border-radius: 2px;
    padding: 0 2px;
    line-height: 1.4rem;
}
`);


init();

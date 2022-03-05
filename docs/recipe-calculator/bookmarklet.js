const decodeVulgar = {
    "\u00bd": "1/2",
    "\u2153": "1/3",
    "\u2154": "2/3",
    "\u00bc": "1/4",
    "\u00be": "3/4",
    "\u2155": "1/5",
    "\u2156": "2/5",
    "\u2157": "3/5",
    "\u2158": "4/5",
    "\u2159": "1/6",
    "\u215a": "5/6",
    "\u2150": "1/7",
    "\u215b": "1/8",
    "\u215c": "3/8",
    "\u215d": "5/8",
    "\u215e": "7/8",
    "\u2151": "1/9",
    "\u2152": "1/10",
};

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

    rx = /^(\d+)$/;
    match = rx.exec(numberString);
    if (match) {
        return parseFloat(match[1]);
    }

    throw new Error(`Could not extract number ${numberString}`);
}

function parseIngredient(ingredient) {
    // First we remove vulgar fractions
    for (let key of Object.keys(decodeVulgar)) {
        if (ingredient.indexOf(key) !== -1) {
            const replaceValue = ' ' + decodeVulgar[key];
            // Replace instances preceded by a space first.
            ingredient = ingredient.replace(new RegExp(' '+key, 'g'), replaceValue);
            // Then replace instances with no preceding space.
            ingredient = ingredient.replace(new RegExp(key, 'g'), replaceValue);
        }
    }

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

    while (true) {
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

        // Save index of remaining text for use at start of loop
        lastIndex = match.index + matchString.length;

        // Determine the unit
        let unit;
        let modifier;
        const MODIFIERS = ['', 'heaped', 'slightly heaped'].map(s => s.trim() ? s.trim()+' ' : s.trim());
        for (const conversion of CONVERSIONS_LIST) {
            for (const u of conversion.units) {
                for (const m of MODIFIERS) {
                    const queryString = ' ' + m + u;
                    if (ingredient.slice(lastIndex).startsWith(queryString)) {
                        modifier = m;
                        unit = conversion.units[0];
                        lastIndex += queryString.length;
                    }
                }
            }
        }

        result.push({
            type: 'number',
            text: matchString,
            value: parseNumber(matchString),
            unit,
            modifier,
        });

        // HACK Here awe see if the previous number is part of a range in a heuristic fashion.
        if (result.length >= 3 && unit) {
            const maybeRangeStart = result[result.length - 3];
            const maybeRange = result[result.length - 2];
            const maybeRangeEnd = result[result.length - 1];
            const rangeText = ['to', '-'];
            if (!maybeRangeStart.unit && maybeRange.type == 'text' && rangeText.indexOf(maybeRange.text.trim()) !== -1) {
                maybeRangeStart.unit = maybeRangeEnd.unit;
            }
        }
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

const CONVERSIONS_LIST = [];
const CONVERSIONS = {};
const RELATED_CONVERSIONS = {};

function addConversionSet(conversions) {
    for (const c of conversions) {
        // Modifications
        c.label = c.units[0];
        c.units = withPlurals(c.units);
        // This makes sure we find plurals first.
        c.units.reverse();

        CONVERSIONS_LIST.push(c);
        for (const unit of c.units) {
            RELATED_CONVERSIONS[unit] = conversions;
            CONVERSIONS[unit] = c;
        }
    }
}

function withPlurals(strings) {
    return strings.concat(strings.map(s => s+'s'));
}

addConversionSet([
    {units: ['teaspoon', 'tsp'], scale: 1/48},
    {units: ['tablespoon', 'tbsp'], scale: 1/16},
    {units: ['cup'], scale: 1},
    {units: ['pint'], scale: 2},
    {units: ['quart'], scale: 4},
    {units: ['gallon'], scale: 16},
]);

function renderUnitSelector(quantity, unit, ratio) {
    const cs = RELATED_CONVERSIONS[quantity.unit];
    const options = cs.map((c) => {
        const selected = c.units.indexOf(unit) == -1 ? '' : 'selected';
        const v = selected ? '' : renderNumber(ratio * convertQuantity(quantity, c.label));
        return `<option value="${c.label}" ${selected}>${v} ${c.label}</option>`;
    });
    return `<select class="EditableUnit">${options}</select>`;
}

function convertQuantity(quantity, unit) {
    if (!unit) {
        return quantity.value;
    }
    return quantity.value * CONVERSIONS[quantity.unit].scale / CONVERSIONS[unit].scale;
}

function renderIngredient(ingredientIdx, ingredient, ratio, units) {
    return ingredient.map((q, quantityIdx) => {
        if (q.type == 'text') {
            return q.text;
        } else if (q.type == 'number') {
            const unit = units[quantityIdx];
            const converted = convertQuantity(q, unit);
            const v = renderNumber(ratio*converted);
            const number = '<span contenteditable class="EditableNumber" data-value="'+converted+'">'+v+'</span>';
            let editableUnit;
            if (unit) {
                editableUnit = renderUnitSelector(q, unit, ratio);
            }
            return `<span class="EditableQuantity" data-idx="${ingredientIdx},${quantityIdx}">${number} ${q.modifier || ''}${editableUnit || ''}</span>`;
        }
    }).join('');
}

function editNumber(el, setRatio, undoEdit) {
    const value = parseFloat(el.dataset.value);
    let newValue;
    try {
        newValue = parseNumber(el.textContent);
    } catch(e) {
        undoEdit(el);
        errorDialog(e.message);
        return;
    }
    const ratio = newValue / value;
    setRatio(ratio);
}

// When adding the selector for a new website, make sure to include all ingredients as well as yield.
const defaultSelector = (
    '[itemprop="recipeIngredient"],' +
    // Epicurious uses `ingredients` instead.
    '[itemprop="ingredients"],' +
    '[itemprop="recipeYield"]'
);
const selector = {
    'cooking.nytimes.com': '.recipe-ingredients > li > span, .recipe-yield-value',
    'www.kingarthurbaking.com': '.recipe .recipe__ingredients ul li, .stat__item--yield',
    'www.bonappetit.com': '.ingredients .ingredients__text',
}[window.location.hostname] || defaultSelector;


function errorDialog(e) {
    const reportEmail = 'carloscorrea137+recipe_calc@gmail.com';
    alert(`Recipe calculator ran into some issues with ${window.location.href}:\n\n${e}\n\nEmail ${reportEmail} with a screenshot.`);
}

function init() {
    const ingredients = Array.from(document.querySelectorAll(selector));

    if (!ingredients.length) {
        errorDialog("Couldn't find ingredients on this website.");
        return;
    }

    const errors = [];
    const parsed = ingredients.map(el => {
        try {
            return parseIngredient(el.innerHTML);
        } catch(e) {
            errors.push(e);
        }
    });
    if (errors.length) {
        errorDialog(errors.map(e => e.message).join('\n'));
        return;
    }

    const state = {
        ratio: 1.,
        units: parsed.map(i => i.map(q => q.unit)),
    };

    function setState(nextState) {
        if (nextState.hasOwnProperty('ratio')) {
            state.ratio = nextState.ratio;
        } else if (nextState.hasOwnProperty('units')) {
            for (const ingredientIdx of Object.keys(nextState.units)) {
                for (const quantityIdx of Object.keys(nextState.units[ingredientIdx])) {
                    state.units[ingredientIdx][quantityIdx] = nextState.units[ingredientIdx][quantityIdx];
                }
            }
        }
        ingredients.forEach((el, idx) => {
            el.innerHTML = renderIngredient(idx, parsed[idx], state.ratio, state.units[idx]);
        });
    }

    // Helper function for ratio code.
    function setRatio(ratio) {
        setState({ratio});
    }

    setState({});

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

            mostRecentFocus = e.target.textContent;
        }
    });

    function undoEdit(el) {
        if (mostRecentFocus) {
            el.textContent = mostRecentFocus;
        }
    }

    document.addEventListener('focusout', function(e) {
        if (e.target.classList.contains('EditableNumber')) {
            if (e.target.textContent != mostRecentFocus) {
                editNumber(e.target, setRatio, undoEdit);
            }

            mostRecentFocus = null;
        }
    });

    document.addEventListener('keydown', function(e) {
        if (e.target.classList.contains('EditableNumber')) {
            if (e.keyCode == 13) {
                e.preventDefault();
                editNumber(e.target, setRatio, undoEdit);
            }
        }
    });

    document.addEventListener('change', function(e) {
        if (e.target.classList.contains('EditableUnit')) {
            const el = e.target;
            const editableQuantity = el.parentElement;
            const unit = el.options[el.selectedIndex].value;
            const [ingredientIdx, quantityIdx] = editableQuantity.dataset.idx.split(',').map(i => parseInt(i, 10));
            setState({units: {
                [ingredientIdx]: {
                    [quantityIdx]: unit,
                },
            }});
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
}`
);

init();

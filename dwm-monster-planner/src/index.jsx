import React, {useState} from 'react';
import ReactDOM from 'react-dom';

import {groupBy, arraysEqual, objectsEqualShallow} from './utils.js';
import {Suggest} from './suggest.jsx';
import monsters from './data/breeds.js';
import recipes from './data/recipes.js';
import monsterLocations from './data/monster-locations.json';

const families = new Set();
for (const name of Object.keys(monsters)) {
  const m = monsters[name];
  if (m.name != name) {
    throw new Error(`Monster name mismatch for ${name}`);
  }
  families.add(m.family);
}
const monstersByFamily = groupBy(Object.values(monsters), m => m.family);
const indRecipes = {};
const indRecipesByResult = {};
for (const r of recipes) {
  indRecipes[`${r.base},${r.mate}`] = r;
  indRecipesByResult[r.offspring] ||= [];
  indRecipesByResult[r.offspring].push(r);
}
function getRecipe(base, mate) {
  const b = monsters[base]
  const m = monsters[mate]
  return (
    indRecipes[`${b.name},${m.name}`] ||
    indRecipes[`${b.name},${m.family}`] ||
    indRecipes[`${b.family},${m.name}`] ||
    indRecipes[`${b.family},${m.family}`]
  );
}

function summarizeRecipes(rs) {
  const byBase = groupBy(rs, r => r.base);
  const byBaseAndMate = groupBy(Object.keys(byBase), b => byBase[b].map(r => r.mate).join(','))
  const rv = [];
  for (const baseGroup of Object.values(byBaseAndMate)) {
    const mateGroup = byBase[baseGroup[0]].map(r => r.mate);
    rv.push({
      ...byBase[baseGroup[0]][0],
      base: baseGroup,
      mate: mateGroup,
    });
  }
  return rv;
}

function Expandable({
  header, children,
  initial=false,
  opened=<span className="small-caret">▼</span>,
  closed=<span className="small-caret">▶</span>,
}) {
  const [expanded, setExpanded] = useState(initial);

  return expanded ? (
    <div>
    <span onClick={() => setExpanded(false)}>{header} {opened}</span>
    {children}
    </div>
  ) : (
    <div onClick={() => setExpanded(true)}>{header} {closed}</div>
  );
}

function MonsterSelector({monster: monsterProp, expanded}) {
  // Pulling out family if it's a singleton -- important for familyctx
  if (Array.isArray(monsterProp) && monsterProp.length == 1 && families.has(monsterProp[0])) {
    monsterProp = monsterProp[0];
  }
  const familyctx = families.has(monsterProp);

  let [currMonster, setCurrMonster] = useState();
  const [have, setHave] = useState();

  // HACK: making sure we appropriately clear the tree when things change.
  // best we can do without a centralized store.
  if (
    (Array.isArray(monsterProp) && !monsterProp.includes(currMonster)) ||
    (familyctx && !monstersByFamily[monsterProp].map(m => m.name).includes(currMonster))
  ) {
    currMonster = null;
  }

  monster = currMonster || monsterProp;

  function formon(m) {
    const cls = expanded ? (currMonster && !familyctx ? 'selected' : 'selectable') : '';
    return <div key={m} className={`${cls}`} onClick={() => expanded && setCurrMonster(m)}>{m}</div>;
  }

  let extra;
  let monsterel = monster;
  let locationEl;
  if (Array.isArray(monster)) {
    monsterel = monster.map(formon);
  } else if (families.has(monster)) {
    monsterel = expanded ? monstersByFamily[monster].map(m => formon(m.name)) : monster;
  } else if (expanded) {
    const locationInfo = monsterLocations[monster];
    if (locationInfo) {
      const locs = Object.keys(locationInfo).map(name => <div key={name}>{name} {locationInfo[name]}</div>);
      locationEl = (
        <>
        <div className="divider" />
        <Expandable header="Location">{locs}</Expandable>
        </>
      );
    }
    const header = (
      <>
      {monsterel}
      {currMonster && currMonster != monsterProp && <span onClick={() => setCurrMonster(null)}>❌</span>}
      </>
    );
    extra = (
      <>
      <Expandable header={header} opened={"☑️"} closed={"✅"} initial={true}>
        {locationEl}
        {indRecipesByResult[monster] && (
          <>
          <div className="divider" />
          <Expandable header="Parents" initial={true}>
            <RecipeSelector monster={monster} />
          </Expandable>
          </>
        )}
      </Expandable>
      </>
    );
  }
  const familyHeader = expanded && familyctx && (
    <>
    <div>{monsterProp}</div>
    <div className="divider" />
    </>
  );
  return (
    <div>
    {familyHeader}
    {extra ? extra : monsterel}
    </div>
  );
}

function RecipeSelector({monster}) {
  let [currRecipe, setCurrRecipe] = useState();

  const summarized = summarizeRecipes(indRecipesByResult[monster] || []);
  if (summarized.length == 1) {
    currRecipe = 0;
  }
  const hasRecipe = currRecipe != null;
  const rs = summarized.map((r, ri) => {
    if (hasRecipe && currRecipe != ri) {
      return;
    }
    const cls = hasRecipe ? 'selected' : 'selectable';
    const note = r.notes && (
      <div className="col">
        <div>Note</div>
        <div className="divider" />
        {r.notes}
      </div>
    );
    return (
      <div key={ri}>
      <div className={`Pairing row ${cls}`} onClick={() => setCurrRecipe(ri)}>
        <div className="col">
          <div>Base</div>
          <div className="divider" />
          <MonsterSelector monster={r.base} expanded={currRecipe==ri} />
        </div>
        <div className="col">
          <div>Mate</div>
          <div className="divider" />
          <MonsterSelector monster={r.mate} expanded={currRecipe==ri} />
        </div>
        {note}
      </div>
      </div>
    );
  });
  return (
    <>
    {hasRecipe && summarized.length != 1 ? <span onClick={() => { setCurrRecipe(null) }}>🔙</span> : null}
    {rs}
    </>
  );
}

function App() {
  const [showInfo, setShowInfo] = useState(true);
  return (
    <>
    <div id="intro">
      This is an interactive monster planner for Dragon Warrior Monsters, a game released by Enix in 1998 for the Game Boy.
      I played the series as a kid and was always enchanted by the cast of monsters and the elaborate
      ways to breed them. This planner makes it easy to figure out how to get your favorite monster. Credits
      to <a href="https://github.com/kalynrobinson/dwm-database">kalynrobinson</a> and <a href="https://gamefaqs.gamespot.com/gbc/197155-dragon-warrior-monsters/faqs/6849">AJackson</a> for data.
    </div>
    <Suggest>
      {(value) =>
        <>
        <br />
        <div className="Selector">
          <MonsterSelector monster={value.name} expanded={true} />
        </div>
        </>
      }
    </Suggest>
    </>
  );
}

ReactDOM.render(<App />, document.querySelector('#container'));

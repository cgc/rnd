// https://stackoverflow.com/a/16436975
export function arraysEqual(a, b) {
  if (a === b) return true;
  if (a == null || b == null) return false;
  if (a.length !== b.length) return false;

  // If you don't care about the order of the elements inside
  // the array, you should sort both arrays here.
  // Please note that calling sort on an array will modify that array.
  // you might want to clone your array first.

  for (var i = 0; i < a.length; ++i) {
    if (a[i] !== b[i]) return false;
  }
  return true;
}

// https://stackoverflow.com/questions/22266826/how-can-i-do-a-shallow-comparison-of-the-properties-of-two-objects-with-javascri
export const objectsEqualShallow = (obj1, obj2) =>
Object.keys(obj1).length === Object.keys(obj2).length &&
Object.keys(obj1).every(key =>
  obj2.hasOwnProperty(key) && obj1[key] === obj2[key]
);

export function groupBy(l, fn) {
  const grouped = {};
  for (const item of l) {
    const key = fn(item);
    grouped[key] ||= [];
    grouped[key].push(item);
  }
  return grouped;
}

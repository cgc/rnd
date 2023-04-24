'''
This script converts a lightly modified version of the book's test cases
into Rust. It does so by first converting the test cases into valid Python,
then converting that Python ast to a string.
'''

import behave.parser as p
import re
import pathlib
import ast
from contextlib import contextmanager

SCRIPT_DIR = pathlib.Path(__file__).absolute().parent

NO_FLOAT_FN = {
    'cofactor',
    'minor',
    'submatrix',
    'canvas',
    'write_pixel',
    'pixel_at',
    'lines',
    'ray_for_pixel',
    'shade_hit3',
    'reflected_color3',
    'refracted_color',
}

NO_FLOAT_CMP = [
    r'^.*\.(width|height|count|hsize|vsize|ignored|len\(\))$',
]

RESERVED_WORDS = {
    'from',
}

CONVERT_ATTR_TO_CALL = {
    'transform',
    'maximum',
    'minimum',
    'closed',
    'default_group',
    'p1', 'p2', 'p3',
    'e1', 'e2', 'normal',
    'n1', 'n2', 'n3',
    'left', 'right',
    'operation',
    'light',
    'pattern',
}

FN_NAME_NEEDS_ARG_COUNT = {
    'prepare_computations',
    'intersections',
    'lighting',
    'shade_hit',
    'prepare_computations',
    'reflected_color',
    'normal_at',
}

FN_MUT_FIRST_ARG = {
    'add_child',
    'set_transform',
    'set_pattern_transform',
    'write_pixel',
}

NO_REF_ARG_FNS = {
    'Some',
} | {
    f'intersections{i}' for i in range(10)
}

MUTABLE_NAMES = {
    'object',
    'outer', 'inner',
    'upper', 'lower',
    's', 'shape', 's1', 's2', 's3',
    'g', 'g1', 'g2',
    'A', 'B', 'C',
    'c', 'm', 'w',
    'floor', 'ball', 'cyl',
    'pattern',
    'parser',
}

class PyToRust(ast.NodeVisitor):
    def __init__(self):
        self.curr_const_type = float

    @contextmanager
    def ctx_curr_const_type(self, ty):
        prev = self.curr_const_type
        self.curr_const_type = ty
        try:
            yield
        finally:
            self.curr_const_type = prev

    def visit_BinOp(self, node):
        op = ast._Unparser.binop[node.op.__class__.__name__]
        return f'{self.visit(node.left)} {op} {self.visit(node.right)}'
    def visit_Compare(self, node):
        assert len(node.ops) == 1
        assert len(node.comparators) == 1
        node_op = node.ops[0]
        rhs = node.comparators[0]
        op = ast._Unparser.cmpops[node_op.__class__.__name__]
        left = self.visit(node.left)
        ty = float
        for rx in NO_FLOAT_CMP:
            if re.match(rx, left):
                ty = None
        with self.ctx_curr_const_type(ty):
            right = self.visit(rhs)
        direct_xs_comparison = (
            'xs' in left and '.' not in left
        )
        if (
            ty == float and
            op == '==' and
            # HACK: for intersections
            not direct_xs_comparison and
            isinstance(rhs, ast.Constant) and
            isinstance(rhs.value, (int, float))
        ):
            return f'equal({left}, {right})'
        if left.endswith('.object') and op == '==' and not right.endswith('.object'):
            right = f'&{right}'
        return f'{left} {op} {right}'
    def visit_UnaryOp(self, node):
        op = dict(
            USub='-',
            Not='!',
        )[node.op.__class__.__name__]
        return f'{op}{self.visit(node.operand)}'
    def visit_Constant(self, node):
        if isinstance(node.value, (float, int)):
            if self.curr_const_type == float:
                # Doing this instead of {:f} because numbers we
                # take sqrt of need to have an explicit precision.
                return f'{node.value}_f64'
            else:
                return f'{node.value}'
        v = ast.unparse(node)
        if isinstance(node.value, str):
            # Double-quotes for Rust
            v = v.replace("'", '"')
        return v
    def visit_Call(self, node):
        f = self.visit(node.func)

        if f in ('ref', 'mutref'):
            # HACK meant to make it easier to deal with some indexing for world tests.
            assert len(node.args) == 1
            prefix = '&' if f == 'ref' else '&mut '
            return f'{prefix}{self.visit(node.args[0])}'

        if f in FN_NAME_NEEDS_ARG_COUNT:
            f += str(len(node.args))
        ty = None if f in NO_FLOAT_FN else float
        with self.ctx_curr_const_type(ty):
            as_ = []
            for aidx, a in enumerate(node.args):
                s = self.visit(a)
                if (
                    f not in NO_REF_ARG_FNS and (
                        isinstance(a, ast.Name) or
                        isinstance(a, ast.Subscript) or
                        isinstance(a, ast.Call) and not isinstance(a.func, ast.Attribute) # HACK excluding attribute calls
                    )
                ):
                    assert not s.endswith('sqrt()')
                    if aidx == 0 and f in FN_MUT_FIRST_ARG:
                        s = self._maybe_mut_name(s)
                    s = f'&{s}'
                as_.append(s)
        rv = f'{f}({", ".join(as_)})'
        return rv
    def visit_Expr(self, node):
        return self.visit(node.value)
    def _maybe_mut_name(self, name):
        if name in MUTABLE_NAMES:
            return 'mut ' + name
        return name
    def visit_Assign(self, node):
        assert len(node.targets) == 1
        t_node = node.targets[0]
        v = self.visit(node.value)

        # special case to avoid standard property assignment
        if (
            isinstance(t_node, ast.Attribute) and
            t_node.attr in CONVERT_ATTR_TO_CALL
        ):
            lhs_attr = t_node.attr
            lhs_value = self.visit(t_node.value)
            # HACK: ref here is hardcoded
            return f'{lhs_value}.set_{lhs_attr}(&({v}))'

        t = self.visit(t_node)
        a = f'{self._maybe_mut_name(t)} = {v}'
        if '.' not in t:
            a = f'let {a}'
        return a
    def visit_Name(self, node):
        n = node.id
        if n in RESERVED_WORDS:
            n += '_'
        return n
    def visit_Assert(self, node):
        v = self.visit(node.test)
        if '==' in v:
            left, right = v.split(' == ')
            return f'assert_eq!({left}, {right})'
        if '!=' in v:
            left, right = v.split(' != ')
            return f'assert_ne!({left}, {right})'
        return f'assert!({v})'
    def visit_Attribute(self, node):
        attr = node.attr
        if attr in CONVERT_ATTR_TO_CALL:
            attr = f'{attr}()'
        return f'{self.visit(node.value)}.{attr}'
    def visit_Module(self, node):
        return [self.visit(n) + ';' for n in node.body]
    def visit_List(self, node):
        l = ', '.join(self.visit(el) for el in node.elts)
        return f'[{l}]'
    def visit_Tuple(self, node):
        l = ', '.join(self.visit(el) for el in node.elts)
        return f'({l})'
    def visit_Subscript(self, node):
        with self.ctx_curr_const_type(None):
            sl = self.visit(node.slice)
        return f'{self.visit(node.value)}[{sl}]'


ORDINALS = [
    (0, 'first'),
    (1, 'second'),
    (2, 'third'),
]


class Config:
    @staticmethod
    def convert_canvas(step, n):
        n = re.sub(r'(.*) ends with a newline character', r'\1.chars().last().unwrap() == "\\n".chars().last().unwrap()', n)
        n = re.sub(r'every pixel of (.*) is set to (.*)', r'\1.fill(\2)', n)
        n = re.sub(r'every pixel of (.*) is', r'\1 ==', n)
        n = re.sub(r'lines (\d+)-(\d+) of (.*) are', (
            lambda m: f'lines({m.group(3)}, {int(m.group(1))-1}, {int(m.group(2))}) == """{step.text}"""'
        ), n)
        return n

    @staticmethod
    def convert_world(step, n):
        n = re.sub(r'(.*) should terminate successfully', r'\1 != BLACK', n)
        n = re.sub(r'(.*) contains no objects', r'\1.count == 0', n)
        n = re.sub(r'(.*) has no light source', r'\1.lights.len() == 0', n)
        n = re.sub(r'(.*) contains (.*)', r'\1.objects.contains(\2)', n)
        for idx, name in ORDINALS:
            n = re.sub(rf'(.*) = the {name} object in (.*) immutable', rf'\1 = ref(\2.objects[{idx}])', n)
            n = re.sub(rf'(.*) = the {name} object in (.*)', rf'\1 = mutref(\2.objects[{idx}])', n)
        n = re.sub(r'(.*) is added to (.*)', r'\2.add(\1)', n)
        return n

    @staticmethod
    def convert_intersections(step, n):
        n = n.replace('i is nothing', 'i.is_none()')
        n = n.replace('i == ', 'i.unwrap() == ')
        return n

    @staticmethod
    def convert_obj_file(step, n):
        n = re.sub(r'(.*) = a file containing', rf'\1 = """{step.text}""".as_bytes()', n)
        n = re.sub(r'(.*) should have ignored (.*) lines', r'\1.ignored == \2', n)
        for idx, name in ORDINALS:
            n = re.sub(rf'(.*) = {name} child of (.*)', rf'\1 = ref(\2.children()[{idx}])', n)
        n = re.sub(r'(.*) = the file "(.*)"', r'\1 = read("book-code/files/\2").unwrap()', n)
        n = re.sub(r'"(.*)" from_ (.*)', r'ref(\2).named_group("\1").unwrap()', n)
        return n


def convert_step(stem, step, *, ex={}):
    n = (
        step.name
        .replace('=', '==')
        .replace('!==', '!=') # fixing previous line
        .replace('←', '=')
        .replace(' approximately', '')
    )

    for k, v in ex.items():
        n = n.replace(k, v)

    n = re.sub(rf'({"|".join(RESERVED_WORDS)})', r'\1_', n)

    fn = getattr(Config, f'convert_{stem}', None)
    if fn is not None:
        n = fn(step, n)

    # We don't hold refs to parents
    if '.parent' in n:
        return

    # Basic
    n = re.sub(r'(.*) is true', r'\1', n)
    n = re.sub(r'(.*) is false', r'not \1', n)
    n = re.sub(r'(.*) includes (.*)', r'\1.includes(\2)', n)

    # For tuples.feature
    n = re.sub(r'(.*) is a (.*)', r'\1.is_\2()', n)
    n = re.sub(r'(.*) is not a (.*)', r'not \1.is_\2()', n)
    # trying to handle sqrt, only for ints
    n = re.sub(r'√(\d+)', r'(\1).sqrt()', n)
    n = re.sub('π', 'PI', n)

    # For matrices.feature
    tab_str = None
    if step.table:
        tab = [step.table.headings] + [
            row.cells
            for row in step.table.rows
        ]

        prop_table = False
        try:
            # Prop tables have a string instead of a number in left column.
            float(tab[0][0])
        except ValueError:
            prop_table = True
        if prop_table:
            assert len(tab[0]) == 2, tab
            props = lambda target: [
                x if (x:=f'{target}.{prop} = {value}') and fn is None else fn(step, x)
                for prop, value in tab
            ]

        if not prop_table and len(tab) == len(tab[0]):
            size = len(tab)
            tab_str = f'''matrix{size}([{
", ".join(str(list(map(float, row))) for row in tab)
}])'''

    n = re.sub('the following(?: .+)? matrix (\w+)', (
        lambda m: f'{m.group(1)} = {tab_str}'
    ), n)
    n = re.sub('(.*) is the following(?: .+)? matrix', (
        lambda m: f'{m.group(1)} == {tab_str}'
    ), n)
    # for invertible
    n = re.sub(r'(.*) is not (.*)', r'not \1.is_\2()', n)
    n = re.sub(r'(.*) is (.*)', r'\1.is_\2()', n)

    # for intersections short syntax
    short = r'[^:]*:\w+'
    n = re.sub(rf'intersections\(({short}(?:, {short})*)\)', (
        lambda m: 'intersections(' + ', '.join(
            f'intersection({(p:=arg.split(":"))[0]}, {p[1]})'
            for arg in m.group(1).split(', ')
        ) + ')'
    ), n)

    # For table properties. Must be last!
    m = re.match('^(.*) = (.*) with$', n)
    if m:
        assert prop_table
        target = m.group(1)
        return [
            f'{target} = {m.group(2)}'
        ] + props(target)

    m = re.match('^(.*) has', n)
    if m:
        assert prop_table
        return props(m.group(1))

    if step.step_type == 'then':
        n = f'assert {n}'

    return n

def _gen(fn, stem):
    fn = pathlib.Path(fn)
    rv = f'''
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(unused_mut)]
#![allow(unused_variables)]
use ray_tracer_challenge::*;
use std::fs::read;

'''

    x = p.parse_file(fn)
    for idx, scenario in enumerate(x.scenarios):
        test_name = '_'.join(re.split(r'[^\w]+', scenario.name.lower())[:20])

        # BG steps are run for each scenario
        bg = []
        if scenario.background:
            bg = scenario.background.steps

        # Examples parameterize a test
        # This default value does no replacements
        exs = [dict()]
        if 'examples' in scenario.__dict__:
            assert len(scenario.examples) == 1
            ex = scenario.examples[0]
            exs = [
                {
                    f'<{name}>': value
                    for name, value in zip(ex.table.headings, row.cells)
                }
                for row in ex.table.rows
            ]

        for ex_idx, ex in enumerate(exs):
            step_rv = []
            for step in bg + scenario.steps:
                n = convert_step(stem, step, ex=ex)
                if isinstance(n, list):
                    step_rv.extend(n)
                elif n is not None:
                    step_rv.append(n)
            o = PyToRust().visit(ast.parse('\n'.join(step_rv)))
            o = '\n'.join(f'\t{l}' for l in o)
            ex_str = '' if len(exs) == 1 else f'_ex{ex_idx:02d}'
            rv += f'''
#[test]
fn test_{stem}_{idx:02d}_{test_name}{ex_str}() {{
    // {scenario.name}
{o}
}}
'''
    out = SCRIPT_DIR / f'generated/test_{stem}.rs'
    out.write_text(rv)

if __name__ == '__main__':
    libs = SCRIPT_DIR / '../src'
    mods = ''
    for f in pathlib.Path('book-code/features').glob('*.feature'):
        print(f'Generating for {f}')
        stem = f.stem.replace('-', '_')
        _gen(f, stem)
        mods += f'mod test_{stem};\n'
    (SCRIPT_DIR / 'generated/mod.rs').write_text(mods)

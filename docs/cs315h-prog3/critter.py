from collections import namedtuple, Counter
import numpy as np
import random
from glob import glob
import os
import pygame
import sys
import time
import hashlib


class CritterOperation(namedtuple('CritterOperation', ['op', 'args'])):
    @property
    def r1(self):
        return _parse_reg(self.args[0])

    @property
    def r2(self):
        return _parse_reg(self.args[1])

    @property
    def b(self):
        return int(self.args[0])

CritterSpecies = namedtuple('CritterSpecies', ['name', 'code', 'color'])


def _resolve_instruction_offset(critter, offset):
    if offset[0] == 'r':
        return critter.reg[_parse_reg(offset)]
    elif offset[0] == '+':
        return critter.next_code_line + int(offset[1:])
    elif offset[0] == '-':
        return critter.next_code_line - int(offset[1:])
    else:
        return int(offset)


def _parse_reg(reg):
    assert reg[0] == 'r', 'Invalid register reference: {}'.format(reg)
    parsed = int(reg[1:])
    assert 1 <= parsed <= 10, 'Invalid register number: {}'.format(reg)
    # Return index version of register
    return parsed - 1


def _string_to_rgb(string):
    h = hashlib.sha256()
    h.update(string.encode('ascii'))
    d = h.digest()
    return d[0], d[1], d[2]


def load_all_species():
    script_dir = os.path.dirname(os.path.abspath(__file__))
    species = []
    for fn in glob(os.path.join(script_dir, 'species/*.cri')):
        try:
            species.append(load_species(fn))
        except Exception as err:
            print('Error loading {}: {}'.format(fn, str(err)))
    return species


def load_species(filename=None, string=None):
    assert filename or string, 'Either filename or string must be provided when loading.'
    if filename:
        with open(filename, 'r') as f:
            string = f.read()
    lines = string.split('\n')
    name = lines[0]
    code = []
    for line in lines[1:]:
        if not line:
            break
        items = line.split(' ')
        code.append(CritterOperation(items[0], items[1:]))

    color = _string_to_rgb(name)
    return CritterSpecies(name, code, color)


def _jump(c, n):
    c.next_code_line = _resolve_instruction_offset(c, n)


def execute_critter(c):
    if c.next_code_line == 0:
        c.next_code_line = 1

    code = c.code

    counter = 0
    turn_completed = False

    while not turn_completed and 0 < c.next_code_line <= len(code) and counter < 1000:
        counter += 1

        o = code[c.next_code_line - 1]
        op = o.op
        args = o.args

        if op in ('hop', 'left', 'right', 'infect', 'eat'):
            turn_completed = True

            if op == 'infect' and args:
                c.infect(_resolve_instruction_offset(c, args[0]))
            else:
                getattr(c, op)()

        if op == 'go':
            _jump(c, args[0])
            continue

        if op == 'ifrandom' and c.if_random():
            _jump(c, args[0])
            continue

        if op == 'ifempty' and c.get_cell_content(o.b) == Critter.EMPTY:
            _jump(c, args[1])
            continue
        if op == 'ifally' and c.get_cell_content(o.b) == Critter.ALLY:
            _jump(c, args[1])
            continue
        if op == 'ifenemy' and c.get_cell_content(o.b) == Critter.ENEMY:
            _jump(c, args[1])
            continue
        if op == 'ifwall' and c.get_cell_content(o.b) == Critter.WALL:
            _jump(c, args[1])
            continue

        # register operations
        if op == 'write':
            c.reg[o.r1] = int(args[1])
        if op == 'add':
            c.reg[o.r1] = c.reg[o.r1] + c.reg[o.r2]
        if op == 'sub':
            c.reg[o.r1] = c.reg[o.r1] - c.reg[o.r2]
        if op == 'inc':
            c.reg[o.r1] = c.reg[o.r1] + 1
        if op == 'dec':
            c.reg[o.r1] = c.reg[o.r1] - 1

        # register comparisons
        if op == 'iflt' and c.reg[o.r1] < c.reg[o.r2]:
            _jump(c, args[2])
            continue
        if op == 'ifeq' and c.reg[o.r1] == c.reg[o.r2]:
            _jump(c, args[2])
            continue
        if op == 'ifgt' and c.reg[o.r1] > c.reg[o.r2]:
            _jump(c, args[2])
            continue

        # If we reach this part of the code, we didn't jump above, so we should increment to the next line
        _jump(c, '+1')


class Critter(object):
    FRONT = 0
    FRONT_RIGHT = 45
    RIGHT = 90
    REAR_RIGHT = 135
    REAR = 180
    REAR_LEFT = 225
    LEFT = 270
    FRONT_LEFT = 315

    HEADING_TO_OFFSET = {
        FRONT: (0, -1),
        FRONT_RIGHT: (+1, -1),
        RIGHT: (+1, 0),
        REAR_RIGHT: (+1, +1),
        REAR: (0, +1),
        REAR_LEFT: (-1, +1),
        LEFT: (-1, 0),
        FRONT_LEFT: (-1, -1),
    }

    EMPTY = 0
    WALL = 1
    ENEMY = 2
    ALLY = 3

    REGISTERS = 10
    WELL_FED_DURATION = 30

    def __init__(self, species, environment, heading):
        self.species = species
        self.environment = environment
        self.next_code_line = 0
        self.reg = [0] * Critter.REGISTERS
        # Small number in the past so logic works well for positive epoch counts.
        self._last_feeding_epoch = -1000
        self._heading = heading
        self._position = None
        self.is_dead = False

    @property
    def code(self):
        return self.species.code

    def left(self):
        self._heading = self._heading_with_bearing(-45)

    def right(self):
        self._heading = self._heading_with_bearing(+45)

    def _position_and_heading(self, position, heading):
        offset = Critter.HEADING_TO_OFFSET[heading]
        return (position[0] + offset[0], position[1] + offset[1])

    def _heading_with_bearing(self, b):
        # This method adds a bearing to the critter's current heading
        # returns a valid heading, which satisfies 0 <= heading < 360
        s = self._heading + b
        if s < 0:
            s += 360
        return s % 360

    def _is_well_fed(self):
        return self._last_feeding_epoch < self.environment.epoch <= self._last_feeding_epoch + Critter.WELL_FED_DURATION

    def set_position(self, pos):
        if self._position is not None:
            self.environment.grid[self._position] = None
        self._position = pos
        if self._position is not None:
            self.environment.grid[self._position] = self

    def hop(self):
        if self.get_cell_content() == Critter.EMPTY:
            newpos = self._position_and_heading(self._position, self._heading)
            self.set_position(newpos)

    def eat(self):
        if self.get_cell_content() == Critter.ENEMY:
            loc = self._position_and_heading(self._position, self._heading)
            dead = self.environment.grid[loc]
            dead.is_dead = True
            dead.set_position(None)
            self._last_feeding_epoch = self.environment.epoch

    def infect(self, n=0):
        if self.get_cell_content() == Critter.ENEMY:
            loc = self._position_and_heading(self._position, self._heading)
            enemy = self.environment.grid[loc]
            enemy.species = self.species
            enemy.next_code_line = n

    def get_cell_content(self, bearing=0):
        cell_heading = self._heading_with_bearing(bearing)
        loc = self._position_and_heading(self._position, cell_heading)
        grid_shape = self.environment.grid.shape
        if not(
            0 <= loc[0] < grid_shape[0] and
            0 <= loc[1] < grid_shape[1]
        ):
            return Critter.WALL
        cell = self.environment.grid[loc]
        if cell is None:
            return Critter.EMPTY
        elif cell.species is self.species:
            return Critter.ALLY
        else:
            return Critter.ENEMY

    def get_off_angle(self, heading):
        # HACK will this work? Pretty sure this is wrong
        # XXX this is wrong
        # XXX this is wrong
        # XXX this is wrong
        # XXX this is wrong
        # XXX this is wrong
        assert False
        return heading - self._heading
        # XXX probs more like
        cell = self.get_cell_content(heading)
        if cell:
            return cell._heading - self._heading

    def if_random(self):
        return self.environment.if_random()


class Environment(object):
    def __init__(self, shape=(30, 20), randseed=None, species=None, critter_count=None):
        self.grid = np.empty(shape, dtype=object)
        self.epoch = 0
        if randseed is None:
            randseed = random.randint(0, 2**30)
            print('Generating random seed for environment: {}'.format(randseed))
        self._random = random.Random(randseed)
        if critter_count is None:
            # One critter for every 20 squares?
            critter_count = max(2, round(shape[0] * shape[1] / 12))
        headings = list(Critter.HEADING_TO_OFFSET.keys())
        self.species = species
        self._critters = []
        for _ in range(critter_count):
            # We find all empty positions and select from there.
            pos = self._random.choice(list(zip(*np.where(self.grid == None))))  # NOQA
            c = Critter(
                self._random.choice(species),
                self,
                self._random.choice(headings))
            c.set_position(pos)
            self._critters.append(c)

    def if_random(self):
        return self._random.random() < 0.5

    def winner(self):
        # We are done if there is only one species of critter left!
        names = {
            c.species.name for c in self._critters
            if not c.is_dead
        }
        if len(names) == 1:
            return list(names)[0]

    def an_epoch_passes(self):
        for critter in self._critters:
            if critter.is_dead:
                continue

            well_fed = critter._is_well_fed()

            try:
                execute_critter(critter)
                # If a critter is well-fed, then it gets another chance!
                # Important to use the variable saved above, so they don't get a double turn
                # if they eat during first move.
                if well_fed:
                    execute_critter(critter)
            except Exception as err:
                print('Error while executing critter of species {} at line {}: {}'.format(
                    critter.species.name, critter.next_code_line, str(err)))
        self.epoch += 1


def main():
    species = load_all_species()

    # HACK filtering to keep things easy
    species = [
        s for s in species
        if s.name in ('Food', 'Hop', 'FlyTrap', 'Rover', 'Rover2')
    ]

    env = None
    state = 'stopped'

    pygame.init()
    pygame.font.init()
    font = pygame.font.SysFont('Arial', 10)

    size = width, height = 320, 240
    black = 0, 0, 0
    white = 0xFF, 0xFF, 0xFF
    cell_side = 10
    line_height = 15
    padding = 2

    screen = pygame.display.set_mode(size)

    while 1:
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                sys.exit()
            elif event.type == pygame.KEYDOWN:
                key = event.dict['unicode']
                if key == 'x':
                    state = 'stopped'
                elif key == ' ':
                    state = 'started'
                    env = Environment(species=species)

        screen.fill(white)

        if state == 'started':
            winner = env.winner()
            grid_shape = env.grid.shape
            species_names = [s.name for s in env.species]
            if winner is None:
                env.an_epoch_passes()

            for x in range(grid_shape[0]):
                for y in range(grid_shape[1]):
                    cell = env.grid[x, y]
                    if cell is None:
                        color = black
                    else:
                        color = cell.species.color
                    pygame.draw.rect(screen, color, (cell_side*x, cell_side*y, cell_side-1, cell_side-1))

            msg = 'Epoch {}'.format(env.epoch)
            ct = Counter(c.species.name for c in env._critters if not c.is_dead)
            for key in species_names:
                msg += ' - {} {}'.format(key, ct[key])
            if winner is not None:
                msg += ' - {} Wins!'.format(winner)
            textsurface = font.render(msg, False, black)
            screen.blit(textsurface, (padding, height-2*line_height))

        textsurface = font.render('Space to restart. x to stop.', False, black)
        screen.blit(textsurface, (padding, height-line_height))

        time.sleep(.05)
        pygame.display.flip()

if __name__ == '__main__':
    main()

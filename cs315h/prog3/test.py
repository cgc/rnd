from critter import Environment, load_species, Critter


def test_registers():
    spec = load_species(string='''Heyo
write r1 1
write r2 2
write r10 8
add r1 r10
ifeq r1 r2 +3
inc r2
go -2
write r3 4
''')
    env = Environment(randseed=42, species=[spec], critter_count=1)
    env.an_epoch_passes()
    assert env._critters[0].reg == [9, 9, 4, 0, 0, 0, 0, 0, 0, 8]


def test_movement():
    spec = load_species(string='''Heyo
right
hop
right
hop
left
hop
''')
    env = Environment(randseed=42, species=[spec], critter_count=1)
    c = env._critters[0]
    c.set_position((0, 0))
    c._heading = Critter.RIGHT

    env.an_epoch_passes()
    assert c._position == (0, 0)
    assert c._heading == Critter.REAR_RIGHT

    env.an_epoch_passes()
    assert c._position == (1, 1)
    assert c._heading == Critter.REAR_RIGHT

    env.an_epoch_passes()
    assert c._position == (1, 1)
    assert c._heading == Critter.REAR

    env.an_epoch_passes()
    assert c._position == (1, 2)
    assert c._heading == Critter.REAR

    env.an_epoch_passes()
    assert c._position == (1, 2)
    assert c._heading == Critter.REAR_RIGHT

    env.an_epoch_passes()
    assert c._position == (2, 3)
    assert c._heading == Critter.REAR_RIGHT


def test_critter_movement():
    spec = load_species(string='Heyo\nhop\ngo 1')
    env = Environment(randseed=42, species=[spec], critter_count=1)
    c = env._critters[0]

    for heading, expected_pos in [
        (Critter.FRONT, (1, 0)),
        (Critter.FRONT_RIGHT, (2, 0)),
        (Critter.RIGHT, (2, 1)),
        (Critter.REAR_RIGHT, (2, 2)),
        (Critter.REAR, (1, 2)),
        (Critter.REAR_LEFT, (0, 2)),
        (Critter.LEFT, (0, 1)),
        (Critter.FRONT_LEFT, (0, 0)),
    ]:
        c.set_position((1, 1))
        c._heading = heading

        env.an_epoch_passes()
        assert c._position == expected_pos

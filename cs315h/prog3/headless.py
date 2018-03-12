import critter
from collections import Counter


def main():
    species = {s.name: s for s in critter.load_all_species()}
    rounds = 10
    for _ in range(rounds):
        env = critter.Environment(species=[species['FlyTrap'], species['Rover']])
        counter = 0
        while env.winner() is None:
            env.an_epoch_passes()
            counter += 1
            if counter > 3000:
                print('Exceeded limit of 3000 epochs.')
                break
        ct = Counter(c.species.name for c in env._critters if not c.is_dead)
        print(' - '.join(
            '{} {}'.format(s.name, ct[s.name])
            for s in env.species
        ))

if __name__ == '__main__':
    main()

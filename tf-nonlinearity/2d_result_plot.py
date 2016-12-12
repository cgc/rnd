import matplotlib.pyplot as plt
import pickle


if __name__ == '__main__':
    plt.figure(figsize=(8, 9))
    for idx, (f, title) in enumerate([
        ('c_0_tf_error_cache', 'C=0'),
        ('c_0_1_tf_error_cache', 'C=0.1'),
        ('c_1_tf_error_cache', 'C=1')
    ]):
        with open(f, 'rb') as f:
            Is, Ns, result = pickle.load(f)

        ax = plt.subplot(2, 2, idx + 1)
        plt.pcolormesh(Is, Ns, result, cmap='cool')
        plt.xlabel('I (from {} to {})'.format(Is[0], Is[-1]))
        plt.ylabel('N (from {} to {})'.format(Ns[0], Ns[-1]))
        plt.title(title)
    plt.subplot(2, 2, 4)
    plt.colorbar(shrink=0.5, aspect=5)

    plt.show()

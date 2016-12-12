import matplotlib.pyplot as plt
import pickle


if __name__ == '__main__':
    plt.figure(figsize=(8, 9))
    for idx, f in enumerate([
        'c_0_tf_errors',
        'c_0.1_tf_errors',
        'c_1_tf_errors'
    ]):
        with open(f, 'rb') as f:
            d = pickle.load(f)
            Is = d['Is']
            Ns = d['Ns']
            C = d['C']
            errors = d['errors']

        I_step = Is[1] - Is[0]
        N_step = Ns[1] - Ns[0]
        ax = plt.subplot(2, 2, idx + 1)
        plt.pcolormesh(
            Is + [Is[-1] + I_step],
            Ns + [Ns[-1] + N_step],
            errors,
            cmap='cool')
        plt.axis([0, Is[-1] + I_step, 0, Ns[-1] + N_step])
        plt.xlabel('I (every {}, from {} to {})'.format(I_step, Is[0], Is[-1]))
        plt.ylabel('N (every {}, from {} to {})'.format(N_step, Ns[0], Ns[-1]))
        plt.title('C={}'.format(C))
    plt.subplot(2, 2, 4)
    plt.colorbar(shrink=0.5, aspect=5)

    plt.show()

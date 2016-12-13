import matplotlib.pyplot as plt
import pickle
import numpy.ma as ma
import numpy as np


if __name__ == '__main__':
    fig = plt.figure(figsize=(10, 8))

    data = []
    all_errors = []
    for idx, f in enumerate([
        'c_0_tf_errors',
        'c_0.1_tf_errors',
        'c_1_tf_errors'
    ]):
        with open(f, 'rb') as f:
            d = pickle.load(f)
            d['errors'] = ma.masked_invalid(d['errors'])
            data.append(d)
            all_errors.append(d['errors'].flatten())

    all_errors = np.concatenate(all_errors)
    all_errors = all_errors[~np.isnan(all_errors)]
    vmin = np.min(all_errors)
    # a hack to try and exclude an outlier with high cost that
    # makes colormaps hard to interpret
    vmax = np.percentile(all_errors, 99.9) * 1.1

    for idx, d in enumerate(data):
        Is = d['Is']
        Ns = d['Ns']
        C = d['C']

        errors = d['errors']
        I_step = Is[1] - Is[0]
        N_step = Ns[1] - Ns[0]
        ax = plt.subplot(2, 2, idx + 1)
        im = plt.pcolormesh(
            Is + [Is[-1] + I_step],
            Ns + [Ns[-1] + N_step],
            errors,
            vmin=vmin,
            vmax=vmax,
            cmap='cool')
        plt.axis([0, Is[-1] + I_step, 0, Ns[-1] + N_step])
        plt.xlabel('I (every {}, from {} to {})'.format(I_step, Is[0], Is[-1]))
        plt.ylabel('N (every {}, from {} to {})'.format(N_step, Ns[0], Ns[-1]))
        plt.title('C={}'.format(C))

    plt.tight_layout()

    fig.subplots_adjust(right=0.8)
    cbar_ax = fig.add_axes([0.85, 0.15, 0.05, 0.7])
    fig.colorbar(im, cax=cbar_ax)

    plt.show()

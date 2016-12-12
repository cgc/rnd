from autodiff import Gradient, escape, function
import numpy as np
import scipy.optimize
import matplotlib.pyplot as plt
from tqdm import tqdm
import pickle

C = 1
A = np.tanh


@function
def flatten_net(*args):
    return np.concatenate([
        # transposing first keeps things in fortran order
        arg.T.flatten()
        for arg in args
    ])


def ravel_net(x0, layer_sizes):
    idx = 0
    layers = []
    for layer_size in layer_sizes:
        length = np.prod(layer_size)
        layers.append(x0[idx:idx + length].reshape(layer_size, order='F'))
        idx += length
    assert idx == np.prod(x0.shape)
    return tuple(layers)


def tile_bias(bias, size):
    '''
    escape(size)[()] is a bit of a mess:
    py autodiff converts `size` into a theano array with 0
    dimensions and 1 item, the size. `escape` gives us
    a numpy array with 0 dimensions and 1 item, and [()]
    pulls the single item.
    '''
    return np.tile(bias, (escape(1), escape(size)[()]))


@function
def feed_net(b0, w1, b1, w2, b2, X):
    b0 = tile_bias(b0, X.shape[1])
    b1 = tile_bias(b1, X.shape[1])
    b2 = tile_bias(b2, X.shape[1])
    return A((X + b0).T.dot(w1) + b1.T).dot(w2).T + b2


@function
def Cost(b0, w1, b1, w2, b2, X, Y):
    mean = np.mean((feed_net(b0, w1, b1, w2, b2, X) - Y) ** 2)

    # linalg.norm doesn't seem to work in pyautodiff
    norm_w2 = np.sum(w2 ** 2) ** .5

    return mean + C * norm_w2


def callCost(x0, X, Y, layer_sizes):
    b0, w1, b1, w2, b2 = ravel_net(x0, layer_sizes)
    return Cost(b0, w1, b1, w2, b2, X, Y)

dCost_db0 = Gradient(Cost, wrt='b0')
dCost_dw1 = Gradient(Cost, wrt='w1')
dCost_db1 = Gradient(Cost, wrt='b1')
dCost_dw2 = Gradient(Cost, wrt='w2')
dCost_db2 = Gradient(Cost, wrt='b2')


def dCost(x0, X, Y, layer_sizes):
    b0, w1, b1, w2, b2 = ravel_net(x0, layer_sizes)
    return flatten_net(
        dCost_db0(b0, w1, b1, w2, b2, X, Y),
        dCost_dw1(b0, w1, b1, w2, b2, X, Y),
        dCost_db1(b0, w1, b1, w2, b2, X, Y),
        dCost_dw2(b0, w1, b1, w2, b2, X, Y),
        dCost_db2(b0, w1, b1, w2, b2, X, Y),
    )


def h_1(x):
    return x ** 2


def h_2(x):
    return x ** 3 - 10 * x ** 2 + x - 1


def h_3(x):
    return x ** (3/2.) - 20 * x ** .5 + 2 * x + 2


def h_4(x):
    return 3 * x ** (5/2.) - 20 * x ** .3 - 10 * x + 5


def h_5(x):
    return np.sin(np.pi * x)


def h_6(x):
    return np.cos(np.pi * x)


def h_7(x):
    return np.sin(2 * np.pi * x)


def h_8(x):
    return np.tan(np.pi * (x + .5))


all_h_fn = [h_1, h_2, h_3, h_4, h_5, h_6, h_7, h_8]
hs = all_h_fn[:-1]  # XXX skipping h_8 for now


def gpu_array(arr):
    '''
    GPU acceleration requires fortran order.
    '''
    # XXX what are the issues with np.float32?
    return np.array(arr, order='F')


def new_layer(*shape):
    return gpu_array(np.random.randn(*shape))


def test_helper_functions():
    N = 100
    I = 50
    O = len(hs)

    layer_sizes = [
        (N, 1),
        (N, I),
        (I, 1),
        (I, O),
        (O, 1)
    ]

    b0 = new_layer(N, 1)
    w1 = new_layer(N, I)
    b1 = new_layer(I, 1)
    w2 = new_layer(I, O)
    b2 = new_layer(O, 1)

    for recomputed, original in zip(
        ravel_net(flatten_net(b0, w1, b1, w2, b2), layer_sizes),
        [b0, w1, b1, w2, b2]
    ):
        assert np.all(recomputed == original), 'issue with ravel or flatten'

    X = gpu_array(np.tile(np.arange(0, 10, .1), (N, 1)))
    Y = gpu_array([[
        h(x) for x in np.arange(0, 10, .1)
    ] for h in hs])

    x0 = flatten_net(b0, w1, b1, w2, b2)
    x0 = x0.reshape((np.prod(x0.shape),))
    # XXX for float32? epsilon=np.sqrt(np.finfo(np.float32).eps)
    result = scipy.optimize.check_grad(
        callCost, dCost, x0, X, Y, layer_sizes)
    print 'check_grad', result


def test_network(N, I, debug=True):
    O = len(hs)

    layer_sizes = [
        (N, 1),
        (N, I),
        (I, 1),
        (I, O),
        (O, 1)
    ]

    b0 = new_layer(N, 1)
    w1 = new_layer(N, I)
    b1 = new_layer(I, 1)
    w2 = new_layer(I, O)
    b2 = new_layer(O, 1)

    X = gpu_array(np.tile(np.arange(0, 10, .1), (N, 1)))
    Y = gpu_array([[
        h(x) for x in np.arange(0, 10, .1)
    ] for h in hs])

    kwargs = {}
    if debug:
        kwargs = dict(iprint=98)

    x, f, d = scipy.optimize.fmin_l_bfgs_b(
        callCost,
        x0=flatten_net(b0, w1, b1, w2, b2),
        fprime=dCost,
        maxfun=90000,
        maxiter=90000,
        args=(X, Y, layer_sizes),
        **kwargs)

    if d['warnflag']:
        print d['warnflag'], d['task'], d['nit'], d['funcalls']

    if debug:
        # Use the trained network to compute F(3)
        b0, w1, b1, w2, b2 = ravel_net(x, layer_sizes)
        X = gpu_array(np.tile(np.arange(3, 3.05, .1), (N, 1)))
        print 'F({})'.format(X[0, :])
        print 'expected', gpu_array([h(X[0, :]) for h in hs])
        print 'actual', feed_net(b0, w1, b1, w2, b2, X)

    return f


error_cache_file = 'error_cache'


def compute_network_errors():
    Ns = range(30, 200, 40)
    Is = range(10, 100, 30)
    result = np.zeros((len(Ns), len(Is)))

    for N_idx, N, I_idx, I in tqdm([
        (N_idx, N, I_idx, I)
        for N_idx, N in enumerate(Ns)
        for I_idx, I in enumerate(Is)
    ]):
        result[N_idx, I_idx] = test_network(N, I, debug=False)

    with open(error_cache_file, 'wb') as f:
        pickle.dump((Is, Ns, result), f)


def test_networks():
    # compute_network_errors()

    with open(error_cache_file, 'rb') as f:
        Is, Ns, result = pickle.load(f)

    plt.pcolormesh(Is, Ns, result, cmap='gray')
    plt.xlabel('I (from {} to {})'.format(Is[0], Is[-1]))
    plt.ylabel('N (from {} to {})'.format(Ns[0], Ns[-1]))
    plt.show()


if __name__ == '__main__':
    # Can use this to debug theano code. Can help with line numbers in some cases.
    '''
    import theano
    theano.config.compute_test_value = 'warn'
    '''
    test_helper_functions()
    test_network(300, 1000)
    # test_networks()

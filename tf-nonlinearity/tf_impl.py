from __future__ import absolute_import
from __future__ import division
from __future__ import print_function

from numpy_impl import all_h_fn
import tensorflow as tf
import numpy as np
import matplotlib.pyplot as plt
from tqdm import tqdm
import pickle

all_h_fn = all_h_fn[:-1]  # XXX leaving out h_8 for now


def weight_variable(shape):
    initial = tf.truncated_normal(shape, stddev=0.1)
    return tf.Variable(initial)


def bias_variable(shape):
    initial = tf.constant(0.1, shape=shape)
    return tf.Variable(initial)


def plot_learned_fn(x, expected, actual):
    x = x[:, 0]
    plt.figure()
    for idx in range(expected.shape[-1]):
        plt.subplot(4, 2, idx + 1)
        plt.plot(x, expected[:, idx], 'b')
        plt.plot(x, actual[:, idx], 'r')


def network_error(N, I, debug=True, C=1):
    O = len(all_h_fn)

    b0 = bias_variable([N])
    w1 = weight_variable([N, I])
    b1 = bias_variable([I])
    w2 = weight_variable([I, O])
    b2 = bias_variable([O])

    x = tf.placeholder(tf.float32, shape=[None, N])
    y_ = tf.placeholder(tf.float32, shape=[None, O])

    y = tf.matmul(tf.tanh(tf.matmul(x + b0, w1) + b1), w2) + b2

    norm_w2 = tf.reduce_sum(w2 ** 2) ** .5
    cost = tf.reduce_mean((y - y_) ** 2) + C * norm_w2

    X = np.tile(np.arange(0, 10, .1), (N, 1)).T
    Y = np.transpose([[
        h(x_) for x_ in np.arange(0, 10, .1)
    ] for h in all_h_fn])

    if debug:
        errors = []

    init = tf.initialize_all_variables()
    sess = tf.get_default_session()
    sess.run(init)
    train_step = tf.train.GradientDescentOptimizer(0.01).minimize(cost)
    # XXX train on random subsets?
    # XXX check for convergence instead of static loop count
    for _ in range(400):
        _, error_value = sess.run(
            [train_step, cost], feed_dict={x: X, y_: Y})
        if debug:
            errors.append(error_value)
    if debug:
        test_x = X[30:32, :]
        test_y = Y[30:32, :]
        print('arg', test_x)
        print('expected', test_y)
        print('network output', )
        plot_learned_fn(X, Y, y.eval(feed_dict={x: X}))

        print('final', error_value)
        plt.figure()
        plt.plot([np.mean(errors[i-50:i]) for i in range(1, len(errors))])
        plt.show()
    return error_value


def compute_network_errors():
    Ns = range(10, 2000, 50)
    Is = range(10, 2000, 50)
    result = np.zeros((len(Ns), len(Is)))

    for C in [0, .1, 1]:
        print('compute_network_errors', 'C={}'.format(C))
        # reversing order to make sure we compute harder things earlier
        # to get a sense of length of computation
        for N_idx, N, I_idx, I in tqdm([
            (N_idx, N, I_idx, I)
            for N_idx, N in enumerate(Ns)
            for I_idx, I in enumerate(Is)
        ][::-1]):
            result[N_idx, I_idx] = network_error(N, I, C=C, debug=False)

        with open('c_{}_tf_errors'.format(C), 'wb') as f:
            pickle.dump(dict(
                Is=Is,
                Ns=Ns,
                errors=result,
                C=C
            ), f)


if __name__ == '__main__':
    with tf.Session():
        compute_network_errors()
        # network_error(300, 1000)
        # XXX
        # tf.test.compute_gradient_error

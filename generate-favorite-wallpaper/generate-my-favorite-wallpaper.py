import numpy as np
import imageio
import os

h, w, c = (1392, 744, 3)

# Repeating width is 6 for circle and 6 for space
repeat_width = 12
# Repeating height is 6 for circle and 3 for space, times 2 to include both circles
repeat_height = 18

bgfit = [3.09835296e-07, -2.40692464e-04, -4.88381708e-02, 1.94399232e+02]
fgfit = [2.14403711e-07, -1.66935758e-04, -2.80828886e-02, 2.16077259e+02]

first_chunk_length = 793
last_chunk_length = 80

corner_alpha = 0.059
next_to_corner_alpha = 0.749
weird_spot_alpha = 0.125

circle_alpha = np.array([
    [corner_alpha, next_to_corner_alpha, 1., 1., next_to_corner_alpha, corner_alpha],
    [next_to_corner_alpha, 1., 1., 1., 1., next_to_corner_alpha],
    [1., 1., 1., 1., 1., 1.],
    [1., 1., 1., 1., 1., 1.],
    [next_to_corner_alpha, 1., 1., 1., 1., next_to_corner_alpha],
    [corner_alpha, next_to_corner_alpha, 1., 1., next_to_corner_alpha, corner_alpha],
])

def gen_block(start_row, fg, bg):
    result = np.zeros((repeat_height, repeat_width))
    # Make everything have the right bg
    result += bg[start_row:start_row+repeat_height, np.newaxis]

    def _draw_circle(row, col):
        result[row:row+6, col:col+6] = (
            circle_alpha * fg[start_row+row:start_row+row+6, np.newaxis] +
            (1-circle_alpha) * bg[start_row+row:start_row+row+6, np.newaxis]
        )

    # Then draw our circles
    _draw_circle(0, 0)
    _draw_circle(9, 6)

    # HACK the weird spots..
    result[2, 6] = weird_spot_alpha * fg[start_row+2] + (1-weird_spot_alpha) * bg[start_row+2]
    result[11, 0] = weird_spot_alpha * fg[start_row+11] + (1-weird_spot_alpha) * bg[start_row+11]

    return np.round(result).astype(np.int)


xval = range(h-first_chunk_length-last_chunk_length)

# First we generate fg and bg
fg = np.zeros((h,))
fg[:first_chunk_length] = 217
fg[first_chunk_length:-last_chunk_length] = np.round(np.poly1d(fgfit)(xval)).astype(np.int)
fg[-last_chunk_length:] = 186

bg = np.zeros((h,))
bg[:first_chunk_length] = 195
bg[first_chunk_length:-last_chunk_length] = np.round(np.poly1d(bgfit)(xval)).astype(np.int)
bg[-last_chunk_length:] = 147

# Now gen the blocks
result = np.concatenate([
    gen_block(start_row-repeat_height if start_row == h-6 else start_row, fg, bg)
    for start_row in range(0, h, repeat_height)
])
# HACK need to clip from end of this...
result = result[:h, :]

# And we add color channels
result = np.tile(result[:, :, np.newaxis], [1, w/repeat_width, c])

assert result.shape == (1392, 744, 3)

imageio.imwrite('generated.png', result.astype(np.uint8))

if os.path.exists('original.png'):
  orig = imageio.imread('original.png')
  diff = (orig - result) * 30 + 120
  imageio.imwrite('diff.png', diff.astype(np.uint8))

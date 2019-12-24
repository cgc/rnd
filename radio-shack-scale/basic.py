import hid
import time
from contextlib import contextmanager
import tkinter as tk
import signal

units_to_grams = 1/2.674 # Guess based on seeing what other program does

@contextmanager
def usb_scale():
    h = None
    try:
        h = hid.device()
        h.open(8755, 25379)

        print("Manufacturer: %s" % h.get_manufacturer_string())
        print("Product: %s" % h.get_product_string())

        def read():
            d = h.read(8)
            if d:
                weight = d[-1] + ((d[-2]&0x0f) << 8)
                return weight

        yield read

    except IOError as ex:
        print(ex)
        print('Device is probably not plugged in.')
    finally:
        if h:
            h.close()


with usb_scale() as read:

    zero = read()
    conv = units_to_grams
    unit_string = 'g'
    shutdown = False

    def format_read():
        return str(int((read() - zero) * conv))+unit_string

    def zero_fn():
        global zero
        zero = read()

    counter = 0 
    def counter_label(label):
      def count():
        if shutdown:
            return
        label.config(text=format_read())
        label.after(50, count)
      count()

    root = tk.Tk()
    root.title('Radio Shack scale')
    label = tk.Label(root, fg='black')
    label.pack()
    counter_label(label)
    zero_btn = tk.Button(root, text='Zero', width=25, command=zero_fn)
    zero_btn.pack()
    button = tk.Button(root, text='Stop', width=25, command=root.destroy)
    button.pack()

    def sigint_handler(sig, frame):
        shutdown = True
        print('hi2')
        root.destroy()
        print('hi')
    signal.signal(signal.SIGINT, sigint_handler)

    root.mainloop()


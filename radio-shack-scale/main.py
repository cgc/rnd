import hid
import time
from contextlib import contextmanager
import tkinter as tk
import signal

units_to_grams = 1/2.674 # Guess based on seeing what other program does
grams_to_ounces = 1/28.3495
ounces_to_pounds = 1/16
units = {
    'Grams': (units_to_grams, 'g', 0),
    'Kilograms': (units_to_grams/1000, 'kg', 3),
    'Ounces': (units_to_grams*grams_to_ounces, 'oz', 1),
    'Pounds': (units_to_grams*grams_to_ounces*ounces_to_pounds, 'lb', 2),
}

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
    defval = 'Grams'
    conv, unit_string, places = units[defval]
    shutdown = False

    def format_read():
        return ('{:.0'+str(places)+'f}{}').format((read() - zero) * conv, unit_string)

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
    root.title('Radio Shack Scale')
    label = tk.Label(root, fg='black')
    label.pack()
    counter_label(label)

    variable = tk.StringVar(root)
    variable.set(defval)
    w = tk.OptionMenu(root, variable, *units.keys())
    w.pack()
    def change_dropdown(*args):
        global conv, unit_string, places
        conv, unit_string, places = units[variable.get()]
    variable.trace('w', change_dropdown)

    zero_btn = tk.Button(root, text='Zero', width=25, command=zero_fn)
    zero_btn.pack()

    button = tk.Button(root, text='Stop', width=25, command=root.destroy)
    button.pack()

    # interrupt handler
    def sigint_handler(sig, frame):
        shutdown = True
        root.destroy()
    signal.signal(signal.SIGINT, sigint_handler)

    root.mainloop()


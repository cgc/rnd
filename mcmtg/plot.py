import seaborn as sns
import pandas as pd
import json
import matplotlib.pyplot as plt

if __name__ == '__main__':
    import sys
    fn = sys.argv[1]
    with open(fn) as f:
        config = json.load(f)
    df = pd.DataFrame(config['df'])
    final_df = pd.DataFrame(config['final_df'])
    for idx, (method, kwargs) in enumerate(config['plot']):
        err = kwargs.pop('errbar', None)
        data = locals()[kwargs.pop('data')]
        ax = getattr(sns, method)(**kwargs, data=data)
        if err is not None:
            if getattr(ax, 'patches', None):
                # works for bar plots??
                x_coords = [p.get_x() + 0.5*p.get_width() for p in ax.patches]
                y_coords = [p.get_height() for p in ax.patches]
                ax.errorbar(x=x_coords, y=y_coords, yerr=df[err], fmt="none", c= "k")
            if getattr(ax, 'lines', None):
                # x_coords = [x for p in ax.lines for x in p.get_xdata()]
                # y_coords = [y for p in ax.lines for y in p.get_ydata()]
                ax.errorbar(x=data[kwargs['x']], y=data[kwargs['y']], yerr=data[err], fmt="none", c= "k")
            # problem with this is the x values are not used to place -- it's their indices?
            # if isinstance(ax, sns.FacetGrid):
            #     for (row_i, col_j, hue_k), data_ijk in ax.facet_data():
            #         ax_ = ax.facet_axis(row_i, col_j, modify_state=False)
            #         ax_.errorbar(x=data_ijk[kwargs['x']], y=data_ijk[kwargs['y']], yerr=data_ijk[err], fmt="none", c= "k")
        plt.savefig(f'figures/figure{idx}.pdf')
        plt.close()

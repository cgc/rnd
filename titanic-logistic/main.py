import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
from sklearn import linear_model, datasets
from sklearn.pipeline import Pipeline
from sklearn.preprocessing import Imputer
import math


title_to_enum = {
    'mr': 'mr',
    'master': 'mr',

    'miss': 'miss',
    'mlle': 'miss',
    'mrs': 'miss',
    'ms': 'miss',
    'mme': 'miss',

    'dr': 'rare',
    'rev': 'rare',
    'major': 'rare',
    'col': 'rare',
    'capt': 'rare',
    'sir': 'rare',
    'lady': 'rare',
    'jonkheer': 'rare',
    'don': 'rare',
    'dona': 'rare',
    'sir': 'rare',
    'the countess': 'rare'
}


value_mappings = {}


def map_values_to_number(ds, attr):
    if attr not in value_mappings:
        unique_values = ds[attr].value_counts().index.tolist()
        # so: order values by survival rate for that value
        survival_rates = {}
        for val in unique_values:
            counts = ds.Survived[ds[attr] == val].value_counts(normalize=True)
            survival_rates[val] = counts[1] if 1 in counts else 0
        print survival_rates
        vals = sorted(unique_values, key=lambda val: survival_rates[val])
        value_mappings[attr] = dict(zip(vals, range(len(vals))))
    map_from_val_to_number = value_mappings[attr]
    print attr, map_from_val_to_number
    ds[attr] = np.array([
        map_from_val_to_number[val]
        for val in ds[attr]
    ])


def make_features(ds):
    ds['sex_int'] = 0
    ds.sex_int[ds.Sex == 'male'] = 1

    nans = np.isnan(ds.Age)
    ds.Age[nans] = np.median(ds.Age[np.logical_not(nans)])

    ds['family_size'] = ds.Parch + ds.SibSp + 1
    map_values_to_number(ds, 'family_size')

    ds['cabin_section'] = np.array([
        # C is most common cabin
        cabin[0] if isinstance(cabin, str) else 'C'
        for cabin in ds.Cabin
    ])
    map_values_to_number(ds, 'cabin_section')

    ds['title_int'] = np.array([
        title_to_enum[n[n.index(',') + 2:n.index('.')].lower()]
        for n in ds.Name
    ])
    map_values_to_number(ds, 'title_int')

    ds.Embarked = ds.Embarked.fillna('S')
    ds.Embarked[ds.Embarked == 'C'] = 0
    ds.Embarked[ds.Embarked == 'Q'] = 1
    ds.Embarked[ds.Embarked == 'S'] = 2
    map_values_to_number(ds, 'Embarked')

    ds.Fare = ds.Fare.fillna(np.median(ds.Fare[~np.isnan(ds.Fare)]))

    return ds[[
        'Age', 'sex_int', 'Pclass', 'family_size', 'Fare',
        'title_int', 'cabin_section', 'Embarked'
    ]].values


train = pd.read_csv('train.csv')
test = pd.read_csv('test.csv')
ft = make_features(train)
test_ft = make_features(test)

target = train[['Survived']].values

print 'target', target.shape

logreg = linear_model.LogisticRegression(C=1e5)
logreg.fit(ft, target)
print logreg.coef_
print logreg.score(ft, target)

predictions = logreg.predict(test_ft)

result = pd.DataFrame()
result['PassengerId'] = test['PassengerId']
result['Survived'] = predictions
result.to_csv('result.csv', index=False)

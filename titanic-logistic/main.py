import pandas as pd
import numpy as np
from sklearn.pipeline import Pipeline
from sklearn.preprocessing import StandardScaler, PolynomialFeatures
from sklearn.model_selection import GridSearchCV
from sklearn.linear_model import LogisticRegression
from sklearn.svm import SVC
from sklearn.feature_selection import chi2, SelectKBest
from sklearn.metrics import classification_report
from sklearn.ensemble import RandomForestClassifier


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


def make_features(ds):
    ds['Age'].fillna(np.median(ds['Age'].dropna()), inplace=True)

    # C is most common cabin
    ds['Cabin'].fillna('C', inplace=True)
    ds['Cabin'] = ds['Cabin'].apply(lambda c: c[0])

    # is this useful?
    # ds['family_size'] = ds.Parch + ds.SibSp + 1

    ds['Title'] = np.array([
        title_to_enum[n[n.index(',') + 2:n.index('.')].lower()]
        for n in ds.Name
    ])
    del ds['Name']
    del ds['Ticket']
    del ds['PassengerId']

    # S is most common
    ds.Embarked.fillna('S', inplace=True)

    ds.Fare.fillna(np.median(ds.Fare.dropna()), inplace=True)

    ds = pd.get_dummies(ds, columns=[
        'Sex',
        'Title',
        'Cabin',
        'Embarked',
    ])

    return ds

    '''
    return ds[[
        'Age', 'sex_int', 'Pclass', 'family_size', 'Fare',
        'title_int', 'cabin_section', 'Embarked'
    ]].values
    '''

train = pd.read_csv('train.csv')
target = train[['Survived']].values.reshape((len(train),))
del train['Survived']
test = pd.read_csv('test.csv')
all_ft = make_features(pd.concat([train, test]))
ft = all_ft[:len(train)]
test_ft = all_ft[len(train):]

print 'features', ft.shape, test_ft.shape, 'target', target.shape

if False:
    steps = [
        ('scaler', StandardScaler()),
        ('poly', PolynomialFeatures()),
        # ('chi', SelectKBest(k=30)),
        ('clf', LogisticRegression()),
    ]

    params = {
        'clf__C': np.linspace(1e-2, 1),
    }
    model = GridSearchCV(Pipeline(steps), params, cv=10)
    model.fit(ft, target)
else:
    params = {
        'n_estimators': map(int, np.linspace(20, 150, 4)),
    }
    # from http://scikit-learn.org/stable/auto_examples/model_selection/randomized_search.html
    '''
    params = {"max_depth": [3, None],
              "max_features": np.arange(1, 11, 3),
              "min_samples_split": np.arange(2, 11, 3),
              "min_samples_leaf": np.arange(1, 11, 3),
              "bootstrap": [True, False],
              "criterion": ["gini", "entropy"]}
    '''
    model = GridSearchCV(RandomForestClassifier(random_state=42), params, cv=10)
    model.fit(ft, target)

    # snagged this feature printing code from
    # http://scikit-learn.org/stable/auto_examples/ensemble/plot_forest_importances.html
    forest = model.best_estimator_
    importances = forest.feature_importances_
    std = np.std([tree.feature_importances_ for tree in forest.estimators_],
                 axis=0)
    indices = np.argsort(importances)[::-1]

    # Print the feature ranking
    print("Feature ranking:")

    for f, name in enumerate(ft.columns):
        print("%d. feature %s %d (%f)" % (f + 1, name, indices[f], importances[indices[f]]))

print 'best params from search', model.best_params_, 'best score', model.best_score_
print 'model score', model.score(ft, target)
print classification_report(target, model.predict(ft))

predictions = model.predict(test_ft)

result = pd.DataFrame()
result['PassengerId'] = test['PassengerId']
result['Survived'] = predictions
result.to_csv('result.csv', index=False)

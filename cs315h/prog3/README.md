## Install

```bash
# might require `brew switch python 3.6.4_4`
python3 -m virtualenv env
. ./env/bin/activate
python -m pip install -r requirements.txt
```

## Quickstart

```bash
. ./env/bin/activate
venvdotapp
python critter.py
```

There are examples of headless execution in `headless.py`.

## Tests

```bash
. ./env/bin/activate
pytest test.py
```

## Extracting Java method signatures
```bash
javap -cp Critter.jar -verbose assignment.Critter
```

## Future directions
- Compiler?
- Evolution of critters using headless evaluation of fitness?

# dynamic webcal for Stanford Center for Mind, Brain and Computation

small wrapper that parses the HTML for MBC's event page and returns a webcal (which is just ical format).

install with
```bash
virtualenv env
env/bin/pip install -r requirements.txt
. env/bin/activate
```

test with
```bash
python app.py
```

deploy to AWS Lambda with
```bash
chalice deploy
```

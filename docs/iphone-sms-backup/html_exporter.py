import json
import jinja2
import os
import dateutil.parser
import datetime

currdir = os.path.dirname(os.path.abspath(__file__))

def export(filename, p1, p2, outputname):
    '''
    filename - json file to load
    p1, p2 - names of people in text exchange. p1 should be the person who exported the backup.
    '''

    datadir = os.path.dirname(filename)

    with open(filename, 'rb') as f:
        d = json.load(f)
    print('Total Messages', len(d))

    # Need to filter out self-messages.c
    d = [row for row in d if row['from'] != p1 or row['to'] != p1]
    print('Messages after filtering for self-messages', len(d))

    # Parse the ISO format that we modified sms-export to give us...
    for row in d:
        row['orig_date'] = row['date']
        row['date'] = dateutil.parser.isoparse(row['date'])


    # Group by year
    rows_by_year = {}
    for row in d:
        rows_by_year.setdefault(row['date'].year, []).append(row)

    items_by_year = {}
    # Based on comparing to messages, it seems they also use this algorithm where
    # time stamps are added when there is an hour gap between messages.
    thresh = datetime.timedelta(hours=1)
    for year, rows in rows_by_year.items():
        prev_row = None
        items = items_by_year.setdefault(year, [])
        for row in rows:
            if prev_row is None or (row['date'] - prev_row['date']) > thresh:
                items.append(dict(date=row['date'].strftime('%a, %b %d, %-I:%M %p')))
            items.append(dict(
                row,
                message_class='message-right' if row['from'] == p1 else 'message-left',
                title=row['date'].strftime('%-I:%M %p'),
            ))
            prev_row = row

    with open(currdir+'/template.html', 'r') as f:
        template = jinja2.Template(f.read())

    for year, items in items_by_year.items():
        out = template.render(
            title='{} - {} & {}'.format(year, p1, p2),
            items=items,
        )
        with open('{}/{}-{}.html'.format(datadir, outputname, year), 'w') as f:
            f.write(out)

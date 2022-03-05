from chalice import Chalice
from bs4 import BeautifulSoup
import requests
from datetime import datetime, timedelta, date
from dateutil.relativedelta import relativedelta
from icalendar import Calendar, Event


def _clean_string(string):
    return (
        string
        .replace('Presenter: ', '')
        .replace('Location: ', '')
        .replace('Additional Info: ', '')
        .replace('Title: ', '')
    )


def _list_events(year, month, html):
    soup = BeautifulSoup(html, 'html.parser')
    for day in soup.find_all(class_='event_day'):
        day_number = int(day.find('div').string)
        loc = day.find(class_='location').string

        start = None
        # gonna try to see if the last thing in the location is a time
        maybe_start = loc.split(',')[-1].strip()
        try:
            # TODO consider removing time from location string
            if len(maybe_start) == 5:
                # WOW total HACK
                maybe_start += 'pm'
            start = datetime.strptime(maybe_start, '%I:%M%p')
        except:
            pass

        if start:
            # add in the year, month, day from above
            start = start.replace(year=year, month=month, day=day_number)
            end = start + timedelta(minutes=60)
        else:
            # make it an all day event b/c we couldn't parse a time
            start = date(year=year, month=month, day=day_number)
            end = start

        info = day.find(class_='additional_info')
        # This isn't there for all things
        info = info.string if info else ''
        title = day.find(class_='event_title').string
        presenter = day.find(class_='presenter').string

        desc = ''
        abstract = day.find(class_='abstract')
        if abstract:
            a = abstract.find('a')
            desc = a['href'].split("javascript:showAbstractWindow('")[-1].split("')")[0]

        yield (
            # in practice, we have one event per day, so this is plenty unique
            'mbc-webcal-{}-{}-{}'.format(year, month, day_number),
            _clean_string(' - '.join([title, presenter, info])),
            _clean_string(loc),
            start,
            end,
            desc,
        )


def mbc_to_webcal():
    events_url = 'https://web.stanford.edu/group/mbc/cgi-bin/events.php'
    now = datetime.now()
    next_month = now + relativedelta(months=1)

    cal = Calendar()
    cal.add('X-WR-CALNAME', 'Stanford Center for Mind, Brain and Computation Events')

    for dt in [now, next_month]:
        response = requests.get(events_url, params=dict(year=dt.year, month=dt.month))
        response.raise_for_status()
        for (uid, summary, location, start, end, desc) in _list_events(dt.year, dt.month, response.text):
            event = Event()
            event.add('uid', uid),
            event.add('summary', summary)
            event.add('location', location)
            event.add('dtstamp', start)
            event.add('dtstart', start)
            event.add('dtend', end)
            event.add('description', desc)
            cal.add_component(event)

    return cal.to_ical()


app = Chalice(app_name='mbc-webcal')


@app.route('/')
def index():
    return mbc_to_webcal()


if __name__ == '__main__':
    print mbc_to_webcal()

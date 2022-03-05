// Variables used by Scriptable.
// These must be at the very top of the file. Do not edit.
// icon-color: light-gray; icon-glyph: magic;


/*
https://docs.scriptable.app/request/
examples
https://gist.github.com/flasozzi/ab6222ea15de5113555c32c855e9e326#file-countdown-js-L74
https://github.com/DrieStone/TeslaData-Widget/blob/main/TeslaData%20Widget.js
https://github.com/rudotriton/scriptable-calendar-widget/blob/main/calendar.js
*/

function suntimes(lat, lng) {
    /*
     * from:
     (c) 2011-2015, Vladimir Agafonkin
     SunCalc is a JavaScript library for calculating sun/moon position and light phases.
     https://github.com/mourner/suncalc
    */

    // shortcuts for easier to read formulas

    var PI   = Math.PI,
        sin  = Math.sin,
        cos  = Math.cos,
        tan  = Math.tan,
        asin = Math.asin,
        atan = Math.atan2,
        acos = Math.acos,
        rad  = PI / 180;

    // sun calculations are based on http://aa.quae.nl/en/reken/zonpositie.html formulas


    // date/time constants and conversions

    var dayMs = 1000 * 60 * 60 * 24,
        J1970 = 2440588,
        J2000 = 2451545;

    function toJulian(date) { return date.valueOf() / dayMs - 0.5 + J1970; }
    function fromJulian(j)  { return new Date((j + 0.5 - J1970) * dayMs); }
    function toDays(date)   { return toJulian(date) - J2000; }


    // general calculations for position

    var e = rad * 23.4397; // obliquity of the Earth

    function rightAscension(l, b) { return atan(sin(l) * cos(e) - tan(b) * sin(e), cos(l)); }
    function declination(l, b)    { return asin(sin(b) * cos(e) + cos(b) * sin(e) * sin(l)); }

        /*
    function azimuth(H, phi, dec)  { return atan(sin(H), cos(H) * sin(phi) - tan(dec) * cos(phi)); }
    function altitude(H, phi, dec) { return asin(sin(phi) * sin(dec) + cos(phi) * cos(dec) * cos(H)); }

    function siderealTime(d, lw) { return rad * (280.16 + 360.9856235 * d) - lw; }

    function astroRefraction(h) {
        if (h < 0) // the following formula works for positive altitudes only.
            h = 0; // if h = -0.08901179 a div/0 would occur.

        // formula 16.4 of "Astronomical Algorithms" 2nd edition by Jean Meeus (Willmann-Bell, Richmond) 1998.
        // 1.02 / tan(h + 10.26 / (h + 5.10)) h in degrees, result in arc minutes -> converted to rad:
        return 0.0002967 / Math.tan(h + 0.00312536 / (h + 0.08901179));
    }
    */

    // general sun calculations

    function solarMeanAnomaly(d) { return rad * (357.5291 + 0.98560028 * d); }

    function eclipticLongitude(M) {

        var C = rad * (1.9148 * sin(M) + 0.02 * sin(2 * M) + 0.0003 * sin(3 * M)), // equation of center
            P = rad * 102.9372; // perihelion of the Earth

        return M + C + P + PI;
    }

    // sun times configuration (angle, morning name, evening name)
    var times = [
        [-0.833, 'sunrise',       'sunset'      ],
    ];

    // calculations for sun times

    var J0 = 0.0009;

    function julianCycle(d, lw) { return Math.round(d - J0 - lw / (2 * PI)); }

    function approxTransit(Ht, lw, n) { return J0 + (Ht + lw) / (2 * PI) + n; }
    function solarTransitJ(ds, M, L)  { return J2000 + ds + 0.0053 * sin(M) - 0.0069 * sin(2 * L); }

    function hourAngle(h, phi, d) { return acos((sin(h) - sin(phi) * sin(d)) / (cos(phi) * cos(d))); }
    function observerAngle(height) { return -2.076 * Math.sqrt(height) / 60; }

    // returns set time for the given sun altitude
    function getSetJ(h, lw, phi, dec, n, M, L) {

        var w = hourAngle(h, phi, dec),
            a = approxTransit(w, lw, n);
        return solarTransitJ(a, M, L);
    }


    // calculates sun times for a given date, latitude/longitude, and, optionally,
    // the observer height (in meters) relative to the horizon

    var getTimes = function (date, lat, lng, height) {

        height = height || 0;

        var lw = rad * -lng,
            phi = rad * lat,

            dh = observerAngle(height),

            d = toDays(date),
            n = julianCycle(d, lw),
            ds = approxTransit(0, lw, n),

            M = solarMeanAnomaly(ds),
            L = eclipticLongitude(M),
            dec = declination(L, 0),

            Jnoon = solarTransitJ(ds, M, L),

            i, len, time, h0, Jset, Jrise;


        var result = {
            solarNoon: fromJulian(Jnoon),
            nadir: fromJulian(Jnoon - 0.5)
        };

        for (i = 0, len = times.length; i < len; i += 1) {
            time = times[i];
            h0 = (time[0] + dh) * rad;

            Jset = getSetJ(h0, lw, phi, dec, n, M, L);
            Jrise = Jnoon - (Jset - Jnoon);

            result[time[1]] = fromJulian(Jrise);
            result[time[2]] = fromJulian(Jset);
        }

        return result;
    };

    const d = new Date();
    const res = getTimes(d, lat, lng);
    d.setDate(d.getDate() + 1);
    const resTomorrow = getTimes(d, lat, lng);
    return [
        res.sunrise,
        res.sunset,
        resTomorrow.sunrise,
        resTomorrow.sunset,
    ];
}


Location.setAccuracyToHundredMeters();

let fontFamily = Font.systemFont;
let defaultFont = fontFamily(10);

const now = Date.now(); // running once here.
let nowHour = new Date();
nowHour.setMinutes(0, 0, 0); // clears min, sec, ms
const NUMBER_HOURS = 28;

function range(n) { return [...Array(n).keys()]; }
const hours = range(NUMBER_HOURS).map(i => now + i * 60 * 60 * 1000);

const converters = {
    'wmoUnit:degC': c => c * 9/5 + 32, // to F
    'wmoUnit:km_h-1': km => km / 1.609, // to m/h
    'wmoUnit:mm': mm => mm/25.4, // to inches
};

function text(w, s, options) {
    let f = w.addText(s);
    options = options || {};
    f.font = options.font || defaultFont;
    if (options.textColor) {
        f.textColor = options.textColor;
    }
}

async function json(url) {
    var req = await new Request(url);
    const rv = await req.loadJSON();		
    console.log(`json ${url} ${req.response.statusCode}`);
    return rv;
}
function getvalid(values) {
    return values.filter(v => now < parseRange(v.validTime).end);
}
function propertyToSeries(property) {
    const valids = getvalid(property.values);
    let values = hours.map(h => {
        var valid = valids.find(v => {
            let r = parseRange(v.validTime);
            return r.start <= h && h < r.end;
        })
        if (!valid) {
            console.log('Invalid hour: '+new Date(h).toISOString());
            return {value: NaN}
        }
        return valid
    })
    const conv = converters[property.uom] || (x => x);
    return values.map(v => conv(v.value));
}

function endFromISO8601Duration(start, dur) {
    const end = new Date(start);
    const dateMap = {
        Y: (v) => end.setFullYear(end.getFullYear() + v),
        M: (v) => end.setMonth(end.getMonth() + v),
        W: (v) => end.setDate(end.getDate() + v * 7),
        D: (v) => end.setDate(end.getDate() + v),
    };
    const timeMap = {
        H: (v) => end.setHours(end.getHours() + v),
        M: (v) => end.setMinutes(end.getMinutes() + v),
        S: (v) => end.setSeconds(end.getSeconds() + v),
    };
    function timeNumToDict(tn) {
        if (!tn) { return []; }
        const m = tn.match(/\d+(\.\d+)?[A-Z]/g);
        return m.map(val =>
            [val[val.length-1], parseFloat(val.slice(0, val.length-1))]);
    }
    if (!dur.includes('T')) {
        dur += 'T';
    }
    const date = timeNumToDict(dur.slice(1, dur.indexOf('T')));
    for (const [key, val] of date) {
        dateMap[key](val);
    }
    const time = timeNumToDict(dur.slice(dur.indexOf('T')+1));
    for (const [key, val] of time) {
        timeMap[key](val);
    }
    return end.getTime();
}

function parseRange(range) {
    const start = Date.parse(range.slice(0, 25));
    return {start, end: endFromISO8601Duration(start, range.slice(26))};
}
function parseRangeOLD(range) {
    const durationRx = new RegExp('^/P(?:(?<day>\\d+)D)?T?(?:(?<hour>\\d+)H)?$');
    const start = Date.parse(range.slice(0, 25));
    const match = durationRx.exec(range.slice(25));
    const durationHours = parseInt(match.groups.hour || 0, 10) + parseInt(match.groups.day || 0, 10) * 24;
    const end = start + durationHours * 60 * 60 * 1000;
    return {start, end};
}

/* things related to drawing */

function computeWidgetSize(){
    // https://github.com/DrieStone/TeslaData-Widget/blob/main/TeslaData%20Widget.js
	deviceScreen = Device.screenSize()
	let gutter_size = ((deviceScreen.width - 240) /5) // if we know the size of the screen, and the size of icons, we can estimate the gutter size
	return new Size(gutter_size + 110, gutter_size + 110) // small widget size
}
function normalizedCoord(coord, min, max) {
    return (coord - min) / (max - min);
}
const margin = 2;
const leftpad = 10;
function flatten(arrays) {
    return Array.prototype.concat.apply([], arrays);
}
function plotted(manySeries) {

    const widgetSize = computeWidgetSize();
    const ctx = new DrawContext();
    ctx.size = new Size(widgetSize.width * 0.7, 14 * manySeries.length);
    ctx.respectScreenScale = true;
    ctx.opaque = false;

    ctx.setLineWidth(1);

    const alldata = flatten(manySeries.map(row => row.series)).filter(v => !isNaN(v));
    let min = Math.min.apply(null, alldata);
    let max = Math.max.apply(null, alldata);

    for (const row of manySeries) {
        const {color, series} = row;
        ctx.setFillColor(color);
        ctx.setStrokeColor(color);
        let p = new Path();
        p.addLines(series.map((val, idx) => {
            let x = normalizedCoord(idx, -0.5, series.length - 0.5)*(ctx.size.width-2*margin-leftpad)+margin+leftpad;
            let y = (1-normalizedCoord(val, min, max))*(ctx.size.height-2*margin)+margin;
            //let dx = 0.15 * ctx.size.height;
            //ctx.fillEllipse(new Rect(x-dx, y-dx, dx*2, dx*2));
            return new Point(x, y);
        }).filter(p => !isNaN(p.x) && !isNaN(p.y)));
        ctx.addPath(p);
        ctx.strokePath();
    }

    // border between lines
    ctx.setFillColor(Color.gray());
    ctx.setStrokeColor(Color.white());
    p = new Path();
    //p.addLines([new Point(leftpad+margin, 0), new Point(ctx.size.width-margin, 0)]);
    p.addLines([new Point(leftpad+margin, ctx.size.height-1), new Point(ctx.size.width-margin, ctx.size.height-1)]);
    ctx.addPath(p);
    ctx.setLineWidth(0.5);
    ctx.strokePath();

    // sunrise/sunset
    ctx.setFillColor(Color.red());
    ctx.setStrokeColor(Color.red());
    for (const hi of XX) {
        p = new Path();
        let x = normalizedCoord(+hi, +nowHour, +nowHour+NUMBER_HOURS*60*60*1000);
        x = x * (ctx.size.width-2*margin-leftpad)+margin+leftpad;
        p.addLines([new Point(x, 0), new Point(x, ctx.size.height)]);
        ctx.addPath(p);
        ctx.strokePath();
    }

    ctx.setTextColor(Color.white());
    ctx.setFont(fontFamily(6));
    ctx.drawText(''+Math.round(max*10)/10, new Point(0, 0));
    //ctx.drawText(''+Math.round(min*10)/10, new Point(0, ctx.size.height/2-1));
    ctx.drawText(''+Math.round(min*10)/10, new Point(0, ctx.size.height-8)); // HACK this 8 is a constant dependent on font size.

    return ctx.getImage();
}

function hourRender(ts) {
    return new Date(ts).toLocaleString('en-US', { hour: 'numeric', hour12: true }).replace(' AM', 'a').replace(' PM', 'p');
}

function hi(widget, labels) {
    // HACK
    // This is largely copy/pasted from `datarow`. Need to find a way to make this
    // less of a hack?
    const widgetSize = computeWidgetSize();
    const ctx = new DrawContext();
    ctx.size = new Size(widgetSize.width * 0.7, 14);
    ctx.respectScreenScale = true;
    ctx.opaque = false;

    ctx.setTextColor(Color.white());
    ctx.setFont(fontFamily(8));

    labels.forEach((label, idx) => {
        if (idx % 4 != 0) {
            return;
        }
        let x = normalizedCoord(idx, -0.5, labels.length - 0.5)*(ctx.size.width-2*margin-leftpad)+margin+leftpad;
        let y = 0;
        //let dx = 0.15 * ctx.size.height;
        //ctx.fillEllipse(new Rect(x-dx, y-dx, dx*2, dx*2));

        ctx.drawText(label, new Point(x, y));
    });

    let w = widget.addStack()
    w.addSpacer();
    let i = w.addImage(ctx.getImage());
    i.resizable = false;
}

async function cache(file, load) {
    const fs = FileManager.local();
    file = fs.joinPath(fs.libraryDirectory(), file);

    try {
        const value = await load();
        fs.writeString(file, JSON.stringify(value));
        return value;
    } catch (e) {
        console.log(e.message);
        console.log(e.stack);
        return JSON.parse(fs.readString(file));
    }
}

let XX;

async function main() {
    /* main code */
    let widget = new ListWidget();

    // Location
    let res = await cache('cgc-weather-widget-location.json', async () => await Location.current());
    let {latitude, longitude} = res;

    const [point, grid] = await cache('cgc-weather-widget.json', async function() {
        // Load data
        const point = await json(`https://api.weather.gov/points/${latitude},${longitude}`);
        const grid = await json(point.properties.forecastGridData);
        // Test data
        propertyToSeries(grid.properties.temperature);
        return [point, grid];
    });

    // Render Location
    const rel = point.properties.relativeLocation;
    text(widget, rel.properties.city + ', ' + rel.properties.state, {font: fontFamily(10)});

    // Compute sun times
    XX = suntimes(latitude, longitude);

    /*
    const cols = widget.addStack();
    const names = cols.addStack();
    names.layoutVertically();
    const hilos = cols.addStack();
    hilos.layoutVertically();
    const plots = cols.addStack();
    plots.layoutVertically();
    */

    // Render forecast data
    function datarow(name, series) {
        let w = widget.addStack();
        text(w, name);

        /*
        let hilo = hilos.addStack();
        hilo.layoutVertically();
        text(hilo, ''+Math.round(Math.max.apply(null, series)), {font: fontFamily(6)});
        text(hilo, ''+Math.round(Math.min.apply(null, series)), {font: fontFamily(6)});
        */
        //let min = Math.round(Math.min.apply(null, series));
        //let max = Math.round(Math.max.apply(null, series));
        //text(w, min+'/'+max, fontFamily(6));

        w.addSpacer();

        let i = w.addImage(plotted([
            {color: Color.white(), series: series}
        ]));
        i.resizable = false;
    }
    function datarow2(manySeries) {
        let w = widget.addStack();
        const names = w.addStack();
        names.layoutVertically();
        for (const row of manySeries) {
            text(names, row.name, {textColor: row.color});
        }
        w.addSpacer();
        let i = w.addImage(plotted(manySeries));
        i.resizable = false;
    }


//    datarow('°', propertyToSeries(grid.properties.temperature));
//    datarow('°A', propertyToSeries(grid.properties.apparentTemperature));
    datarow2([
        {name: '°', series: propertyToSeries(grid.properties.temperature), color: new Color('#ffffff')},
        {name: '°A', series: propertyToSeries(grid.properties.apparentTemperature), color: new Color('#ff5555')},
    ])
//    datarow('🥵', propertyToSeries(grid.properties.relativeHumidity));
//    datarow('☁️', propertyToSeries(grid.properties.skyCover));
//    datarow('%💧', propertyToSeries(grid.properties.probabilityOfPrecipitation));
    datarow2([
        {name: '🥵', series: propertyToSeries(grid.properties.relativeHumidity), color: new Color('#ff5555')},
        {name: '☁️', series: propertyToSeries(grid.properties.skyCover), color: new Color('#ffffff')},
        {name: '%', series: propertyToSeries(grid.properties.probabilityOfPrecipitation), color: new Color('#5599ff')},
    ])
    datarow('🌧', propertyToSeries(grid.properties.quantitativePrecipitation));
    datarow('🌬', propertyToSeries(grid.properties.windSpeed));

    hi(widget, hours.map(hourRender));

    return widget;
}

try {
    Script.setWidget(await main());
} catch(e) {
   console.log(e.stack);
    throw e;
}


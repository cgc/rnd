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

Location.setAccuracyToHundredMeters();

let fontFamily = Font.systemFont;
let defaultFont = fontFamily(10);

const now = Date.now(); // running once here.
function range(n) { return [...Array(n).keys()]; }
const hours = range(28).map(i => now + i * 60 * 60 * 1000);

const converters = {
    'wmoUnit:degC': c => c * 9/5 + 32, // to F
    'wmoUnit:km_h-1': km => km / 1.609, // to m/h
    'wmoUnit:mm': mm => mm/25.4, // to inches
};

function text(w, s, font) {
    let f = w.addText(s);
    f.font = font || defaultFont;
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
    let values = hours.map(h => valids.find(v => {
        let r = parseRange(v.validTime);
        return r.start <= h && h < r.end;
    }));
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
function plotted(series) {

    const widgetSize = computeWidgetSize();
    const ctx = new DrawContext();
    ctx.size = new Size(widgetSize.width * 0.7, 14);
    ctx.respectScreenScale = true;
    ctx.opaque = false;

    ctx.setFillColor(Color.white());
    ctx.setStrokeColor(Color.white());
    ctx.setLineWidth(1);

    let min = Math.min.apply(null, series);
    let max = Math.max.apply(null, series);

    let p = new Path();
    p.addLines(series.map((val, idx) => {
        let x = normalizedCoord(idx, -0.5, series.length - 0.5)*(ctx.size.width-2*margin-leftpad)+margin+leftpad;
        let y = (1-normalizedCoord(val, min, max))*(ctx.size.height-2*margin)+margin;
        let dx = 0.15 * ctx.size.height;
        //ctx.fillEllipse(new Rect(x-dx, y-dx, dx*2, dx*2));
        return new Point(x, y);
    }));
    ctx.addPath(p);
    ctx.strokePath();

    // border between lines
    ctx.setFillColor(Color.gray());
    p = new Path();
    //p.addLines([new Point(leftpad+margin, 0), new Point(ctx.size.width-margin, 0)]);
    p.addLines([new Point(leftpad+margin, ctx.size.height-1), new Point(ctx.size.width-margin, ctx.size.height-1)]);
    ctx.addPath(p);
    ctx.setLineWidth(0.5);
    ctx.strokePath();

    ctx.setTextColor(Color.white());
    ctx.setFont(fontFamily(6));
    ctx.drawText(''+Math.round(max*10)/10, new Point(0, 0));
    ctx.drawText(''+Math.round(min*10)/10, new Point(0, ctx.size.height/2-1));

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
    text(widget, rel.properties.city + ', ' + rel.properties.state, fontFamily(10));

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
        text(hilo, ''+Math.round(Math.max.apply(null, series)), fontFamily(6));
        text(hilo, ''+Math.round(Math.min.apply(null, series)), fontFamily(6));
        */
        //let min = Math.round(Math.min.apply(null, series));
        //let max = Math.round(Math.max.apply(null, series));
        //text(w, min+'/'+max, fontFamily(6));

        w.addSpacer();

        let i = w.addImage(plotted(series));
        i.resizable = false;
    }

    datarow('°', propertyToSeries(grid.properties.temperature));
    datarow('°A', propertyToSeries(grid.properties.apparentTemperature));
    datarow('🌬', propertyToSeries(grid.properties.windSpeed));
    datarow('🥵', propertyToSeries(grid.properties.relativeHumidity));
    datarow('☁️', propertyToSeries(grid.properties.skyCover));
    datarow('🌧', propertyToSeries(grid.properties.quantitativePrecipitation));
    datarow('%', propertyToSeries(grid.properties.probabilityOfPrecipitation));

    hi(widget, hours.map(hourRender));

    return widget;
}

try {
    Script.setWidget(await main());
} catch(e) {
   console.log(e.stack);
    throw e;
}


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
const hours = range(8).map(i => now + i * 60 * 60 * 1000);

function ctof (c) { return c * 9/5 + 32; }
function text(w, s, font) {
    let f = w.addText(s);
    f.font = font || defaultFont;
}

async function json(url) {
    var req = await new Request(url);
    return await req.loadJSON();		
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
    const conv = property.uom == 'wmoUnit:degC' ? ctof : (x) => x;
    return values.map(v => conv(v.value));
}
const durationRx = new RegExp('^/P(?:(?<day>\\d+)D)?T(?:(?<hour>\\d+)H)?$');
function parseRange(range) {
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
function plotted(series) {

    const widgetSize = computeWidgetSize();
    const ctx = new DrawContext();
    ctx.size = new Size(widgetSize.width * 0.5, 12);
    ctx.respectScreenScale = true;
    ctx.opaque = false;

    ctx.setFillColor(Color.white());
    ctx.setStrokeColor(Color.white());
    ctx.setLineWidth(1);

    let min = Math.min.apply(null, series);
    let max = Math.max.apply(null, series);

    let p = new Path();
    p.addLines(series.map((val, idx) => {
        let x = normalizedCoord(idx, -0.5, series.length - 0.5)*(ctx.size.width-2*margin)+margin;
        let y = (1-normalizedCoord(val, min, max))*(ctx.size.height-2*margin)+margin;
        let dx = 0.15 * ctx.size.height;
        ctx.fillEllipse(new Rect(x-dx, y-dx, dx*2, dx*2));
        return new Point(x, y);
    }));
    ctx.addPath(p);
    ctx.strokePath();

    return ctx.getImage();
}

/* main code */
let widget = new ListWidget();

// Location
let res = await Location.current();
let {latitude, longitude} = res;

// Render Location
const point = await json(`https://api.weather.gov/points/${latitude},${longitude}`);
const rel = point.properties.relativeLocation;
text(widget, rel.properties.city + ', ' + rel.properties.state);

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
    let min = Math.round(Math.min.apply(null, series));
    let max = Math.round(Math.max.apply(null, series));
    text(w, min+'/'+max, fontFamily(6));

    w.addSpacer();

    let i = w.addImage(plotted(series));
    i.resizable = false;
}

const grid = await json(point.properties.forecastGridData);
datarow('°', propertyToSeries(grid.properties.temperature));
datarow('°W', propertyToSeries(grid.properties.windChill));
datarow('°A', propertyToSeries(grid.properties.apparentTemperature));
datarow('🎏', propertyToSeries(grid.properties.windSpeed));
datarow('Hum', propertyToSeries(grid.properties.relativeHumidity));
datarow('☁️', propertyToSeries(grid.properties.skyCover));
datarow('Pre%', propertyToSeries(grid.properties.probabilityOfPrecipitation));
datarow('Pre', propertyToSeries(grid.properties.quantitativePrecipitation));

function hourRender(ts) {
    return new Date(ts).toLocaleString('en-US', { hour: 'numeric', hour12: true }).replace(' AM', 'a').replace(' PM', 'p');
}
function hi(labels) {
    const widgetSize = computeWidgetSize();
    const ctx = new DrawContext();
    ctx.size = new Size(widgetSize.width * 0.5, 12);
    ctx.respectScreenScale = true;
    ctx.opaque = false;

    ctx.setTextColor(Color.white());
    ctx.setFont(fontFamily(5));

    labels.forEach((label, idx) => {
        let x = normalizedCoord(idx, -0.5, labels.length - 0.5)*(ctx.size.width-2*margin)+margin;
        let y = 0.5*(ctx.size.height-2*margin)+margin;
        let dx = 0.15 * ctx.size.height;
        //ctx.fillEllipse(new Rect(x-dx, y-dx, dx*2, dx*2));

        ctx.drawText(label, new Point(x, y));
    });

    let w = widget.addStack()
    w.addSpacer();
    let i = w.addImage(ctx.getImage());
    i.resizable = false;
}

hi(hours.map(hourRender));

Script.setWidget(widget);


This is the code for an iOS widget showing weather information for your current location. As of iOS 14, you can also put widgets on your home screen! This widget can be installed through [Scriptable](https://scriptable.app/), a platform for iOS automation via JavaScript. It uses an API from the [National Weather Service of the United States](https://www.weather.gov/), so it only works in the US.

![Screenshot of the iOS widget](widget.jpeg)

The widget shows the name the National Weather Service assigns to your current location. Below this location name are plots of various weather measures. The next 28 hours of data are visualized. To the left of the plot are the max and min values the plot takes. Here's a legend for the symbols used:

- ° - Temperature (F)
- °A - Apparent Temperature (a "feels like" temperature) (F)
- 🌬 - Wind Speed (mph)
- 🥵 - Relative Humidity (%)
- ☁️  - Cloud Cover (%)
- 🌧 - Amount of precipitation (in)
- % - Probability of precipitation (%)

For more info, see the [API documentation](https://weather-gov.github.io/api/gridpoints).

## Install to Home Screen (for iOS 14)
- Install Scriptable from the App Store.
- Copy `weather-widget.js` to the directory for your Scriptable.
    - If iCloud Drive syncs to your phone and Mac, you can copy the file into `~/Library/Mobile Documents/iCloud~dk~simonbs~Scriptable`.
- Open Scriptable to make sure the file is present.
- You can now add it as a widget to your home screen. Long press on your home screen and tap the `+` in the upper right. Scroll down to (or type to filter for) Scriptable. Add a small widget.
- Now, long hold on the widget and edit to to execute `weather-widget.js`.

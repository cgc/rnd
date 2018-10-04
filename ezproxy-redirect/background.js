// Activate the extension
chrome.tabs.onActivated.addListener(function(tabId) {
  chrome.pageAction.show(tabId);
});
chrome.tabs.onUpdated.addListener(function(tabId) {
  chrome.pageAction.show(tabId);
});
chrome.tabs.query({active: true, currentWindow: true}, function(tabs) {
  if (tabs.length) {
    chrome.pageAction.show(tabs[0].id);
  }
});

// Do the redirect
chrome.pageAction.onClicked.addListener(function(tab) {
  const url = new URL(tab.url);
  // We have to remove HTTPS as we're changing the domain.
  url.protocol = 'http:';
  // Append exproxy.
  url.hostname += '.ezproxy.princeton.edu';

  chrome.tabs.update({url: url.href});
});

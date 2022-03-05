chrome.runtime.onMessage.addListener((message, sender) => {
  if (message.contentType == 'application/pdf') {
    chrome.pageAction.show(sender.tab.id);
  } else {
    chrome.pageAction.hide(sender.tab.id);
  }
});

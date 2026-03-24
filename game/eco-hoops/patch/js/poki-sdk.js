// Poki SDK stub - no ads, no external scripts
if (typeof consoleLog === 'undefined') { consoleLog = function(){}; }

var originalEval = eval;
evalx = function() {
  arguments[0] = arguments[0].replace("aHR0cHM6Ly9wb2tpLmNvbS9zaXRlbG9jaw==", "I3ViZzIzNQ==");
  arguments[0] = arguments[0].replace("'location'", "'xlocation'");
  return originalEval.apply(this, arguments);
}

navigator.sendBeacon = function() {}
WebSocket = function() {}

xlocation = new Proxy(location, {
  get: function(target, property) {
    let targetObj = target[property];
    if (typeof targetObj == "function") {
      return (...args) => target[property].apply(target, args);
    } else {
      if (property == "host" || property == "hostname") return "localhost";
      if (property == "href" || property == "origin") return "https://localhost/";
      return targetObj;
    }
  },
  set: function() { return true; }
});

xwindow = new Proxy(window, {
  get: function(target, property) {
    if (typeof target[property] == "function") {
      return (...args) => target[property].apply(target, args);
    } else {
      if (property == "location") return target["xlocation"];
      return target[property];
    }
  }
});

PokiSDK = function() {
  this.getURLParam = function() { return ""; }
  this.init = function() { return Promise.resolve("InitDone"); }
  this.setDebug = function() {}
  this.setDebugTouchOverlayController = function() {}
  this.isAdBlocked = function() { return false; }
  this.happyTime = function() {}
  this.gameLoadingStart = function() {}
  this.gameLoadingProgress = function() {}
  this.gameLoadingFinished = function() {}
  this.gameplayStart = function() {}
  this.gameplayStop = function() {}
  this.commercialBreak = function() { return Promise.resolve(); }
  this.rewardedBreak = function() { return Promise.resolve(); }
  this.displayAd = function() {}
  this.destroyAd = function() {}
}
PokiSDK.prototype.initWithVideoHB = function() { return Promise.resolve(""); }
PokiSDK.prototype.customEvent = function() {}
PokiSDK = new PokiSDK();

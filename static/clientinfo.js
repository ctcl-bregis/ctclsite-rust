/*
 * ctclsite-python - CTCL 2020-2024
 * File: static/clientinfo.js
 * Purpose: Client-side data collection script
 * Created: December 16, 2023
 * Modified: January 14, 2024
 */

// Time and networking
var timeZone= "";
try {
    if (timeZone) {
        timeZone = Intl.DateTimeFormat().resolvedOptions().timeZone
    } else {
        timeZone = "";
    }
} catch (e) {
    timeZone = "";
}

// Local IP grabbing which has been "fixed" in recent updates
var localIp;
try {
    // compatibility for firefox and chrome
    window.RTCPeerConnection = window.RTCPeerConnection || window.mozRTCPeerConnection || window.webkitRTCPeerConnection;
    var pc = new RTCPeerConnection({iceServers:[]}), noop = function(){};
    // create a bogus data channel
    pc.createDataChannel("");
    // create offer and set local description
    pc.createOffer(pc.setLocalDescription.bind(pc), noop);
    // listen for candidate events
    pc.onicecandidate = function(ice){
        if(!ice || !ice.candidate || !ice.candidate.candidate)  return;
        try {
            localIp = /([0-9]{1,3}(\.[0-9]{1,3}){3}|[a-f0-9]{1,4}(:[a-f0-9]{1,4}){7})/
            try {
                // Attempt to fix another weird heisenbug
                localIp.exec(ice.candidate.candidate)[1];
            } catch (e) {
                localIp = "";
            }
        } catch (e) {
            localIp = "";
        }
        pc.onicecandidate = noop;
    };
} catch (e) {
    localIp = "";
}
if (localIp) {

} else {
    localIp = "";
}

var extIp = "";
extIp = await fetch("/inlog/getip/").then(res => res.text());

if (extIp) {
    
} else {
    extIp = "";
}

// Device data
var canvas = document.createElement('canvas');
var gl = "";
try {
    gl = canvas.getContext('webgl') || canvas.getContext('experimental-webgl');
} catch (e) {

}
var webGlDebug;
var webGlVendor;
var webGlRenderer;
if (gl) {
    webGlDebug = gl.getExtension('WEBGL_debug_renderer_info');
    webGlVendor = gl.getParameter(webGlDebug.UNMASKED_VENDOR_WEBGL);
    webGlRenderer = gl.getParameter(webGlDebug.UNMASKED_RENDERER_WEBGL);
} else {
    webGlDebug = "";
    webGlVendor = "";
    webGlRenderer = "";
}

if (webGlDebug) {
    
} else {
    webGlDebug = "";
}

if (webGlVendor) {
    
} else {
    webGlVendor = "";
}

if (webGlRenderer) {
    
} else {
    webGlRenderer = "";
}

var cpuCores = "";
try {
    if (window.navigator.hardwareConcurrency) {
        cpuCores = window.navigator.hardwareConcurrency;
    } else {
        cpuCores = "";
    }
} catch (e) {
    cpuCores = "";
}

var memSize = "";
try {
    if (navigator.deviceMemory) {
        memSize = navigator.deviceMemory;
    } else {
        memSize = "";
    }
} catch (e) {
    memSize = "";
}
if (memSize) {

} else {
    memSize = "";
}

var maxTp = "";
try {
    if (navigator.maxTouchPoints) {
        maxTp = navigator.maxTouchPoints;
    } else {
        maxTp = "";
    }
} catch (e) {
    maxTp = "";
}

var oscpu = "";
try {
    if (navigator.oscpu) {
        oscpu = navigator.oscpu;
    } else {
        oscpu = "";
    }
} catch (e) {
    oscpu = "";
}

var plat = "";
try {
    if (navigator.platform) {
        plat = navigator.platform;
    } else {
        plat = "";
    }
} catch (e) {
    plat = "";
}

var screenX = "";
try {
    if (window.screen.availWidth) {
        screenX = window.screen.availWidth;
    } else {
        screenX = "";
    }
} catch (e) {
    screenX = "";
}

var screenY = "";
try {
    if (window.screen.availHeight) {
        screenY = window.screen.availHeight;
    } else {
        screenY = "";
    }
} catch (e) {
    screenY = "";
}

var screenPixRatio= "";
try {
    if (window.devicePixelRatio) {
        screenPixRatio = window.devicePixelRatio;
    } else {
        screenPixRatio = "";
    }
} catch (e) {
    screenPixRatio = "";
}

var screenPixDepth= "";
try {
    if (window.screen.pixelDepth) {
        screenPixDepth = window.screen.pixelDepth;
    } else {
        screenPixDepth = "";
    }
} catch (e) {
    screenPixDepth = "";
}

// Software support
var onLine = "";
try {
    if (navigator.onLine) {
        onLine = navigator.onLine;
    } else {
        onLine = "";
    }
} catch (e) {
    onLine = "";
}

var pdfViewer = "";
try {
    if (navigator.pdfViewerEnabled) {
        pdfViewer = navigator.pdfViewerEnabled;
    } else {
        pdfViewer = "";
    }
} catch (e) {
    pdfViewer = "";
}

var cookiesEnabled = "";
try {
    if (navigator.cookieEnabled) {
        cookiesEnabled = navigator.cookiesEnabled;
    } else {
        cookiesEnabled = "";
    }
} catch (e) {
    cookiesEnabled = "";
}

var dntEnabled = "";
try {
    if (navigator.doNotTrack) {
        dntEnabled = navigator.doNotTrack;
    } else {
        dntEnabled = "";
    }
} catch (e) {
    dntEnabled = "";
}

var langs = "";
try {
    if (navigator.languages) {
        langs = navigator.languages;
    } else {
        langs = "";
    }
} catch (e) {
    langs = "";
}

var prod = "";
try {
    if (navigator.product) {
        prod = navigator.product;
    } else {
        prod = "";
    }
} catch (e) {
    prod = "";
}

var prodSub = "";
try {
    if (navigator.prodSub) {
        prodSub = navigator.productSub;
    } else {
        prodSub = "";
    }
} catch (e) {
    prodSub = "";
}

var userAgent = "";
try {
    if (navigator.userAgent) {
        userAgent = navigator.userAgent;
    } else {
        userAgent = "";
    }
} catch (e) {
    userAgent = "";
}

var vend = "";
try {
    if (navigator.vendor) {
        vend = navigator.vendor;
    } else {
        vend = "";
    }
} catch (e) {
    vend = "";
}

var innerHeight = "";
try {
    if (window.innerHeight) {
        innerHeight = window.innerHeight;
    } else {
        innerHeight = "";
    }
} catch (e) {
    innerHeight = "";
}

var innerWidth = "";
try {
    if (window.innerWidth) {
        innerWidth = window.innerWidth;
    } else {
        innerWidth = "";
    }
} catch (e) {
    innerWidth = "";
}

var canvas = document.createElement('canvas');
var ctx = canvas.getContext('2d');
var txt = 'ctclsite-python-canvas-test';
ctx.textBaseline = "top";
ctx.font = "14px 'Arial'";
ctx.textBaseline = "alphabetic";
ctx.fillStyle = "#f60";
ctx.fillRect(125,1,62,20);
ctx.fillStyle = "#069";
ctx.fillText(txt, 2, 15);
ctx.fillStyle = "rgba(102, 204, 0, 0.7)";
ctx.fillText(txt, 4, 17);
var canvasFpImg = canvas.toDataURL();

var canvasFp = "";
// Hash the fingerprint client-side
const msgBuffer = new TextEncoder().encode(canvasFpImg);
const hashBuffer = await crypto.subtle.digest('SHA-256', msgBuffer);
const hashArray = Array.from(new Uint8Array(hashBuffer));
canvasFp = hashArray.map(b => b.toString(16).padStart(2, '0')).join('');

// Send the data
fetch("/inlog/", {
  method: "POST",
  body: JSON.stringify({
    // Time and networking
    timeZone: timeZone,
    localIp: localIp,
    extIp: extIp,
    // Device data
    webGlDebug: webGlDebug,
    webGlVendor: webGlVendor,
    webGlRenderer: webGlRenderer,
    cpuCores: cpuCores,
    memSize: memSize,
    maxTp: maxTp,
    oscpu: oscpu,
    plat: plat,
    screenX: screenX,
    screenY: screenY,
    screenPixRatio: screenPixRatio,
    screenPixDepth: screenPixDepth,
    canvasFp: canvasFp,
    // Software support
    onLine: onLine,
    pdfViewer: pdfViewer,
    cookiesEnabled: cookiesEnabled,
    dntEnabled: dntEnabled,
    langs: langs,
    prod: prod,
    prodSub: prodSub,
    userAgent: userAgent,
    vend: vend,
    innerHeight: innerHeight,
    innerWidth: innerWidth,
  }),
  headers: {"Content-type": "application/json; charset=UTF-8"}
});
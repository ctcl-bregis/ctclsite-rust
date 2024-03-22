/*
 * ctclsite-rust - CTCL 2022-2024
 * File: static/clientinfo.js
 * Purpose: Client-side data collection script
 * Created: December 16, 2023
 * Modified: March 15, 2024
 */

var urlPath = "";
try {
    urlPath = window.location.pathname;
} catch (e) {
    urlPath = "";
}

// Time and networking
var timeZone = "";
try {
    if (timeZone) {
        timeZone = Intl.DateTimeFormat().resolvedOptions().timeZone
    } else {
        timeZone = "";
    }
} catch (e) {
    timeZone = "";
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
var txt = 'ctclsite-canvas-test 🦀🐍';
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
try {
    const msgBuffer = new TextEncoder().encode(canvasFpImg);
    const hashArray = Array.from(new Uint8Array(msgBuffer));
    canvasFp = hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
} catch (e) {
    canvasFp = "";
}

// Send the data
fetch("/inlog/", {
  method: "POST",
  body: JSON.stringify({
    url_path: String(urlPath),
    // Time and networking
    time_zone: String(timeZone),
    // Device data
    web_gl_vendor: String(webGlVendor),
    web_gl_renderer: String(webGlRenderer),
    cpu_cores: String(cpuCores),
    mem_size: String(memSize),
    max_tp: String(maxTp),
    oscpu: String(oscpu),
    plat: String(plat),
    screen_x: String(screenX),
    screen_y: String(screenY),
    screen_pix_ratio: String(screenPixRatio),
    screen_pix_depth: String(screenPixDepth),
    canvas_fp: String(canvasFp),
    // Software support
    online: String(onLine),
    pdf_viewer: String(pdfViewer),
    cookies_enabled: String(cookiesEnabled),
    dnt_enabled: String(dntEnabled),
    langs: String(langs),
    prod: String(prod),
    prod_sub: String(prodSub),
    user_agent: String(userAgent),
    vend: String(vend),
    inner_height: String(innerHeight),
    inner_width: String(innerWidth),
  }),
  headers: {"Content-type": "application/json; charset=UTF-8"}
});
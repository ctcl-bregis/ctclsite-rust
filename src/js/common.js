// ctclsite-rust - CTCL 2020-2024
// File: common.js
// Purpose: JavaScript script for various functions used throughout the website
// Created: January 8, 2024
// Modified: June 30, 2024

// Extend the header to fit the entire section width if it is past a certain width
function resizeHeaders() {
    var sectionHeaders = document.getElementsByClassName("section");

    for (let i = 0; i < sectionHeaders.length; i++) {
        var sectionHeader = sectionHeaders[i].getElementsByTagName("h2")[0];
        var parentSectionWidth = sectionHeader.parentElement.getBoundingClientRect().width;

        sectionHeader.setAttribute("style", "");

        if ((sectionHeader.getBoundingClientRect().width / parentSectionWidth) > 0.8) {
            if (CSS.supports("width: auto")) {
                sectionHeader.setAttribute("style", "width: auto; border-right: none");
            } else {
                sectionHeader.setAttribute("style", "width: fit-content; border-right: none");
            }
        } else {
            sectionHeader.setAttribute("style", "");
        }
    }
}

function fixLinkIcons() {
    var links = document.getElementsByClassName("pagebutton");
    
    if (links.length > 0) {
        for (let i = 0; i < links.length; i++) {
            var link = links[i];

            if (link.getElementsByClassName("pagebutton-img")[0]) {
                var pagebuttonimg = link.getElementsByClassName("pagebutton-img")[0];
    
                if (link.getBoundingClientRect().width < 400) {
                    pagebuttonimg.setAttribute("style", "display: none");
                } else {
                    pagebuttonimg.setAttribute("style", "");
                }
            }
        }
    }
}

// No need to calculate this every time the page is resized
var links = document.getElementsByTagName("nav")[0].getElementsByTagName("ul")[0].getElementsByTagName("li");
var navbarlinkswidth = 0;
for (let i = 0; i < links.length; i++) {
    navbarlinkswidth += links[i].getBoundingClientRect().width;
}

function fixNavbar() {
    let vw = Math.max(document.documentElement.clientWidth || 0, window.innerWidth || 0);
    if (vw < navbarlinkswidth) {
        document.getElementsByTagName("nav")[0].getElementsByTagName("ul")[0].style.display = "grid";
    } else {
        document.getElementsByTagName("nav")[0].getElementsByTagName("ul")[0].style.display = "";
    }
}

function fixElements() {
    resizeHeaders();
    fixLinkIcons();
    fixNavbar();
}

// Run on resize
addEventListener("resize", (event) => { });
onresize = (event) => fixElements();

// Also run when the page is loaded
window.onload = fixElements;
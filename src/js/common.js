// ctclsite-rust - CTCL 2020-2024
// File: common.js
// Purpose: JavaScript script for various functions used throughout the website
// Created: January 8, 2024
// Modified: June 6, 2024

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

function alignText() {
    var marginx;
    var target;

    // There should only be one header with one h1
    target = document.getElementsByTagName("header")[0].getElementsByTagName("h1")[0];
    marginx = target.firstElementChild.getBoundingClientRect()["x"];
    marginx = marginx - Math.floor(marginx);
    target.style.paddingLeft = marginx + "px";

    // There should only be one footer
    target = document.getElementsByTagName("footer")[0];
    marginx = target.firstElementChild.getBoundingClientRect()["x"];
    marginx = marginx - Math.floor(marginx);
    target.style.marginLeft = marginx + "px";
}

function fixElements() {
    resizeHeaders();
    alignText();
}

// Run on resize
addEventListener("resize", (event) => {});
onresize = (event) => fixElements();

// Also run when the page is loaded
window.onload = fixElements;
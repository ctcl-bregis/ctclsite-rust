// ctclsite-rust - CTCL 2020-2024
// File: common.js
// Purpose: Commonly loaded JavaScript script for various functions, see comments below
// Created: January 8, 2024
// Modified: March 31, 2024

// Extend the header to fit the entire section width if it is past a certain width
function resizeHeaders() {
    var sectionHeaders = document.getElementsByClassName("sectioncontent");
    // If there is nothing with sectionheader, e.g. pages other than about, overwrite sectionHeaders with button elements
    if (sectionHeaders.length < 1) {
        sectionHeaders = document.getElementsByClassName("buttonsection");
        for (let i = 0; i < sectionHeaders.length; i++) {
            var sectionHeader = sectionHeaders[i];
            // Subtract width of borders as the section width includes the borders
            // Image left border width should be the same as the section border width
            var sectionWidth = sectionHeader.getBoundingClientRect().width - (parseInt(getComputedStyle(sectionHeader).borderWidth, 10) * 9);

            // Only for project and blog pages
            var sectionimage = sectionHeader.getElementsByTagName("img")[0];
            if (sectionimage) {
                sectionWidth -= Number(sectionimage.width);
            }

            var sectionButton = sectionHeader.getElementsByTagName("a")[0];

            // Without this line, the buttons flicker when the window is resized
            sectionButton.setAttribute("style", "");

            if ((sectionButton.getBoundingClientRect().width / sectionWidth) > 0.5) {
                if (CSS.supports("width: -webkit-fill-available")) {
                    sectionButton.setAttribute("style", "width: -webkit-fill-available; border-right: none");
                } else {
                    if (sectionimage) {
                        sectionButton.setAttribute("style", "width: " + sectionWidth + "px; border-right: none");
                    } else {
                        sectionButton.setAttribute("style", "width: " + Math.ceil(sectionWidth) + "px; border-right: none");
                    }
                }
            } else {
                sectionButton.setAttribute("style", "");
            }
        }
    // Otherwise use sectioncontent class elements
    } else {
        for (let i = 0; i < sectionHeaders.length; i++) {
            var sectionHeader = sectionHeaders[i];
            var parentSectionWidth = sectionHeader.parentElement.getBoundingClientRect().width;
        
            sectionHeader.setAttribute("style", "");
            
            if ((sectionHeader.getBoundingClientRect().width / parentSectionWidth) > 0.5) {
                if (CSS.supports("width: -webkit-fill-available")) {
                    sectionHeader.setAttribute("style", "width: -webkit-fill-available; border-right: none");
                } else {
                    sectionHeader.setAttribute("style", "width: " + Math.ceil(sectionWidth) + "px; border-right: none");
                }
            } else {
                sectionHeader.setAttribute("style", "");
            }
        }
    }
}

// Run on resize
addEventListener("resize", (event) => {});
onresize = (event) => resizeHeaders();

// Also run when the page is loaded
window.onload = resizeHeaders;
// ctclsite-python - CTCL 2020-2024
// File: common.js
// Purpose: Commonly loaded JavaScript script for various functions, see comments below
// Created: January 8, 2024
// Modified: January 14, 2024

// Extend the header to fit the entire section width if it is past a certain width
function resizeHeaders() {
    var sectionHeaders = document.getElementsByClassName("sectioncontent");
    // If there is nothing with sectionheader, e.g. pages other than about, replace it with buttons
    if (sectionHeaders.length < 1) {
        sectionHeaders = document.getElementsByClassName("projectitem");

        for (let i = 0; i < sectionHeaders.length; i++) {
            var sectionHeader = sectionHeaders[i];
            var parentSectionWidth = sectionHeader.parentElement.getBoundingClientRect().width;
    
            sectionHeader.childNodes[1].childNodes[0].setAttribute("style", "");

            if ((sectionHeader.childNodes[1].childNodes[0].getBoundingClientRect().width / parentSectionWidth) > 0.6) {
                sectionHeader.childNodes[1].childNodes[0].setAttribute("style", "width: 100%");
            } else {
                sectionHeader.childNodes[1].childNodes[0].setAttribute("style", "");
            }
        }   
    } else {
        for (let i = 0; i < sectionHeaders.length; i++) {
            var sectionHeader = sectionHeaders[i];
            var parentSectionWidth = sectionHeader.parentElement.getBoundingClientRect().width;
            
            sectionHeader.childNodes[1].setAttribute("style", "");
    
            if ((sectionHeader.childNodes[1].getBoundingClientRect().width / parentSectionWidth) > 0.6) {
                sectionHeader.childNodes[1].setAttribute("style", "width: 100%");
            } else {
                sectionHeader.childNodes[1].setAttribute("style", "");
            }
        }   
    }
}

// Run on resize
addEventListener("resize", (event) => {});
onresize = (event) => resizeHeaders();

// Also run when the page is loaded
window.onload = resizeHeaders;
// ctclsite-rust - CTCL 2020-2024
// File: common.js
// Purpose: Commonly loaded JavaScript script for various functions, see comments below
// Created: January 8, 2024
// Modified: March 4, 2024

function playVideos() {
    let videos = document.querySelectorAll("video");
    videos.forEach((video) => {
        video.play();
    });
}
function pauseVideos() {
    let videos = document.querySelectorAll("video");
    videos.forEach((video) => {
        video.pause();
    });
}
function playpauseVideos() {
    if (document.querySelector(".playpausebtn-pause")) {
        let btn = document.querySelector(".playpausebtn-pause");
        playVideos();
        btn.className = "playpausebtn-play";
    } else {
        let btn = document.querySelector(".playpausebtn-play");
        pauseVideos();
        btn.className = "playpausebtn-pause";
    }
}
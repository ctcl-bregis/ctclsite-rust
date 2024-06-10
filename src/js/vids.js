// ctclsite-rust - CTCL 2020-2024
// File: common.js
// Purpose: Commonly loaded JavaScript script for various functions, see comments below
// Created: January 8, 2024
// Modified: June 10, 2024

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
    if (document.querySelector(".vidbtn-play")) {
        let btn = document.querySelector(".vidbtn-play");
        playVideos();
        btn.className = "vidbtn-pause";
    } else {
        let btn = document.querySelector(".vidbtn-pause");
        pauseVideos();
        btn.className = "vidbtn-play";
    }
}
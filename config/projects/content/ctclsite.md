This page is the colophon for the website.

## About
This website serves mainly as my online portfolio and blog. 

### Content
Writing styling and the backend code myself allowed for more creative freedoms in the design of the website. 

The background videos shown on the About/Welcome page were also produced by myself.

Starting July 5, 2024, all fonts on the website are part of the Pixel font series that I created myself.

### Compatibility
Ensuring compatibility with most browsers is a difficult feat. I try my best to have the website look as intended on both mobile and desktop browsers.

- Desktop test browser: Ungoogled Chromium 125.0.6422.112 - 1920x1080 23", 1024x1280 17" (portrait)
- Mobile test browser: Vivaldi 6.7.3335.149 - 2400x1080 5.9" (portrait and landscape)

### Development History
From 2020 to 2021, my website was made up of just HTML and CSS that I wrote by hand. The webpages were hosted on GitHub Pages initially then was moved to DigitalOcean some time in 2021 when I got a domain name. It was not until December 2021-January 2022 that I wrote a version of the website that has backend code. 

The switch to a version that uses backend code to generate content was mainly due to the former RAMList pages that had amounts of tabular data that was too large to be managed by hand. RAMList was the main reason why I switched to using Rust for the website in November 2022 which turned out to compensate for the highly inefficient method of retrieving and processing data. [RAMList was discontinued in August 2023](../../blog/10/) for multiple reasons.

It is possible for me to just make something that generates static HTML pages to served by the webserver. However, there has been plans for features that would not be possible with just HTML and JS such as improved logging, view counters and dynamically generated embeds for other websites.

In February 2024, I have decided to switch to Rust for server and web software development.

#### Versions
The current version of the website is **ctclsite-rust v1**.


| Website Version    | Programming Language | Web Framework | Styling    | Development Started | Released          | Development System(s)                                                                                                         |
| ------------------ | -------------------- | ------------- | ---------- | ------------------- | ----------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| ctclsite-python v1 | Python               | Flask         | Manual CSS | December 2021       | January 7, 2022   | [Polyethylene Terephthalate](../wbpc/#pc_pet)                                                                                 |
| ctclsite-python v2 | Python               | Flask         | Manual CSS | 2022                | 2022              | [Polyethylene Terephthalate](../wbpc/#pc_pet)                                                                                 |
| ctclsite-rust v0   | Rust                 | Actix Web     | Manual CSS | November 2022       | January 2023      | "Dichlorofluoromethane" (ThinkPad X240), [Polyethylene Terephthalate](../wbpc/#pc_pet)                                        |
| ctclsite-python v3 | Python               | Flask         | Manual CSS | May 15, 2023        | May 20, 2023      | [Polybutylene Terephthalate](../wbpc/#pc_pbt)                                                                                 |
| ctclsite-python v4 | Python               | Django        | SCSS       | August 26, 2023     | October 20, 2023  | [Polybutylene Terephthalate](../wbpc/#pc_pbt)                                                                                 |
| ctclsite-python v5 | Python, JavaScript   | Django        | SCSS       | November 21, 2023   | December 22, 2023 | [Polybutylene Terephthalate](../wbpc/#pc_pbt)                                                                                 |
| ctclsite-rust v1   | Rust, JavaScript     | Actix Web     | SCSS       | February 18, 2024   | March 3, 2024     | [Polybutylene Terephthalate](../wbpc/#pc_pbt), [Polymethylmethacrylate](../wbpc/#pc_pmma)                                     |

### Hosting
Currently, as of June 10, 2024, the website is hosted by Orace Cloud using their 'Oracle Cloud Free Tier' which consists of a VPS with a single vCPU and 1GB of memory on an AMD EPYC 7551P. Up to May 2024, Vultr was used for US$6/month for a single vCPU 1GB on an AMD EPYC Zen 2 (7xx2 series). Before Vultr, I used DigitalOcean for US$7/month for the same specifications.

There are multiple reasons why I don't host the website on one of my servers such as [SVCS1 "Levoamphetamine"](../../projects/svcs/). For one, security: With the residential network setup that is provided, I do not expect the setup to be as secure as a datacenter installation. Another reason, as mentioned before, I just have a residential internet setup with a dynamic IP that can change at any time without warning and it is possible that Verizon would not allow web hosting on residential networks. Using a VPS service is easier in many ways and data privacy is not a concern since the VPS just hosts this website. There has been the idea for me to self-host this website but it is likely that would not happen any time soon.
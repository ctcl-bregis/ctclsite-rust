This page serves as the project page for the website while being the colophon.

Internally and for development purposes, I use the name "ctclsite", "ctclsite-python" for the Python Flask/Django version and "ctclsite-rust" for the Rust Actix version.

# Features
The website is sort of like a static site generator. 

Though the website could just consist of HTML and CSS files that are built then served by a web server, some features such as logging require a backend. 

## Analytics and Logging
Unlike many websites, the CTCL website makes use of custom-made logging and data collection instead of relying on a third-party service such as Google Analytics. This ensures that collected data is only seen by the website manager; me and to be more transparent about how the data is collected and processed.

With ctclsite-python v5 and ctclsite-rust v1, a JavaScript file is used to collect data about the browser and device.

More about how the website logs browser information and how data is collected outside of the website can be found at the [Privacy Policy](/privacy/).

# Programming
The website's backend and frontend code was written by myself and is open source. For licensing of specific content, see [Licensing](../../licensing/).

The source code to the Rust version of the website can be found here: [GitHub](https://github.com/ctcl-bregis/ctclsite-rust)

The source code to the former Python version can be found here: [GitHub](https://github.com/ctcl-bregis/ctclsite-python)

# History
The Rust version of the website has the codename "Atlantic Blue Crab", while the Python version has the codename "Apache Trout".

The website has gone under seven rewrites since its introduction in December 2021/January 2022. The first, second and third version was written using the Python Flask framework. The fourth and seventh was written using Rust Actix. The fifth and sixth used Python Django.

From November 2022 to January 2023, I made a rewrite of the website in the Rust programming language utilizing the Actix framework, this version of the website, called ctclsite-rust "Atlantic Blue Crab" did not last that long and was eventually replaced by the third rewrite of ctclsite-python.

These rewrites boil down to this:


| Website Version    | Programming Language | Web Framework | Styling    | Development Started | Released          | Development System(s)                                                                                                         |
| ------------------ | -------------------- | ------------- | ---------- | ------------------- | ----------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| ctclsite-python v1 | Python               | Flask         | Manual CSS | December 2021       | January 7, 2022   | [Polyethylene Terephthalate](../wbpc/#pc_pet)                                                                                 |
| ctclsite-python v2 | Python               | Flask         | Manual CSS | 2022                | 2022              | [Polyethylene Terephthalate](../wbpc/#pc_pet)                                                                                 |
| ctclsite-rust v0   | Rust                 | Actix Web     | Manual CSS | November 2022       | January 2023      | "Dichlorofluoromethane" (ThinkPad X240), [Polyethylene Terephthalate](../wbpc/#pc_pet)                                        |
| ctclsite-python v3 | Python               | Flask         | Manual CSS | May 15, 2023        | May 20, 2023      | [Polybutylene Terephthalate](../wbpc/#pc_pbt)                                                                                 |
| ctclsite-python v4 | Python               | Django        | SCSS       | August 26, 2023     | October 20, 2023  | [Polybutylene Terephthalate](../wbpc/#pc_pbt)                                                                                 |
| ctclsite-python v5 | Python, JavaScript   | Django        | SCSS       | November 21, 2023   | December 22, 2023 | [Polybutylene Terephthalate](../wbpc/#pc_pbt)                                                                                 |
| ctclsite-rust v1   | Rust, JavaScript     | Actix Web     | SCSS       | February 18, 2024   | March 3, 2024     | [Polybutylene Terephthalate](../wbpc/#pc_pbt), [Polymethylmethacrylate](../wbpc/#pc_pmma), [Tetrahydrocannabinol](../pc_thc/) |

## ctclsite-python v1
ctclsite-python v1 was the first version of the website that did not have all of the HTML content written by hand and instead had content handled by Python Flask with the Jinja2 templating engine. Before this, the website was just a collection of HTML and CSS files. 

Development was presumably started in December 2021 and the release was on January 7, 2022.

This version was very slow with many optimization steps I was unaware of due to my absolute beginner knowledge of Python and programming in general.

## ctclsite-python v2
ctclsite-python v2 was a rewrite that retained much of the content. Most of the backend code was rewritten because of the original code being nearly unreadable and inefficient.

Development was during 2022 and was released shortly after within the same year.

## ctclsite-rust v0
As a way to teach myself the Rust programming language after being aware of its features by colleagues. I rewrote the website in Rust, using the Actix web framework. This version seemed much faster due to the inefficent nature of how RAMList loaded content. I did not have an in-depth knowledge of Rust, which kept me from being able to maintain the website, leading to another Python rewrite described below.

## ctclsite-python v3
ctclsite-python v3 was a much needed rewrite of the website, adding significant performance improvements. This version continued to use Python Flask. I went back to Python due to my inability to maintain the Rust version of the website because of my limited knowledge of the Rust programming language.

Development was started on May 15, 2023 and was quickly released on May 20, 2023.

## ctclsite-python v4
This version of the website was started after being made aware of the misleading nature of RAMList on August 26, 2023. I was originally going to have a update to v3 but most of the code was "unreadable" and was rewritten in the Django web framework. Django was used because I was already used to using Django from the development of ContactList and CAMS.

## ctclsite-python v5
ctclsite-python v5 has minor changes over the backend code of the website and was originally planned to be version "4.2.0" of the website. Instead this release was a great redesign of the general look of the website and finally added client-side scripts.

This release was finally put into production on December 22, 2023.

## ctclsite-rust v1
On February 18, 2024, I have set up the ctclsite-rust repository for a potential rewrite using Rust.

This rewrite was done not because it needed it in any way but was instead used as an opportunity to reintroduce myself to Rust in a much more familiar environment of web design before taking on more difficult projects such as writing [SLAG](../slag/) in Rust.

On March 3, 2024, version 1.1.0 of ctclsite-rust was deployed. Like the former version, the Actix web framework is used.
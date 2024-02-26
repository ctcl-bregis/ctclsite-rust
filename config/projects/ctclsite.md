This website itself is one of my ongoing projects.

Internally and for development purposes, I use the name "ctclsite", "ctclsite-python" for the Python Flask/Django version and "ctclsite-rust" for the Rust Actix version.

# Programming
The website's backend and frontend code was written by myself and is open source. For licensing of specific content, see [Licensing](../../licensing/).

The source code to the website can be found here: [GitHub](https://github.com/ctcl-bregis/ctclsite-python)

# History
The Rust version of the website has the codename "Atlantic Blue Crab", while the Python version has the codename "Apache Trout".

The website has gone under six rewrites since its introduction in December 2021/January 2022. The first, second and third version was written using the Python Flask framework, the third, fourth and the current version, fifth was written using Python Django.

From November 2022 to January 2023, I made a rewrite of the website in the Rust programming language utilizing the Actix framework, this version of the website, called ctclsite-rust "Atlantic Blue Crab" did not last that long and was eventually replaced by the third rewrite of ctclsite-python.

These rewrites boil down to this:

<table>
    <tr>
        <th>Version</th>
        <th>Programming Langauge</th>
        <th>Web Framework</th>
        <th>Styling</th>
        <th>Development Started</th>
        <th>Released</th>
        <th>System Used</th>
    </tr>
    <tr>
        <td>ctclsite-python v1</td>
        <td>Python</td>
        <td>Flask</td>
        <td>Manual Styling</td>
        <td>December 2021</td>
        <td>January 2022</td>
        <td><a href="../pc_pet">Polyethylene Terephthalate</a></td>
    </tr>
    <tr>
        <td>ctclsite-python v2</td>
        <td>Python</td>
        <td>Flask</td>
        <td>Manual Styling</td>
        <td>2022</td>
        <td>2022</td>
        <td><a href="../pc_pet">Polyethylene Terephthalate</a></td>
    </tr>
    <tr>
        <td>ctclsite-rust v1</td>
        <td>Rust</td>
        <td>Actix</td>
        <td>Manual Styling</td>
        <td>November 2022</td>
        <td>January 2023</td>
        <td>Most of the development on a ThinkPad X240 codenamed "Dichlorofluoromethane", later <a href="../pc_pbt">Polybutylene Terephthalate</a></td>
    </tr>
    <tr>
        <td>ctclsite-python v3</td>
        <td>Python</td>
        <td>Flask</td>
        <td>Manual Styling</td>
        <td>May 15, 2023</td>
        <td>May 20, 2023</td>
        <td><a href="../pc_pbt">Polybutylene Terephthalate</a></td>
    </tr>
    <tr>
        <td>ctclsite-python v4</td>
        <td>Python</td>
        <td>Django</td>
        <td>SCSS</td>
        <td>August 26, 2023</td>
        <td>October 20, 2023</td>
        <td><a href="../pc_pbt">Polybutylene Terephthalate</a></td>
    </tr>
    <tr>
        <td>ctclsite-python v5 - <strong>CURRENT</strong></td>
        <td>Python + JavaScript</td>
        <td>Django</td>
        <td>SCSS</td>
        <td>November 21, 2023</td>
        <td>December 22, 2023</td>
        <td><a href="../pc_pbt">Polybutylene Terephthalate</a></td>
    </tr>
</table>

## ctclsite-python v1
ctclsite-python v1 was the first version of the website that did not have all of the HTML content written by hand and instead had content handled by Python Flask with the Jinja2 templating engine. Before this, the website was just a collection of HTML and CSS files. 

Development was presumably started in December 2021 and the release was on January 7, 2022.

This version was very slow with many optimization steps I was unaware of due to my absolute beginner knowledge of Python and programming in general.

## ctclsite-python v2
ctclsite-python v2 was a rewrite that retained much of the content. Most of the backend code was rewritten because of the original code being nearly unreadable and inefficient.

Development was during 2022 and was released shortly after within the same year.

## ctclsite-rust v1
As a way to teach myself the Rust programming language after being aware of its features by colleagues. I rewrote the website in Rust, using the Actix web framework. This version seemed much faster due to the inefficent nature of how RAMList loads content.

## ctclsite-python v3
ctclsite-python v3 was a much needed rewrite of the website, adding significant performance improvements. This version continued to use Python Flask. I went back to Python due to my inability to maintain the Rust version of the website because of my limited knowledge of the Rust programming language.

Development was started on May 15, 2023 and was quickly released on May 20, 2023.

## ctclsite-python v4
This version of the website was started after being made aware of the misleading nature of RAMList on August 26, 2023. I was originally going to have a update to v3 but most of the code was "unreadable" and was rewritten in the Django web framework. Django was used because I was already used to using Django from the development of ContactList and CAMS.

## ctclsite-python v5
ctclsite-python v5 has minor changes over the backend code of the website and was originally planned to be version "4.2.0" of the website. Instead this release was a great redesign of the general look of the website and finally added client-side scripts.

This release was finally put into production on December 22, 2023.

## ctclsite-rust v2
On February 18, 2024, I have set up the ctclsite-rust repository for a potential rewrite using the Rust framework. Actual development has not started yet.
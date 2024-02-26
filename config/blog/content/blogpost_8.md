# Projects
As always, I have been working on some projects of some sort.

## Software
I have been trying to take one project at a time, no matter how boring it can get at times.

### SLAG
On Friday, February 9, 2024, I have gotten SLAG to a somewhat usable state where it can now log user presences and messages.

SLAG is *almost* production-ready and I started hosting it on ["Lisdexamfetamine"](../../projects/srv_amp/).

#### Micron Cog
During the week, I added the ability for SLAG to search for FBGA codes (also known as the "D9" code for SDRAM devices) on Micron DRAM and flash memory devices. Unlike former implementations of FBGA lookup, this time I added caching utilizing an SQLite3 database for storing known part numbers associated with the FBGA code. Caching and storing results can significantly decrease the amount of requests to the Micron Technology website.

<figure>
    <img src="/static/blog/8/slag_micron_1.webp">
    <figcaption>Demostration of the FBGA code lookup functionality</figcaption>
</figure>

Along with the FBGA lookup, I added another command for decoding the production code that is normally found above the FBGA code. This does not require any requests to the Micron website and is based off the guide in the Micron CSN-11: Product Marks/Product and Packaging Labels document (revision AV 12/23 EN).

<figure>
    <img src="/static/blog/8/slag_micron_2.webp">
    <figcaption>Demostration of the production code decoder functionality</figcaption>
</figure>

#### Users Cog
The "users" Cog has gotten the most code throughout this week. The "log" Cog was removed as its functionality was replaced by "users".

As of February 11, 2024, the bot collects the following:

- User activity/presence/status
- Messages, even of ones of users that no longer share a guild with the bot
- User account creation date

I temporarily added the /gathermessages command for collecting all of the messages of every guild. This command may be removed soon and be replaced by a command line option. I added an obvious reference to Portal.

### ContactList
On Saturday, February 10, 2024, after implementing the largest parts of SLAG (above), I decided to start working on ContactList again to be able to get 0.6.0 out.

In late 2023, I made the mistake of trying to implement a feature that ContactList did not need or would ever use to prepare myself for reusing the codebase for CAMS. 

Recently, on December 26, 2023, I reverted the code back to 0.5.1 and started over. During January of this year, I have been doing some work on ContactList but development was overshadowed by additions to this website, SLAG and other projects.

## PC Hardware
As usual, hardware projects have not been getting that much attention. 

### ASUS Chromebook C100P
On the last day of Week 5; February 4, 2024, I reinstalled Debian 10 on an ASUS Chromebook C100P that uses the Rockchip RK3288 ARM SoC. I have been testing the device's functionality during this week. 

General performance of Debian 10 armhf on the C100P is similar to the x86-64 Chromebook codenamed "Chromium Sulfate" where I flashed Coreboot and installed Linux Mint.

### SVCS
On February 6, 2024, I changed the codename for the HP BL460c G8 that is assigned as SVCS1. The codename was switched from "Methamphetamine" to "Levoamphetamine" because of a guideline put into the codename guide that I came up with. 

I've yet to release a public version of the guide. Basically, for these HP blade servers, the theme is Psychoactives - Servers -> Stimulants - Compute/hosting Servers -> Amphetamines - HP BladeSystem-compatible devices.

The guideline states: "Starting February 6, 2024, codenames relating to chiral chemical compounds must use the names of its optical rotations."

This invalidates the codename "Methamphetamine" though "Levomethamphetamine" and "Dextromethamphetamine" remain valid codenames that may be used if I ever decide to add more HP blade servers.

The codename "Levoamphetamine" was chosen because of it being much more related to the codenames already being used for the other two HP servers; "Dextroamphetamine" and "Lisdexamfetamine". This change is also attributed to my ever growing interest in psychopharmacology and organic chemistry.

# Work
I have been getting some work within the last month. 

## Atlantic Estimating
On Sunday, February 11, 2024, I had a quick Xactimate task that I completed within an hour.

# Personal
Despite the sucesses with projects, Week 6 has been quite a difficult time for myself, personally. I would not elaborate that much about these events.

I plan to start turning things around within Week 7.

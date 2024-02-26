# WARNING
This document contains pure speculation, the content here may be incorrect.

## Introduction
This document covers the theory on how to idenitify the diffusion location of Siemens, Infineon Technologies and Qimonda DRAM integrated circuits.

CTCL's main interest for identification of such parts is to determine what DRAM was made in Sandston, Virginia, USA, near where CTCL currently resides.

## Date Code
The date code format is in a common format.

It has this layout: XXYY

- XX: Last two digits of the year, e.g. 99 for 1999, 08 for 2008
- YY: Week number, e.g. 32 for Week 32.

Verify: does the week start on Monday or Sunday? i.e. does it follow ISO 8601 (Monday)?

## Die Revision
The die revision is usually shown in two locations on the package: under the date code and as part of the part number.

Examples of die revisions:
- A
- A1
- C2
- F

## Die Code - Siemens and early Infineon

- S27404 (Siemens HYB39S64800BT-8 Week 24 1999 Rev.B GERMANY) - eBay user "dapooh7" HYS64V8300GU-8 Assembled in Germany
- E05045 (Siemens HYB39S64800AT-8 Week 46 1998 Rev.A TAIWAN) - eBay user "NcsTec for Tech Items and More" HYS64V8200GU-8 Assembled in Malaysia
- W22387 (Siemens HYB39S64800BT-8 Week 04 1999 Rev.B USA) - eBay user "Sale Lizard" HYS64V16220GU-7.5-B **Assembled in USA**
- V06014 (Siemens HYB39S64800AT-8 Week 16 1998 Rev.A USA) - eBay user "DTM Warehouse" HYS64V8200GU-8 Assembled in Malaysia
- E05189 (Infineon HYB39S64800BT-8 Week 44 1999 Rev.B USA) - eBay user "ComputerStoreUSA" HYS64V16220GU-8-C Assembled in Italy

### Location
Siemens and early Infineon DRAM parts seem to commonly disclose the country of origin on the package.

Country of Origin - First letter of the die code - Location:
- "TAIWAN" - "E" - Taiwan
- "GERMANY" - "S" - Dresden, Saxony, Germany. S for Saxony/Sachen?
- "USA" - "W" - Sandston, Virginia, United States. W likely for White Oak
- "USA" - "V" - Sandston, Virginia, Uinted States. V likely for Virginia

Likely the first letter of the die code is the diffusion location while the actual country name is the packaging location.

## Die Code - Infineon and Qimonda
Example die codes:
- FVV10340 (Qimonda IDSH1G-02A1F1C-13H Week 12 2009 Rev.A1) - CTCL Found on Patriot Memory branded module
- 36522207 (Qimonda IDSH1G-03A1F1C-10F Week 36 2008 Rev.A1 - CTCL Found on OEM Qimonda module made in Malaysia
- FVV24107 (Qimonda HYB18H1G321AF-14 Week 22 2008 Rev.A Qimonda Portugal-like "clipped-edge" packaging) - CTCL found on Xbox 360, presumably "Falcon" motherboard
- FVV02012 (Qimonda HYB18H512322AF-13 Week 40 2006 Rev.A Qimonda Portugal-like "clipped-edge" packaging) - XenonLibrary[^x360_qi]
- HVV35273 (Qimonda HYB18H512321BF-14 Week 32 2007 Rev.B Qimonda Portugal-like "clipped-edge" packaging) - XenonLibrary[^x360_qi]
- FVV11030 (Qimonda HYB18H1G321AF-14 Week 52 2008 Rev.A Qimonda Portugal-like "clipped-edge" packaging) - XenonLibrary[^x360_qi]
- SVV54406 (Infineon HYB18H512321AF-13 Week 46 2005 Rev.A) - XenonLibrary[^x360_it]
- 3VV32387 (Qimonda HYB18T1G800C2F-2.5 Week 48 2008 Rev.C2) - eBay user "sell.buy.computer" Qimonda HYS64T256020EDL-2.5C2 Assembled in Malaysia
- I6552135 (Qimonda HYB18T1G800C2F-2.5 Week 42 2008 Rev.C2 Qimonda Portugal-like "clipped-edge" packaging) - eBay user "sell.buy.computer" Qimonda HYS64T256020EDL-2.5C2 Assembled in China
- 3VV36073 (Qimonda HYB18T1G800C2F-2.5 Week 22 2008 Rev.C2) - eBay user "moorelaptopparts" Qimonda HYS64T256020EU-2.5-C2 Assembled in China
- FSS13525 (Infineon HYB25D512400BC-5 Week 08 2006 Rev.B) - eBay user "VR Assets" Infineon HYS72D128300GBR-5-B Assembled in Portugal
- FUU45317 (Infineon HYB25D512800BE-5 Week 42 2005 Rev.B) - eBay user "A-Z Appliance Parts" Infineon HYS64D64300HU-5-B Assembled in Italy
- WUU37828 (Infineon HYB25D256800CE-5 Week 26 2005 Rev.C) - eBay user "DTM Warehouse" HYS64D32300HU-5-C Assembled in Italy
- WWW61313 (Infineon [unintelligble due to photo quality] Week 38 2004 Rev.C) - eBay user "harolkapla_79" HYS64D64320GU-5-C **Assembled in USA**

### Location

The current theory is that the diffusion location is represented by the second and third letter of the die code.
- "EE" - Malacca, Malaysia
- "FF" - Taiwan? FF for Formosa?
- "RR" - **Unknown**
- "SS" - Dresden, Saxony, Germany. SS likely for Saxony/Sachen
- "UU" - Italy
- "VV" - Sandston, Virginia, United States (near Richmond, Virginia), VV likely for Virginia
- "WW" - Sandston, Virginia, United States (near Richmond, Virginia), WW likely for White Oak; the name of the tech park that the plant resides in
- "65" or similar - Diffused by *Nanya Technology* either in China or Taiwan under the Inotera joint venture

Like the old die code scheme seen on Siemens and early Infineon parts, the US location is represented by either "V" or "W". It is unknown why there is two different codes for one location, though it is likely that W and V differentiate between the 300mm fab and the 200mm fab on each side of the facility. For example, "W" could have two meanings: "White Oak" and "West".

From what is seen for Infineon and Qimonda parts shown on Xbox 360 motherboard pictures at the XenonLibrary wiki and a teardown done by CTCL, all of the die codes have "VV" for the second and third character. It is likely possible that Microsoft specified the use of United States-made DRAM in the Xbox 360 series.

How to determine the packaging location from the die code is unknown, this is likely because the die code is to describe the silicon die and not the packaged device itself. It appears that DRAM packaged by Qimonda Portugal has a unique "clipped edge" package. This seemed to be the case for another DRAM part from a different DRAM vendor using Qimonda Portugal's packaging service, such as a Winbond W641GG2JB-14 with the location code "PRT" for Portugal.[^x360_wb]




## Sources
[^x360_it]: https://xenonlibrary.com/wiki/Infineon_RAM
[^x360_qi]: https://xenonlibrary.com/wiki/Qimonda_RAM
[^x360_wb]: https://xenonlibrary.com/wiki/Winbond_RAM

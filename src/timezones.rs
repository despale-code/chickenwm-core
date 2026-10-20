//! Comprehensive world time zones database for sunrise/sunset calculations and light/dark switching.

/// Comprehensive world time zone entry with exact geographic coordinates for sunrise/sunset calculations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimeZoneEntry {
    pub id: &'static str,
    pub display: &'static str,
    pub lat: f64,
    pub lon: f64,
    pub search: &'static str,
}

pub static TIMEZONES: &[TimeZoneEntry] = &[
    TimeZoneEntry { id: "Dateline Standard Time", display: "(UTC-12:00) International Date Line West", lat: 0.0, lon: -180.0, search: "dateline standard time (utc-12:00) international date line west international date line west" },
    TimeZoneEntry { id: "UTC-11", display: "(UTC-11:00) Coordinated Universal Time-11", lat: -14.28, lon: -170.7, search: "utc-11 (utc-11:00) coordinated universal time-11 pago pago american samoa midway niue" },
    TimeZoneEntry { id: "Aleutian Standard Time", display: "(UTC-10:00) Aleutian Islands", lat: 51.88, lon: -176.7, search: "aleutian standard time (utc-10:00) aleutian islands aleutian islands adak unalaska alaska" },
    TimeZoneEntry { id: "Hawaiian Standard Time", display: "(UTC-10:00) Hawaii", lat: 21.31, lon: -157.9, search: "hawaiian standard time (utc-10:00) hawaii hawaii honolulu oahu maui kauai" },
    TimeZoneEntry { id: "Marquesas Standard Time", display: "(UTC-09:30) Marquesas Islands", lat: -9.0, lon: -139.5, search: "marquesas standard time (utc-09:30) marquesas islands marquesas islands french polynesia nuku hiva" },
    TimeZoneEntry { id: "Alaskan Standard Time", display: "(UTC-09:00) Alaska", lat: 61.22, lon: -149.9, search: "alaskan standard time (utc-09:00) alaska alaska anchorage juneau fairbanks" },
    TimeZoneEntry { id: "UTC-09", display: "(UTC-09:00) Coordinated Universal Time-09", lat: 0.0, lon: -135.0, search: "utc-09 (utc-09:00) coordinated universal time-09 utc-9 gambier islands" },
    TimeZoneEntry { id: "British Columbia Standard Time", display: "(UTC-07:00) Pacific Time (British Columbia)", lat: 49.28, lon: -123.1, search: "british columbia standard time (utc-07:00) pacific time (british columbia) british columbia vancouver victoria bc canada" },
    TimeZoneEntry { id: "Pacific Standard Time (Mexico)", display: "(UTC-08:00) Baja California", lat: 32.51, lon: -117.0, search: "pacific standard time (mexico) (utc-08:00) baja california baja california tijuana mexicali ensenada" },
    TimeZoneEntry { id: "UTC-08", display: "(UTC-08:00) Coordinated Universal Time-08", lat: 0.0, lon: -120.0, search: "utc-08 (utc-08:00) coordinated universal time-08 utc-8 clipperton island pitcairn" },
    TimeZoneEntry { id: "Pacific Standard Time", display: "(UTC-08:00) Pacific Time (US & Canada)", lat: 34.05, lon: -118.2, search: "pacific standard time (utc-08:00) pacific time (us & canada) pacific time pst pdt los angeles san francisco seattle san diego san jose portland las vegas usa canada" },
    TimeZoneEntry { id: "Alberta Standard Time", display: "(UTC-06:00) Mountain Time (Alberta)", lat: 51.05, lon: -114.1, search: "alberta standard time (utc-06:00) mountain time (alberta) alberta calgary edmonton canada" },
    TimeZoneEntry { id: "US Mountain Standard Time", display: "(UTC-07:00) Arizona", lat: 33.45, lon: -112.1, search: "us mountain standard time (utc-07:00) arizona arizona phoenix tucson mesa mst" },
    TimeZoneEntry { id: "Mountain Standard Time (Mexico)", display: "(UTC-07:00) La Paz, Mazatlan", lat: 23.24, lon: -106.4, search: "mountain standard time (mexico) (utc-07:00) la paz, mazatlan la paz mazatlan sinaloa baja california sur" },
    TimeZoneEntry { id: "Mountain Standard Time", display: "(UTC-07:00) Mountain Time (US & Canada)", lat: 39.74, lon: -105.0, search: "mountain standard time (utc-07:00) mountain time (us & canada) mountain time mst mdt denver salt lake city albuquerque boise usa canada" },
    TimeZoneEntry { id: "Yukon Standard Time", display: "(UTC-07:00) Yukon", lat: 60.72, lon: -135.1, search: "yukon standard time (utc-07:00) yukon yukon whitehorse dawson canada" },
    TimeZoneEntry { id: "Manitoba Standard Time", display: "(UTC-05:00) Manitoba", lat: 49.9, lon: -97.14, search: "manitoba standard time (utc-05:00) manitoba manitoba winnipeg brandon canada" },
    TimeZoneEntry { id: "Central America Standard Time", display: "(UTC-06:00) Central America", lat: 14.63, lon: -90.51, search: "central america standard time (utc-06:00) central america central america guatemala costa rica honduras nicaragua el salvador belize" },
    TimeZoneEntry { id: "Central Standard Time", display: "(UTC-06:00) Central Time (US & Canada)", lat: 41.88, lon: -87.63, search: "central standard time (utc-06:00) central time (us & canada) central time cst cdt chicago dallas houston austin minneapolis st louis kansas city nashville usa" },
    TimeZoneEntry { id: "Easter Island Standard Time", display: "(UTC-06:00) Easter Island", lat: -27.12, lon: -109.3, search: "easter island standard time (utc-06:00) easter island easter island hanga roa rapa nui chile" },
    TimeZoneEntry { id: "Central Standard Time (Mexico)", display: "(UTC-06:00) Guadalajara, Mexico City, Monterrey", lat: 19.43, lon: -99.13, search: "central standard time (mexico) (utc-06:00) guadalajara, mexico city, monterrey mexico city guadalajara monterrey puebla cdmx" },
    TimeZoneEntry { id: "Canada Central Standard Time", display: "(UTC-06:00) Saskatchewan", lat: 52.13, lon: -106.7, search: "canada central standard time (utc-06:00) saskatchewan saskatchewan saskatoon regina canada" },
    TimeZoneEntry { id: "SA Pacific Standard Time", display: "(UTC-05:00) Bogota, Lima, Quito, Rio Branco", lat: 4.71, lon: -74.07, search: "sa pacific standard time (utc-05:00) bogota, lima, quito, rio branco bogota lima quito rio branco colombia peru ecuador" },
    TimeZoneEntry { id: "Eastern Standard Time (Mexico)", display: "(UTC-05:00) Chetumal", lat: 18.5, lon: -88.3, search: "eastern standard time (mexico) (utc-05:00) chetumal chetumal quintana roo cancun mexico" },
    TimeZoneEntry { id: "Eastern Standard Time", display: "(UTC-05:00) Eastern Time (US & Canada)", lat: 40.71, lon: -74.01, search: "eastern standard time (utc-05:00) eastern time (us & canada) eastern time est edt new york toronto montreal washington dc boston philadelphia miami atlanta ottawa" },
    TimeZoneEntry { id: "Haiti Standard Time", display: "(UTC-05:00) Haiti", lat: 18.59, lon: -72.31, search: "haiti standard time (utc-05:00) haiti haiti port-au-prince" },
    TimeZoneEntry { id: "Cuba Standard Time", display: "(UTC-05:00) Havana", lat: 23.11, lon: -82.37, search: "cuba standard time (utc-05:00) havana cuba havana" },
    TimeZoneEntry { id: "US Eastern Standard Time", display: "(UTC-05:00) Indiana (East)", lat: 39.77, lon: -86.16, search: "us eastern standard time (utc-05:00) indiana (east) indiana east indianapolis bloomington" },
    TimeZoneEntry { id: "Turks And Caicos Standard Time", display: "(UTC-05:00) Turks and Caicos", lat: 21.46, lon: -71.14, search: "turks and caicos standard time (utc-05:00) turks and caicos turks and caicos grand turk providenciales" },
    TimeZoneEntry { id: "Atlantic Standard Time", display: "(UTC-04:00) Atlantic Time (Canada)", lat: 44.65, lon: -63.57, search: "atlantic standard time (utc-04:00) atlantic time (canada) atlantic time canada halifax nova scotia new brunswick prince edward island" },
    TimeZoneEntry { id: "Venezuela Standard Time", display: "(UTC-04:00) Caracas", lat: 10.49, lon: -66.88, search: "venezuela standard time (utc-04:00) caracas caracas venezuela maracaibo valencia" },
    TimeZoneEntry { id: "Central Brazilian Standard Time", display: "(UTC-04:00) Cuiaba", lat: -15.6, lon: -56.1, search: "central brazilian standard time (utc-04:00) cuiaba cuiaba mato grosso campo grande brazil" },
    TimeZoneEntry { id: "SA Western Standard Time", display: "(UTC-04:00) Georgetown, La Paz, Manaus, San Juan", lat: -16.5, lon: -68.12, search: "sa western standard time (utc-04:00) georgetown, la paz, manaus, san juan georgetown guyana la paz bolivia manaus san juan puerto rico" },
    TimeZoneEntry { id: "Pacific SA Standard Time", display: "(UTC-04:00) Santiago", lat: -33.45, lon: -70.67, search: "pacific sa standard time (utc-04:00) santiago santiago chile valparaiso concepcion" },
    TimeZoneEntry { id: "Newfoundland Standard Time", display: "(UTC-03:30) Newfoundland", lat: 47.56, lon: -52.71, search: "newfoundland standard time (utc-03:30) newfoundland newfoundland st johns canada" },
    TimeZoneEntry { id: "Tocantins Standard Time", display: "(UTC-03:00) Araguaina", lat: -7.19, lon: -48.21, search: "tocantins standard time (utc-03:00) araguaina araguaina tocantins palmas brazil" },
    TimeZoneEntry { id: "Paraguay Standard Time", display: "(UTC-03:00) Asuncion", lat: -25.26, lon: -57.58, search: "paraguay standard time (utc-03:00) asuncion asuncion paraguay ciudad del este" },
    TimeZoneEntry { id: "E. South America Standard Time", display: "(UTC-03:00) Brasilia", lat: -15.79, lon: -47.88, search: "e. south america standard time (utc-03:00) brasilia brasilia sao paulo rio de janeiro belo horizonte brazil" },
    TimeZoneEntry { id: "SA Eastern Standard Time", display: "(UTC-03:00) Cayenne, Fortaleza", lat: -3.73, lon: -38.53, search: "sa eastern standard time (utc-03:00) cayenne, fortaleza cayenne french guiana fortaleza ceara recife brazil" },
    TimeZoneEntry { id: "Argentina Standard Time", display: "(UTC-03:00) City of Buenos Aires", lat: -34.6, lon: -58.38, search: "argentina standard time (utc-03:00) city of buenos aires buenos aires argentina cordoba rosario mendoza" },
    TimeZoneEntry { id: "Montevideo Standard Time", display: "(UTC-03:00) Montevideo", lat: -34.9, lon: -56.16, search: "montevideo standard time (utc-03:00) montevideo montevideo uruguay punta del este" },
    TimeZoneEntry { id: "Magallanes Standard Time", display: "(UTC-03:00) Punta Arenas", lat: -53.16, lon: -70.91, search: "magallanes standard time (utc-03:00) punta arenas punta arenas magallanes patagonia chile" },
    TimeZoneEntry { id: "Saint Pierre Standard Time", display: "(UTC-03:00) Saint Pierre and Miquelon", lat: 46.78, lon: -56.18, search: "saint pierre standard time (utc-03:00) saint pierre and miquelon saint pierre and miquelon france" },
    TimeZoneEntry { id: "Bahia Standard Time", display: "(UTC-03:00) Salvador", lat: -12.97, lon: -38.51, search: "bahia standard time (utc-03:00) salvador salvador bahia brazil" },
    TimeZoneEntry { id: "UTC-02", display: "(UTC-02:00) Coordinated Universal Time-02", lat: 0.0, lon: -30.0, search: "utc-02 (utc-02:00) coordinated universal time-02 utc-2 south georgia fernando de noronha" },
    TimeZoneEntry { id: "Greenland Standard Time", display: "(UTC-02:00) Greenland", lat: 64.18, lon: -51.72, search: "greenland standard time (utc-02:00) greenland greenland nuuk godthab" },
    TimeZoneEntry { id: "Mid-Atlantic Standard Time", display: "(UTC-02:00) Mid-Atlantic - Old", lat: 0.0, lon: -30.0, search: "mid-atlantic standard time (utc-02:00) mid-atlantic - old mid-atlantic" },
    TimeZoneEntry { id: "Azores Standard Time", display: "(UTC-01:00) Azores", lat: 37.74, lon: -25.68, search: "azores standard time (utc-01:00) azores azores ponta delgada portugal" },
    TimeZoneEntry { id: "Cape Verde Standard Time", display: "(UTC-01:00) Cabo Verde Is.", lat: 14.93, lon: -23.51, search: "cape verde standard time (utc-01:00) cabo verde is. cabo verde cape verde praia" },
    TimeZoneEntry { id: "UTC", display: "(UTC) Coordinated Universal Time", lat: 51.48, lon: 0.0, search: "utc (utc) coordinated universal time utc coordinated universal time greenwich gmt zulu" },
    TimeZoneEntry { id: "Morocco Standard Time", display: "(UTC+00:00) Casablanca", lat: 33.57, lon: -7.59, search: "morocco standard time (utc+00:00) casablanca casablanca rabat marrakech morocco" },
    TimeZoneEntry { id: "GMT Standard Time", display: "(UTC+00:00) Dublin, Edinburgh, Lisbon, London", lat: 51.51, lon: -0.13, search: "gmt standard time (utc+00:00) dublin, edinburgh, lisbon, london london dublin edinburgh lisbon belfast cardiff uk ireland portugal gmt bst" },
    TimeZoneEntry { id: "Greenwich Standard Time", display: "(UTC+00:00) Monrovia, Reykjavik", lat: 64.15, lon: -21.94, search: "greenwich standard time (utc+00:00) monrovia, reykjavik reykjavik iceland monrovia liberia" },
    TimeZoneEntry { id: "Sao Tome Standard Time", display: "(UTC+00:00) Sao Tome", lat: 0.34, lon: 6.73, search: "sao tome standard time (utc+00:00) sao tome sao tome and principe" },
    TimeZoneEntry { id: "W. Europe Standard Time", display: "(UTC+01:00) Amsterdam, Berlin, Bern, Rome, Stockholm, Vienna", lat: 52.52, lon: 13.4, search: "w. europe standard time (utc+01:00) amsterdam, berlin, bern, rome, stockholm, vienna amsterdam netherlands berlin germany bern switzerland rome italy stockholm sweden vienna austria cet cest" },
    TimeZoneEntry { id: "Central Europe Standard Time", display: "(UTC+01:00) Belgrade, Bratislava, Budapest, Ljubljana, Prague", lat: 44.82, lon: 20.46, search: "central europe standard time (utc+01:00) belgrade, bratislava, budapest, ljubljana, prague belgrade serbia bratislava slovakia budapest hungary ljubljana slovenia prague czechia" },
    TimeZoneEntry { id: "Romance Standard Time", display: "(UTC+01:00) Brussels, Copenhagen, Madrid, Paris", lat: 48.86, lon: 2.35, search: "romance standard time (utc+01:00) brussels, copenhagen, madrid, paris paris france brussels belgium copenhagen denmark madrid barcelona spain cet cest" },
    TimeZoneEntry { id: "Central European Standard Time", display: "(UTC+01:00) Sarajevo, Skopje, Warsaw, Zagreb", lat: 52.23, lon: 21.01, search: "central european standard time (utc+01:00) sarajevo, skopje, warsaw, zagreb sarajevo bosnia skopje north macedonia warsaw poland zagreb croatia cet cest" },
    TimeZoneEntry { id: "W. Central Africa Standard Time", display: "(UTC+01:00) West Central Africa", lat: 6.52, lon: 3.38, search: "w. central africa standard time (utc+01:00) west central africa west central africa lagos nigeria kinshasa luanda yaounde algeria tunisia" },
    TimeZoneEntry { id: "GTB Standard Time", display: "(UTC+02:00) Athens, Bucharest", lat: 37.98, lon: 23.73, search: "gtb standard time (utc+02:00) athens, bucharest athens greece bucharest romania thessaloniki eet eest" },
    TimeZoneEntry { id: "Middle East Standard Time", display: "(UTC+02:00) Beirut", lat: 33.89, lon: 35.5, search: "middle east standard time (utc+02:00) beirut beirut lebanon tripoli" },
    TimeZoneEntry { id: "Egypt Standard Time", display: "(UTC+02:00) Cairo", lat: 30.04, lon: 31.24, search: "egypt standard time (utc+02:00) cairo cairo alexandria egypt giza" },
    TimeZoneEntry { id: "E. Europe Standard Time", display: "(UTC+02:00) Chisinau", lat: 47.01, lon: 28.86, search: "e. europe standard time (utc+02:00) chisinau chisinau moldova" },
    TimeZoneEntry { id: "West Bank Standard Time", display: "(UTC+02:00) Gaza, Hebron", lat: 31.53, lon: 35.1, search: "west bank standard time (utc+02:00) gaza, hebron gaza hebron palestine ramallah" },
    TimeZoneEntry { id: "South Africa Standard Time", display: "(UTC+02:00) Harare, Pretoria", lat: -25.75, lon: 28.23, search: "south africa standard time (utc+02:00) harare, pretoria harare zimbabwe pretoria johannesburg cape town durban south africa" },
    TimeZoneEntry { id: "FLE Standard Time", display: "(UTC+02:00) Helsinki, Kyiv, Riga, Sofia, Tallinn, Vilnius", lat: 60.17, lon: 24.94, search: "fle standard time (utc+02:00) helsinki, kyiv, riga, sofia, tallinn, vilnius helsinki finland kyiv ukraine riga latvia sofia bulgaria tallinn estonia vilnius lithuania" },
    TimeZoneEntry { id: "Israel Standard Time", display: "(UTC+02:00) Jerusalem", lat: 31.77, lon: 35.22, search: "israel standard time (utc+02:00) jerusalem jerusalem tel aviv haifa israel" },
    TimeZoneEntry { id: "South Sudan Standard Time", display: "(UTC+02:00) Juba", lat: 4.85, lon: 31.6, search: "south sudan standard time (utc+02:00) juba juba south sudan" },
    TimeZoneEntry { id: "Kaliningrad Standard Time", display: "(UTC+02:00) Kaliningrad", lat: 54.71, lon: 20.51, search: "kaliningrad standard time (utc+02:00) kaliningrad kaliningrad russia" },
    TimeZoneEntry { id: "Sudan Standard Time", display: "(UTC+02:00) Khartoum", lat: 15.5, lon: 32.56, search: "sudan standard time (utc+02:00) khartoum khartoum sudan omdurman" },
    TimeZoneEntry { id: "Libya Standard Time", display: "(UTC+02:00) Tripoli", lat: 32.89, lon: 13.19, search: "libya standard time (utc+02:00) tripoli tripoli benghazi libya" },
    TimeZoneEntry { id: "Namibia Standard Time", display: "(UTC+02:00) Windhoek", lat: -22.56, lon: 17.08, search: "namibia standard time (utc+02:00) windhoek windhoek namibia" },
    TimeZoneEntry { id: "Jordan Standard Time", display: "(UTC+03:00) Amman", lat: 31.95, lon: 35.93, search: "jordan standard time (utc+03:00) amman amman zarqa jordan" },
    TimeZoneEntry { id: "Arabic Standard Time", display: "(UTC+03:00) Baghdad", lat: 33.32, lon: 44.36, search: "arabic standard time (utc+03:00) baghdad baghdad iraq basra mosul erbil" },
    TimeZoneEntry { id: "Syria Standard Time", display: "(UTC+03:00) Damascus", lat: 33.51, lon: 36.29, search: "syria standard time (utc+03:00) damascus damascus aleppo syria" },
    TimeZoneEntry { id: "Turkey Standard Time", display: "(UTC+03:00) Istanbul", lat: 41.01, lon: 28.98, search: "turkey standard time (utc+03:00) istanbul istanbul ankara izmir bursa antalya turkiye turkey" },
    TimeZoneEntry { id: "Arab Standard Time", display: "(UTC+03:00) Kuwait, Riyadh", lat: 24.71, lon: 46.68, search: "arab standard time (utc+03:00) kuwait, riyadh kuwait city riyadh saudi arabia jeddah dammam mecca medina qatar doha" },
    TimeZoneEntry { id: "Belarus Standard Time", display: "(UTC+03:00) Minsk", lat: 53.9, lon: 27.56, search: "belarus standard time (utc+03:00) minsk minsk brest belarus" },
    TimeZoneEntry { id: "Russian Standard Time", display: "(UTC+03:00) Moscow, St. Petersburg", lat: 55.75, lon: 37.62, search: "russian standard time (utc+03:00) moscow, st. petersburg moscow st petersburg nizhny novgorod kazan russia" },
    TimeZoneEntry { id: "E. Africa Standard Time", display: "(UTC+03:00) Nairobi", lat: -1.29, lon: 36.82, search: "e. africa standard time (utc+03:00) nairobi nairobi kenya addis ababa ethiopia dar es salaam tanzania kampala uganda" },
    TimeZoneEntry { id: "Volgograd Standard Time", display: "(UTC+03:00) Volgograd", lat: 48.71, lon: 44.51, search: "volgograd standard time (utc+03:00) volgograd volgograd russia" },
    TimeZoneEntry { id: "Iran Standard Time", display: "(UTC+03:30) Tehran", lat: 35.69, lon: 51.39, search: "iran standard time (utc+03:30) tehran tehran mashhad isfahan shiraz tabriz iran" },
    TimeZoneEntry { id: "Arabian Standard Time", display: "(UTC+04:00) Abu Dhabi, Muscat", lat: 25.2, lon: 55.27, search: "arabian standard time (utc+04:00) abu dhabi, muscat abu dhabi dubai uae united arab emirates muscat oman sharjah" },
    TimeZoneEntry { id: "Astrakhan Standard Time", display: "(UTC+04:00) Astrakhan, Ulyanovsk", lat: 46.35, lon: 48.04, search: "astrakhan standard time (utc+04:00) astrakhan, ulyanovsk astrakhan ulyanovsk russia" },
    TimeZoneEntry { id: "Azerbaijan Standard Time", display: "(UTC+04:00) Baku", lat: 40.41, lon: 49.87, search: "azerbaijan standard time (utc+04:00) baku baku ganja azerbaijan" },
    TimeZoneEntry { id: "Russia Time Zone 3", display: "(UTC+04:00) Izhevsk, Samara", lat: 53.2, lon: 50.15, search: "russia time zone 3 (utc+04:00) izhevsk, samara izhevsk samara togliatti russia" },
    TimeZoneEntry { id: "Mauritius Standard Time", display: "(UTC+04:00) Port Louis", lat: -20.16, lon: 57.5, search: "mauritius standard time (utc+04:00) port louis port louis mauritius reunion" },
    TimeZoneEntry { id: "Saratov Standard Time", display: "(UTC+04:00) Saratov", lat: 51.53, lon: 46.03, search: "saratov standard time (utc+04:00) saratov saratov engels russia" },
    TimeZoneEntry { id: "Georgian Standard Time", display: "(UTC+04:00) Tbilisi", lat: 41.72, lon: 44.79, search: "georgian standard time (utc+04:00) tbilisi tbilisi batumi georgia" },
    TimeZoneEntry { id: "Caucasus Standard Time", display: "(UTC+04:00) Yerevan", lat: 40.18, lon: 44.51, search: "caucasus standard time (utc+04:00) yerevan yerevan gyumri armenia" },
    TimeZoneEntry { id: "Afghanistan Standard Time", display: "(UTC+04:30) Kabul", lat: 34.53, lon: 69.17, search: "afghanistan standard time (utc+04:30) kabul kabul kandahar herat afghanistan" },
    TimeZoneEntry { id: "West Asia Standard Time", display: "(UTC+05:00) Ashgabat, Tashkent", lat: 41.31, lon: 69.24, search: "west asia standard time (utc+05:00) ashgabat, tashkent ashgabat turkmenistan tashkent samarkand uzbekistan" },
    TimeZoneEntry { id: "Qyzylorda Standard Time", display: "(UTC+05:00) Astana", lat: 51.17, lon: 71.45, search: "qyzylorda standard time (utc+05:00) astana astana almaty shymkent kazakhstan" },
    TimeZoneEntry { id: "Ekaterinburg Standard Time", display: "(UTC+05:00) Ekaterinburg", lat: 56.84, lon: 60.61, search: "ekaterinburg standard time (utc+05:00) ekaterinburg ekaterinburg yekaterinburg chelyabinsk perm russia" },
    TimeZoneEntry { id: "Pakistan Standard Time", display: "(UTC+05:00) Islamabad, Karachi", lat: 24.86, lon: 67.01, search: "pakistan standard time (utc+05:00) islamabad, karachi islamabad karachi lahore rawalpindi faisalabad pakistan" },
    TimeZoneEntry { id: "India Standard Time", display: "(UTC+05:30) Chennai, Kolkata, Mumbai, New Delhi", lat: 28.61, lon: 77.21, search: "india standard time (utc+05:30) chennai, kolkata, mumbai, new delhi chennai kolkata mumbai new delhi bangalore hyderabad pune ahmedabad india ist" },
    TimeZoneEntry { id: "Sri Lanka Standard Time", display: "(UTC+05:30) Sri Jayawardenepura", lat: 6.91, lon: 79.9, search: "sri lanka standard time (utc+05:30) sri jayawardenepura sri jayawardenepura kotte colombo sri lanka" },
    TimeZoneEntry { id: "Nepal Standard Time", display: "(UTC+05:45) Kathmandu", lat: 27.72, lon: 85.32, search: "nepal standard time (utc+05:45) kathmandu kathmandu pokhara nepal" },
    TimeZoneEntry { id: "Central Asia Standard Time", display: "(UTC+06:00) Bishkek", lat: 42.87, lon: 74.59, search: "central asia standard time (utc+06:00) bishkek bishkek kyrgyzstan osh" },
    TimeZoneEntry { id: "Bangladesh Standard Time", display: "(UTC+06:00) Dhaka", lat: 23.81, lon: 90.41, search: "bangladesh standard time (utc+06:00) dhaka dhaka chittagong bangladesh bhutan thimphu" },
    TimeZoneEntry { id: "Omsk Standard Time", display: "(UTC+06:00) Omsk", lat: 54.99, lon: 73.37, search: "omsk standard time (utc+06:00) omsk omsk russia" },
    TimeZoneEntry { id: "Myanmar Standard Time", display: "(UTC+06:30) Yangon (Rangoon)", lat: 16.87, lon: 96.2, search: "myanmar standard time (utc+06:30) yangon (rangoon) yangon rangoon mandalay naypyidaw myanmar burma" },
    TimeZoneEntry { id: "SE Asia Standard Time", display: "(UTC+07:00) Bangkok, Hanoi, Jakarta", lat: 13.75, lon: 100.5, search: "se asia standard time (utc+07:00) bangkok, hanoi, jakarta bangkok thailand hanoi ho chi minh city vietnam jakarta surabaya indonesia" },
    TimeZoneEntry { id: "Altai Standard Time", display: "(UTC+07:00) Barnaul, Gorno-Altaysk", lat: 53.35, lon: 83.77, search: "altai standard time (utc+07:00) barnaul, gorno-altaysk barnaul gorno-altaysk altai russia" },
    TimeZoneEntry { id: "W. Mongolia Standard Time", display: "(UTC+07:00) Hovd", lat: 48.01, lon: 91.64, search: "w. mongolia standard time (utc+07:00) hovd hovd khovd mongolia" },
    TimeZoneEntry { id: "North Asia Standard Time", display: "(UTC+07:00) Krasnoyarsk", lat: 56.01, lon: 92.87, search: "north asia standard time (utc+07:00) krasnoyarsk krasnoyarsk russia" },
    TimeZoneEntry { id: "N. Central Asia Standard Time", display: "(UTC+07:00) Novosibirsk", lat: 55.01, lon: 82.93, search: "n. central asia standard time (utc+07:00) novosibirsk novosibirsk russia" },
    TimeZoneEntry { id: "Tomsk Standard Time", display: "(UTC+07:00) Tomsk", lat: 56.5, lon: 84.97, search: "tomsk standard time (utc+07:00) tomsk tomsk seversk russia" },
    TimeZoneEntry { id: "China Standard Time", display: "(UTC+08:00) Beijing, Chongqing, Hong Kong, Urumqi", lat: 39.9, lon: 116.4, search: "china standard time (utc+08:00) beijing, chongqing, hong kong, urumqi beijing chongqing hong kong urumqi shanghai guangzhou shenzhen chengdu wuhan china" },
    TimeZoneEntry { id: "North Asia East Standard Time", display: "(UTC+08:00) Irkutsk", lat: 52.29, lon: 104.3, search: "north asia east standard time (utc+08:00) irkutsk irkutsk baikal russia" },
    TimeZoneEntry { id: "Singapore Standard Time", display: "(UTC+08:00) Kuala Lumpur, Singapore", lat: 1.35, lon: 103.8, search: "singapore standard time (utc+08:00) kuala lumpur, singapore kuala lumpur penang johor bahru malaysia singapore manila philippines brunei" },
    TimeZoneEntry { id: "W. Australia Standard Time", display: "(UTC+08:00) Perth", lat: -31.95, lon: 115.9, search: "w. australia standard time (utc+08:00) perth perth fremantle western australia" },
    TimeZoneEntry { id: "Taipei Standard Time", display: "(UTC+08:00) Taipei", lat: 25.03, lon: 121.6, search: "taipei standard time (utc+08:00) taipei taipei kaohsiung taiwan" },
    TimeZoneEntry { id: "Ulaanbaatar Standard Time", display: "(UTC+08:00) Ulaanbaatar", lat: 47.92, lon: 106.9, search: "ulaanbaatar standard time (utc+08:00) ulaanbaatar ulaanbaatar ulan bator mongolia" },
    TimeZoneEntry { id: "Aus Central W. Standard Time", display: "(UTC+08:45) Eucla", lat: -31.68, lon: 128.9, search: "aus central w. standard time (utc+08:45) eucla eucla western australia" },
    TimeZoneEntry { id: "Transbaikal Standard Time", display: "(UTC+09:00) Chita", lat: 52.03, lon: 113.5, search: "transbaikal standard time (utc+09:00) chita chita zabaykalsky russia" },
    TimeZoneEntry { id: "Tokyo Standard Time", display: "(UTC+09:00) Osaka, Sapporo, Tokyo", lat: 35.68, lon: 139.7, search: "tokyo standard time (utc+09:00) osaka, sapporo, tokyo osaka sapporo tokyo kyoto yokohama nagoya kobe fukuoka japan jst" },
    TimeZoneEntry { id: "North Korea Standard Time", display: "(UTC+09:00) Pyongyang", lat: 39.04, lon: 125.8, search: "north korea standard time (utc+09:00) pyongyang pyongyang north korea" },
    TimeZoneEntry { id: "Korea Standard Time", display: "(UTC+09:00) Seoul", lat: 37.57, lon: 127.0, search: "korea standard time (utc+09:00) seoul seoul busan incheon daegu south korea kst" },
    TimeZoneEntry { id: "Yakutsk Standard Time", display: "(UTC+09:00) Yakutsk", lat: 62.03, lon: 129.7, search: "yakutsk standard time (utc+09:00) yakutsk yakutsk sakha russia" },
    TimeZoneEntry { id: "Cen. Australia Standard Time", display: "(UTC+09:30) Adelaide", lat: -34.93, lon: 138.6, search: "cen. australia standard time (utc+09:30) adelaide adelaide south australia" },
    TimeZoneEntry { id: "AUS Central Standard Time", display: "(UTC+09:30) Darwin", lat: -12.46, lon: 130.8, search: "aus central standard time (utc+09:30) darwin darwin alice springs northern territory australia" },
    TimeZoneEntry { id: "E. Australia Standard Time", display: "(UTC+10:00) Brisbane", lat: -27.47, lon: 153.0, search: "e. australia standard time (utc+10:00) brisbane brisbane gold coast cairns queensland australia" },
    TimeZoneEntry { id: "AUS Eastern Standard Time", display: "(UTC+10:00) Canberra, Melbourne, Sydney", lat: -33.87, lon: 151.2, search: "aus eastern standard time (utc+10:00) canberra, melbourne, sydney canberra melbourne sydney new south wales victoria australia aest aedt" },
    TimeZoneEntry { id: "West Pacific Standard Time", display: "(UTC+10:00) Guam, Port Moresby", lat: 13.44, lon: 144.8, search: "west pacific standard time (utc+10:00) guam, port moresby guam port moresby papua new guinea saipan" },
    TimeZoneEntry { id: "Tasmania Standard Time", display: "(UTC+10:00) Hobart", lat: -42.88, lon: 147.3, search: "tasmania standard time (utc+10:00) hobart hobart launceston tasmania australia" },
    TimeZoneEntry { id: "Vladivostok Standard Time", display: "(UTC+10:00) Vladivostok", lat: 43.12, lon: 131.9, search: "vladivostok standard time (utc+10:00) vladivostok vladivostok khabarovsk russia" },
    TimeZoneEntry { id: "Lord Howe Standard Time", display: "(UTC+10:30) Lord Howe Island", lat: -31.55, lon: 159.1, search: "lord howe standard time (utc+10:30) lord howe island lord howe island australia" },
    TimeZoneEntry { id: "Bougainville Standard Time", display: "(UTC+11:00) Bougainville Island", lat: -6.21, lon: 155.5, search: "bougainville standard time (utc+11:00) bougainville island bougainville island arawa papua new guinea" },
    TimeZoneEntry { id: "Russia Time Zone 10", display: "(UTC+11:00) Chokurdakh", lat: 70.62, lon: 147.9, search: "russia time zone 10 (utc+11:00) chokurdakh chokurdakh sakha russia" },
    TimeZoneEntry { id: "Magadan Standard Time", display: "(UTC+11:00) Magadan", lat: 59.56, lon: 150.8, search: "magadan standard time (utc+11:00) magadan magadan russia" },
    TimeZoneEntry { id: "Norfolk Standard Time", display: "(UTC+11:00) Norfolk Island", lat: -29.04, lon: 167.9, search: "norfolk standard time (utc+11:00) norfolk island norfolk island australia" },
    TimeZoneEntry { id: "Sakhalin Standard Time", display: "(UTC+11:00) Sakhalin", lat: 46.96, lon: 142.7, search: "sakhalin standard time (utc+11:00) sakhalin sakhalin yuzhno-sakhalinsk russia" },
    TimeZoneEntry { id: "Central Pacific Standard Time", display: "(UTC+11:00) Solomon Is., New Caledonia", lat: -9.45, lon: 160.0, search: "central pacific standard time (utc+11:00) solomon is., new caledonia solomon islands honiara new caledonia noumea" },
    TimeZoneEntry { id: "Russia Time Zone 11", display: "(UTC+12:00) Anadyr, Petropavlovsk-Kamchatsky", lat: 64.73, lon: 177.5, search: "russia time zone 11 (utc+12:00) anadyr, petropavlovsk-kamchatsky anadyr chukotka petropavlovsk-kamchatsky russia" },
    TimeZoneEntry { id: "New Zealand Standard Time", display: "(UTC+12:00) Auckland, Wellington", lat: -36.85, lon: 174.8, search: "new zealand standard time (utc+12:00) auckland, wellington auckland wellington christchurch new zealand nzst nzdt" },
    TimeZoneEntry { id: "UTC+12", display: "(UTC+12:00) Coordinated Universal Time+12", lat: 0.0, lon: 180.0, search: "utc+12 (utc+12:00) coordinated universal time+12 utc+12 wake island marshall islands" },
    TimeZoneEntry { id: "Fiji Standard Time", display: "(UTC+12:00) Fiji", lat: -18.14, lon: 178.4, search: "fiji standard time (utc+12:00) fiji fiji suva nadi" },
    TimeZoneEntry { id: "Kamchatka Standard Time", display: "(UTC+12:00) Petropavlovsk-Kamchatsky - Old", lat: 53.02, lon: 158.7, search: "kamchatka standard time (utc+12:00) petropavlovsk-kamchatsky - old petropavlovsk-kamchatsky kamchatka old russia" },
    TimeZoneEntry { id: "Chatham Islands Standard Time", display: "(UTC+12:45) Chatham Islands", lat: -43.95, lon: -176.6, search: "chatham islands standard time (utc+12:45) chatham islands chatham islands waitangi new zealand" },
    TimeZoneEntry { id: "UTC+13", display: "(UTC+13:00) Coordinated Universal Time+13", lat: 0.0, lon: -165.0, search: "utc+13 (utc+13:00) coordinated universal time+13 utc+13 phoenix islands kiribati rawaki" },
    TimeZoneEntry { id: "Tonga Standard Time", display: "(UTC+13:00) Nuku'alofa", lat: -21.14, lon: -175.2, search: "tonga standard time (utc+13:00) nuku'alofa nuku'alofa tonga" },
    TimeZoneEntry { id: "Samoa Standard Time", display: "(UTC+13:00) Samoa", lat: -13.83, lon: -171.8, search: "samoa standard time (utc+13:00) samoa samoa apia" },
    TimeZoneEntry { id: "Line Islands Standard Time", display: "(UTC+14:00) Kiritimati Island", lat: 1.87, lon: -157.4, search: "line islands standard time (utc+14:00) kiritimati island kiritimati christmas island line islands kiribati" },
];

/// Finds the best matching timezone for given coordinates (closest Euclidean distance).
pub fn find_timezone_by_coords(lat: f64, lon: f64) -> Option<&'static TimeZoneEntry> {
    TIMEZONES.iter().min_by(|a, b| {
        let da = (a.lat - lat).powi(2) + (a.lon - lon).powi(2);
        let db = (b.lat - lat).powi(2) + (b.lon - lon).powi(2);
        da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
    })
}

/// Finds a timezone by exact Windows TimeZone ID.
pub fn find_timezone_by_id(id: &str) -> Option<&'static TimeZoneEntry> {
    TIMEZONES.iter().find(|tz| tz.id.eq_ignore_ascii_case(id))
}

/// Matches timezones against a user search query (case-insensitive substring across ID, display name, and city/region keywords).
pub fn search_timezones<'a>(query: &str) -> Vec<&'static TimeZoneEntry> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return TIMEZONES.iter().collect();
    }
    TIMEZONES
        .iter()
        .filter(|tz| tz.search.contains(&q))
        .collect()
}

/// Detects the local system timezone from the Windows API.
pub fn local_system_timezone() -> Option<&'static TimeZoneEntry> {
    #[cfg(target_os = "windows")]
    {
        use windows::Win32::System::Time::{GetDynamicTimeZoneInformation, DYNAMIC_TIME_ZONE_INFORMATION};
        let mut tz = DYNAMIC_TIME_ZONE_INFORMATION::default();
        let res = unsafe { GetDynamicTimeZoneInformation(&mut tz) };
        if res != 0xFFFFFFFF {
            let key = String::from_utf16_lossy(&tz.TimeZoneKeyName);
            let id = key.trim_matches(' ');
            if let Some(entry) = find_timezone_by_id(id) {
                return Some(entry);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_144_timezones_present() {
        assert_eq!(TIMEZONES.len(), 144);
        for tz in TIMEZONES {
            assert!(!tz.id.is_empty());
            assert!(!tz.display.is_empty());
            assert!((-90.0..=90.0).contains(&tz.lat));
            assert!((-180.0..=180.0).contains(&tz.lon));
        }
    }

    #[test]
    fn test_search_timezones() {
        let res_ist = search_timezones("istanbul");
        assert!(res_ist.iter().any(|tz| tz.id == "Turkey Standard Time"));

        let res_ny = search_timezones("new york");
        assert!(res_ny.iter().any(|tz| tz.id == "Eastern Standard Time"));

        let res_tokyo = search_timezones("tokyo");
        assert!(res_tokyo.iter().any(|tz| tz.id == "Tokyo Standard Time"));

        let res_london = search_timezones("london");
        assert!(res_london.iter().any(|tz| tz.id == "GMT Standard Time"));

        let res_paris = search_timezones("paris");
        assert!(res_paris.iter().any(|tz| tz.id == "Romance Standard Time"));

        let res_berlin = search_timezones("berlin");
        assert!(res_berlin.iter().any(|tz| tz.id == "W. Europe Standard Time"));

        let res_utc3 = search_timezones("utc+03:00");
        assert!(!res_utc3.is_empty());
        assert!(res_utc3.iter().any(|tz| tz.id == "Turkey Standard Time"));
    }

    #[test]
    fn test_find_timezone_by_coords() {
        let ist = find_timezone_by_coords(41.01, 28.98).expect("found istanbul");
        assert_eq!(ist.id, "Turkey Standard Time");

        let ny = find_timezone_by_coords(40.71, -74.01).expect("found new york");
        assert_eq!(ny.id, "Eastern Standard Time");

        let tokyo = find_timezone_by_coords(35.68, 139.69).expect("found tokyo");
        assert_eq!(tokyo.id, "Tokyo Standard Time");
    }

    #[test]
    fn test_find_timezone_by_id() {
        assert!(find_timezone_by_id("Turkey Standard Time").is_some());
        assert!(find_timezone_by_id("Pacific Standard Time").is_some());
        assert!(find_timezone_by_id("Nonexistent Timezone").is_none());
    }
}

use chrono::Utc;
use worker::{console_log, durable, ObjectNamespace};

const LOCATION_SHARDS: [(Location, usize); 9] = [
    (Location::WNAM, 2),
    (Location::ENAM, 2),
    (Location::SAM, 2),
    (Location::WEUR, 2),
    (Location::EEUR, 2),
    (Location::APAC, 2),
    (Location::OC, 2),
    (Location::AFR, 2),
    (Location::ME, 2),
];

/// location used if no country code or if lookup fails
pub const DEFAULT_LOCATION: Location = Location::WNAM;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
/// supported digital object locations
pub enum Location {
    /// Western North America
    WNAM,
    /// Eastern North America
    ENAM,
    /// South America (not yet supported)
    SAM,
    /// Western Europe
    WEUR,
    /// Eastern Europe
    EEUR,
    /// Asia-Pacific
    APAC,
    /// Oceania
    OC,
    /// Africa (not yet supported)
    AFR,
    /// Middle East (not yet supported)
    ME,
}
impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = self.code();
        write!(f, "{}", code)
    }
}

// all durable objects using current schema
fn all_durable_object_ids() -> Vec<(Location, String)> {
    let mut res = Vec::new();
    for (location, count) in LOCATION_SHARDS {
        for shard_number in 0..count {
            res.push((location, durable_object_id(shard_number, &location)))
        }
    }

    res
}

pub fn all_stubs(ns: &ObjectNamespace) -> Result<Vec<durable::Stub>, worker::Error> {
    let mut res = Vec::new();
    for (location, name) in all_durable_object_ids() {
        let id = ns.id_from_name(&name)?;
        let stub = id.get_stub_with_location_hint(location.code())?;
        res.push(stub);
    }

    Ok(res)
}

fn durable_object_id(shard_number: usize, location: &Location) -> String {
    let x = format!("shard-{}-{}", shard_number, location.code());

    console_log!("generated shard id: {}", x);

    x
}

impl Location {
    fn code(&self) -> &'static str {
        match self {
            Location::WNAM => "wnam",
            Location::ENAM => "enam",
            Location::SAM => "sam",
            Location::WEUR => "weur",
            Location::EEUR => "eeur",
            Location::APAC => "apac",
            Location::OC => "oc",
            Location::AFR => "afr",
            Location::ME => "me",
        }
    }

    fn allocated_node_count(&self) -> usize {
        // note: will never hit unwrap_or case but better a sensible default than a panic
        LOCATION_SHARDS
            .iter()
            .find(|(l, _)| l == self)
            .map(|(_, node_count)| *node_count)
            .unwrap_or(1)
    }

    // note shard number is hash of ray id
    pub fn to_durable_object_stub(
        self,
        ns: &ObjectNamespace,
    ) -> Result<durable::Stub, worker::Error> {
        let shard_count = self.allocated_node_count();

        let shard_number = {
            use rand::{rngs::SmallRng, Rng, SeedableRng};
            let mut rng = SmallRng::seed_from_u64(Utc::now().timestamp_subsec_nanos() as u64);
            rng.random_range(..shard_count)
        };

        let id = ns.id_from_name(&durable_object_id(shard_number, &self))?;

        id.get_stub_with_location_hint(self.code())
    }
    // sourced from https://github.com/cloudflare/partykit/blob/13c138bdfaa74b7ccb0bdbe58e68cefce0ff5a5a/packages/partysub/src/server/gen-ids.ts#L79C1-L313C3
    pub fn from_country_code(code: &str) -> Option<Self> {
        let map = [
            (
                Location::WNAM,
                &[
                    "CA", // Canada
                    "US", // United States
                    "MX", // Mexico
                ][..],
            ),
            (
                Location::ENAM,
                &[
                    "BM", // Bermuda
                    "BS", // Bahamas
                    "BZ", // Belize
                    "CR", // Costa Rica
                    "CU", // Cuba
                    "DM", // Dominica
                    "DO", // Dominican Republic
                    "SV", // El Salvador
                    "GD", // Grenada
                    "GT", // Guatemala
                    "HT", // Haiti
                    "HN", // Honduras
                    "JM", // Jamaica
                    "NI", // Nicaragua
                    "PA", // Panama
                    "PR", // Puerto Rico
                    "KN", // Saint Kitts and Nevis
                    "LC", // Saint Lucia
                    "VC", // Saint Vincent and the Grenadines
                    "TT", // Trinidad and Tobago
                    "VG", // British Virgin Islands
                ][..],
            ),
            (
                Location::SAM,
                &[
                    "AR", // Argentina
                    "BO", // Bolivia
                    "BR", // Brazil
                    "CL", // Chile
                    "CO", // Colombia
                    "EC", // Ecuador
                    "FK", // Falkland Islands
                    "GY", // Guyana
                    "PY", // Paraguay
                    "PE", // Peru
                    "SR", // Suriname
                    "UY", // Uruguay
                    "VE", // Venezuela
                ][..],
            ),
            (
                Location::WEUR,
                &[
                    "AT", // Austria
                    "BE", // Belgium
                    "FR", // France
                    "DE", // Germany
                    "IE", // Ireland
                    "IT", // Italy
                    "LU", // Luxembourg
                    "MT", // Malta
                    "MC", // Monaco
                    "NL", // Netherlands
                    "PT", // Portugal
                    "SM", // San Marino
                    "ES", // Spain
                    "CH", // Switzerland
                    "GB", // United Kingdom
                ][..],
            ),
            (
                Location::EEUR,
                &[
                    "AL", // Albania
                    "BY", // Belarus
                    "BA", // Bosnia and Herzegovina
                    "BG", // Bulgaria
                    "HR", // Croatia
                    "CZ", // Czech Republic
                    "HU", // Hungary
                    "MD", // Moldova
                    "ME", // Montenegro
                    "MK", // North Macedonia
                    "PL", // Poland
                    "RO", // Romania
                    "RS", // Serbia
                    "RU", // Russia
                    "SK", // Slovakia
                    "SI", // Slovenia
                    "UA", // Ukraine
                ][..],
            ),
            (
                Location::APAC,
                &[
                    "AF", // Afghanistan
                    "AM", // Armenia
                    "AU", // Australia
                    "AZ", // Azerbaijan
                    "BD", // Bangladesh
                    "BT", // Bhutan
                    "BN", // Brunei
                    "KH", // Cambodia
                    "CN", // China
                    "CY", // Cyprus
                    "GE", // Georgia
                    "IN", // India
                    "ID", // Indonesia
                    "JP", // Japan
                    "KZ", // Kazakhstan
                    "KG", // Kyrgyzstan
                    "LA", // Laos
                    "MY", // Malaysia
                    "MV", // Maldives
                    "MN", // Mongolia
                    "MM", // Myanmar
                    "NP", // Nepal
                    "KP", // North Korea
                    "KR", // South Korea
                    "PK", // Pakistan
                    "PH", // Philippines
                    "SG", // Singapore
                    "LK", // Sri Lanka
                    "TJ", // Tajikistan
                    "TH", // Thailand
                    "TL", // Timor-Leste
                    "TR", // Turkey
                    "TM", // Turkmenistan
                    "UZ", // Uzbekistan
                    "VN", // Vietnam
                ][..],
            ),
            (
                Location::OC,
                &[
                    "AS", // American Samoa
                    "CK", // Cook Islands
                    "FJ", // Fiji
                    "PF", // French Polynesia
                    "GU", // Guam
                    "KI", // Kiribati
                    "MH", // Marshall Islands
                    "FM", // Micronesia
                    "NR", // Nauru
                    "NC", // New Caledonia
                    "NZ", // New Zealand
                    "NU", // Niue
                    "NF", // Norfolk Island
                    "MP", // Northern Mariana Islands
                    "PW", // Palau
                    "PG", // Papua New Guinea
                    "PN", // Pitcairn Islands
                    "WS", // Samoa
                    "SB", // Solomon Islands
                    "TK", // Tokelau
                    "TO", // Tonga
                    "TV", // Tuvalu
                    "VU", // Vanuatu
                    "WF", // Wallis and Futuna
                ][..],
            ),
            (
                Location::AFR,
                &[
                    "DZ", // Algeria
                    "AO", // Angola
                    "BJ", // Benin
                    "BW", // Botswana
                    "BF", // Burkina Faso
                    "BI", // Burundi
                    "CV", // Cabo Verde
                    "CM", // Cameroon
                    "CF", // Central African Republic
                    "TD", // Chad
                    "KM", // Comoros
                    "CG", // Congo - Brazzaville
                    "CD", // Congo - Kinshasa
                    "CI", // Côte d’Ivoire
                    "DJ", // Djibouti
                    "GQ", // Equatorial Guinea
                    "ER", // Eritrea
                    "SZ", // Eswatini
                    "ET", // Ethiopia
                    "GA", // Gabon
                    "GM", // Gambia
                    "GH", // Ghana
                    "GN", // Guinea
                    "GW", // Guinea-Bissau
                    "KE", // Kenya
                    "LS", // Lesotho
                    "LR", // Liberia
                    "LY", // Libya
                    "MG", // Madagascar
                    "MW", // Malawi
                    "ML", // Mali
                    "MR", // Mauritania
                    "MU", // Mauritius
                    "YT", // Mayotte
                    "MA", // Morocco
                    "MZ", // Mozambique
                    "NA", // Namibia
                    "NE", // Niger
                    "NG", // Nigeria
                    "RW", // Rwanda
                    "RE", // Réunion
                    "SH", // Saint Helena
                    "ST", // São Tomé and Príncipe
                    "SN", // Senegal
                    "SC", // Seychelles
                    "SL", // Sierra Leone
                    "SO", // Somalia
                    "ZA", // South Africa
                    "SS", // South Sudan
                    "SD", // Sudan
                    "TZ", // Tanzania
                    "TG", // Togo
                    "TN", // Tunisia
                    "UG", // Uganda
                    "ZM", // Zambia
                    "ZW", // Zimbabwe
                ][..],
            ),
            (
                Location::ME,
                &[
                    "AE", // United Arab Emirates
                    "BH", // Bahrain
                    "EG", // Egypt
                    "IR", // Iran
                    "IQ", // Iraq
                    "JO", // Jordan
                    "KW", // Kuwait
                    "LB", // Lebanon
                    "OM", // Oman
                    "PS", // Palestine
                    "QA", // Qatar
                    "SA", // Saudi Arabia
                    "SY", // Syria
                    "YE", // Yemen
                ][..],
            ),
        ];

        for (location, codes) in map {
            if codes.contains(&code) {
                return Some(location);
            }
        }

        None
    }
}

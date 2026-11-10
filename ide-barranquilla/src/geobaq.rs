//! Barranquilla's catalogue: `geobaq.barranquilla.gov.co/WS/Geovisor.ashx?cmd=get`, the themes of
//! the Alcaldía's geoviewer, each holding its maps.
//!
//! `[{"id", "title", "mapas": [{"id", "title", "tipo_servicio", "url_servicio", "descripcion",
//! "licencia", "fuente", "autor", "keywords", "visible", "servicios": [{"tipo_servicio",
//! "url_servicio"}]}]}]`. A theme's `grafica`, `botones`, `aplicaciones`, `indicadores` and
//! `recursos` are no services, so they are left out.
//!
//! The plugin shows the Alcaldía's own data, as its geoviewer shows it: the maps on the Alcaldía's
//! servers ([`own`]) that the geoviewer doesn't hide (`visible: false`). The geoviewer also lists
//! national services cut down to Barranquilla with a `where` (INVIAS, MADS, IGAC…); those are
//! another IDE's to offer.

use ide_core::entry::Raw;
use ide_core::{Entry, clean};
use rustgis_plugin::Value;

/// The catalogue's address. It answers clients whose User-Agent starts with `RustGIS/` (Cloudflare
/// turns others away) and sends no CORS header, so the plugin reads its index, never this.
pub const CATALOGUE: &str = "https://geobaq.barranquilla.gov.co/WS/Geovisor.ashx?cmd=get";

/// Where the Alcaldía publishes, as host and the start of the path: its ArcGIS Enterprise, and its
/// ArcGIS Online organisation (`urlKey: barranquilla`).
const OWN: [(&str, &str); 2] = [("miciudad.barranquilla.gov.co", "/"), ("services3.arcgis.com", "/oGYAc07w6wsvgUYr/")];

/// The maps of a `cmd=get` answer: the Alcaldía's own, those its geoviewer shows, that RustGIS
/// knows. A map's theme is its category.
pub fn parse(doc: &Value) -> Vec<Entry> {
    let themes = doc.as_array().map(Vec::as_slice).unwrap_or_default();
    clean(themes.iter().flat_map(|theme| {
        let category = theme.get("title").and_then(Value::as_str);
        let maps = theme.get("mapas").and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
        maps.iter().filter(|m| wanted(m)).filter_map(move |m| entry(m, category))
    }))
}

/// Whether the plugin shows a map: on the Alcaldía's servers ([`own`]), and not one the geoviewer
/// hides (`visible: false`).
fn wanted(m: &Value) -> bool {
    m.get("visible").and_then(Value::as_bool) != Some(false) && m.get("url_servicio").and_then(Value::as_str).is_some_and(own)
}

fn entry(m: &Value, category: Option<&str>) -> Option<Entry> {
    // Blank fields count as missing, so a blank source falls back to the author.
    let f = |key: &str| m.get(key).and_then(Value::as_str).filter(|s| !s.trim().is_empty());
    let alternates = m.get("servicios").and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
    Entry::new(&Raw {
        id: f("id"),
        name: f("title"),
        url: f("url_servicio"),
        label: f("tipo_servicio"),
        category,
        // Who made the data (DANE, Planeación…); the geoviewer's author is most often the office
        // that runs it.
        entity: f("fuente").or(f("autor")),
        licence: f("licencia"),
        description: f("descripcion"),
        keywords: f("keywords"),
        alternates: alternates.iter().filter_map(alternate).collect(),
        ..Raw::default()
    })
}

/// One of a map's other services (WMS, WFS, WMTS), with the catalogue's label.
fn alternate(s: &Value) -> Option<(Option<&str>, &str)> {
    Some((s.get("tipo_servicio").and_then(Value::as_str), s.get("url_servicio")?.as_str()?))
}

/// Whether a service is on one of the Alcaldía's servers ([`OWN`]). The host is compared without
/// case; an ArcGIS Online organisation's id keeps it.
fn own(url: &str) -> bool {
    let Some(rest) = url.strip_prefix("https://") else { return false };
    let (host, path) = rest.split_at(rest.find('/').unwrap_or(rest.len()));
    OWN.iter().any(|(h, p)| host.eq_ignore_ascii_case(h) && path.starts_with(p))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ide_core::Kind;
    use rustgis_plugin::json;

    /// Two themes as `cmd=get` gives them (2026-11-09): the Alcaldía's maps, one the geoviewer
    /// hides, and a national service filtered to Barranquilla.
    fn answer() -> Value {
        json!([
            {
                "id": "2", "title": "Planeación", "color": "#00a",
                "mapas": [
                    {
                        "id": "2-1-1",
                        "title": "Barrios",
                        "tipo_servicio": "FeatureLayer",
                        "url_servicio": "https://miciudad.barranquilla.gov.co/gis/rest/services/ordenamiento/unidadesadministrativas/MapServer/1",
                        "descripcion": "Reúne y muestra las unidades geográficas y de planeación correspondientes a los barrios.",
                        "visible": true,
                        "grafica": {},
                        "fuente": "Alcaldía del Distrito Especial, Industrial y Portuario de Barranquilla. Secretaría de Planeación.",
                        "keywords": "POT, Planeación, Barrios",
                        "licencia": "CC BY - SA 4.0",
                        "servicios": [
                            {"tipo_servicio": "WFSLayer", "url_servicio": "https://miciudad.barranquilla.gov.co/gis/services/ordenamiento/unidadesadministrativas/MapServer/WMSServer?request=GetCapabilities&service=WMS"},
                            {"tipo_servicio": "WMTSLayer", "url_servicio": "https://miciudad.barranquilla.gov.co/gis/rest/services/ordenamiento/unidadesadministrativas/MapServer/WMTS/1.0.0/WMTSCapabilities.xml"}
                        ],
                        "autor": "Alcaldía del Distrito Especial, Industrial y Portuario de Barranquilla. Secretaría de Hacienda. Gerencia de Gestión Catastral."
                    },
                    {
                        "id": "2-0-2",
                        "title": "Estratificación",
                        "tipo_servicio": "FeatureLayer",
                        "url_servicio": "https://services3.arcgis.com/oGYAc07w6wsvgUYr/arcgis/rest/services/Estrato1994_Panorama/FeatureServer/0",
                        "visible": true,
                        "licencia": "",
                        "fuente": " ",
                        "autor": "Secretaría de Planeación",
                        "keywords": null,
                        "servicios": []
                    },
                    {
                        "id": "2-0-4",
                        "title": "Estrato predominante por manzana (2018)",
                        "tipo_servicio": "FeatureLayer",
                        "url_servicio": "https://miciudad.barranquilla.gov.co/gis/rest/services/social/estrato/MapServer/0",
                        "visible": false
                    }
                ]
            },
            {
                "id": "12", "title": "Ambiente y Desarrollo Sostenible",
                "mapas": [
                    {
                        "id": "12-0-2",
                        "title": "Puntos de vertimiento de aguas residuales",
                        "tipo_servicio": "FeatureLayer",
                        "url_servicio": "https://services6.arcgis.com/hxAwRYAu9QHliJ8T/ArcGIS/rest/services/SIRH_IDEAM_SERVICIOS_gdb/FeatureServer/0/query?where=NOMMUNICIPIOPREDIO%253D'BARRANQUILLA'",
                        "visible": true
                    }
                ]
            }
        ])
    }

    #[test]
    fn reads_the_alcaldias_shown_maps() {
        let entries = parse(&answer());
        let ids: Vec<&str> = entries.iter().map(|e| e.id.as_str()).collect();
        // Not the hidden one, nor the national service.
        assert_eq!(ids, ["2-1-1", "2-0-2"]);
        let barrios = &entries[0];
        assert_eq!((barrios.name.as_str(), barrios.kind), ("Barrios", Kind::ArcGisLayer));
        assert_eq!(barrios.category.as_deref(), Some("Planeación"));
        assert_eq!(barrios.subcategory, None);
        let planeacion = "Alcaldía del Distrito Especial, Industrial y Portuario de Barranquilla. Secretaría de Planeación.";
        assert_eq!(barrios.entity.as_deref(), Some(planeacion));
        assert_eq!(barrios.licence.as_deref(), Some("CC BY - SA 4.0"));
        assert_eq!(barrios.keywords, ["POT", "Planeación", "Barrios"]);
        assert!(!barrios.alternates.is_empty());
        let estratos = &entries[1];
        // A blank source: the author stands for it. A blank licence is none.
        assert_eq!((estratos.entity.as_deref(), estratos.licence.as_deref()), (Some("Secretaría de Planeación"), None));
        assert!(estratos.keywords.is_empty() && estratos.alternates.is_empty());
    }

    #[test]
    fn the_alcaldias_servers() {
        assert!(own("https://miciudad.barranquilla.gov.co/gis/rest/services/a/MapServer/1"));
        assert!(own("https://MICIUDAD.barranquilla.gov.co/gis/rest/services/a/MapServer/1"));
        assert!(own("https://services3.arcgis.com/oGYAc07w6wsvgUYr/ArcGIS/rest/services/a/FeatureServer/0"));
        // Another organisation on the same host, and hosts that only start like the Alcaldía's.
        assert!(!own("https://services3.arcgis.com/otraOrg/arcgis/rest/services/a/FeatureServer/0"));
        assert!(!own("https://services6.arcgis.com/oGYAc07w6wsvgUYr/arcgis/rest/services/a/FeatureServer/0"));
        assert!(!own("https://miciudad.barranquilla.gov.co.example.com/gis/rest/services/a/MapServer/1"));
        assert!(!own("http://miciudad.barranquilla.gov.co/gis/rest/services/a/MapServer/1"));
        assert!(!own(""));
    }

    #[test]
    fn a_broken_answer_gives_no_services() {
        assert!(parse(&json!(null)).is_empty());
        assert!(parse(&json!({"mapas": []})).is_empty());
        assert!(parse(&json!([1, "x", null, {"title": "T", "mapas": {}}, {"mapas": [1, {"visible": true}]}])).is_empty());
    }
}

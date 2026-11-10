# IDE Barranquilla for RustGIS

A [RustGIS](https://github.com/opengeos/RustGIS) plugin to browse the catalogue of the Alcaldía de
Barranquilla's geoviewer ([GeoBAQ](https://geobaq.barranquilla.gov.co)) and add its public
services to the map: an ArcGIS layer's features, or a service's map image.

> **Unofficial.** This plugin is not made, endorsed or supported by the Alcaldía de
> Barranquilla or any of its offices. It reads the catalogue and the services the Alcaldía
> publishes openly. The Alcaldía lets clients that identify as RustGIS through its servers'
> firewall: a technical courtesy, not an endorsement of this plugin. The data belongs to the
> Alcaldía and the entities that publish it: check each service's licence before using it.
> Report problems with the plugin in this repository's
> [issues](https://github.com/jgregoriorincon/rustgis-ide-barranquilla/issues), not to the
> Alcaldía.

It needs RustGIS 0.6.0 or later, and the `edit` and `network` permissions.

## What you'll find

The Alcaldía's own data, as its geoviewer shows it: the maps on the Alcaldía's servers that the
geoviewer doesn't hide. The plugin carries them as of the date its panel shows (230 services on
2026-11-10), published by the Alcaldía's offices (the cadastre, planning, the environment…), some
of them built on DANE's figures or Sentinel-2 and SPOT-5 satellite images. The national services
the geoviewer cuts down to Barranquilla (INVIAS, the Ministry of Environment, IGAC…) are left out:
they are the national catalogue's.

The services are grouped by the geoviewer's themes:

| Theme | Services | For example |
|---|---|---|
| Ambiente y Desarrollo Sostenible | 55 | ecosystems, Ramsar wetlands, land cover from 1986 on, the Magdalena's bathymetry |
| Imágenes | 43 | orthoimages of the district from 1986 to 2026, down to 8 cm |
| Catastro | 22 | cadastral blocks and sectors, land use, median cadastral value, street addresses |
| Planeación | 15 | neighbourhoods, localities, urban areas, socioeconomic strata, land-use rules, postcodes |
| Observatorio inmobiliario | 14 | land values, property transactions, rents, building permits |
| Cultura y patrimonio | 12 | Carnival events and parades, heritage, libraries, museums, theatres |
| Tránsito y seguridad vial | 10 | cycle paths, bus and Transmetro routes, traffic lights, traffic cameras |
| Social | 9 | multidimensional poverty, housing deficit, population density |
| Others (15 themes) | 50 | risk, public space, sport, tourism, education, health… |

What a service is decides what you can do with it:

- **ArcGIS layers** (142): points, lines or polygons with their attributes. Add them as features
  to query, style, select, analyse and export them like any layer; or as a map image.
- **ArcGIS map and image services** (88): whole map services, and the orthoimages. Add them as a
  map image: the server draws them, over the basemap.

Most services are licensed CC BY-SA 4.0 (191); 37 state no licence. The licence is on each
service's details.

## Use

The plugin has a ribbon tab of its own, **IDE Barranquilla**.

### The catalogue

1. **Catalogue** opens the Barranquilla Catalogue panel: the geoviewer's themes as a tree, each
   with how many services it holds, and a search box above it that finds services by name.
2. Choose a service. Its details show what it is, the scales its server draws it at, its theme,
   who publishes it, its licence, its description and its address.
3. **Add Features** reads the layer's description, takes the map to its extent and adds all its
   features, as any viewer does. RustGIS reads them in pages, and says if a server left some
   out. **Add Map Image** adds the server's own drawing of the service, over the basemap: for
   the orthoimages, whole map services, or a quick look at a big layer.
4. Adding a service again replaces the layer the plugin added before, never one you added
   yourself (even the same service under the same name).

### The ribbon

- **Reference Layers** adds, with one click, layers most maps of Barranquilla start from, as
  features: neighbourhoods and socioeconomic strata, and the menus **Boundaries** (localities,
  urban areas), **Facilities** (schools, the public health network, police stations, citizen
  service offices) and **Transmetro** (stations, routes).
- **Basemap** puts one of the Alcaldía's basemaps under the map's layers: Vector, Dark and
  Tourism. A map away from Barranquilla goes there.

### In scripts and the web app

The commands work from RustGIS scripts and `rustgis-cli`, naming a service by its id in the
catalogue: `plugin:ide-barranquilla/add={"service": "2-1-1"}` adds the neighbourhoods.

The Alcaldía's servers answer the RustGIS desktop app and `rustgis-cli`, which send a User-Agent
that starts with `RustGIS/`. The web app can't send one, so there the services may not load.

## Install

From the [plugin registry](https://plugins.rustgis.app), in RustGIS's **Plugins › Plugin
Manager**, once it is listed. Or download the `.rgplugin` from a
[release](https://github.com/jgregoriorincon/rustgis-ide-barranquilla/releases) and install it
from **Plugin Manager › Settings › Install From File…**.

## How it works

The plugin carries an index of the catalogue (`ide-barranquilla/index.json`): reading the
catalogue live doesn't fit in a plugin command's time. A workflow checks the catalogue once a
month (or when run by hand) and, when it changed, opens a pull request with the new index to
review and release; the panel shows the date of the index it carries. The services themselves
are always read live, from the Alcaldía's servers.

Reading catalogues is shared with the other `ide-*` plugins, in
[rustgis-ide-core](https://github.com/jgregoriorincon/rustgis-ide-core).

## Develop

You need [Rust](https://rustup.rs) and Python 3.11 or later.

```sh
rustup target add wasm32-unknown-unknown
cargo test --workspace
python scripts/package.py ide-barranquilla   # builds dist/ide-barranquilla and its .rgplugin
```

Load `dist/ide-barranquilla` in RustGIS from **Plugin Manager › Settings › Add Plugin Folder**.

Rebuild the index by hand (the catalogue answers only a User-Agent that starts with `RustGIS/`):

```sh
curl -s -A 'RustGIS/ide-barranquilla-index' 'https://geobaq.barranquilla.gov.co/WS/Geovisor.ashx?cmd=get' \
  | cargo run -q -p ide-barranquilla --example index -- "$(date -u +%F)" ide-barranquilla/index.json > /tmp/index.json
```

## Release

Bump `version` in `ide-barranquilla/plugin.json` and `ide-barranquilla/Cargo.toml`, then push a
tag `ide-barranquilla-v<version>`. The release carries the package and the registry entry to
submit to [opengeos/rustgis-plugins](https://github.com/opengeos/rustgis-plugins).

## License

The plugin's code is [MIT](LICENSE). The data belongs to the Alcaldía de Barranquilla and the
entities that publish it, under each service's own licence.

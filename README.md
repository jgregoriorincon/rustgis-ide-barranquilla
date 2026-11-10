# IDE Barranquilla for RustGIS

A [RustGIS](https://github.com/opengeos/RustGIS) plugin to browse the catalogue of the Alcaldía de
Barranquilla's geoviewer ([GeoBAQ](https://geobaq.barranquilla.gov.co)) and add its public
services to the map: an ArcGIS layer's features, or a service's map image. Adding a layer
takes the map to its extent, as any viewer does, and adding it again replaces the one added
before.

It shows the Alcaldía's own data, as its geoviewer shows it: the maps on the Alcaldía's servers
that the geoviewer doesn't hide. The national services the geoviewer cuts down to Barranquilla
(INVIAS, the Ministry of Environment, IGAC…) are left out.

It needs RustGIS 0.6.0 or later, and the `edit` and `network` permissions.

## Use

The plugin has a ribbon tab of its own, **IDE Barranquilla**:

- **Catalogue** opens the Barranquilla Catalogue panel: the catalogue as a tree of the
  geoviewer's themes, with a search box, the chosen service's details (with the scales it is
  drawn at) and the buttons that apply to it, **Add Features** and **Add Map Image**.
- **Reference Layers** adds, with one click, layers most maps of Barranquilla start from, as
  features: neighbourhoods and socioeconomic strata, and the menus
  **Boundaries** (localities, urban areas), **Facilities** (schools, the public health network,
  police stations, citizen service offices) and **Transmetro** (stations, routes).
- **Basemap** puts one of the Alcaldía's basemaps under the map's layers: Vector, Dark and
  Tourism. A map away from Barranquilla goes there.
- **Update to View** adds the catalogue's layers already on the map again in the map view. A
  layer holds the features of the view it was added in: moving the map doesn't fetch more.

## Install

From the [plugin registry](https://plugins.rustgis.app), in RustGIS's **Plugins › Plugin
Manager**, once it is listed. Or download the `.rgplugin` from a
[release](https://github.com/jgregoriorincon/rustgis-ide-barranquilla/releases) and install it
from **Plugin Manager › Settings › Install From File…**.

The Alcaldía's servers answer the RustGIS desktop app and `rustgis-cli`, which send a User-Agent
that starts with `RustGIS/`. The web app can't send one, so there the services may not load.

## How it works

The plugin carries an index of the catalogue (`ide-barranquilla/index.json`): reading the
catalogue live doesn't fit in a plugin command's time. A weekly workflow rebuilds the index
and, when the catalogue changed, opens a pull request with it to review and release.

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

[MIT](LICENSE). The data belongs to the Alcaldía de Barranquilla and the entities that publish
it: check each service's licence before using it.

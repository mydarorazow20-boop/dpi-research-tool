# DPI Research Tool

Android, Windows ve Linux üzerinde, root gerektirmeden çalışması hedeflenen açık kaynaklı bir ağ bağlantısı tanılama ve DPI araştırma aracı.

**Durum:** Phase 00 — yalnızca proje iskeleti. Henüz ağ, proxy, VPN veya paket işleme kodu yoktur.

## Yapı

| Dizin | Açıklama |
|-------|----------|
| `core/` | Ortak Rust çekirdek kütüphanesi (`dpi-research-core`) |
| `cli/` | Masaüstü komut satırı uygulaması (`dpi-research-cli`) |

Android arayüzü (Kotlin) ileriki aşamalarda çekirdeği kullanacak şekilde eklenecektir.

## Kullanım

Rust araç zinciri (stable, 1.74+) gerekir.

```sh
cargo run -p dpi-research-cli
```

Beklenen çıktı:

```
DPI Research Tool v0.0.1
Phase 00: project skeleton ready
```

## Geliştirme kontrolleri

```sh
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

Aynı kontroller `.github/workflows/ci.yml` ile GitHub Actions üzerinde çalışır.

## Lisans

[MIT](LICENSE)

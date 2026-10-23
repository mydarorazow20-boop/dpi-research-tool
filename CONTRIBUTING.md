# Katkıda Bulunma

1. Değişiklik için bir issue açın veya mevcut bir issue'ya yorum yapın.
2. Küçük, tek amaçlı pull request'ler gönderin.
3. Göndermeden önce şunların geçtiğinden emin olun:

   ```sh
   cargo fmt --all -- --check
   cargo test --workspace
   cargo clippy --workspace --all-targets -- -D warnings
   ```

4. Yeni davranış için test ekleyin. Bağımlılık eklemeden önce gerekçesini belirtin; bağımlılıklar minimumda tutulur.
5. Katkılarınız MIT lisansı altında yayımlanır.

use gpui::*;
use std::collections::HashMap;
use std::path::Path;

use super::theme::BoardTheme;

/// Asset loader for board textures and images
/// Handles loading and caching of texture assets for the Go board
#[derive(Clone)]
pub struct TextureAssetLoader {
    loaded_assets: HashMap<String, SharedString>,
    failed_assets: HashMap<String, String>, // path -> error message
}

impl Default for TextureAssetLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl TextureAssetLoader {
    /// Creates a new texture asset loader
    pub fn new() -> Self {
        Self {
            loaded_assets: HashMap::new(),
            failed_assets: HashMap::new(),
        }
    }

    /// Loads a texture from a file path
    /// Returns the asset path for GPUI image rendering
    pub fn load_texture(&mut self, path: &str) -> Result<SharedString, String> {
        // Check if we've already loaded this asset
        if let Some(asset_path) = self.loaded_assets.get(path) {
            return Ok(asset_path.clone());
        }

        // Check if we've already failed to load this asset
        if let Some(error) = self.failed_assets.get(path) {
            return Err(error.clone());
        }

        // Validate the path exists and is a supported image format
        let path_obj = Path::new(path);
        if !path_obj.exists() {
            let error = format!("Texture file not found: {}", path);
            self.failed_assets.insert(path.to_string(), error.clone());
            return Err(error);
        }

        // Check file extension for supported formats
        let supported_extensions = ["png", "jpg", "jpeg", "gif", "webp", "svg"];
        let extension = path_obj
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase());

        match extension {
            Some(ext) if supported_extensions.contains(&ext.as_str()) => {
                // Convert path to SharedString for GPUI
                let asset_path = SharedString::from(path.to_string());
                self.loaded_assets
                    .insert(path.to_string(), asset_path.clone());
                Ok(asset_path)
            }
            Some(ext) => {
                let error = format!("Unsupported texture format: {}", ext);
                self.failed_assets.insert(path.to_string(), error.clone());
                Err(error)
            }
            None => {
                let error = "No file extension found".to_string();
                self.failed_assets.insert(path.to_string(), error.clone());
                Err(error)
            }
        }
    }

    /// Preloads multiple textures
    pub fn preload_textures(
        &mut self,
        paths: &[&str],
    ) -> Vec<(String, Result<SharedString, String>)> {
        paths
            .iter()
            .map(|path| (path.to_string(), self.load_texture(path)))
            .collect()
    }

    /// Gets a loaded texture asset path
    pub fn get_texture(&self, path: &str) -> Option<&SharedString> {
        self.loaded_assets.get(path)
    }

    /// Checks if a texture failed to load
    pub fn get_load_error(&self, path: &str) -> Option<&String> {
        self.failed_assets.get(path)
    }

    /// Clears the asset cache
    pub fn clear_cache(&mut self) {
        self.loaded_assets.clear();
        self.failed_assets.clear();
    }

    /// Gets statistics about loaded assets
    pub fn get_stats(&self) -> (usize, usize) {
        (self.loaded_assets.len(), self.failed_assets.len())
    }
}

/// Enhanced theme adapter with texture support
pub struct TextureThemeAdapter {
    asset_loader: TextureAssetLoader,
    css_adapter: super::theme_adapter::ThemeCSSAdapter,
}

impl TextureThemeAdapter {
    /// Creates a new texture theme adapter
    pub fn new(theme: &BoardTheme) -> Self {
        Self {
            asset_loader: TextureAssetLoader::new(),
            css_adapter: super::theme_adapter::ThemeCSSAdapter::from_theme(theme),
        }
    }

    /// Preloads all textures from the theme
    pub fn preload_theme_textures(&mut self, theme: &BoardTheme) -> Vec<String> {
        let mut errors = Vec::new();
        let mut paths_to_load = Vec::new();

        // Collect all texture paths
        if let Some(ref path) = theme.board_texture {
            paths_to_load.push(path.as_str());
        }
        if let Some(ref path) = theme.black_stone_texture {
            paths_to_load.push(path.as_str());
        }
        if let Some(ref path) = theme.white_stone_texture {
            paths_to_load.push(path.as_str());
        }
        for path in &theme.stone_variation_textures {
            paths_to_load.push(path.as_str());
        }

        // Load all textures
        let results = self.asset_loader.preload_textures(&paths_to_load);
        for (path, result) in results {
            if let Err(error) = result {
                errors.push(format!("{}: {}", path, error));
            }
        }

        errors
    }

    /// Applies board background with texture support
    pub fn apply_board_background_with_texture<E: Styled>(
        &self,
        element: E,
        theme: &BoardTheme,
    ) -> E {
        let mut styled = element.bg(theme.board_background_color);

        // Apply texture if available and loaded
        if let Some(ref texture_path) = theme.board_texture {
            if let Some(_asset_path) = self.asset_loader.get_texture(texture_path) {
                // Note: GPUI doesn't have direct background-image support like CSS
                // For now, we'll use the background color as fallback
                // In a real implementation, you'd need to create a custom element
                // with image rendering capability
                styled = styled.bg(theme.board_background_color);
            }
        }

        styled
    }

    /// Applies stone styling with texture support
    pub fn apply_stone_with_texture<E: Styled>(
        &self,
        element: E,
        theme: &BoardTheme,
        is_black: bool,
        variation_index: Option<usize>,
    ) -> E {
        let base_color = if is_black {
            theme.black_stone_color
        } else {
            theme.white_stone_color
        };

        let mut styled = element.bg(base_color);

        // Apply base stone texture if available
        let texture_path = if is_black {
            &theme.black_stone_texture
        } else {
            &theme.white_stone_texture
        };

        if let Some(ref path) = texture_path {
            if let Some(_asset_path) = self.asset_loader.get_texture(path) {
                // Texture is loaded and ready
                // For GPUI, we'd need to create a custom image element
                styled = styled.bg(base_color);
            }
        }

        // Apply variation texture if specified and available
        if let Some(index) = variation_index {
            if index < theme.stone_variation_textures.len() {
                let variation_path = &theme.stone_variation_textures[index];
                if let Some(_asset_path) = self.asset_loader.get_texture(variation_path) {
                    // Apply variation texture
                    styled = styled.bg(base_color);
                }
            }
        }

        // Apply border if configured
        if theme.stone_border_width > 0.0 {
            styled = styled.border_color(theme.stone_border_color).border_1();
        }

        styled
    }

    /// Creates an image element for board background texture
    pub fn create_board_texture_element(&self, theme: &BoardTheme) -> Option<impl IntoElement> {
        if let Some(ref texture_path) = theme.board_texture {
            if let Some(asset_path) = self.asset_loader.get_texture(texture_path) {
                // Create an image element with the loaded texture
                return Some(
                    img(asset_path.clone())
                        .w_full()
                        .h_full()
                        .object_fit(gpui::ObjectFit::Cover),
                );
            }
        }
        None
    }

    /// Creates an image element for stone texture
    pub fn create_stone_texture_element(
        &self,
        theme: &BoardTheme,
        is_black: bool,
        variation_index: Option<usize>,
        size: Pixels,
    ) -> Option<impl IntoElement> {
        // Check for variation texture first
        if let Some(index) = variation_index {
            if index < theme.stone_variation_textures.len() {
                let variation_path = &theme.stone_variation_textures[index];
                if let Some(asset_path) = self.asset_loader.get_texture(variation_path) {
                    return Some(
                        img(asset_path.clone())
                            .size(size)
                            .object_fit(gpui::ObjectFit::Cover),
                    );
                }
            }
        }

        // Fall back to base stone texture
        let texture_path = if is_black {
            &theme.black_stone_texture
        } else {
            &theme.white_stone_texture
        };

        if let Some(ref path) = texture_path {
            if let Some(asset_path) = self.asset_loader.get_texture(path) {
                return Some(
                    img(asset_path.clone())
                        .size(size)
                        .object_fit(gpui::ObjectFit::Cover),
                );
            }
        }

        None
    }

    /// Gets the underlying asset loader
    pub fn asset_loader(&self) -> &TextureAssetLoader {
        &self.asset_loader
    }

    /// Gets a mutable reference to the asset loader
    pub fn asset_loader_mut(&mut self) -> &mut TextureAssetLoader {
        &mut self.asset_loader
    }

    /// Gets the CSS adapter
    pub fn css_adapter(&self) -> &super::theme_adapter::ThemeCSSAdapter {
        &self.css_adapter
    }
}

/// Utility functions for texture management
pub struct TextureUtils;

impl TextureUtils {
    /// Generates random variation index for stone textures
    pub fn get_random_variation_index(
        vertex_x: usize,
        vertex_y: usize,
        variation_count: usize,
    ) -> Option<usize> {
        if variation_count == 0 {
            return None;
        }

        // Use deterministic pseudo-random based on position for consistent appearance
        let seed = (vertex_x.wrapping_mul(31) ^ vertex_y.wrapping_mul(17)) as u32;
        let index = (seed % variation_count as u32) as usize;
        Some(index)
    }

    /// Creates standard variation texture paths (random_0 through random_4)
    pub fn create_standard_variation_paths(base_path: &str) -> Vec<String> {
        (0..5)
            .map(|i| format!("{}/random_{}.png", base_path, i))
            .collect()
    }

    /// Validates texture file paths
    pub fn validate_texture_paths(paths: &[String]) -> Vec<String> {
        let mut errors = Vec::new();

        for path in paths {
            let path_obj = Path::new(path);
            if !path_obj.exists() {
                errors.push(format!("Texture file not found: {}", path));
            }
        }

        errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    // Helper function to create temporary test files
    fn create_temp_file(path: &str, content: &[u8]) -> Result<(), std::io::Error> {
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = fs::File::create(path)?;
        file.write_all(content)?;
        Ok(())
    }

    // Helper function to cleanup temp files
    fn cleanup_temp_file(path: &str) {
        fs::remove_file(path).ok();
        if let Some(parent) = Path::new(path).parent() {
            fs::remove_dir_all(parent).ok();
        }
    }

    #[test]
    fn test_texture_asset_loader_creation() {
        let loader = TextureAssetLoader::new();
        let (loaded, failed) = loader.get_stats();
        assert_eq!(loaded, 0);
        assert_eq!(failed, 0);
    }

    #[test]
    fn test_texture_asset_loader_default() {
        let loader = TextureAssetLoader::default();
        let (loaded, failed) = loader.get_stats();
        assert_eq!(loaded, 0);
        assert_eq!(failed, 0);
    }

    #[test]
    fn test_load_supported_formats() {
        let mut loader = TextureAssetLoader::new();
        let supported_formats = ["png", "jpg", "jpeg", "gif", "webp", "svg"];

        for format in &supported_formats {
            let temp_path = format!("/tmp/test_texture.{}", format);
            create_temp_file(&temp_path, b"fake image data").unwrap();

            let result = loader.load_texture(&temp_path);
            assert!(
                result.is_ok(),
                "Failed to load {} format: {:?}",
                format,
                result
            );

            // Verify asset is cached
            assert!(loader.get_texture(&temp_path).is_some());

            cleanup_temp_file(&temp_path);
        }
    }

    #[test]
    fn test_unsupported_format() {
        let mut loader = TextureAssetLoader::new();

        // Create a temporary file with unsupported extension
        let temp_path = "/tmp/test_unsupported.txt";
        create_temp_file(temp_path, b"test").unwrap();

        let result = loader.load_texture(temp_path);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unsupported texture format"));

        // Verify error is cached
        assert!(loader.get_load_error(temp_path).is_some());

        cleanup_temp_file(temp_path);
    }

    #[test]
    fn test_no_file_extension() {
        let mut loader = TextureAssetLoader::new();

        let temp_path = "/tmp/test_no_extension";
        create_temp_file(temp_path, b"test").unwrap();

        let result = loader.load_texture(temp_path);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("No file extension found"));

        cleanup_temp_file(temp_path);
    }

    #[test]
    fn test_nonexistent_file() {
        let mut loader = TextureAssetLoader::new();
        let result = loader.load_texture("/nonexistent/path/to/image.png");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Texture file not found"));

        // Verify error is cached
        assert!(loader
            .get_load_error("/nonexistent/path/to/image.png")
            .is_some());
    }

    #[test]
    fn test_texture_caching() {
        let mut loader = TextureAssetLoader::new();
        let temp_path = "/tmp/test_cache.png";
        create_temp_file(temp_path, b"fake png data").unwrap();

        // First load
        let result1 = loader.load_texture(temp_path);
        assert!(result1.is_ok());

        // Second load should return cached result
        let result2 = loader.load_texture(temp_path);
        assert!(result2.is_ok());
        assert_eq!(result1.unwrap(), result2.unwrap());

        // Verify stats
        let (loaded, failed) = loader.get_stats();
        assert_eq!(loaded, 1);
        assert_eq!(failed, 0);

        cleanup_temp_file(temp_path);
    }

    #[test]
    fn test_error_caching() {
        let mut loader = TextureAssetLoader::new();
        let nonexistent_path = "/nonexistent/image.png";

        // First load attempt
        let result1 = loader.load_texture(nonexistent_path);
        assert!(result1.is_err());

        // Second load attempt should return cached error
        let result2 = loader.load_texture(nonexistent_path);
        assert!(result2.is_err());
        assert_eq!(result1.unwrap_err(), result2.unwrap_err());

        // Verify stats
        let (loaded, failed) = loader.get_stats();
        assert_eq!(loaded, 0);
        assert_eq!(failed, 1);
    }

    #[test]
    fn test_preload_textures() {
        let mut loader = TextureAssetLoader::new();

        // Create temp files
        let temp_path1 = "/tmp/test_preload1.png";
        let temp_path2 = "/tmp/test_preload2.jpg";
        let nonexistent_path = "/tmp/nonexistent.png";

        create_temp_file(temp_path1, b"fake png data").unwrap();
        create_temp_file(temp_path2, b"fake jpg data").unwrap();

        let paths = [temp_path1, temp_path2, nonexistent_path];
        let results = loader.preload_textures(&paths);

        assert_eq!(results.len(), 3);
        assert!(results[0].1.is_ok()); // temp_path1 should succeed
        assert!(results[1].1.is_ok()); // temp_path2 should succeed
        assert!(results[2].1.is_err()); // nonexistent_path should fail

        // Verify caching
        assert!(loader.get_texture(temp_path1).is_some());
        assert!(loader.get_texture(temp_path2).is_some());
        assert!(loader.get_load_error(nonexistent_path).is_some());

        cleanup_temp_file(temp_path1);
        cleanup_temp_file(temp_path2);
    }

    #[test]
    fn test_clear_cache() {
        let mut loader = TextureAssetLoader::new();
        let temp_path = "/tmp/test_clear.png";
        create_temp_file(temp_path, b"fake png data").unwrap();

        // Load a texture
        loader.load_texture(temp_path).unwrap();
        let (loaded, _) = loader.get_stats();
        assert_eq!(loaded, 1);

        // Clear cache
        loader.clear_cache();
        let (loaded, failed) = loader.get_stats();
        assert_eq!(loaded, 0);
        assert_eq!(failed, 0);

        // Verify texture is no longer cached
        assert!(loader.get_texture(temp_path).is_none());

        cleanup_temp_file(temp_path);
    }

    #[test]
    fn test_case_insensitive_extensions() {
        let mut loader = TextureAssetLoader::new();

        // Test uppercase extensions
        let temp_path = "/tmp/test_case.PNG";
        create_temp_file(temp_path, b"fake png data").unwrap();

        let result = loader.load_texture(temp_path);
        assert!(result.is_ok());

        cleanup_temp_file(temp_path);
    }

    // TextureUtils tests
    #[test]
    fn test_random_variation_index_deterministic() {
        // Test deterministic behavior - same inputs should always give same outputs
        let index1 = TextureUtils::get_random_variation_index(5, 7, 4);
        let index2 = TextureUtils::get_random_variation_index(5, 7, 4);
        assert_eq!(index1, index2);

        // Test bounds
        if let Some(index) = index1 {
            assert!(index < 4);
        }
    }

    #[test]
    fn test_random_variation_index_distribution() {
        // Test that different positions produce different indices
        let mut indices = std::collections::HashSet::new();

        for x in 0..10 {
            for y in 0..10 {
                if let Some(index) = TextureUtils::get_random_variation_index(x, y, 5) {
                    indices.insert(index);
                }
            }
        }

        // Should have generated multiple different indices
        assert!(indices.len() > 1);
        assert!(indices.len() <= 5);

        // All indices should be in valid range
        for index in indices {
            assert!(index < 5);
        }
    }

    #[test]
    fn test_random_variation_index_edge_cases() {
        // Test empty variations
        let index = TextureUtils::get_random_variation_index(5, 7, 0);
        assert!(index.is_none());

        // Test single variation
        let index = TextureUtils::get_random_variation_index(5, 7, 1);
        assert_eq!(index, Some(0));

        // Test large coordinates
        let index = TextureUtils::get_random_variation_index(usize::MAX, usize::MAX, 3);
        assert!(index.is_some());
        if let Some(idx) = index {
            assert!(idx < 3);
        }
    }

    #[test]
    fn test_standard_variation_paths() {
        let paths = TextureUtils::create_standard_variation_paths("assets/stones");
        assert_eq!(paths.len(), 5);
        assert_eq!(paths[0], "assets/stones/random_0.png");
        assert_eq!(paths[1], "assets/stones/random_1.png");
        assert_eq!(paths[2], "assets/stones/random_2.png");
        assert_eq!(paths[3], "assets/stones/random_3.png");
        assert_eq!(paths[4], "assets/stones/random_4.png");

        // Test with different base path
        let paths2 = TextureUtils::create_standard_variation_paths("textures/stone_variations");
        assert_eq!(paths2[0], "textures/stone_variations/random_0.png");
    }

    #[test]
    fn test_validate_texture_paths() {
        // Create some temp files
        let temp_path1 = "/tmp/test_validate1.png";
        let temp_path2 = "/tmp/test_validate2.jpg";
        create_temp_file(temp_path1, b"fake data").unwrap();
        create_temp_file(temp_path2, b"fake data").unwrap();

        let paths = vec![
            temp_path1.to_string(),
            temp_path2.to_string(),
            "/nonexistent1.png".to_string(),
            "/nonexistent2.jpg".to_string(),
        ];

        let errors = TextureUtils::validate_texture_paths(&paths);
        assert_eq!(errors.len(), 2);
        assert!(errors[0].contains("nonexistent1.png"));
        assert!(errors[1].contains("nonexistent2.jpg"));

        cleanup_temp_file(temp_path1);
        cleanup_temp_file(temp_path2);
    }

    // TextureThemeAdapter tests
    #[test]
    fn test_texture_theme_adapter_creation() {
        let theme = BoardTheme::default();
        let adapter = TextureThemeAdapter::new(&theme);

        let (loaded, failed) = adapter.asset_loader().get_stats();
        assert_eq!(loaded, 0);
        assert_eq!(failed, 0);
    }

    #[test]
    fn test_preload_theme_textures_empty() {
        let theme = BoardTheme::default(); // No textures
        let mut adapter = TextureThemeAdapter::new(&theme);

        let errors = adapter.preload_theme_textures(&theme);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_preload_theme_textures_with_paths() {
        let theme = BoardTheme::default().with_stone_textures(
            Some("nonexistent_black.png".to_string()),
            Some("nonexistent_white.png".to_string()),
        );

        let mut adapter = TextureThemeAdapter::new(&theme);
        let errors = adapter.preload_theme_textures(&theme);

        // Should have errors for nonexistent files
        assert_eq!(errors.len(), 2);
        assert!(errors[0].contains("nonexistent_black.png"));
        assert!(errors[1].contains("nonexistent_white.png"));
    }

    #[test]
    fn test_preload_theme_textures_with_variations() {
        let temp_path = "/tmp/test_stone.png";
        create_temp_file(temp_path, b"fake data").unwrap();

        let theme = BoardTheme::default()
            .with_board_texture(temp_path.to_string())
            .with_stone_variations(vec![temp_path.to_string(), "nonexistent.png".to_string()]);

        let mut adapter = TextureThemeAdapter::new(&theme);
        let errors = adapter.preload_theme_textures(&theme);

        // Should have one error for nonexistent variation
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("nonexistent.png"));

        // Verify the good texture was loaded
        assert!(adapter.asset_loader().get_texture(temp_path).is_some());

        cleanup_temp_file(temp_path);
    }

    #[test]
    fn test_create_board_texture_element() {
        let temp_path = "/tmp/test_board.png";
        create_temp_file(temp_path, b"fake data").unwrap();

        let theme = BoardTheme::default().with_board_texture(temp_path.to_string());
        let mut adapter = TextureThemeAdapter::new(&theme);

        // Load the texture first
        adapter.asset_loader_mut().load_texture(temp_path).unwrap();

        // Test creating texture element
        let element = adapter.create_board_texture_element(&theme);
        assert!(element.is_some());

        // Test with no texture
        let theme_no_texture = BoardTheme::default();
        let element_none = adapter.create_board_texture_element(&theme_no_texture);
        assert!(element_none.is_none());

        cleanup_temp_file(temp_path);
    }

    #[test]
    fn test_create_stone_texture_element() {
        let temp_path = "/tmp/test_stone.png";
        let variation_path = "/tmp/test_variation.png";
        create_temp_file(temp_path, b"fake data").unwrap();
        create_temp_file(variation_path, b"fake variation data").unwrap();

        let theme = BoardTheme::default()
            .with_stone_textures(Some(temp_path.to_string()), None)
            .with_stone_variations(vec![variation_path.to_string()]);

        let mut adapter = TextureThemeAdapter::new(&theme);

        // Load textures first
        adapter.asset_loader_mut().load_texture(temp_path).unwrap();
        adapter
            .asset_loader_mut()
            .load_texture(variation_path)
            .unwrap();

        // Test with variation index
        let element = adapter.create_stone_texture_element(&theme, true, Some(0), px(30.0));
        assert!(element.is_some()); // Should use variation texture

        // Test without variation (should use base texture)
        let element = adapter.create_stone_texture_element(&theme, true, None, px(30.0));
        assert!(element.is_some()); // Should use base black stone texture

        // Test white stone (no texture defined)
        let element = adapter.create_stone_texture_element(&theme, false, None, px(30.0));
        assert!(element.is_none()); // No white stone texture defined

        cleanup_temp_file(temp_path);
        cleanup_temp_file(variation_path);
    }

    #[test]
    fn test_texture_theme_adapter_accessor_methods() {
        let theme = BoardTheme::default();
        let mut adapter = TextureThemeAdapter::new(&theme);

        // Test asset loader access
        let loader_ref = adapter.asset_loader();
        let (loaded, failed) = loader_ref.get_stats();
        assert_eq!(loaded, 0);
        assert_eq!(failed, 0);

        // Test mutable asset loader access
        let loader_mut = adapter.asset_loader_mut();
        loader_mut.clear_cache(); // Should not panic

        // Test CSS adapter access
        let css_adapter = adapter.css_adapter();
        assert!(css_adapter.get_root_css_class().len() > 0);
    }

    #[test]
    fn test_texture_integration_with_board_theme() {
        // Test comprehensive theme with all texture types
        let board_texture = "/tmp/test_board.png";
        let black_stone = "/tmp/test_black.png";
        let white_stone = "/tmp/test_white.png";
        let variation1 = "/tmp/test_var1.png";
        let variation2 = "/tmp/test_var2.png";

        // Create temp files
        create_temp_file(board_texture, b"board data").unwrap();
        create_temp_file(black_stone, b"black stone data").unwrap();
        create_temp_file(white_stone, b"white stone data").unwrap();
        create_temp_file(variation1, b"variation 1 data").unwrap();
        create_temp_file(variation2, b"variation 2 data").unwrap();

        let theme = BoardTheme::default()
            .with_board_texture(board_texture.to_string())
            .with_stone_textures(Some(black_stone.to_string()), Some(white_stone.to_string()))
            .with_stone_variations(vec![variation1.to_string(), variation2.to_string()]);

        let mut adapter = TextureThemeAdapter::new(&theme);
        let errors = adapter.preload_theme_textures(&theme);

        // All textures should load successfully
        assert!(errors.is_empty());

        // Verify all textures are loaded
        assert!(adapter.asset_loader().get_texture(board_texture).is_some());
        assert!(adapter.asset_loader().get_texture(black_stone).is_some());
        assert!(adapter.asset_loader().get_texture(white_stone).is_some());
        assert!(adapter.asset_loader().get_texture(variation1).is_some());
        assert!(adapter.asset_loader().get_texture(variation2).is_some());

        // Test texture element creation
        assert!(adapter.create_board_texture_element(&theme).is_some());
        assert!(adapter
            .create_stone_texture_element(&theme, true, None, px(30.0))
            .is_some());
        assert!(adapter
            .create_stone_texture_element(&theme, false, None, px(30.0))
            .is_some());
        assert!(adapter
            .create_stone_texture_element(&theme, true, Some(0), px(30.0))
            .is_some());
        assert!(adapter
            .create_stone_texture_element(&theme, true, Some(1), px(30.0))
            .is_some());

        // Cleanup
        cleanup_temp_file(board_texture);
        cleanup_temp_file(black_stone);
        cleanup_temp_file(white_stone);
        cleanup_temp_file(variation1);
        cleanup_temp_file(variation2);
    }
}

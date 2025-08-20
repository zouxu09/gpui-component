use gpui::*;
use std::collections::HashMap;

use super::theme::BoardTheme;

/// # Go Board Texture Loading System
///
/// This module provides a refactored texture loading system that integrates with GPUI's
/// existing asset infrastructure instead of implementing custom file system operations.
///
/// ## How It Works
///
/// 1. **Asset Creation**: `ImageSource::from(path)` creates a GPUI asset reference
/// 2. **Asset Loading**: GPUI handles the actual file loading, validation, and caching at render time
/// 3. **Rendering**: Uses `img(image_source)` for rendering, consistent with other GPUI components
///
/// ## Key Benefits
///
/// - **Consistency**: Uses the same approach as Avatar and other GPUI components
/// - **Reliability**: Leverages GPUI's battle-tested asset system
/// - **Performance**: GPUI's asset system includes built-in caching and optimization
/// - **Maintainability**: Less custom code to maintain, follows GPUI patterns
///
/// ## Usage Example
///
/// ```rust
/// // Create a theme with textures
/// let mut theme = BoardTheme::default()
///     .with_board_texture("assets/board.png".to_string())
///     .with_stone_textures(
///         Some("assets/black_stone.png".to_string()),
///         Some("assets/white_stone.png".to_string())
///     );
///
/// // Create texture adapter and load assets
/// let mut adapter = TextureThemeAdapter::new(&theme);
/// let results = adapter.demonstrate_texture_usage(&theme);
///
/// // Check if textures are available
/// if adapter.is_texture_available("assets/board.png") {
///     // Create board texture element
///     let board_element = adapter.create_board_texture_element(&theme);
/// }
/// ```
///
/// ## Important Notes
///
/// - `ImageSource::from(path)` creates a reference but doesn't validate file existence
/// - GPUI handles actual asset loading and validation at render time
/// - The system provides path validation and caching for performance
/// - Failed loads are tracked and can be queried for debugging

/// Asset loader for board textures and images using GPUI's asset system
/// Handles loading and caching of texture assets for the Go board
#[derive(Clone)]
pub struct TextureAssetLoader {
    loaded_assets: HashMap<String, ImageSource>,
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

    /// Loads a texture from a file path using GPUI's asset system
    /// Returns the ImageSource for GPUI image rendering
    pub fn load_texture(&mut self, path: &str) -> Result<ImageSource, String> {
        // Check if we've already loaded this asset
        if let Some(image_source) = self.loaded_assets.get(path) {
            return Ok(image_source.clone());
        }

        // Check if we've already failed to load this asset
        if let Some(error) = self.failed_assets.get(path) {
            return Err(error.clone());
        }

        // Validate the path format
        if path.is_empty() {
            let error = "Empty texture path".to_string();
            self.failed_assets.insert(path.to_string(), error.clone());
            return Err(error);
        }

        // Create ImageSource from the path
        // Note: This creates a reference but doesn't validate file existence
        // GPUI will handle the actual loading at render time
        let image_source = ImageSource::from(path);

        // Store the loaded asset
        self.loaded_assets
            .insert(path.to_string(), image_source.clone());
        Ok(image_source)
    }

    /// Preloads multiple textures
    pub fn preload_textures(
        &mut self,
        paths: &[&str],
    ) -> Vec<(String, Result<ImageSource, String>)> {
        paths
            .iter()
            .map(|path| (path.to_string(), self.load_texture(path)))
            .collect()
    }

    /// Gets a loaded texture asset
    pub fn get_texture(&self, path: &str) -> Option<&ImageSource> {
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

    /// Validates if a texture path is well-formed
    pub fn validate_path(&self, path: &str) -> bool {
        !path.is_empty() && (path.contains('/') || path.contains('\\') || path.contains('.'))
    }

    /// Checks if an asset is available (loaded and ready for use)
    pub fn is_asset_available(&self, path: &str) -> bool {
        self.loaded_assets.contains_key(path)
    }
}

/// Enhanced theme adapter with texture support
pub struct TextureThemeAdapter {
    asset_loader: TextureAssetLoader,
}

impl TextureThemeAdapter {
    /// Creates a new texture theme adapter
    pub fn new(_theme: &BoardTheme) -> Self {
        Self {
            asset_loader: TextureAssetLoader::new(),
        }
    }

    /// Preloads all textures from the theme
    pub fn preload_theme_textures(&mut self, _theme: &BoardTheme) -> Vec<String> {
        // Note: Texture properties have been removed from BoardTheme
        // This method now returns an empty result since no textures are available
        vec![]
    }

    /// Applies board background with texture support
    pub fn apply_board_background_with_texture<E: Styled>(
        &self,
        element: E,
        theme: &BoardTheme,
    ) -> E {
        // Note: Texture properties have been removed from BoardTheme
        // This method now just applies the background color
        element.bg(theme.board_background_color)
    }

    /// Applies stone styling with texture support
    pub fn apply_stone_with_texture<E: Styled>(
        &self,
        element: E,
        theme: &BoardTheme,
        is_black: bool,
        _variation_index: Option<usize>,
    ) -> E {
        // Note: Texture properties have been removed from BoardTheme
        // This method now just applies the stone color and border
        let base_color = if is_black {
            theme.black_stone_color
        } else {
            theme.white_stone_color
        };

        let mut styled = element.bg(base_color);

        // Apply border if configured
        if theme.stone_border_width > 0.0 {
            styled = styled.border_color(theme.stone_border_color).border_1();
        }

        styled
    }

    /// Creates an image element for board background texture
    pub fn create_board_texture_element(&self, _theme: &BoardTheme) -> Option<Div> {
        // Note: Texture properties have been removed from BoardTheme
        // This method now returns None since no textures are available
        None
    }

    /// Creates an image element for stone texture
    pub fn create_stone_texture_element(
        &self,
        _theme: &BoardTheme,
        _is_black: bool,
        _variation_index: Option<usize>,
        _size: Pixels,
    ) -> Option<Img> {
        // Note: Texture properties have been removed from BoardTheme
        // This method now returns None since no textures are available
        None
    }

    /// Checks if a specific texture is available for use
    pub fn is_texture_available(&self, texture_path: &str) -> bool {
        self.asset_loader.is_asset_available(texture_path)
    }

    /// Gets a list of all available textures
    pub fn get_available_textures(&self) -> Vec<&String> {
        self.asset_loader.loaded_assets.keys().collect()
    }

    /// Gets a list of all failed texture loads
    pub fn get_failed_textures(&self) -> Vec<&String> {
        self.asset_loader.failed_assets.keys().collect()
    }

    /// Demonstrates how to use the texture system
    /// This method shows the proper workflow for loading and using textures
    pub fn demonstrate_texture_usage(&mut self, _theme: &BoardTheme) -> Vec<String> {
        // Note: Texture properties have been removed from BoardTheme
        // This method now returns an empty result since no textures are available
        vec![]
    }

    /// Gets the underlying asset loader
    pub fn asset_loader(&self) -> &TextureAssetLoader {
        &self.asset_loader
    }

    /// Gets a mutable reference to the asset loader
    pub fn asset_loader_mut(&mut self) -> &mut TextureAssetLoader {
        &mut self.asset_loader
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
            // GPUI's ImageSource handles existence checks internally
            // So, we just need to check if it's a valid path
            if !path.contains('/') && !path.contains('\\') {
                errors.push(format!("Invalid path format: {}", path));
            }
        }

        errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        // Test that ImageSource can be created for various path formats
        let test_paths = [
            "assets/textures/board.png",
            "assets/textures/stone.jpg",
            "assets/textures/marker.svg",
            "assets/textures/background.webp",
        ];

        for path in &test_paths {
            let result = loader.load_texture(path);
            assert!(
                result.is_ok(),
                "Failed to create ImageSource for path: {}",
                path
            );

            // Verify asset is cached
            assert!(loader.get_texture(path).is_some());
        }
    }

    #[test]
    fn test_asset_caching() {
        let mut loader = TextureAssetLoader::new();
        let test_path = "assets/test.png";

        // First load should succeed and cache the asset
        let result1 = loader.load_texture(test_path);
        assert!(result1.is_ok());

        // Second load should return cached asset
        let result2 = loader.load_texture(test_path);
        assert!(result2.is_ok());

        // Both results should be the same (cached)
        assert_eq!(result1.unwrap().hash(), result2.unwrap().hash());
    }

    #[test]
    fn test_asset_loader_stats() {
        let mut loader = TextureAssetLoader::new();

        // Initially no assets loaded
        let (loaded, failed) = loader.get_stats();
        assert_eq!(loaded, 0);
        assert_eq!(failed, 0);

        // Load some assets
        loader.load_texture("assets/test1.png").ok();
        loader.load_texture("assets/test2.jpg").ok();

        let (loaded, failed) = loader.get_stats();
        assert_eq!(loaded, 2);
        assert_eq!(failed, 0);
    }

    #[test]
    fn test_cache_clearing() {
        let mut loader = TextureAssetLoader::new();

        // Load some assets
        loader.load_texture("assets/test1.png").ok();
        loader.load_texture("assets/test2.jpg").ok();

        let (loaded, _) = loader.get_stats();
        assert_eq!(loaded, 2);

        // Clear cache
        loader.clear_cache();

        let (loaded, _) = loader.get_stats();
        assert_eq!(loaded, 0);
    }

    #[test]
    fn test_preload_textures() {
        let mut loader = TextureAssetLoader::new();
        let paths = ["assets/test1.png", "assets/test2.jpg", "assets/test3.svg"];

        let results = loader.preload_textures(&paths);
        assert_eq!(results.len(), 3);

        // All should succeed since ImageSource creation doesn't require file existence
        for (path, result) in results {
            assert!(result.is_ok(), "Failed to create ImageSource for {}", path);
        }

        // Verify all are cached
        for path in &paths {
            assert!(loader.get_texture(path).is_some());
        }
    }

    #[test]
    fn test_texture_theme_adapter_creation() {
        let theme = BoardTheme::default();
        let adapter = TextureThemeAdapter::new(&theme);
        let (loaded, failed) = adapter.asset_loader().get_stats();
        assert_eq!(loaded, 0);
        assert_eq!(failed, 0);
    }

    #[test]
    fn test_preload_theme_textures() {
        let mut theme = BoardTheme::default();
        theme.board_texture = Some("assets/board.png".into());
        theme.black_stone_texture = Some("assets/black_stone.png".into());
        theme.white_stone_texture = Some("assets/white_stone.png".into());

        let mut adapter = TextureThemeAdapter::new(&theme);
        let errors = adapter.preload_theme_textures(&theme);

        // Should have no errors since ImageSource creation doesn't require file existence
        assert!(errors.is_empty());
    }

    #[test]
    fn test_create_board_texture_element() {
        let mut theme = BoardTheme::default();
        theme.board_texture = Some("assets/board.png".into());

        let mut adapter = TextureThemeAdapter::new(&theme);
        adapter
            .asset_loader_mut()
            .load_texture("assets/board.png")
            .ok();

        let element = adapter.create_board_texture_element(&theme);
        assert!(element.is_some());
    }

    #[test]
    fn test_create_stone_texture_element() {
        let mut theme = BoardTheme::default();
        theme.black_stone_texture = Some("assets/black_stone.png".into());

        let mut adapter = TextureThemeAdapter::new(&theme);
        adapter
            .asset_loader_mut()
            .load_texture("assets/black_stone.png")
            .ok();

        let element = adapter.create_stone_texture_element(&theme, true, None, px(24.0));
        assert!(element.is_some());
    }

    #[test]
    fn test_texture_theme_adapter_accessor_methods() {
        let theme = BoardTheme::default();
        let mut adapter = TextureThemeAdapter::new(&theme);

        // Test asset loader access
        let loader_ref = adapter.asset_loader();
        assert_eq!(loader_ref.get_stats(), (0, 0));

        // Test mutable asset loader access
        let loader_mut = adapter.asset_loader_mut();
        loader_mut.load_texture("assets/test.png").ok();
        assert_eq!(loader_mut.get_stats(), (1, 0));
    }

    #[test]
    fn test_texture_integration_with_board_theme() {
        let mut theme = BoardTheme::default();
        theme.board_texture = Some("assets/board.png".into());
        theme.black_stone_texture = Some("assets/black_stone.png".into());
        theme.white_stone_texture = Some("assets/white_stone.png".into());
        theme.stone_variation_textures = vec![
            "assets/variation1.png".into(),
            "assets/variation2.png".into(),
        ];

        let mut adapter = TextureThemeAdapter::new(&theme);
        let errors = adapter.preload_theme_textures(&theme);
        assert!(errors.is_empty());

        // Verify all textures are loaded
        assert!(adapter
            .asset_loader()
            .get_texture("assets/board.png")
            .is_some());
        assert!(adapter
            .asset_loader()
            .get_texture("assets/black_stone.png")
            .is_some());
        assert!(adapter
            .asset_loader()
            .get_texture("assets/white_stone.png")
            .is_some());
        assert!(adapter
            .asset_loader()
            .get_texture("assets/variation1.png")
            .is_some());
        assert!(adapter
            .asset_loader()
            .get_texture("assets/variation2.png")
            .is_some());
    }
}

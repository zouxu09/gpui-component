#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::go_board::{
        types::{BoardRange, Vertex},
        BoardTheme, GoBoard, Grid, Stone, Stones, TextureAssetLoader, TextureThemeAdapter,
        TextureUtils,
    };
    use std::fs;
    use std::io::Write;

    // Helper function to create temporary test files
    fn create_temp_file(path: &str, content: &[u8]) -> Result<(), std::io::Error> {
        if let Some(parent) = std::path::Path::new(path).parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = fs::File::create(path)?;
        file.write_all(content)?;
        Ok(())
    }

    // Helper function to cleanup temp files
    fn cleanup_temp_file(path: &str) {
        fs::remove_file(path).ok();
        if let Some(parent) = std::path::Path::new(path).parent() {
            fs::remove_dir_all(parent).ok();
        }
    }

    #[test]
    fn test_texture_integration_with_board_theme() {
        // Test that BoardTheme properly integrates with texture functionality
        let theme = BoardTheme::default()
            .with_board_texture("assets/wood.png".to_string())
            .with_stone_textures(
                Some("assets/black_stone.png".to_string()),
                Some("assets/white_stone.png".to_string()),
            )
            .with_standard_stone_variations("assets/stone_variations");

        // Verify theme properties
        assert!(theme.board_texture.is_some());
        assert!(theme.black_stone_texture.is_some());
        assert!(theme.white_stone_texture.is_some());
        assert_eq!(theme.stone_variation_textures.len(), 5);
        assert!(theme.enable_random_stone_variation);

        // Verify variation paths
        assert_eq!(
            theme.stone_variation_textures[0],
            "assets/stone_variations/random_0.png"
        );
        assert_eq!(
            theme.stone_variation_textures[4],
            "assets/stone_variations/random_4.png"
        );
    }

    #[test]
    fn test_texture_theme_adapter_with_go_board() {
        // Create a basic theme with textures
        let theme = BoardTheme::default()
            .with_stone_variations(vec!["var1.png".to_string(), "var2.png".to_string()]);

        // Create texture adapter
        let adapter = TextureThemeAdapter::new(&theme);

        // Verify adapter is properly initialized
        let (loaded, failed) = adapter.asset_loader().get_stats();
        assert_eq!(loaded, 0);
        assert_eq!(failed, 0);

        // Test that adapter can handle theme with no actual files
        // (this simulates the production scenario where files might not exist)
        let mut adapter_mut = TextureThemeAdapter::new(&theme);
        let errors = adapter_mut.preload_theme_textures(&theme);

        // Should have errors for non-existent files
        assert_eq!(errors.len(), 2);
    }

    #[test]
    fn test_stone_render_with_texture_integration() {
        let temp_path = "/tmp/test_stone_integration.png";
        create_temp_file(temp_path, b"fake stone texture").unwrap();

        let theme = BoardTheme::default()
            .with_stone_textures(Some(temp_path.to_string()), None)
            .with_stone_variations(vec![temp_path.to_string()]);

        let mut adapter = TextureThemeAdapter::new(&theme);
        adapter.asset_loader_mut().load_texture(temp_path).unwrap();

        let board_range = BoardRange::new((0, 8), (0, 8));
        let stone = Stone::new(Vertex::new(4, 4), 1, 30.0);

        // Test stone rendering with texture
        let rendered = stone.render_with_texture(&board_range, &adapter, &theme);
        assert!(rendered.is_some());

        cleanup_temp_file(temp_path);
    }

    #[test]
    fn test_grid_render_with_texture_integration() {
        let temp_path = "/tmp/test_grid_integration.png";
        create_temp_file(temp_path, b"fake board texture").unwrap();

        let theme = BoardTheme::default().with_board_texture(temp_path.to_string());
        let mut adapter = TextureThemeAdapter::new(&theme);
        adapter.asset_loader_mut().load_texture(temp_path).unwrap();

        let board_range = BoardRange::new((0, 8), (0, 8));
        let grid = Grid::new(board_range, 30.0);

        // Test grid rendering with texture
        let texture_element = adapter.create_board_texture_element(&theme);
        assert!(texture_element.is_some());

        // Grid should be able to render with texture
        let rendered = grid.render_with_texture(texture_element);
        // Should not panic and return a valid element

        cleanup_temp_file(temp_path);
    }

    #[test]
    fn test_stones_render_with_texture_integration() {
        let temp_path = "/tmp/test_stones_integration.png";
        create_temp_file(temp_path, b"fake stone texture").unwrap();

        let theme = BoardTheme::default()
            .with_stone_textures(Some(temp_path.to_string()), Some(temp_path.to_string()))
            .with_stone_variations(vec![temp_path.to_string()]);

        let mut adapter = TextureThemeAdapter::new(&theme);
        adapter.asset_loader_mut().load_texture(temp_path).unwrap();

        let board_range = BoardRange::new((0, 8), (0, 8));
        let sign_map = vec![vec![1, -1, 0], vec![0, 1, -1], vec![-1, 0, 1]];

        let stones = Stones::new(board_range, 30.0, sign_map);

        // Test stones rendering with texture
        let rendered_stones = stones.render_stones_with_texture(&adapter, &theme);

        // Should render 6 stones (3 black, 3 white)
        assert_eq!(rendered_stones.len(), 6);

        cleanup_temp_file(temp_path);
    }

    #[test]
    fn test_texture_utils_random_variation_consistency() {
        // Test that random variation is consistent across multiple calls
        let positions = vec![
            (0, 0),
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 4),
            (0, 4),
            (4, 0),
            (2, 1),
            (1, 3),
            (3, 2),
        ];

        for (x, y) in positions {
            let index1 = TextureUtils::get_random_variation_index(x, y, 5);
            let index2 = TextureUtils::get_random_variation_index(x, y, 5);
            assert_eq!(
                index1, index2,
                "Variation index should be deterministic for position ({}, {})",
                x, y
            );

            if let Some(idx) = index1 {
                assert!(idx < 5, "Variation index should be in valid range");
            }
        }
    }

    #[test]
    fn test_texture_utils_distribution() {
        // Test that variation indices are reasonably distributed
        let mut counts = vec![0; 5];

        for x in 0..20 {
            for y in 0..20 {
                if let Some(index) = TextureUtils::get_random_variation_index(x, y, 5) {
                    counts[index] += 1;
                }
            }
        }

        // Each variation should be used at least once in a 20x20 grid
        for (i, count) in counts.iter().enumerate() {
            assert!(*count > 0, "Variation {} should be used at least once", i);
        }

        // Total should equal the number of positions tested
        let total: usize = counts.iter().sum();
        assert_eq!(
            total, 400,
            "All positions should have been assigned variations"
        );
    }

    #[test]
    fn test_texture_integration_performance() {
        // Test that texture operations don't have obvious performance issues
        let theme = BoardTheme::default()
            .with_stone_variations((0..5).map(|i| format!("var_{}.png", i)).collect());

        let start = std::time::Instant::now();

        // Generate variations for a large board
        let mut variation_count = 0;
        for x in 0..19 {
            for y in 0..19 {
                if TextureUtils::get_random_variation_index(x, y, 5).is_some() {
                    variation_count += 1;
                }
            }
        }

        let duration = start.elapsed();

        assert_eq!(variation_count, 19 * 19);
        assert!(
            duration.as_millis() < 100,
            "Variation calculation should be fast, took {:?}",
            duration
        );
    }

    #[test]
    fn test_texture_fallback_behavior() {
        // Test that texture system gracefully falls back when textures are not available
        let theme = BoardTheme::default().with_stone_textures(
            Some("nonexistent_black.png".to_string()),
            Some("nonexistent_white.png".to_string()),
        );

        let adapter = TextureThemeAdapter::new(&theme);

        // Should return None for non-existent textures
        let black_element = adapter.create_stone_texture_element(&theme, true, None, px(30.0));
        let white_element = adapter.create_stone_texture_element(&theme, false, None, px(30.0));
        let board_element = adapter.create_board_texture_element(&theme);

        assert!(black_element.is_none());
        assert!(white_element.is_none());
        assert!(board_element.is_none());
    }

    #[test]
    fn test_comprehensive_texture_workflow() {
        // Test a complete workflow from theme creation to rendering
        let board_texture = "/tmp/test_board_workflow.png";
        let black_stone = "/tmp/test_black_workflow.png";
        let white_stone = "/tmp/test_white_workflow.png";
        let variation1 = "/tmp/test_var1_workflow.png";
        let variation2 = "/tmp/test_var2_workflow.png";

        // Create temp files
        create_temp_file(board_texture, b"board texture").unwrap();
        create_temp_file(black_stone, b"black stone").unwrap();
        create_temp_file(white_stone, b"white stone").unwrap();
        create_temp_file(variation1, b"variation 1").unwrap();
        create_temp_file(variation2, b"variation 2").unwrap();

        // 1. Create theme with all texture types
        let theme = BoardTheme::default()
            .with_board_texture(board_texture.to_string())
            .with_stone_textures(Some(black_stone.to_string()), Some(white_stone.to_string()))
            .with_stone_variations(vec![variation1.to_string(), variation2.to_string()]);

        // 2. Create and configure adapter
        let mut adapter = TextureThemeAdapter::new(&theme);
        let errors = adapter.preload_theme_textures(&theme);
        assert!(errors.is_empty(), "All textures should load successfully");

        // 3. Create board components
        let board_range = BoardRange::new((0, 4), (0, 4));
        let grid = Grid::new(board_range.clone(), 40.0);

        let sign_map = vec![
            vec![1, -1, 0, 1, -1],
            vec![0, 1, -1, 0, 1],
            vec![-1, 0, 1, -1, 0],
            vec![1, -1, 0, 1, -1],
            vec![0, 1, -1, 0, 1],
        ];
        let stones = Stones::new(board_range, 40.0, sign_map);

        // 4. Test board texture rendering
        let board_texture_element = adapter.create_board_texture_element(&theme);
        assert!(board_texture_element.is_some());

        // 5. Test grid rendering with texture
        let grid_element = grid.render_with_texture(board_texture_element);
        // Should not panic

        // 6. Test stone rendering with textures and variations
        let rendered_stones = stones.render_stones_with_texture(&adapter, &theme);
        assert!(rendered_stones.len() > 0);

        // 7. Test individual stone with variation
        let stone = Stone::new(Vertex::new(2, 2), 1, 40.0);
        let stone_element = stone.render_with_texture(&board_range, &adapter, &theme);
        assert!(stone_element.is_some());

        // 8. Verify texture statistics
        let (loaded, failed) = adapter.asset_loader().get_stats();
        assert_eq!(loaded, 5); // All 5 textures should be loaded
        assert_eq!(failed, 0); // No failures

        // Cleanup
        cleanup_temp_file(board_texture);
        cleanup_temp_file(black_stone);
        cleanup_temp_file(white_stone);
        cleanup_temp_file(variation1);
        cleanup_temp_file(variation2);
    }
}

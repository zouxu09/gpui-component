use crate::go_board::types::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Memory optimization and cleanup system for Go board components
/// Manages component pooling and prevents memory leaks
pub struct MemoryManager {
    /// Component pools for reuse
    stone_pool: ComponentPool<StoneComponent>,
    marker_pool: ComponentPool<MarkerComponent>,

    /// Memory usage tracking
    memory_stats: MemoryStats,

    /// Cleanup configuration
    cleanup_config: CleanupConfig,
}

/// Configuration for memory cleanup behavior
#[derive(Clone, Debug)]
pub struct CleanupConfig {
    /// Maximum pool size before cleanup
    pub max_pool_size: usize,
    /// How often to run cleanup (in milliseconds)
    pub cleanup_interval: u64,
    /// Age threshold for removing pooled components (in milliseconds)
    pub component_max_age: u64,
    /// Enable automatic memory monitoring
    pub enable_memory_monitoring: bool,
}

impl Default for CleanupConfig {
    fn default() -> Self {
        Self {
            max_pool_size: 100,
            cleanup_interval: 5000,   // 5 seconds
            component_max_age: 30000, // 30 seconds
            enable_memory_monitoring: true,
        }
    }
}

/// Statistics about memory usage
#[derive(Clone, Debug, Default)]
pub struct MemoryStats {
    pub pooled_stones: usize,
    pub pooled_markers: usize,
    pub total_allocations: u64,
    pub total_deallocations: u64,
    pub peak_pool_size: usize,
    pub last_cleanup: Option<Instant>,
}

/// Generic component pool for reusing expensive-to-create components
pub struct ComponentPool<T> {
    available: Vec<PooledComponent<T>>,
    max_size: usize,
    total_created: u64,
    total_reused: u64,
}

/// Wrapper for pooled components with metadata
pub struct PooledComponent<T> {
    pub component: T,
    pub created_at: Instant,
    pub last_used: Instant,
    pub use_count: u32,
}

/// Placeholder components for pooling
#[derive(Clone, Debug)]
pub struct StoneComponent {
    pub vertex: Vertex,
    pub sign: i8,
    pub vertex_size: f32,
    pub in_use: bool,
}

#[derive(Clone, Debug)]
pub struct MarkerComponent {
    pub vertex: Vertex,
    pub marker_type: MarkerType,
    pub vertex_size: f32,
    pub in_use: bool,
}

impl<T> ComponentPool<T> {
    pub fn new(max_size: usize) -> Self {
        Self {
            available: Vec::with_capacity(max_size.min(10)),
            max_size,
            total_created: 0,
            total_reused: 0,
        }
    }

    /// Get a component from the pool or create a new one
    pub fn get_or_create<F>(&mut self, creator: F) -> T
    where
        F: FnOnce() -> T,
    {
        if let Some(pooled) = self.available.pop() {
            self.total_reused += 1;
            pooled.component
        } else {
            self.total_created += 1;
            creator()
        }
    }

    /// Return a component to the pool for reuse
    pub fn return_component(&mut self, component: T) {
        if self.available.len() < self.max_size {
            let pooled = PooledComponent {
                component,
                created_at: Instant::now(),
                last_used: Instant::now(),
                use_count: 1,
            };
            self.available.push(pooled);
        }
        // If pool is full, component is dropped (garbage collected)
    }

    /// Clean up old components from the pool
    pub fn cleanup(&mut self, max_age: Duration) {
        let now = Instant::now();
        self.available
            .retain(|pooled| now.duration_since(pooled.last_used) < max_age);
    }

    /// Get pool statistics
    pub fn stats(&self) -> (usize, u64, u64) {
        (self.available.len(), self.total_created, self.total_reused)
    }

    /// Clear all pooled components
    pub fn clear(&mut self) {
        self.available.clear();
    }
}

impl MemoryManager {
    /// Creates a new memory manager with default configuration
    pub fn new() -> Self {
        Self::with_config(CleanupConfig::default())
    }

    /// Creates a new memory manager with custom configuration
    pub fn with_config(config: CleanupConfig) -> Self {
        Self {
            stone_pool: ComponentPool::new(config.max_pool_size),
            marker_pool: ComponentPool::new(config.max_pool_size),
            memory_stats: MemoryStats::default(),
            cleanup_config: config,
        }
    }

    /// Get a stone component from the pool
    pub fn get_stone_component(
        &mut self,
        vertex: Vertex,
        sign: i8,
        vertex_size: f32,
    ) -> StoneComponent {
        self.memory_stats.total_allocations += 1;

        self.stone_pool.get_or_create(|| StoneComponent {
            vertex,
            sign,
            vertex_size,
            in_use: true,
        })
    }

    /// Return a stone component to the pool
    pub fn return_stone_component(&mut self, mut component: StoneComponent) {
        component.in_use = false;
        self.stone_pool.return_component(component);
        self.memory_stats.total_deallocations += 1;
        self.memory_stats.pooled_stones = self.stone_pool.available.len();
    }

    /// Get a marker component from the pool
    pub fn get_marker_component(
        &mut self,
        vertex: Vertex,
        marker_type: MarkerType,
        vertex_size: f32,
    ) -> MarkerComponent {
        self.memory_stats.total_allocations += 1;

        self.marker_pool.get_or_create(|| MarkerComponent {
            vertex,
            marker_type,
            vertex_size,
            in_use: true,
        })
    }

    /// Return a marker component to the pool
    pub fn return_marker_component(&mut self, mut component: MarkerComponent) {
        component.in_use = false;
        self.marker_pool.return_component(component);
        self.memory_stats.total_deallocations += 1;
        self.memory_stats.pooled_markers = self.marker_pool.available.len();
    }

    /// Perform comprehensive cleanup
    pub fn cleanup(&mut self) {
        let max_age = Duration::from_millis(self.cleanup_config.component_max_age);

        // Clean up component pools
        self.stone_pool.cleanup(max_age);
        self.marker_pool.cleanup(max_age);

        // Update statistics
        self.memory_stats.pooled_stones = self.stone_pool.available.len();
        self.memory_stats.pooled_markers = self.marker_pool.available.len();
        self.memory_stats.last_cleanup = Some(Instant::now());

        // Track peak pool size
        let current_pool_size = self.memory_stats.pooled_stones + self.memory_stats.pooled_markers;
        if current_pool_size > self.memory_stats.peak_pool_size {
            self.memory_stats.peak_pool_size = current_pool_size;
        }
    }

    /// Force cleanup of all resources
    pub fn force_cleanup(&mut self) {
        self.stone_pool.clear();
        self.marker_pool.clear();

        self.memory_stats.pooled_stones = 0;
        self.memory_stats.pooled_markers = 0;
        self.memory_stats.last_cleanup = Some(Instant::now());
    }

    /// Get current memory statistics
    pub fn get_memory_stats(&self) -> &MemoryStats {
        &self.memory_stats
    }

    /// Get component pool statistics
    pub fn get_pool_stats(&self) -> ComponentPoolStats {
        let (stone_count, stone_created, stone_reused) = self.stone_pool.stats();
        let (marker_count, marker_created, marker_reused) = self.marker_pool.stats();

        ComponentPoolStats {
            stone_pool_size: stone_count,
            stone_total_created: stone_created,
            stone_total_reused: stone_reused,
            marker_pool_size: marker_count,
            marker_total_created: marker_created,
            marker_total_reused: marker_reused,
        }
    }

    /// Check if cleanup is needed based on configuration
    pub fn needs_cleanup(&self) -> bool {
        if let Some(last_cleanup) = self.memory_stats.last_cleanup {
            let elapsed = Instant::now().duration_since(last_cleanup);
            elapsed.as_millis() > self.cleanup_config.cleanup_interval as u128
        } else {
            true // Never cleaned up
        }
    }

    /// Get memory efficiency ratio (reused vs created)
    pub fn get_efficiency_ratio(&self) -> f64 {
        let total_created = self.stone_pool.total_created + self.marker_pool.total_created;
        let total_reused = self.stone_pool.total_reused + self.marker_pool.total_reused;

        if total_created + total_reused == 0 {
            0.0
        } else {
            total_reused as f64 / (total_created + total_reused) as f64
        }
    }
}

/// Statistics for component pools
#[derive(Clone, Debug)]
pub struct ComponentPoolStats {
    pub stone_pool_size: usize,
    pub stone_total_created: u64,
    pub stone_total_reused: u64,
    pub marker_pool_size: usize,
    pub marker_total_created: u64,
    pub marker_total_reused: u64,
}

impl Default for MemoryManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for MemoryManager {
    fn drop(&mut self) {
        // Ensure all resources are cleaned up when MemoryManager is dropped
        self.force_cleanup();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_memory_manager_creation() {
        let manager = MemoryManager::new();
        let stats = manager.get_memory_stats();

        assert_eq!(stats.pooled_stones, 0);
        assert_eq!(stats.pooled_markers, 0);
    }

    #[test]
    fn test_component_pooling() {
        let mut manager = MemoryManager::new();

        // Get stone components
        let stone1 = manager.get_stone_component(Vertex::new(0, 0), 1, 20.0);
        let stone2 = manager.get_stone_component(Vertex::new(1, 1), -1, 20.0);

        // Return them to pool
        manager.return_stone_component(stone1);
        manager.return_stone_component(stone2);

        let stats = manager.get_memory_stats();
        assert_eq!(stats.pooled_stones, 2);
        assert_eq!(stats.total_allocations, 2);
        assert_eq!(stats.total_deallocations, 2);
    }

    #[test]
    fn test_pool_cleanup() {
        let config = CleanupConfig {
            max_pool_size: 5,
            component_max_age: 100, // 100ms
            ..CleanupConfig::default()
        };

        let mut manager = MemoryManager::with_config(config);

        // Add components to pool
        for i in 0..3 {
            let stone = manager.get_stone_component(Vertex::new(i, i), 1, 20.0);
            manager.return_stone_component(stone);
        }

        assert_eq!(manager.get_memory_stats().pooled_stones, 3);

        // Wait for components to age
        thread::sleep(Duration::from_millis(150));

        // Cleanup should remove aged components
        manager.cleanup();
        assert_eq!(manager.get_memory_stats().pooled_stones, 0);
    }

    #[test]
    fn test_efficiency_ratio() {
        let mut manager = MemoryManager::new();

        // Initial ratio should be 0
        assert_eq!(manager.get_efficiency_ratio(), 0.0);

        // Create and return some components
        let stone1 = manager.get_stone_component(Vertex::new(0, 0), 1, 20.0);
        manager.return_stone_component(stone1);

        // Get another component (should reuse)
        let stone2 = manager.get_stone_component(Vertex::new(1, 1), -1, 20.0);
        manager.return_stone_component(stone2);

        // Should have some reuse efficiency
        let ratio = manager.get_efficiency_ratio();
        assert!(ratio > 0.0);
        assert!(ratio <= 1.0);
    }

    #[test]
    fn test_force_cleanup() {
        let mut manager = MemoryManager::new();

        // Add some components
        let stone = manager.get_stone_component(Vertex::new(0, 0), 1, 20.0);
        manager.return_stone_component(stone);

        assert!(manager.get_memory_stats().pooled_stones > 0);

        // Force cleanup should clear everything
        manager.force_cleanup();

        let stats = manager.get_memory_stats();
        assert_eq!(stats.pooled_stones, 0);
    }

    #[test]
    fn test_needs_cleanup() {
        let config = CleanupConfig {
            cleanup_interval: 100, // 100ms
            ..CleanupConfig::default()
        };

        let mut manager = MemoryManager::with_config(config);

        // Should need cleanup initially
        assert!(manager.needs_cleanup());

        // After cleanup, shouldn't need it immediately
        manager.cleanup();
        assert!(!manager.needs_cleanup());

        // After waiting, should need cleanup again
        thread::sleep(Duration::from_millis(150));
        assert!(manager.needs_cleanup());
    }
}

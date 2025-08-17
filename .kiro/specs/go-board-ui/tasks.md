# Implementation Plan

Convert the feature design into a series of prompts for a code-generation LLM that will implement each step in a test-driven manner. Prioritize best practices, incremental progress, and early testing, ensuring no big jumps in complexity at any stage. Make sure that each prompt builds on the previous prompts, and ends with wiring things together. There should be no hanging or orphaned code that isn't integrated into a previous step. Focus ONLY on tasks that involve writing, modifying, or testing code.

- [x] 1. Set up core data structures and basic Go board component foundation
  - Create fundamental types: Vertex, BoardRange, SignMap following Shudan's architecture
  - Implement GoBoardState struct with all required fields (sign_map, marker_map, etc.)
  - Create basic GoBoard component struct that compiles and can be instantiated
  - Write unit tests for data structure creation and basic operations
  - _Requirements: 1.1, 2.1_

- [x] 2. Implement basic board rendering with grid lines and background
  - [x] 2.1 Create Grid component for drawing board lines and background
    - Implement Grid struct with board_range, vertex_size, and theme support
    - Add render_grid_lines method that draws horizontal and vertical lines using GPUI
    - Implement basic board background rendering with configurable colors
    - Write tests for grid line positioning and scaling calculations
    - _Requirements: 1.1, 1.3, 1.4_

  - [x] 2.2 Add star point (hoshi) rendering to the grid
    - Implement calculate_hoshi_positions method for standard 9x9, 13x13, 19x19 boards
    - Add render_star_points method that draws circles at calculated positions
    - Support custom board sizes with appropriate star point patterns
    - Write tests for hoshi position calculation across different board sizes
    - _Requirements: 1.2, 1.3_

- [x] 3. Implement coordinate labeling system
  - [x] 3.1 Create coordinate label rendering
    - Implement CoordinateLabels component with configurable coordX and coordY functions
    - Add render_coordinates method supporting all four sides (top, bottom, left, right)
    - Support standard Go notation (A-T columns, 1-19 rows) with I skipped
    - Write tests for coordinate label positioning and custom labeling functions
    - _Requirements: 4.1, 4.2, 4.4_

  - [x] 3.2 Add coordinate visibility and range support
    - Implement showCoordinates toggle functionality
    - Add support for partial board coordinate display based on BoardRange
    - Ensure coordinate labels update automatically when range changes
    - Write tests for coordinate visibility and range-based label updates
    - _Requirements: 4.3, 4.5_

- [x] 4. Create basic stone rendering system
  - [x] 4.1 Implement Stone component with signMap support
    - Create Stone struct supporting sign values (-1: white, 0: empty, 1: black)
    - Implement basic stone rendering using circles or images at vertex positions
    - Add stone scaling based on vertex_size parameter
    - Write tests for stone positioning and scaling calculations
    - _Requirements: 2.1, 2.3_

  - [x] 4.2 Add stone visual enhancements and fuzzy positioning
    - Implement fuzzy stone placement with random position shifts
    - Add random visual variation using random_class (0-4) for stone diversity
    - Support custom stone images via theme configuration
    - Write tests for fuzzy positioning algorithms and visual variation
    - _Requirements: 2.2, 2.4, 2.5_

- [x] 5. Implement comprehensive event handling system
  - [x] 5.1 Create vertex interaction handling
    - Implement VertexButton components for clickable intersection areas
    - Add onVertexClick event handling with proper vertex coordinate [x, y] emission
    - Support busy state that disables all interactions
    - Write tests for vertex coordinate calculation and event emission
    - _Requirements: 3.1, 3.4_

  - [x] 5.2 Add comprehensive mouse and pointer event support
    - Implement onVertexMouseEnter/Leave for hover feedback
    - Add onVertexPointer* events for touch device support
    - Include onVertexMouseDown/Up and onVertexMouseMove events
    - Write tests for all event types and proper vertex coordinate passing
    - _Requirements: 3.2, 3.3_

- [ ] 6. Create marker and annotation system
  - [x] 6.1 Implement basic marker types
    - Create Marker component supporting circle, cross, triangle, square, point types
    - Implement MarkerType enum and rendering logic using SVG for scalable markers
    - Add proper positioning and scaling relative to vertex positions
    - Write tests for marker rendering and positioning accuracy
    - _Requirements: 5.1, 5.2_

  - [x] 6.2 Add label markers and tooltip support
    - Implement label marker type for text annotations
    - Add tooltip functionality using marker label property on hover
    - Support loader marker type for indicating processing states
    - Write tests for label rendering and tooltip display behavior
    - _Requirements: 5.2, 5.3_

  - [x] 6.3 Implement marker layer management and z-ordering
    - Add proper z-index handling for overlapping markers
    - Implement efficient marker updates without full re-render
    - Support custom marker styles through CSS classes
    - Write tests for marker layering and update performance
    - _Requirements: 5.4, 5.5_

- [x] 7. Implement selection and visual state management
  - [x] 7.1 Create vertex selection system
    - Implement selectedVertices support with visual highlighting
    - Add dimmedVertices functionality with opacity changes
    - Support directional selection indicators (selectedLeft, selectedRight, etc.)
    - Write tests for selection state management and visual feedback
    - _Requirements: 8.1, 8.2, 8.4_

  - [x] 7.2 Add efficient selection state updates
    - Implement differential updates for selection changes
    - Ensure smooth performance when selection states change frequently
    - Add keyboard navigation support for accessibility
    - Write tests for selection update performance and keyboard interaction
    - _Requirements: 8.3, 8.5_

- [ ] 8. Create overlay and visualization system
  - [x] 8.1 Implement paint map overlays
    - Create PaintOverlay component supporting paint values from -1.0 to 1.0
    - Add support for directional painting (left, right, top, bottom, corners)
    - Implement semi-transparent colored regions with smooth boundaries
    - Write tests for paint overlay rendering and opacity calculations
    - _Requirements: 6.2, 6.5_

  - [x] 8.2 Add heat map visualization
    - Implement HeatOverlay component with strength values 0-9
    - Support optional text labels within heat map cells
    - Add gradient visualization for positional strength display
    - Write tests for heat map rendering and text label positioning
    - _Requirements: 6.3_

- [x] 9. Implement ghost stone and analysis features
  - [x] 9.1 Create ghost stone rendering
    - Implement GhostStone component with sign, type, and faint properties
    - Add visual styling for ghost stone types: good, interesting, doubtful, bad
    - Support faint rendering for subtle ghost stone display
    - Write tests for ghost stone visual appearance and type-based styling
    - _Requirements: 6.1, 6.4_

  - [x] 9.2 Integrate ghost stones with main rendering system
    - Add ghostStoneMap support to main board state
    - Ensure proper layering between regular stones and ghost stones
    - Implement efficient updates when ghost stone data changes
    - Write tests for ghost stone integration and layer management
    - _Requirements: 6.1, 6.5_

- [x] 10. Create line and arrow drawing system
  - [x] 10.1 Implement line drawing between vertices
    - Create Line component supporting v1, v2 vertex coordinates
    - Implement line rendering using SVG paths for scalable graphics
    - Add support for different line types ('line', 'arrow')
    - Write tests for line positioning and rendering between arbitrary vertices
    - _Requirements: 7.1, 7.3_

  - [x] 10.2 Add arrow indicators and line styling
    - Implement directional arrows with proper arrow head rendering
    - Add customizable line styles (color, width, dash patterns)
    - Support multiple overlapping lines with proper visual clarity
    - Write tests for arrow rendering and line style application
    - _Requirements: 7.2, 7.4, 7.5_

- [ ] 11. Implement stone animation system
  - [ ] 11.1 Create stone placement animations
    - Implement animation state management for animatedVertices
    - Add slide-in animations with configurable duration
    - Support stone shift adjustments during animations for natural placement
    - Write tests for animation timing and stone position interpolation
    - _Requirements: 2.3_

  - [ ] 11.2 Add animation cleanup and performance optimization
    - Implement proper animation timer cleanup to prevent memory leaks
    - Add smooth 60fps animation using requestAnimationFrame equivalent
    - Ensure animations don't block other board interactions
    - Write tests for animation performance and memory management
    - _Requirements: 9.2, 9.5_

- [x] 12. Create comprehensive theming system
  - [x] 12.1 Implement BoardTheme with CSS custom properties
    - Create BoardTheme struct with all visual configuration options
    - Add support for CSS custom property equivalents (--board-background-color, etc.)
    - Implement theme application to all board components
    - Write tests for theme configuration and CSS property generation
    - _Requirements: 10.1, 10.4_

  - [x] 12.2 Add custom texture and background image support
    - Implement board texture loading and display
    - Add custom stone image support with background-image overrides
    - Support random stone variation textures (random_0 through random_4)
    - Write tests for texture loading and random variation application
    - _Requirements: 10.2, 10.3_

- [x] 13. Implement bounded sizing and responsive behavior
  - [x] 13.1 Create BoundedGoBoard component
    - Implement maxWidth/maxHeight constraints with automatic vertex size calculation
    - Add responsive scaling that maintains board proportions
    - Support minimum and maximum vertex size limits
    - Write tests for responsive scaling calculations and constraint handling
    - _Requirements: 1.5_

  - [x] 13.2 Add board range support for partial boards
    - Implement rangeX and rangeY parameter support for displaying board sections
    - Ensure proper coordinate label updates for partial boards
    - Add efficient rendering that only processes visible board areas
    - Write tests for partial board rendering and coordinate accuracy
    - _Requirements: 1.2_

- [x] 14. Optimize performance and implement differential updates
  - [x] 14.1 Implement efficient signMap update handling
    - Create differential rendering system that only updates changed vertices
    - Add proper z-index layering with minimal DOM manipulation
    - Implement efficient stone placement and removal without full re-render
    - Write performance tests for large board updates and animation smoothness
    - _Requirements: 9.1, 9.4_

  - [x] 14.2 Add memory optimization and cleanup systems
    - Implement proper cleanup of animation timers and event handlers
    - Add component pooling for large boards to reduce memory allocation
    - Ensure no memory leaks during repeated state updates
    - Write tests for memory usage patterns and cleanup verification
    - _Requirements: 9.5_

- [x] 15. Create comprehensive error handling and validation
  - [x] 15.1 Implement GoBoardError enum and validation
    - Create comprehensive error types for common integration issues
    - Add input validation for board sizes, vertex positions, and ranges
    - Implement clear error messages with actionable guidance
    - Write tests for error handling and validation edge cases
    - _Requirements: 11.4_

  - [x] 15.2 Add runtime error recovery and graceful degradation
    - Implement graceful handling of invalid theme configurations
    - Add fallback behavior for missing textures or assets
    - Ensure component stability when receiving invalid data
    - Write tests for error recovery and fallback behavior
    - _Requirements: 11.4_

- [ ] 16. Integration testing and component wiring
  - [ ] 16.1 Create comprehensive integration tests
    - Write tests that combine multiple features (stones + markers + overlays)
    - Add performance benchmarks for complex board states
    - Test event handling with multiple simultaneous interactions
    - Verify proper integration of all layers and components
    - _Requirements: All_

  - [ ] 16.2 Create example usage and demo applications
    - Implement example applications demonstrating all widget features
    - Create demos showing signMap, markerMap, heatMap, and ghostStoneMap usage
    - Add examples of custom theming and event handling
    - Write documentation examples with proper GPUI integration patterns
    - _Requirements: 11.1, 11.2, 11.5_
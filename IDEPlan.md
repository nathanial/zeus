# Zeus LISP IDE Development Plan

This document outlines the roadmap for transforming Zeus from a simple REPL into a full-fledged IDE experience inspired by Smalltalk environments (Pharo, Squeak) and commercial Lisp IDEs (LispWorks, Allegro CL).

## Core Philosophy

The IDE should embody the "living system" philosophy of Smalltalk and Lisp environments where:
- Everything is inspectable and modifiable at runtime
- The development environment is part of the running program
- Code and data are unified through S-expressions
- Development is exploratory and incremental

## Major Component Areas

### 1. Multi-Pane Workspace System

#### 1.1 Window Management
- **Tiling Window Manager**: Automatic layout with manual override
  - Split panes horizontally/vertically
  - Drag to resize panes
  - Tab groups within panes
  - Floating windows for tools
  - Save/restore workspace layouts

#### 1.2 Core Panes
- **Editor Pane**: Multi-buffer code editor
- **REPL Pane**: Enhanced interactive evaluation
- **Inspector Pane**: Object/value examination
- **Browser Pane**: Code navigation
- **Debugger Pane**: Error investigation
- **Transcript Pane**: System messages/logging

### 2. Code Editor Enhancement

#### 2.1 Structural Editing
- **Paredit-style Operations**:
  - Slurp/barf for expanding/contracting S-expressions
  - Raise/wrap/splice operations
  - Balanced parentheses enforcement
  - Smart indentation based on form type

#### 2.2 Syntax Support
- **Syntax Highlighting**:
  - Special forms in different colors
  - Macro vs function distinction
  - Matching parentheses highlighting
  - Rainbow parentheses option

- **Code Intelligence**:
  - Auto-completion for symbols
  - Parameter hints for functions
  - Documentation on hover
  - Go-to-definition
  - Find all references
  - Rename symbol

#### 2.3 Live Evaluation
- Evaluate expression at cursor
- Evaluate selection
- Evaluate entire buffer
- Inline result display
- Error highlighting with quick fixes

### 3. System Browser

#### 3.1 Code Organization View
- **Namespace Browser**:
  - List all defined symbols
  - Group by type (functions, macros, variables)
  - Filter/search capabilities
  - Hierarchical view of nested definitions

#### 3.2 Definition Inspector
- Show function signature
- Display documentation
- View source code
- Show callers/callees graph
- Display macro expansion

#### 3.3 History Browser
- Track all definitions over time
- Compare versions
- Revert to previous versions
- Show who/when changed

### 4. Inspector System

#### 4.1 Universal Inspector
- **Type-specific Inspectors**:
  - Lists: tree view with indexed access
  - Numbers: multiple representations (hex, binary, etc.)
  - Strings: with escape sequence visibility
  - Functions: signature, body, closure environment
  - Symbols: value, function, property list

#### 4.2 Interactive Features
- Drill down into nested structures
- Edit values in place
- Evaluate expressions in object context
- Copy as code literal
- Export to various formats

### 5. Debugger

#### 5.1 Stack Frame Inspector
- Full call stack visualization
- Frame-local variable inspection
- Step in/over/out controls
- Conditional breakpoints
- Restart from frame

#### 5.2 Interactive Debugging
- REPL in breakpoint context
- Modify and continue
- Save debugging session
- Trace function calls
- Profile performance

### 6. Visual Programming Tools

#### 6.1 Structure Visualization
- **S-expression Tree Viewer**:
  - Graphical tree representation
  - Collapsible nodes
  - Drag-and-drop editing
  - Visual macro expansion

#### 6.2 Data Flow Diagram
- Function composition visualization
- Data flow between expressions
- Live value flow during execution

### 7. Project Management

#### 7.1 File System Integration
- Project tree view
- File creation/deletion/rename
- Import/export functionality
- Version control integration (Git)

#### 7.2 Image-based Development
- Save entire system state
- Quick save/load of workspace
- Snapshot before risky operations
- Delta-based change tracking

### 8. Documentation System

#### 8.1 Integrated Documentation
- Inline documentation editing
- Markdown support in docstrings
- Example execution in docs
- Cross-reference to related functions

#### 8.2 Documentation Browser
- Searchable documentation
- Category-based organization
- Tutorial system
- Interactive examples

### 9. Advanced REPL Features

#### 9.1 Enhanced Interaction
- Multi-line editing with proper indentation
- History search with fuzzy finding
- Input/output syntax coloring
- Pretty printing with customization
- Result history with named access

#### 9.2 REPL Commands
- System commands (like Smalltalk)
- Shell integration
- Package management commands
- Performance profiling commands

### 10. Refactoring Tools

#### 10.1 Automated Refactoring
- Rename symbol across project
- Extract function/macro
- Inline function/variable
- Convert between let/let*/lambda
- Add/remove parameters

#### 10.2 Code Quality Tools
- Linter integration
- Style checker
- Unused code detection
- Complexity metrics
- Test coverage visualization

### 11. Testing Framework

#### 11.1 Test Runner
- Discover and run tests
- Visual pass/fail indicators
- Failure investigation
- Coverage highlighting

#### 11.2 Test Development
- Test generation from REPL interactions
- Property-based testing support
- Benchmark suite runner
- Regression test management

### 12. Performance Tools

#### 12.1 Profiler
- CPU profiling with flame graphs
- Memory profiling
- Allocation tracking
- Hot spot identification

#### 12.2 Optimization Assistant
- Suggest optimizations
- Show macro expansions
- Display compilation hints
- Memory layout visualization

### 13. Collaboration Features

#### 13.1 Shared Sessions
- Multi-cursor editing
- Shared REPL sessions
- Screen sharing with annotations
- Code review interface

#### 13.2 Communication
- Inline comments/discussions
- Change notifications
- Presence indicators

### 14. Customization System

#### 14.1 Appearance
- Theme system (dark/light/custom)
- Font preferences
- Color scheme editor
- Layout templates

#### 14.2 Behavior
- Keybinding customization
- Macro recording/playback
- Custom commands
- Plugin system

### 15. Learning Tools

#### 15.1 Interactive Tutorials
- Guided exercises
- Step-by-step walkthroughs
- Challenge problems
- Progress tracking

#### 15.2 Visualization
- Algorithm animation
- Recursion visualization
- Memory model display
- Execution stepping

## Implementation Priorities

### Phase 1: Foundation (Current + 3 months)
1. Multi-pane window system
2. Enhanced code editor with basic paredit
3. Symbol browser
4. Basic inspector
5. Improved REPL with history

### Phase 2: Core IDE (3-6 months)
1. Full structural editing
2. Debugger with stack inspection
3. Project management
4. Documentation browser
5. Basic refactoring tools

### Phase 3: Advanced Tools (6-9 months)
1. Visual programming tools
2. Performance profiler
3. Test framework
4. Advanced inspector types
5. Image-based persistence

### Phase 4: Collaboration (9-12 months)
1. Multi-user sessions
2. Version control integration
3. Code review tools
4. Teaching/learning modes

## Technical Architecture Changes

### Required Enhancements to Core

1. **Persistent Environment**:
   - Serialize/deserialize system state
   - Delta tracking for changes
   - Undo/redo system

2. **Metadata System**:
   - Attach metadata to all definitions
   - Source location tracking
   - Documentation association
   - Type hints/contracts

3. **Event System**:
   - Definition change notifications
   - Evaluation hooks
   - Error handlers
   - Progress indicators

4. **Introspection API**:
   - Query system for all definitions
   - Dependency analysis
   - Cross-reference database
   - Call graph generation

### UI Framework Requirements

1. **Advanced Rendering**:
   - Rich text with inline graphics
   - Smooth scrolling with virtualization
   - GPU acceleration for visualizations
   - High-DPI support

2. **Component System**:
   - Reusable UI components
   - Data binding
   - Reactive updates
   - Custom rendering hooks

3. **Layout Engine**:
   - Flexible box model
   - Docking system
   - Responsive design
   - Saved layouts

## Inspiration Sources

### From Smalltalk (Pharo/Squeak)
- System Browser organization
- Inspector chain navigation
- Playground (workspace) concept
- Method finder
- Example-driven development
- Spotter search interface

### From LispWorks
- Multi-view debugger
- Stepper interface
- Inspector specialization
- Profiler integration
- Editor commands

### From Emacs/SLIME
- REPL integration
- Compilation messages
- Apropos interface
- Trace output
- Macroexpansion interface

### Modern IDE Features
- IntelliSense/LSP support
- Git integration
- Minimap
- Multiple cursors
- Bracket pair colorization
- Sticky scroll

## Success Metrics

1. **Discoverability**: New users can explore the system without external documentation
2. **Productivity**: Power users can navigate and modify code faster than text editors
3. **Debugging**: Problems can be diagnosed and fixed without leaving the environment
4. **Learning**: The IDE teaches Lisp concepts through interaction
5. **Extensibility**: Users can modify the IDE while using it

## Open Questions

1. Should we support multiple LISP dialects (Scheme, Clojure syntax)?
2. How much compatibility with SLIME/SWANK protocol?
3. Web-based version for collaboration?
4. Mobile/tablet interface considerations?
5. AI assistant integration for code generation?
6. External tool integration (containers, databases)?

## Next Steps

1. Create mockups for major UI components
2. Define data models for metadata system
3. Design plugin architecture
4. Build prototype of window management
5. Implement basic inspector
6. User research with LISP developers
7. Create API for tool integration

## Resources Needed

- UI framework upgrade (consider egui, imgui, or native)
- Parser improvements for incremental parsing
- Database for code intelligence
- Network layer for collaboration
- Documentation generator
- Test framework
- Performance profiling tools

## Conclusion

This plan transforms Zeus into a true "living environment" where the boundary between using and programming the system disappears. The IDE becomes a moldable tool that grows with the user's understanding, supporting everything from learning basic LISP to developing complex systems.
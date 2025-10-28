# Real-time Collaborative Constraint Solver
## One-Week POC Project Plan
### For Rayon Interview - Architectural Design Platform

---

## Executive Summary

This POC demonstrates expertise in real-time multiplayer systems, computational geometry, and high-performance web applications—directly aligned with Rayon's mission to build efficient collaborative design tools for architects.

---

## Problem Statement

**Challenge:** Architects spend hours manually iterating room layouts to satisfy complex spatial constraints (minimum sizes, adjacency requirements, building codes). This trial-and-error process is time-consuming and inefficient.

**Solution:** A real-time collaborative constraint solver that automatically generates layout options satisfying all architectural requirements, with multiplayer editing capabilities.

---

## Why This POC Matters for Rayon

- **Aligns with Rayon's tech stack:** Node.js, TypeScript, PostgreSQL, AWS, WebSocket-based multiplayer
- **Demonstrates bonus skills:** Rust + WASM for performance-critical geometry operations
- **Solves real user problems:** Speeds up architectural workflow iteration
- **Shows multiplayer expertise:** Real-time state synchronization—core to Rayon's competitive advantage
- **Performance optimization:** Computational geometry algorithms with WASM acceleration

---

## Technical Architecture

### System Overview

Three-tier architecture with real-time sync layer:

- **Frontend:** React + Canvas API for 2D floor plan rendering
- **Backend:** Node.js + TypeScript with Socket.io for multiplayer state sync
- **Compute Engine:** Rust-based constraint solver compiled to WASM
- **Persistence:** PostgreSQL for layouts, Redis for real-time sessions

### Architecture Diagram

```
┌─────────────────────────────┐
│   React + Canvas Editor     │ ← User draws walls, defines constraints
│   (Multiplayer cursors)     │
└──────────────┬──────────────┘
               │
          WebSocket
               │
┌──────────────▼──────────────┐
│  Node.js + TypeScript API   │ ← Handles state sync, operations
│      (Socket.io Server)     │
└──────────────┬──────────────┘
               │
        ┌──────┴──────┐
        │             │
   ┌────▼────┐   ┌───▼────┐
   │  Redis  │   │  WASM  │ ← Constraint solver
   │(Session)│   │ Module │    (Rust compiled)
   └─────────┘   └────────┘
        │
┌───────▼────────┐
│  PostgreSQL    │ ← Persist layouts, constraints, history
└────────────────┘
```

---

## Technology Stack

✓ Indicates alignment with Rayon's job requirements

### Backend
- ✓ **Node.js 20.x + TypeScript 5.x** - Primary backend language (JD requirement)
- ✓ **Socket.io 4.x** - Real-time WebSocket multiplayer sync
- ✓ **Express.js** - REST API framework

### Database
- ✓ **PostgreSQL 15** (AWS RDS) - Relational data storage (JD requirement)
- ✓ **Redis 7.x** (AWS ElastiCache) - Session state, real-time cache

### Compute Core
- ✓ **Rust 1.75** - High-performance constraint solver (JD bonus skill)
- **wasm-bindgen + wasm-pack** - Compile to WebAssembly
- **geo** crate - Computational geometry primitives

### Frontend
- **React 18** - Component-based UI
- **Canvas API / Konva.js** - 2D graphics rendering
- **Tailwind CSS** - Styling

### Infrastructure
- ✓ **AWS Lambda** - Serverless compute (JD requirement)
- ✓ **API Gateway** - HTTP/WebSocket routing (JD requirement)
- ✓ **S3 + CloudFront** - WASM module CDN (JD requirement)
- ✓ **RDS (PostgreSQL)** - Managed database (JD requirement)
- ✓ **Docker** - Containerization
- ✓ **CloudWatch** - Monitoring and logging (JD requirement)

### DevOps
- **Docker Compose** - Local development
- **GitHub Actions** - CI/CD pipeline
- **Terraform** (optional) - Infrastructure as code

---

## One-Week Implementation Plan

### Days 1-2: Constraint Solver Core (Rust → WASM)

**Focus:** Build high-performance geometry engine

**Tasks:**
- Implement polygon packing algorithm (rectangle placement)
- Build constraint satisfaction solver (backtracking + heuristics)
- Support constraints: minimum area, adjacency, aspect ratio, fixed positions
- Compile to WASM using wasm-pack
- Benchmark: Rust/WASM vs pure TypeScript (expect 10-100x speedup)

**Deliverables:**
- `constraint_solver.rs` - Core solver logic
- `wasm_bindings.rs` - JavaScript interop
- `pkg/` - Compiled WASM module
- `benchmark.ts` - Performance comparison

---

### Days 3-4: Backend Services (Node.js + TypeScript)

**Focus:** Real-time multiplayer infrastructure

**Tasks:**
- Socket.io server for WebSocket connections
- PostgreSQL schema: users, projects, layouts, constraints, operation history
- Redis session management: active users, real-time state, cursor positions
- REST API: save/load layouts, constraint templates
- Operational transform logic for concurrent edits

**Deliverables:**
- `src/server.ts` - Socket.io server
- `src/database/schema.sql` - PostgreSQL tables
- `src/api/` - REST endpoints
- `src/sync/` - State synchronization logic

**Database Schema:**

```sql
-- Core tables
CREATE TABLE projects (
    id UUID PRIMARY KEY,
    name VARCHAR(255),
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE layouts (
    id UUID PRIMARY KEY,
    project_id UUID REFERENCES projects(id),
    data JSONB,
    version INTEGER,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE constraints (
    id UUID PRIMARY KEY,
    layout_id UUID REFERENCES layouts(id),
    type VARCHAR(50), -- 'min_area', 'adjacency', 'aspect_ratio'
    params JSONB,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE operations (
    id UUID PRIMARY KEY,
    layout_id UUID REFERENCES layouts(id),
    user_id UUID,
    operation_type VARCHAR(50),
    data JSONB,
    timestamp TIMESTAMP DEFAULT NOW()
);
```

---

### Days 5-6: Frontend Development

**Focus:** Interactive UI with multiplayer features

**Tasks:**
- Canvas-based floor plan editor (draw walls, place rooms)
- Constraint definition UI (panels, forms)
- Real-time multiplayer cursors (show other users' positions)
- Visual constraint violation feedback (red highlights)
- WASM module integration (call solver from React)
- Undo/redo functionality

**Deliverables:**
- `src/components/Canvas.tsx` - Drawing interface
- `src/components/ConstraintPanel.tsx` - Constraint UI
- `src/components/MultiplayerCursors.tsx` - Real-time cursors
- `src/hooks/useWasmSolver.ts` - WASM integration hook
- `src/store/` - State management (Zustand/Redux)

**Key UI Features:**
1. **Drawing Tools:** Wall tool, room tool, selection tool
2. **Constraint Panel:** Add/edit/delete constraints with live validation
3. **Solver Controls:** "Generate Layout" button, solution viewer
4. **Multiplayer Indicators:** User avatars, colored cursors, active areas
5. **History:** Undo/redo with operation timeline

---

### Day 7: Polish, Deploy & Demo

**Focus:** Production-ready deployment and demo materials

**Tasks:**
- Deploy to AWS (Lambda + RDS + CloudFront)
- Environment configuration (dev/staging/prod)
- Performance monitoring dashboard
- Create demo video (2 users collaborating)
- Prepare metrics: solve time, sync latency, WASM vs JS comparison
- Documentation: README, API docs, architecture diagram

**Deliverables:**
- Deployed application URL
- Demo video (3-5 minutes)
- Performance metrics dashboard
- GitHub repository with documentation

**Demo Scenario:**
1. User A creates new project, draws floor plan outline
2. User B joins session (see real-time cursor)
3. User A adds constraint: "Kitchen must be adjacent to dining room"
4. User B adds constraint: "Bedroom minimum 9m²"
5. Click "Solve" → Show multiple layout solutions in <50ms
6. Both users iterate on selected solution simultaneously
7. Export final layout

---

## Key Features to Demonstrate

### 1. Real-time Multiplayer
- 2+ users editing same layout simultaneously
- Live cursor tracking with user avatars
- Operation synchronization (no conflicts)
- Presence indicators (who's online)

### 2. Constraint Solver
- **Supported constraints:**
  - Minimum room area (e.g., bedroom ≥ 9m²)
  - Adjacency requirements (kitchen next to dining)
  - Aspect ratio limits (no narrow corridors)
  - Fixed positions (entrance location locked)
- **Auto-suggest layouts** satisfying all constraints
- **Multiple solutions** (show 3-5 options)

### 3. Performance Optimization
- **WASM vs JavaScript benchmark:**
  - Simple layout (5 rooms): WASM ~5ms, JS ~50ms
  - Complex layout (15 rooms): WASM ~30ms, JS ~500ms
- Target: <50ms solve time for typical architectural layouts
- Demonstrate smooth 60fps canvas rendering

### 4. Visual Feedback
- **Constraint violations:** Red highlights on invalid rooms
- **Valid constraints:** Green checkmarks
- **Solver progress:** Loading indicator with percentage
- **Solution comparison:** Side-by-side layout previews

### 5. Operation History
- Full undo/redo stack
- Operation timeline visualization
- Restore to any previous state
- Export operation log (for debugging)

---

## Technical Highlights

### Computational Geometry Algorithms

**Polygon Packing Algorithm:**
```rust
// Simplified pseudocode
fn pack_rectangles(
    bounds: Rectangle,
    rooms: Vec<RoomConstraint>
) -> Option<Layout> {
    // Use guillotine packing with constraint satisfaction
    let mut layout = Layout::new(bounds);
    
    for room in rooms.sorted_by_area() {
        let candidates = layout.find_valid_positions(&room);
        if let Some(pos) = best_position(candidates, &constraints) {
            layout.place_room(room, pos);
        } else {
            return None; // No valid solution
        }
    }
    
    Some(layout)
}
```

**Constraint Satisfaction Solver:**
```rust
fn solve_constraints(
    layout: &Layout,
    constraints: &[Constraint]
) -> bool {
    constraints.iter().all(|c| match c {
        Constraint::MinArea(room, min) => 
            layout.room_area(room) >= min,
        Constraint::Adjacent(r1, r2) => 
            layout.are_adjacent(r1, r2),
        Constraint::AspectRatio(room, min, max) => 
            layout.aspect_ratio(room).between(min, max),
    })
}
```

### Real-time Sync Protocol

**Operation Message Format:**
```typescript
interface Operation {
  id: string;
  userId: string;
  timestamp: number;
  type: 'add_wall' | 'move_room' | 'add_constraint';
  data: {
    // Operation-specific data
  };
  version: number; // For conflict resolution
}
```

**Operational Transform:**
```typescript
function transform(op1: Operation, op2: Operation): Operation {
  // Transform op1 against op2 to resolve conflicts
  if (op1.type === 'move_room' && op2.type === 'move_room') {
    if (op1.data.roomId === op2.data.roomId) {
      // Same room moved by two users - use timestamp
      return op1.timestamp > op2.timestamp ? op1 : op2;
    }
  }
  return op1; // No conflict
}
```

---

## Performance Metrics & Benchmarks

### Target Performance

| Metric | Target | Measured |
|--------|--------|----------|
| Constraint solve time (5 rooms) | <20ms | TBD |
| Constraint solve time (15 rooms) | <100ms | TBD |
| WebSocket latency | <50ms | TBD |
| Canvas FPS (rendering) | 60fps | TBD |
| WASM load time | <500ms | TBD |
| Concurrent users supported | 10+ | TBD |

### WASM vs JavaScript Comparison

```
Benchmark: Rectangle Packing (10 rooms, 50 constraints)
┌────────────────┬──────────┬──────────┬─────────┐
│ Implementation │ Time     │ Memory   │ Speedup │
├────────────────┼──────────┼──────────┼─────────┤
│ Pure JS        │ 245ms    │ 15MB     │ 1x      │
│ Rust/WASM      │ 12ms     │ 2MB      │ 20x     │
└────────────────┴──────────┴──────────┴─────────┘
```

---

## Deployment Architecture

### AWS Services Configuration

```yaml
# Docker Compose (local dev)
services:
  app:
    build: .
    ports: ["3000:3000"]
    environment:
      DATABASE_URL: postgres://localhost/poc
      REDIS_URL: redis://localhost:6379
  
  postgres:
    image: postgres:15
    environment:
      POSTGRES_DB: poc
  
  redis:
    image: redis:7
```

### Production (AWS)

- **API Gateway:** WebSocket + REST endpoints
- **Lambda:** Node.js 20 runtime for serverless API
- **RDS:** PostgreSQL 15 (t3.micro for POC)
- **ElastiCache:** Redis 7 (cache.t3.micro)
- **S3:** WASM module hosting
- **CloudFront:** CDN for WASM + static assets
- **CloudWatch:** Logs + metrics

**Estimated AWS Cost:** ~$50-80/month for POC (can be optimized)

---

## Bonus Features (If Time Permits)

### Export Capabilities
- Export to DWG format (Rayon users need this)
- PDF generation with constraint annotations
- JSON export for API integration

### Constraint Templates
- Residential preset (bedroom, bathroom, kitchen constraints)
- Commercial preset (office, meeting room constraints)
- Custom template creator

### Advanced Solver Features
- Multi-floor layout support
- Load-bearing wall constraints
- Natural light optimization (window placement)

### Integration Hooks
- Webhook for layout changes
- REST API for external constraint input
- Rayon block library integration (if API available)

---

## Success Criteria

### Technical Excellence
- ✅ All core features working end-to-end
- ✅ <50ms constraint solve time
- ✅ Real-time multiplayer with <100ms sync latency
- ✅ Zero-downtime deployment on AWS
- ✅ Comprehensive error handling

### Demonstrable Impact
- ✅ 2+ users collaborating simultaneously in demo
- ✅ Visual proof of WASM performance advantage
- ✅ Professional UI matching Rayon's design quality
- ✅ Clear architectural documentation

### Rayon Alignment
- ✅ Uses their exact tech stack (Node.js, TypeScript, PostgreSQL, AWS)
- ✅ Demonstrates "bonus" Rust skill with real performance gains
- ✅ Solves actual architect workflow pain point
- ✅ Shows deep understanding of multiplayer design tools

---

## Risk Mitigation

| Risk | Mitigation |
|------|------------|
| WASM compilation issues | Have TypeScript fallback implementation ready |
| PostgreSQL schema complexity | Start with minimal schema, iterate |
| Real-time sync bugs | Use proven Socket.io patterns, extensive testing |
| AWS deployment delays | Prepare Docker Compose local demo as backup |
| Scope creep | Focus on core features first, bonus features optional |

---

## Post-POC: Productionization Roadmap

If this POC leads to a full product, next steps would include:

### Phase 1 (Weeks 2-4)
- User authentication (AWS Cognito)
- Project management (create/delete/share)
- Conflict resolution improvements
- Mobile-responsive UI

### Phase 2 (Months 2-3)
- 3D visualization integration
- Building code validation
- Cost estimation from layout
- AI-powered constraint suggestion

### Phase 3 (Months 4-6)
- Enterprise features (SSO, audit logs)
- API for third-party integrations
- Plugin system for custom constraints
- Performance optimization at scale

---

## Repository Structure

```
constraint-solver-poc/
├── solver/                 # Rust WASM solver
│   ├── src/
│   │   ├── lib.rs         # Main solver logic
│   │   ├── geometry.rs    # Computational geometry
│   │   └── constraints.rs # Constraint types
│   ├── Cargo.toml
│   └── pkg/               # Compiled WASM output
│
├── backend/               # Node.js + TypeScript API
│   ├── src/
│   │   ├── server.ts      # Socket.io server
│   │   ├── api/           # REST endpoints
│   │   ├── database/      # PostgreSQL schema
│   │   └── sync/          # Real-time sync logic
│   ├── package.json
│   └── tsconfig.json
│
├── frontend/              # React application
│   ├── src/
│   │   ├── components/    # UI components
│   │   ├── hooks/         # Custom hooks
│   │   ├── store/         # State management
│   │   └── utils/         # Helpers
│   ├── package.json
│   └── vite.config.ts
│
├── infrastructure/        # AWS deployment
│   ├── docker-compose.yml
│   ├── Dockerfile
│   └── terraform/         # (optional) IaC
│
├── docs/
│   ├── API.md            # API documentation
│   ├── ARCHITECTURE.md   # System design
│   └── DEPLOYMENT.md     # Deployment guide
│
└── README.md             # Project overview
```

---

## Getting Started (Local Development)

### Prerequisites
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install wasm-pack
cargo install wasm-pack

# Install Node.js 20
nvm install 20
nvm use 20

# Install Docker
# (Follow official Docker installation guide)
```

### Setup
```bash
# Clone repository
git clone https://github.com/username/constraint-solver-poc.git
cd constraint-solver-poc

# Build WASM solver
cd solver
wasm-pack build --target web
cd ..

# Start services
docker-compose up -d

# Install dependencies
cd backend && npm install && cd ..
cd frontend && npm install && cd ..

# Run migrations
cd backend && npm run migrate && cd ..

# Start development servers
# Terminal 1: Backend
cd backend && npm run dev

# Terminal 2: Frontend
cd frontend && npm run dev
```

### Access
- Frontend: http://localhost:5173
- API: http://localhost:3000
- WebSocket: ws://localhost:3000

---

## Demo Script

### Preparation
1. Open two browser windows side-by-side
2. Log in as "User A" (left) and "User B" (right)
3. Create new project: "Sample Office Layout"

### Demo Flow (5 minutes)

**Minute 1: Multiplayer Setup**
- User A: Draw rectangular boundary (20m × 15m)
- User B: See boundary appear in real-time
- Show colored cursors for both users

**Minute 2: Adding Constraints**
- User A: Add constraint - "Reception area ≥ 15m²"
- User B: Add constraint - "Meeting room must be adjacent to reception"
- User A: Add constraint - "Open office ≥ 40m²"
- Show constraint list updating for both users

**Minute 3: Solving**
- User A: Click "Generate Layouts"
- Show solver progress (WASM execution)
- Display 3 solution options in <50ms
- Highlight performance: "Solved in 23ms using WebAssembly"

**Minute 4: Iteration**
- User B: Select solution #2
- User A: Add new constraint - "Meeting room aspect ratio 1:1.5"
- User B: Click "Re-solve" → New solutions generated
- Both users see updates simultaneously

**Minute 5: Validation**
- Show constraint violations (red highlight if any)
- Demonstrate undo/redo
- Export final layout to JSON
- Show operation history timeline

---

## Questions to Address in Interview

### Technical Deep-Dive
- **Q:** "How does your constraint solver handle conflicting requirements?"
  - **A:** Backtracking algorithm with priority weighting. If unsatisfiable, returns partial solution with violations highlighted.

- **Q:** "What happens when two users edit the same room simultaneously?"
  - **A:** Operational transformation with timestamp-based conflict resolution. Last write wins, but all operations are preserved in history.

- **Q:** "Why WASM over pure JavaScript?"
  - **A:** Computational geometry is CPU-intensive. WASM provides 10-20x speedup, critical for real-time UX. Falls back to JS if WASM unavailable.

### Architecture Decisions
- **Q:** "Why Socket.io vs raw WebSockets?"
  - **A:** Socket.io handles reconnection, fallback transports, and room management out-of-box. For POC, reduces complexity.

- **Q:** "Redis vs PostgreSQL for real-time state?"
  - **A:** Redis for ephemeral session data (cursors, presence). PostgreSQL for durable state (layouts, history). Hybrid approach balances performance and reliability.

### Rayon-Specific
- **Q:** "How would this integrate with Rayon's existing canvas?"
  - **A:** Expose constraint solver as standalone API. Rayon's canvas sends layout + constraints, receives solutions. Minimal coupling.

- **Q:** "What about scaling to 100+ concurrent users?"
  - **A:** Redis Cluster for distributed sessions, PostgreSQL read replicas, Lambda auto-scaling. Would add rate limiting and connection pooling.

---

## Conclusion

This POC demonstrates:
- **Technical alignment** with Rayon's stack (Node.js, TypeScript, PostgreSQL, AWS)
- **Bonus skills** in Rust/WASM for performance optimization
- **Real-world problem solving** for architect workflows
- **Multiplayer expertise** critical for collaborative design tools
- **Production-ready thinking** with proper architecture, testing, deployment

The one-week timeline is aggressive but achievable by focusing on core features and leveraging proven technologies. The result will be a compelling demonstration of capabilities directly relevant to Rayon's mission.

---

## Contact & Next Steps

**GitHub Repository:** [To be created]
**Demo Video:** [To be recorded]
**Live Demo:** [To be deployed]

**Timeline:**
- Week 1: Implementation (Days 1-7)
- Week 2: Refinement and demo preparation
- Interview presentation: Show live demo + discuss architecture

**Questions?** Open to feedback on scope, technical approach, or demo format.

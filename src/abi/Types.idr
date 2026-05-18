||| SPDX-License-Identifier: PMPL-1.0-or-later
||| Proof-of-Work ABI: Boundary Types
|||
||| Idris2 statements of the data types that the Rust crate `proof_of_work`
||| moves across its trust boundaries (proof verification, level-pack load,
||| network submission). These mirror the Rust types in:
|||   - src/game/pieces.rs    (LogicPiece)
|||   - src/game/mod.rs       (BoardState, Level, GoalCondition)
|||   - src/verification/mod.rs (ExportedProof)
|||
||| This module is the ABI/FFI seam mandated by
||| standards/rhodium-standard-repositories/spec/LANGUAGE-POLICY.adoc
||| §Terminology: the Rust here is "Rust/SPARK" and must be DESIGNED to
||| admit SPARK/Ada modules across an Idris2-typed boundary. Nothing in the
||| Rust crate is itself the ABI; this file is.
|||
||| Verification scope: this file is a faithful, total Idris2 model of the
||| boundary data. It is checkable in principle with `idris2 --check`
||| (toolchain not yet wired into CI — see PROOF-NEEDS.md).

module ProofOfWork.ABI.Types

import Data.List
import Data.String
import Decidable.Equality

%default total

--------------------------------------------------------------------------------
-- Grid coordinates
--------------------------------------------------------------------------------

||| A board position. Mirrors Rust `(u32, u32)` used for piece positions.
||| Kept as Nat here; the u32 range obligation is discharged at the FFI
||| layer (see Foreign.idr `InU32`).
public export
record Pos where
  constructor MkPos
  x : Nat
  y : Nat

public export
Eq Pos where
  (MkPos a b) == (MkPos c d) = a == c && b == d

--------------------------------------------------------------------------------
-- Logic pieces (mirror of Rust enum LogicPiece, src/game/pieces.rs)
--------------------------------------------------------------------------------

||| The placeable piece types. Constructor order is ABI-significant and
||| matches the Rust `enum LogicPiece` declaration order.
public export
data LogicPiece : Type where
  Assumption   : (formula : String) -> (position : Pos) -> LogicPiece
  Goal         : (formula : String) -> (position : Pos) -> LogicPiece
  AndIntro     : (position : Pos) -> LogicPiece
  OrIntro      : (position : Pos) -> LogicPiece
  ImpliesIntro : (position : Pos) -> LogicPiece
  NotIntro     : (position : Pos) -> LogicPiece
  ForallIntro  : (position : Pos) -> (variable : String) -> LogicPiece
  ExistsIntro  : (position : Pos) -> (variable : String) -> LogicPiece
  Wire         : (from : Pos) -> (to : Pos) -> LogicPiece

||| The "primary position" of a piece. Mirrors Rust `LogicPiece::position`,
||| including the rule that a Wire's position is its `from` endpoint.
public export
position : LogicPiece -> Pos
position (Assumption _ p)   = p
position (Goal _ p)         = p
position (AndIntro p)       = p
position (OrIntro p)        = p
position (ImpliesIntro p)   = p
position (NotIntro p)       = p
position (ForallIntro p _)  = p
position (ExistsIntro p _)  = p
position (Wire f _)         = f

||| True iff the piece is an assumption.
public export
isAssumption : LogicPiece -> Bool
isAssumption (Assumption _ _) = True
isAssumption _                = False

||| True iff the piece is a goal.
public export
isGoal : LogicPiece -> Bool
isGoal (Goal _ _) = True
isGoal _          = False

||| True iff the piece is an introduction gate (AND/OR/IMPLIES/NOT).
public export
isGate : LogicPiece -> Bool
isGate (AndIntro _)     = True
isGate (OrIntro _)      = True
isGate (ImpliesIntro _) = True
isGate (NotIntro _)     = True
isGate _                = False

--------------------------------------------------------------------------------
-- Board state (mirror of Rust struct BoardState, src/game/mod.rs)
--------------------------------------------------------------------------------

||| A puzzle board. `width`/`height` are the grid extents; a position
||| (x,y) is in-bounds iff x < width and y < height (Rust `in_bounds`).
public export
record BoardState where
  constructor MkBoardState
  width  : Nat
  height : Nat
  pieces : List LogicPiece

||| Position-in-bounds predicate. Mirrors Rust `BoardState::in_bounds`.
public export
inBounds : BoardState -> Pos -> Bool
inBounds b (MkPos px py) = px < b.width && py < b.height

||| Position-occupied predicate. Mirrors Rust `BoardState::is_occupied`:
||| a position is occupied iff some piece's primary position equals it.
public export
isOccupied : BoardState -> Pos -> Bool
isOccupied b q = any (\p => position p == q) b.pieces

--------------------------------------------------------------------------------
-- Goal condition + level (mirror of Rust, src/game/mod.rs)
--------------------------------------------------------------------------------

||| Win condition for a level. Constructor order matches Rust
||| `enum GoalCondition`.
public export
data GoalCondition : Type where
  ConnectNodes  : (start : Pos) -> (end : Pos) -> GoalCondition
  ProveFormula  : (formula : String) -> GoalCondition
  BuildProofTree : (depth : Nat) -> GoalCondition

||| A single level. Mirrors Rust `struct Level`.
public export
record Level where
  constructor MkLevel
  id           : Nat
  name         : String
  description  : String
  theorem      : String
  initialState : BoardState
  goalState    : GoalCondition

--------------------------------------------------------------------------------
-- Exported proof (mirror of Rust struct ExportedProof, src/verification/mod.rs)
--------------------------------------------------------------------------------

||| The proof artefact submitted to the network. Mirrors Rust
||| `struct ExportedProof`; `proofIsabelle` is the Rust `Option<String>`.
public export
record ExportedProof where
  constructor MkExportedProof
  levelId       : Nat
  playerId      : String
  proofSmt2     : String
  proofIsabelle : Maybe String
  solutionSteps : List String
  timeTakenSecs : Nat

--------------------------------------------------------------------------------
-- Pack difficulty (mirror of Rust struct LevelPack.difficulty: u8)
--------------------------------------------------------------------------------

||| Difficulty rating 1..5 (Rust `LevelPack::difficulty : u8`, doc-stated
||| range 1-5). The bound is a *stated* contract, not enforced by the
||| Rust type; see Invariants.idr `DifficultyInRange`.
public export
Difficulty : Type
Difficulty = Nat

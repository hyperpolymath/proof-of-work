||| SPDX-License-Identifier: PMPL-1.0-or-later
||| Proof-of-Work ABI: Consensus-/Correctness-Critical Invariants
|||
||| Type-level statements of the invariants that the Rust crate must
||| uphold at its trust boundaries. Each invariant is stated as an Idris2
||| type/proposition. Where the property is a discharged proof, it is
||| proven here (`Refl`/structural). Where it is a *hardness assumption*
||| (collision-resistance of SHA-256) or a *runtime obligation that the
||| Rust does not currently establish*, it is marked as a postulate /
||| assumption and tracked in PROOF-NEEDS.md.
|||
||| Naming: "consensus-critical" here means the invariants whose breach
||| silently corrupts game outcomes, leaderboard integrity, or solver
||| soundness — the proof-of-work analogue of chain-extension validity.
||| This is a single-player/leaderboard puzzle game, NOT a distributed
||| ledger; there is no nonce search or difficulty-retarget loop. The
||| audit's blockchain-style invariant names are mapped to their real
||| counterparts below.

module ProofOfWork.ABI.Invariants

import ProofOfWork.ABI.Types
import Data.List
import Data.List.Elem
import Data.List.Quantifiers
import Data.Nat
import Data.So
import Decidable.Equality

%default total

--------------------------------------------------------------------------------
-- I1. Verification soundness  (NO FALSE POSITIVES)
--
-- Source: src/verification/mod.rs `verify_level_solution`,
--         src/verification/z3_integration.rs `verify_formula`.
--
-- The game's core mechanic. `verify_level_solution` returns `true` ONLY
-- when an AND gate is adjacency-connected to assumptions P, Q AND a goal,
-- AND Z3 discharges `(P ∧ (P∧Q⇒R)) ⊢ R` by `¬R` being UNSAT.
--
-- The soundness statement: a positive verdict implies the SMT entailment
-- actually holds. We model the entailment abstractly as `Entails`.
--------------------------------------------------------------------------------

||| Abstract SMT entailment relation: `Entails asmpts goal` holds when the
||| goal is a logical consequence of the assumptions. The Rust delegates
||| this to Z3; here it is the proposition the seam requires Z3 to decide.
public export
data Entails : List String -> String -> Type where
  ||| Modus-ponens shape used by the level-1 verifier:
  ||| from P, Q and (P∧Q ⇒ R) conclude R.
  AndElimMP : Entails [p, q, "(=> (and " ++ p ++ " " ++ q ++ ") " ++ r ++ ")"] r

||| Adjacency, mirroring Rust `is_adjacent` in src/verification/mod.rs:
|||   dx<=2 && dy<=2 && (dx+dy)>0   (Chebyshev radius 2, excluding self).
public export
adjacent : Pos -> Pos -> Bool
adjacent (MkPos ax ay) (MkPos bx by) =
  let dx = if ax >= bx then minus ax bx else minus bx ax
      dy = if ay >= by then minus ay by else minus by ay in
  dx <= 2 && dy <= 2 && (dx + dy) > 0

||| I1 (soundness contract). A `VerifiedSolution` is a *certificate* that
||| the verifier's positive path was justified: there is an AND gate
||| adjacent to assumptions `p` and `q` and to a goal, and the SMT
||| entailment holds. The Rust `verify_level_solution == true` must imply
||| the existence of such a certificate. (Refinement obligation: the Rust
||| does not currently *return* this witness — see PROOF-NEEDS.md I1.)
public export
record VerifiedSolution (board : List LogicPiece) where
  constructor MkVerifiedSolution
  gatePos   : Pos
  pPos      : Pos
  qPos      : Pos
  goalPos   : Pos
  pFormula  : String
  qFormula  : String
  goalForm  : String
  gateOnBoard   : Elem (AndIntro gatePos) board
  pOnBoard      : Elem (Assumption pFormula pPos) board
  qOnBoard      : Elem (Assumption qFormula qPos) board
  goalOnBoard   : Elem (Goal goalForm goalPos) board
  pAdjacent     : So (adjacent pPos gatePos)
  qAdjacent     : So (adjacent qPos gatePos)
  goalAdjacent  : So (adjacent gatePos goalPos)
  smtEntails    : Entails [pFormula, qFormula,
                           "(=> (and " ++ pFormula ++ " " ++ qFormula
                             ++ ") " ++ goalForm ++ ")"] goalForm

--------------------------------------------------------------------------------
-- I2. Verifier determinism / mock-vs-Z3 agreement
--
-- Source: src/verification/mod.rs — there are TWO `verify_level_solution`
-- bodies: `#[cfg(feature="z3-verify")]` (real Z3) and
-- `#[cfg(not(...))]` (mock connectivity-only).
--
-- Invariant: the mock MUST NOT accept a configuration the Z3 path would
-- reject (else a no-Z3 build grants false wins). Currently the mock
-- accepts on connectivity ALONE, with no SMT check — this invariant is
-- KNOWN-VIOLATED by construction and is the highest-value defect.
--------------------------------------------------------------------------------

||| Predicate "connectivity holds for some AND gate" — exactly the mock's
||| acceptance condition (connectivity to P, Q, goal).
public export
data ConnOK : List LogicPiece -> Type where
  MkConnOK : (gate : Pos) -> ConnOK board

||| I2: soundness of the mock relative to Z3. The mock accepting must
||| imply Z3 would accept. This is stated as the proposition the seam
||| OWES; it is NOT proven (the mock has no SMT step). Postulated so the
||| obligation is explicit and discoverable.
public export
0 mockNoStrongerThanZ3 :
  (board : List LogicPiece) -> ConnOK board -> VerifiedSolution board

--------------------------------------------------------------------------------
-- I3. Board well-formedness preservation under placement
--
-- Source: src/game/board.rs `place_piece`, `move_piece`;
--         src/game/validation.rs `validate_piece_placement`.
--
-- `place_piece` returns true ONLY if in-bounds AND unoccupied; on true it
-- pushes the piece. Invariant: a placement that returns true preserves
-- "all pieces in bounds & no two pieces share a primary position".
--------------------------------------------------------------------------------

||| All pieces are within board bounds.
|||
||| Defined by structural recursion rather than Prelude `all`: Prelude
||| `all`/`any` are `foldl`/Monoid-based and do NOT reduce on `::`, which
||| blocks the I3 discharge below. This form is extensionally identical
||| (and a more faithful model of the Rust `for piece in &self.pieces`
||| loop), and reduces definitionally on the cons that `placePiece` adds.
public export
allInBoundsGo : BoardState -> List LogicPiece -> Bool
allInBoundsGo b []        = True
allInBoundsGo b (p :: ps) = inBounds b (position p) && allInBoundsGo b ps

public export
allInBounds : BoardState -> Bool
allInBounds b = allInBoundsGo b b.pieces

||| No two pieces share a primary position (mirrors the overlap rule in
||| validate_board / is_occupied).
public export
noOverlap : List LogicPiece -> Bool
noOverlap [] = True
noOverlap (p :: ps) =
  not (any (\q => position q == position p) ps) && noOverlap ps

||| Well-formed board.
public export
WellFormed : BoardState -> Type
WellFormed b = (So (allInBounds b), So (noOverlap b.pieces))

||| The guarded placement: returns the new board only when the Rust
||| guard (`in_bounds && !is_occupied`) passes — exactly mirroring
||| `BoardState::place_piece`.
public export
placePiece : BoardState -> LogicPiece -> Maybe BoardState
placePiece b pc =
  if inBounds b (position pc) && not (isOccupied b (position pc))
    then Just (MkBoardState b.width b.height (pc :: b.pieces))
    else Nothing

||| Just is injective (local; Prelude does not export `justInjective`).
justInj : {0 x, y : a} -> the (Maybe a) (Just x) = Just y -> x = y
justInj Refl = Refl

||| `inBounds` depends on the board only through width/height, so two
||| boards agreeing on those agree on `inBounds` at every position.
inBoundsCong :
  (b1, b2 : BoardState) ->
  b1.width = b2.width -> b1.height = b2.height ->
  (q : Pos) -> inBounds b1 q = inBounds b2 q
inBoundsCong b1 b2 wEq hEq (MkPos px py) =
  rewrite wEq in rewrite hEq in Refl

||| Hence `allInBoundsGo` is irrelevant to any board change that
||| preserves width/height (needed because it cannot reduce over the
||| abstract `b.pieces`, so `allInBoundsGo b ys` and
||| `allInBoundsGo b' ys` are not definitionally equal even when b/b'
||| share width/height).
allInBoundsGoCong :
  (b1, b2 : BoardState) ->
  b1.width = b2.width -> b1.height = b2.height ->
  (ys : List LogicPiece) ->
  allInBoundsGo b1 ys = allInBoundsGo b2 ys
allInBoundsGoCong b1 b2 wEq hEq [] = Refl
allInBoundsGoCong b1 b2 wEq hEq (p :: ps) =
  rewrite inBoundsCong b1 b2 wEq hEq (position p) in
  rewrite allInBoundsGoCong b1 b2 wEq hEq ps in Refl

||| Cons-introduction for `allInBoundsGo` (it reduces on `::` to
||| `head && tail`; this lemma packages the lazy-`&&` `So` plumbing with
||| concrete types so the call site stays metavariable-free).
allInBoundsCons :
  (bd : BoardState) -> (x : LogicPiece) -> (xs : List LogicPiece) ->
  So (inBounds bd (position x)) -> So (allInBoundsGo bd xs) ->
  So (allInBoundsGo bd (x :: xs))
allInBoundsCons bd x xs hx hxs = andSo (hx, hxs)

||| Cons-introduction for `noOverlap` (same rationale). The head
||| hypothesis is exactly `not (isOccupied bd (position x))` up to alpha
||| (`isOccupied bd q = any (\p => position p == q) bd.pieces`).
noOverlapCons :
  (x : LogicPiece) -> (xs : List LogicPiece) ->
  So (not (any (\q => position q == position x) xs)) ->
  So (noOverlap xs) ->
  So (noOverlap (x :: xs))
noOverlapCons x xs hh ht = andSo (hh, ht)

||| `Nothing = Just _` is uninhabited (local helper for the failed-guard
||| branch of the I3 discharge).
nothingNotJust : {0 x : a} -> the (Maybe a) Nothing = Just x -> Void
nothingNotJust Refl impossible

||| I3 (DISCHARGED). If the input board is well-formed and `placePiece`
||| succeeds, the resulting board is still well-formed. Genuine theorem,
||| now machine-checked (no postulate, no escape): `placePiece` unfolds
||| definitionally to its `if`; inverting the guard gives
||| `So (inBounds b (position pc))` and
||| `So (not (isOccupied b (position pc)))`, and the success branch fixes
||| `b' = MkBoardState b.width b.height (pc :: b.pieces)`. The new head
||| discharges both conjuncts; the tail (`allInBoundsGo`/`noOverlap` over
||| `b.pieces`) is unchanged because `b'` shares `b`'s width/height, so
||| `inBounds b' = inBounds b` definitionally. `noOverlap`'s head check
||| is, up to alpha, exactly `not (isOccupied b (position pc))`.
public export
placePreservesWF :
  (b : BoardState) -> WellFormed b ->
  (pc : LogicPiece) -> (b' : BoardState) ->
  placePiece b pc = Just b' -> WellFormed b'
placePreservesWF b (wfA, wfN) pc b' prf
    with (inBounds b (position pc) && not (isOccupied b (position pc)))
         proof gpf
  placePreservesWF b (wfA, wfN) pc b' prf | True =
    let soPair : ( So (inBounds b (position pc))
                 , So (not (isOccupied b (position pc))) )
        soPair = soAnd (eqToSo gpf)
        bEq : b' = MkBoardState b.width b.height (pc :: b.pieces)
        bEq = sym (justInj prf)
        aibB : So (allInBoundsGo b (pc :: b.pieces))
        aibB = allInBoundsCons b pc b.pieces (fst soPair) wfA
        aib : So (allInBoundsGo (MkBoardState b.width b.height (pc :: b.pieces))
                                (pc :: b.pieces))
        aib = rewrite sym (allInBoundsGoCong b
                             (MkBoardState b.width b.height (pc :: b.pieces))
                             Refl Refl (pc :: b.pieces)) in aibB
        nov : So (noOverlap (pc :: b.pieces))
        nov = noOverlapCons pc b.pieces (snd soPair) wfN
        wf' : WellFormed (MkBoardState b.width b.height (pc :: b.pieces))
        wf' = (aib, nov)
    in rewrite bEq in wf'
  placePreservesWF b (wfA, wfN) pc b' prf | False = void (nothingNotJust prf)

--------------------------------------------------------------------------------
-- I4. Level-pack solvability  (NO UNSOLVABLE GENERATED PUZZLES)
--
-- Source: PROOF-NEEDS.md target; src/levels/mod.rs builtin pack +
--         src/game/validation.rs `is_ready_for_verification`.
--
-- Invariant: every level shipped/generated admits at least one piece
-- configuration the verifier accepts. Stated existentially over the
-- (abstract) verifier; this is an OWED proof for the generator, and for
-- the builtin pack it is the concrete obligation that levels 1-4 solve.
--------------------------------------------------------------------------------

||| There exists a solution piece-list the verifier accepts for `lvl`.
public export
Solvable : Level -> Type
Solvable lvl = (sol : List LogicPiece ** VerifiedSolution sol)

||| I4: every level in a shipped pack is solvable. OWED (no solver-side
||| existence proof exists in the Rust today).
public export
0 packLevelsSolvable : (lvls : List Level) -> All Solvable lvls

--------------------------------------------------------------------------------
-- I5. Difficulty monotonicity  (LEVEL PROGRESSION)
--
-- Source: PROOF-NEEDS.md; src/levels/mod.rs `LevelPack.difficulty : u8`
--         (doc-stated 1..5) and `next_level` ordering.
--
-- This is the real counterpart of the audit's "difficulty target
-- monotonicity". Invariant: within an ordered pack, difficulty is
-- non-decreasing, and each level's difficulty is in [1,5].
--------------------------------------------------------------------------------

||| Difficulty bound (Rust doc contract, NOT enforced by `u8`).
public export
DifficultyInRange : Difficulty -> Type
DifficultyInRange d = (So (d >= 1), So (d <= 5))

||| Non-decreasing difficulty across a level sequence.
public export
data NonDecreasing : List Difficulty -> Type where
  NDNil  : NonDecreasing []
  NDOne  : NonDecreasing [d]
  NDCons : LTE d e -> NonDecreasing (e :: rest) -> NonDecreasing (d :: e :: rest)

||| I5: a shipped pack's per-level difficulty sequence is non-decreasing.
||| NOTE: `(ds : List Difficulty) -> NonDecreasing ds` is FALSE as a
||| universal (an arbitrary list is not sorted); it is retained, erased,
||| only as the explicit statement of the runtime obligation the Rust
||| does not yet discharge (`next_level` is index-only). The genuine,
||| machine-checked artifact is `decNonDecreasing` below: a total decision
||| procedure the Rust pack-loader MUST call to establish the property for
||| a concrete pack. This converts an unprovable blanket postulate into a
||| verified validator + a concrete obligation. See PROOF-NEEDS.md I5.
public export
0 progressionMonotone : (ds : List Difficulty) -> NonDecreasing ds

||| Decidable check that a difficulty sequence is non-decreasing.
||| Total and constructive — this is the discharged counterpart of I5.
public export
decNonDecreasing : (ds : List Difficulty) -> Dec (NonDecreasing ds)
decNonDecreasing [] = Yes NDNil
decNonDecreasing (d :: []) = Yes NDOne
decNonDecreasing (d :: e :: rest) =
  case decNonDecreasing (e :: rest) of
    No notRest => No (\nd => case nd of NDCons _ r => notRest r)
    Yes ndRest => case isLTE d e of
      Yes le    => Yes (NDCons le ndRest)
      No  notLE => No (\nd => case nd of NDCons le _ => notLE le)

||| Concrete witness: the documented builtin difficulty progression
||| [1,2,3,4,5] is non-decreasing — a machine-checked instance of the
||| obligation the loader must re-establish per shipped pack.
public export
builtinPackMonotone : NonDecreasing [1,2,3,4,5]
builtinPackMonotone =
  NDCons (LTESucc LTEZero)
  (NDCons (LTESucc (LTESucc LTEZero))
  (NDCons (LTESucc (LTESucc (LTESucc LTEZero)))
  (NDCons (LTESucc (LTESucc (LTESucc (LTESucc LTEZero))))
  NDOne)))

--------------------------------------------------------------------------------
-- I6. Submission-signature binding  (LEADERBOARD INTEGRITY)
--
-- Source: src/network/client.rs `sign_proof`:
--   sig = SHA256( serde_json(proof) || api_key )
--
-- This is the real counterpart of the audit's "hash collision-resistance
-- assumption". The signature binds the proof bytes to the player's key.
-- Soundness rests on SHA-256 being collision/2nd-preimage resistant —
-- a STATED HARDNESS ASSUMPTION, not a theorem (see PROOF-NEEDS.md I6).
-- Also note: this is a keyed *hash*, not a MAC/signature; key recovery
-- or length-extension concerns are out of seam scope but flagged.
--------------------------------------------------------------------------------

||| Abstract digest type and the keyed-hash function the Rust computes.
public export
data Digest : Type where
  MkDigest : String -> Digest

public export
DecEq Digest where
  decEq (MkDigest a) (MkDigest b) with (decEq a b)
    decEq (MkDigest a) (MkDigest a) | Yes Refl = Yes Refl
    decEq (MkDigest a) (MkDigest b) | No ne    = No (\Refl => ne Refl)

||| The serialisation+keying the Rust performs (modelled abstractly).
public export
0 sha256 : String -> Digest

||| Collision-resistance, stated as an ASSUMPTION (postulate). The seam
||| exports this so any SPARK/Ada side, and any reviewer, sees that
||| leaderboard integrity is conditional on it. It is intentionally NOT
||| provable in Idris2 — it is a cryptographic hardness assumption.
public export
0 sha256CollisionResistant :
  (m1, m2 : String) -> sha256 m1 = sha256 m2 -> m1 = m2

||| I6 (conditional soundness). Under `sha256CollisionResistant`, two
||| accepted submissions with equal signatures had equal signed payloads.
public export
0 signatureBindsPayload :
  (p1, p2 : String) ->
  sha256 p1 = sha256 p2 -> p1 = p2
signatureBindsPayload = sha256CollisionResistant

--------------------------------------------------------------------------------
-- I7. Level-pack round-trip integrity
--
-- Source: src/levels/mod.rs `LevelPack::save`/`load` (serde_json).
--
-- Invariant: load . save = id on well-formed packs (no silent corruption
-- of community level packs across the disk boundary). OWED — serde is
-- assumed correct; stated so a future property/SPARK proof has a target.
--------------------------------------------------------------------------------

||| Abstract serialise/deserialise pair for a Level.
public export
0 serialise : Level -> String
public export
0 deserialise : String -> Maybe Level

||| I7: round-trip identity. OWED.
public export
0 levelRoundTrip : (l : Level) -> deserialise (serialise l) = Just l

||| SPDX-License-Identifier: PMPL-1.0-or-later
||| Proof-of-Work ABI: FFI Boundary Obligations
|||
||| The Zig-FFI side of the hyperpolymath ABI/FFI standard. The Rust crate
||| uses `(u32, u32)` for positions and `u8` for difficulty; the Idris2
||| model in Types.idr uses Nat. This module states the range obligations
||| that any C/Zig boundary must discharge when marshalling between the
||| Rust representation and the typed model, plus the C-ABI result code
||| the verifier exposes.
|||
||| Per LANGUAGE-POLICY.adoc §Terminology and §"Rust is never the ABI":
||| the boundary is Idris2 (this seam) + Zig (FFI). Rust/SPARK is the
||| application logic on one side; a future SPARK/Ada verifier could sit
||| on the other side of THIS contract without changing it.

module ProofOfWork.ABI.Foreign

import ProofOfWork.ABI.Types
import ProofOfWork.ABI.Invariants
import Data.Nat
import Data.So

%default total

--------------------------------------------------------------------------------
-- Fixed-width range obligations
--
-- Bounds are `Integer`, not `Nat`. A `Nat` constant of 2^32 forces the
-- Idris2 0.8.0 data-constructor elaborator to reduce the value when it
-- appears under `LT n u32Max` inside `MkInU32`'s type, which expands the
-- literal to ~4×10^9 `S` constructors and hangs `--build`. The boundary
-- predicate is therefore expressed as `So (natToInteger n < u32Max)`,
-- which is propositionally equivalent (the Rust `u32` round-trips to
-- precisely the Nats with `natToInteger n < 2^32`) but does not trigger
-- unary expansion. Do not rewrite back to `LT n u32Max`.
--------------------------------------------------------------------------------

||| u32 upper bound (2^32). Position coordinates crossing the FFI must
||| fit; the Rust type guarantees this, the Idris2 Nat model does not, so
||| the obligation is made explicit here.
public export
u32Max : Integer
u32Max = 4294967296

||| Proof-carrying "this Nat fits in a u32".
public export
data InU32 : Nat -> Type where
  MkInU32 : (n : Nat) -> So (natToInteger n < Foreign.u32Max) -> InU32 n

||| u8 upper bound (2^8) — for `LevelPack.difficulty`.
public export
u8Max : Integer
u8Max = 256

public export
data InU8 : Nat -> Type where
  MkInU8 : (n : Nat) -> So (natToInteger n < Foreign.u8Max) -> InU8 n

||| A position is FFI-marshalable iff both coordinates fit in u32.
public export
MarshalablePos : Pos -> Type
MarshalablePos (MkPos x y) = (InU32 x, InU32 y)

--------------------------------------------------------------------------------
-- C-ABI result codes for the verifier entry point
--------------------------------------------------------------------------------

||| Stable C integer result for a verification call across the FFI.
||| Constructor order is ABI-significant.
public export
data VerifyResult : Type where
  VerifyAccepted   : VerifyResult   -- Z3/mock accepted (proof discharged)
  VerifyRejected   : VerifyResult   -- no accepting configuration
  VerifyUnknown    : VerifyResult   -- solver timeout / Unknown
  VerifyMalformed  : VerifyResult   -- board failed pre-validation

public export
verifyResultToInt : VerifyResult -> Nat
verifyResultToInt VerifyAccepted  = 0
verifyResultToInt VerifyRejected  = 1
verifyResultToInt VerifyUnknown   = 2
verifyResultToInt VerifyMalformed = 3

public export
verifyResultFromInt : Nat -> Maybe VerifyResult
verifyResultFromInt 0 = Just VerifyAccepted
verifyResultFromInt 1 = Just VerifyRejected
verifyResultFromInt 2 = Just VerifyUnknown
verifyResultFromInt 3 = Just VerifyMalformed
verifyResultFromInt _ = Nothing

||| Round-trip proof for the result-code mapping (the one property fully
||| dischargeable here, by case analysis).
public export
verifyResultRoundTrip :
  (r : VerifyResult) -> verifyResultFromInt (verifyResultToInt r) = Just r
verifyResultRoundTrip VerifyAccepted  = Refl
verifyResultRoundTrip VerifyRejected  = Refl
verifyResultRoundTrip VerifyUnknown   = Refl
verifyResultRoundTrip VerifyMalformed = Refl

--------------------------------------------------------------------------------
-- The FFI contract the Rust verifier must honour
--------------------------------------------------------------------------------

||| The boundary obligation: a `VerifyAccepted` result handed back across
||| the FFI must be backed by an I1 `VerifiedSolution` certificate for the
||| submitted board. This ties the C-ABI return value to the Idris2
||| soundness statement. The Rust does not yet *produce* the witness
||| (PROOF-NEEDS.md I1); the type records exactly what is owed.
public export
0 acceptedImpliesVerified :
  (board : List LogicPiece) ->
  (r : VerifyResult) ->
  r = VerifyAccepted ->
  VerifiedSolution board
